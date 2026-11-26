//! Everything an island asks for: buildings, one stock goal, the blueprint, the
//! deliveries and the richer goals that use citizens, workshops, visitors, trade
//! and the island's twist. Each goal can explain itself in one short line.

use serde::{Deserialize, Serialize};

use super::{
    Build, Good, Island, Level, Quest, Twist, encounters::Event, levels::Requirement,
    simulation::load_goods,
};
use crate::model::{Building, MAX_LEVEL};

/// Counts that only ever grow. The richer goals read them, so a full store or a
/// delivery never takes progress away.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tally {
    /// Units of each good the workshops have made.
    #[serde(default, deserialize_with = "load_goods")]
    pub made: [u16; Good::ALL.len()],
    /// Units Nili and the citizens took from the island's nodes.
    #[serde(default, deserialize_with = "load_goods")]
    pub gathered: [u16; Good::ALL.len()],
    /// Units taken from the rich ore vein.
    #[serde(default)]
    pub vein: u16,
    /// The most different workshops that ever worked at the same moment.
    #[serde(default)]
    pub busy: u8,
    /// Visitors Nili helped to the end.
    #[serde(default)]
    pub helped: u8,
    /// One bit for each kind of [`Event`] that was solved.
    #[serde(default)]
    pub events: u8,
}

impl Tally {
    pub fn solved(&self, event: Event) -> bool {
        self.events & 1 << event as u8 != 0
    }
}

/// A goal that asks for more than a building or a stock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Challenge {
    /// So many citizens live on the island.
    Citizens(u16),
    /// So many different workshops work at the same moment.
    Busy(u8),
    /// The workshops make each of these goods, `n` units each.
    Chain([Good; 3], u16),
    /// So many visitors were helped.
    Visitors(u8),
    /// A visitor with this event was helped.
    Event(Event),
    /// So many trade ships came back from the neighbour harbour.
    Crossings(u16),
    /// So many units came out of the rich ore vein.
    Vein(u16),
    /// So many units were collected from the island's nodes of one good.
    Harvest(Good, u16),
}

impl Challenge {
    fn same_kind(self, other: Self) -> bool {
        std::mem::discriminant(&self) == std::mem::discriminant(&other)
    }
}

/// One row of the island's goal list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Objective {
    Build(Requirement),
    Stock(Good, u16),
    Plan,
    Orders(u8),
    Masterwork,
    Challenge(Challenge),
}

/// How far one goal has come.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    pub have: u16,
    pub need: u16,
    /// False for goals that are simply open or done, like the blueprint.
    pub counted: bool,
}

impl Progress {
    fn counted(have: u16, need: u16) -> Self {
        Self {
            have,
            need,
            counted: true,
        }
    }

    fn flag(done: bool) -> Self {
        Self {
            have: u16::from(done),
            need: 1,
            counted: false,
        }
    }

    pub fn done(self) -> bool {
        self.have >= self.need
    }

    /// The figure shown next to the goal, like "3/5". Empty for yes-or-no goals.
    pub fn figure(self) -> String {
        if self.counted {
            format!("{}/{}", self.have.min(self.need), self.need)
        } else {
            String::new()
        }
    }
}

impl Level {
    /// Citizens one plain home of this region houses.
    fn home_staff(self) -> u16 {
        Building::new(self.home()).residents().saturating_sub(1)
    }

    /// The richer goals of this island. The first island of a region has none. Every
    /// later one has a goal for its twist; the second and third add their first
    /// visitor, from the fourth on a citizen goal joins, and from the fifth on one more
    /// goal that changes from island to island.
    pub fn challenges(self) -> Vec<Challenge> {
        use Challenge as C;
        use Good::*;
        if self.stage == 0 {
            return Vec::new();
        }
        let stage = self.stage as u16;
        let mut list = Vec::new();
        if self.stage >= 3 {
            let homes = 1 + stage / 3;
            list.push(C::Citizens(self.home_staff() * homes + stage / 3));
        }
        let twist = self.twist().map(|twist| match twist {
            Twist::RichWoods => C::Harvest(Wood, 30 + stage * 4),
            Twist::FertileSoil => C::Harvest(Food, 20 + stage * 4),
            Twist::OreVein => C::Vein(18 + stage * 2),
            Twist::BusyHands => C::Busy(if stage < 3 { 2 } else { 3 }),
            Twist::FestiveMood => C::Event(Event::Festival),
            Twist::FairWind => C::Crossings(3),
        });
        list.extend(twist);
        // The island's first visitor, who arrives soon after the first building.
        let first = self.encounter_in(0, 0).map(|spec| C::Event(spec.event));
        let signature = self.signature();
        let options = match self.stage {
            1 | 2 => vec![first],
            4 => vec![Some(C::Visitors(2)), first],
            5 => vec![Some(C::Chain([Boards, Tools, Meals], 5)), Some(C::Busy(3))],
            6 => vec![first, Some(C::Visitors(2))],
            7 => vec![Some(C::Busy(3)), Some(C::Chain([Boards, Bricks, Meals], 6))],
            8 => vec![Some(C::Crossings(2)), Some(C::Visitors(2))],
            9 => vec![
                Some(C::Chain([Bricks, Tools, signature], 6)),
                Some(C::Busy(3)),
            ],
            10 => vec![Some(C::Visitors(3)), Some(C::Crossings(2))],
            11 => vec![
                Some(C::Busy(3)),
                Some(C::Chain([Tools, Meals, signature], 8)),
            ],
            _ => Vec::new(),
        };
        list.extend(
            options
                .into_iter()
                .flatten()
                .find(|option| twist.is_none_or(|twist| !twist.same_kind(*option))),
        );
        list
    }

    /// Every row of the island's goal list, in the order the list shows them. Where a
    /// citizen goal asks for people, it stands in for the row of the first home.
    pub fn objectives(self) -> Vec<Objective> {
        let (good, count) = self.reserve();
        let challenges = self.challenges();
        let people = challenges
            .iter()
            .any(|challenge| matches!(challenge, Challenge::Citizens(_)));
        let home = Build::Harbor(self.home());
        self.goals()
            .filter(|goal| !(people && goal.kind == home))
            .map(Objective::Build)
            .chain(challenges.into_iter().map(Objective::Challenge))
            .chain([Objective::Stock(good, count)])
            .chain(self.plan().map(|_| Objective::Plan))
            .chain((self.order_count() > 0).then_some(Objective::Orders(self.order_count())))
            .chain(self.work_order().map(|_| Objective::Masterwork))
            .collect()
    }
}

impl Objective {
    /// A short word for the kind of goal, used to count the goal mix.
    #[cfg(test)]
    pub fn family(self) -> &'static str {
        match self {
            Self::Build(goal) if goal.tier > 1 => "Ausbauen",
            Self::Build(_) => "Bauen",
            Self::Stock(..) => "Vorrat",
            Self::Plan => "Bauplan",
            Self::Orders(_) => "Lieferungen",
            Self::Masterwork => "Meisterauftrag",
            Self::Challenge(challenge) => match challenge {
                Challenge::Citizens(_) => "Bürger",
                Challenge::Busy(_) => "Gleichzeitig",
                Challenge::Chain(..) => "Warenkette",
                Challenge::Visitors(_) => "Besuch",
                Challenge::Event(_) => "Ereignis",
                Challenge::Crossings(_) => "Handel",
                Challenge::Vein(_) => "Erzader",
                Challenge::Harvest(..) => "Ernte",
            },
        }
    }

    /// True for the plain goals: buildings and the stock.
    #[cfg(test)]
    pub fn plain(self) -> bool {
        matches!(self, Self::Build(_) | Self::Stock(..))
    }

    pub fn title(self, level: Level) -> String {
        let region = level.region;
        match self {
            Self::Build(goal) => {
                let name = goal.kind.name(region);
                if goal.tier > 1 {
                    format!("{name} · Stufe {}", goal.tier)
                } else {
                    name.to_owned()
                }
            }
            Self::Stock(good, _) => format!("Vorrat: {}", good.name(region)),
            Self::Plan => "Bauplan finden".to_owned(),
            Self::Orders(_) => "Lieferungen zum Außenposten".to_owned(),
            Self::Masterwork => "Meisterauftrag".to_owned(),
            Self::Challenge(challenge) => match challenge {
                Challenge::Citizens(_) => "Bürger auf der Insel".to_owned(),
                Challenge::Busy(n) => format!("{n} Werkstätten gleichzeitig"),
                Challenge::Chain(goods, _) => format!(
                    "Herstellen: {}, {}, {}",
                    goods[0].name(region),
                    goods[1].name(region),
                    goods[2].name(region)
                ),
                Challenge::Visitors(_) => "Besuchern helfen".to_owned(),
                Challenge::Event(event) => event.goal().to_owned(),
                Challenge::Crossings(_) => {
                    format!("Handelsfahrten nach {}", level.trading_neighbor())
                }
                Challenge::Vein(_) => format!("{} aus der Erzader", Good::Ore.name(region)),
                Challenge::Harvest(Good::Wood, _) => {
                    format!("{} aus den Wäldern", Good::Wood.name(region))
                }
                Challenge::Harvest(good, _) => format!("{} ernten", good.name(region)),
            },
        }
    }

    /// The icon of the goal. Building goals use the building's own picture instead.
    pub fn icon(self, level: Level, island: &Island) -> &'static str {
        let region = level.region;
        match self {
            Self::Build(_) => "hammer",
            Self::Stock(good, _) => good.icon(region),
            Self::Plan => "adventure_blueprint",
            Self::Orders(_) => "storehouse",
            Self::Masterwork => Good::Masterwork.icon(region),
            Self::Challenge(challenge) => match challenge {
                Challenge::Citizens(_) => "age_adult",
                Challenge::Busy(_) => "adventure_labor",
                Challenge::Chain(goods, n) => goods
                    .into_iter()
                    .find(|&good| island.tally.made[good as usize] < n)
                    .unwrap_or(goods[0])
                    .icon(region),
                Challenge::Visitors(_) => "star",
                Challenge::Event(event) => event.icon(),
                Challenge::Crossings(_) => "sea_trade",
                Challenge::Vein(_) => Good::Ore.icon(region),
                Challenge::Harvest(good, _) => good.icon(region),
            },
        }
    }

    /// One line on how to reach the goal, for the goal list and the advice.
    pub fn hint(self, level: Level, island: &Island) -> String {
        let region = level.region;
        let name = |good: Good| good.name(region);
        match self {
            Self::Build(goal) => build_hint(level, island, goal),
            Self::Stock(good, count) => match maker(level, good) {
                Some(maker) if !Good::RAW.contains(&good) => format!(
                    "Halte {count} {} im Lager bereit. {} gibt es in „{}“.",
                    name(good),
                    name(good),
                    maker.name(region)
                ),
                _ => format!(
                    "Halte {count} {} im Lager bereit. Sammle es auf der Insel.",
                    name(good)
                ),
            },
            Self::Plan => match island.quest {
                Quest::Unheard => format!(
                    "Geh zu „{}“. Dort erfährst du, wie du den Bauplan bekommst.",
                    level.landmark
                ),
                Quest::Searching => format!("Hol „{}“ vom markierten Hang.", level.relic),
                Quest::Carrying => {
                    format!("Bring „{}“ zu „{}“.", level.relic, level.landmark)
                }
                Quest::Offering => level.offering().map_or_else(String::new, |(good, n)| {
                    format!("Gib {n} {} bei „{}“ ab.", name(good), level.landmark)
                }),
                Quest::Finished => "Der Bauplan liegt in deinem Baumenü.".to_owned(),
            },
            Self::Orders(_) => {
                "Pack die Waren im Rathaus und bring sie zum Außenposten.".to_owned()
            }
            Self::Masterwork => level.work_order().map_or_else(String::new, |order| {
                format!(
                    "Stell „{}“ auf {} um und liefere sie zum Außenposten.",
                    order.workshop.name(region),
                    name(Good::Masterwork)
                )
            }),
            Self::Challenge(challenge) => match challenge {
                Challenge::Citizens(_) => {
                    "Baue neue Wohnhäuser oder baue sie aus. Jede Stufe bringt einen Bürger."
                        .to_owned()
                }
                Challenge::Busy(n) => {
                    format!("Gib {n} verschiedenen Werkstätten je einen Bürger und genug Waren.")
                }
                Challenge::Chain(goods, n) => {
                    let open: Vec<String> = goods
                        .into_iter()
                        .filter(|&good| island.tally.made[good as usize] < n)
                        .map(|good| {
                            format!("{} {}/{n}", name(good), island.tally.made[good as usize])
                        })
                        .collect();
                    if open.is_empty() {
                        "Alles hergestellt!".to_owned()
                    } else {
                        format!("Aus deinen Werkstätten: {}.", open.join(", "))
                    }
                }
                Challenge::Visitors(_) => {
                    "Ein Stern zeigt Besuch. Sprich mit der Person und bring, was sie braucht."
                        .to_owned()
                }
                Challenge::Event(event) => {
                    let here = |wanted: Option<Event>| {
                        (0..level.encounter_count()).any(|slot| {
                            island.encounter(level, slot).is_some_and(|spec| {
                                wanted.is_none_or(|event| spec.event == event)
                                    && !matches!(
                                        island.encounters.visits[slot].status,
                                        super::EncounterStatus::Hidden
                                            | super::EncounterStatus::Finished
                                    )
                            })
                        })
                    };
                    if !here(Some(event)) && here(None) {
                        "Hilf erst dem Besuch, der schon da ist. Danach kommt der Besuch für dieses Ziel."
                            .to_owned()
                    } else {
                        event.how().to_owned()
                    }
                }
                Challenge::Crossings(_) => {
                    level
                        .export(super::Export::Regular)
                        .map_or_else(String::new, |offer| {
                            format!(
                                "Pack {} {} als Handelsfracht und bring sie zum Handelshafen.",
                                offer.units,
                                name(offer.good)
                            )
                        })
                }
                Challenge::Vein(_) => {
                    "Die goldene Ader glänzt in der Mine. Bau dort ab oder schick Bürger hin."
                        .to_owned()
                }
                Challenge::Harvest(Good::Wood, _) => {
                    "Fälle Bäume im Wald oder schick Bürger zum Holzfällen.".to_owned()
                }
                Challenge::Harvest(good, _) => format!(
                    "Sammle {} auf den Feldern oder schick Bürger zum Sammeln.",
                    name(good)
                ),
            },
        }
    }
}

/// The cheapest building of the island that makes `good`.
fn maker(level: Level, good: Good) -> Option<Build> {
    Build::all(level)
        .into_iter()
        .filter(|plan| {
            plan.recipe(1, level.region)
                .is_some_and(|recipe| recipe.output.0 == good)
        })
        .min_by_key(|plan| plan.cost(1).iter().map(|&(_, units)| units).sum::<u16>())
}

fn build_hint(level: Level, island: &Island, goal: Requirement) -> String {
    let region = level.region;
    let name = goal.kind.name(region);
    if goal.kind == Build::Monument && !island.market.blueprint {
        return format!("Kauf den Sonderbauplan im Handelshafen und baue „{name}“.");
    }
    if level.plan() == Some(goal.kind) && island.locked(level, goal.kind, goal.tier) {
        return format!(
            "Den Bauplan für „{name}“ bekommst du bei „{}“.",
            level.landmark
        );
    }
    let built = island.lots.iter().flatten().any(|p| p.kind == goal.kind);
    if goal.tier > 1 {
        return if built {
            format!("Geh zu „{name}“ und baue es auf Stufe {} aus.", goal.tier)
        } else {
            format!(
                "Plane „{name}“ und baue es dann auf Stufe {} aus.",
                goal.tier
            )
        };
    }
    let purpose = match goal.kind {
        Build::Harbor(kind) if Building::new(kind).residents() > 0 => {
            " Dort wohnen Bürger.".to_owned()
        }
        Build::Harbor(crate::model::Kind::Storehouse) => {
            " Mehr Platz im Lager, Nili trägt mehr.".to_owned()
        }
        kind => kind.recipe(1, region).map_or_else(String::new, |recipe| {
            format!(" Dort gibt es {}.", recipe.output.0.name(region))
        }),
    };
    format!("Plane „{name}“ auf einem freien Bauplatz.{purpose}")
}

impl Island {
    /// How far the island has come with one goal.
    pub fn progress_of(&self, level: Level, objective: Objective) -> Progress {
        match objective {
            Objective::Build(goal) => {
                Progress::counted(u16::from(self.progress(goal)), u16::from(goal.count))
            }
            Objective::Stock(good, count) => Progress::counted(self.amount(good), count),
            Objective::Plan => Progress::flag(self.quest == Quest::Finished),
            Objective::Orders(count) => {
                Progress::counted(u16::from(self.orders_delivered), u16::from(count))
            }
            Objective::Masterwork => {
                Progress::flag(level.work_order_delivered(self.orders_delivered))
            }
            Objective::Challenge(challenge) => self.challenge_progress(challenge),
        }
    }

    fn challenge_progress(&self, challenge: Challenge) -> Progress {
        let tally = &self.tally;
        match challenge {
            Challenge::Citizens(n) => Progress::counted(self.staff(), n),
            Challenge::Busy(n) => Progress::counted(u16::from(tally.busy), u16::from(n)),
            Challenge::Chain(goods, n) => Progress::counted(
                goods
                    .into_iter()
                    .filter(|&good| tally.made[good as usize] >= n)
                    .count() as u16,
                goods.len() as u16,
            ),
            Challenge::Visitors(n) => Progress::counted(u16::from(tally.helped), u16::from(n)),
            Challenge::Event(event) => Progress::flag(tally.solved(event)),
            Challenge::Crossings(n) => Progress::counted(self.market.crossings, n),
            Challenge::Vein(n) => Progress::counted(tally.vein, n),
            Challenge::Harvest(good, n) => Progress::counted(tally.gathered[good as usize], n),
        }
    }

    pub fn reached(&self, level: Level, objective: Objective) -> bool {
        self.progress_of(level, objective).done()
    }

    /// Notes how many different workshops work right now, for [`Challenge::Busy`].
    pub(super) fn track_busy(&mut self, level: Level) {
        let mut kinds = Vec::new();
        for lot in 0..self.lots.len() {
            let Some(property) = self.lots[lot] else {
                continue;
            };
            if self.batch(lot).is_some_and(|batch| batch.working())
                && self
                    .recipe_for(level, lot)
                    .is_some_and(|recipe| !recipe.need.is_empty())
                && !kinds.contains(&property.kind)
            {
                kinds.push(property.kind);
            }
        }
        self.tally.busy = self.tally.busy.max(kinds.len() as u8);
    }

    /// The home with the lowest tier that can still grow.
    pub fn smallest_home(&self, level: Level) -> Option<usize> {
        let home = Build::Harbor(level.home());
        (0..self.lots.len())
            .filter(|&lot| {
                self.lots[lot].is_some_and(|p| p.kind == home && p.tier < MAX_LEVEL)
                    && self.site(lot).is_none()
            })
            .min_by_key(|&lot| self.lots[lot].map_or(0, |p| p.tier))
    }
}

impl Event {
    /// The goal line for helping a visitor with this event.
    pub fn goal(self) -> &'static str {
        match self {
            Self::Arrival => "Einem Gast aushelfen",
            Self::Fire => "Ein Feuer löschen",
            Self::Ruin => "Eine Ruine aufbauen",
            Self::Fever => "Kranke gesund pflegen",
            Self::Festival => "Ein Fest feiern",
            Self::Relic => "Ein Fundstück zurückbringen",
        }
    }

    /// How a visitor with this event is helped, in one line that also reads well
    /// while the visitor has not come yet.
    pub fn how(self) -> &'static str {
        match self {
            Self::Arrival => "Kommt ein Gast, bring ihm die Waren, um die er bittet.",
            Self::Fire => "Brennt es, hol Wasser am Brunnen oder am Meer und lösch schnell.",
            Self::Ruin => "Bittet jemand um Hilfe, bring Baumaterial zur Ruine und bau mit.",
            Self::Fever => "Braucht jemand Pflege, bring Mahlzeiten aus dem Lager.",
            Self::Festival => "Kommt das Fest, bring Mahlzeiten zu den Festtischen.",
            Self::Relic => "Fehlt ein Fundstück, hol es vom Hang und bring es zum Heiligtum.",
        }
    }
}
