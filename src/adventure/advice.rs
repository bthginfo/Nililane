use bevy::prelude::*;

use super::{
    Adventure, Build, Good, IMPORT_PRICE, Island, Level, PLAN_PRICE, Quest, Target, Task, Terrain,
    port_trade::{Export, Purchase},
};
use crate::model::Kind;

/// The one next step the island needs, derived from the first open goal.
#[derive(Clone, Debug, PartialEq)]
pub struct Advice {
    pub icon: &'static str,
    pub text: String,
    pub target: Option<Target>,
}

impl Advice {
    fn new(icon: &'static str, text: impl Into<String>, target: Option<Target>) -> Option<Self> {
        Some(Self {
            icon,
            text: text.into(),
            target,
        })
    }
}

const FULL_STORE: &str = "Das Lager ist voll. Sortiere aus oder baue einen Speicher.";
const MAX_STOCK_ROW: usize = 9;

impl Adventure {
    pub fn advice(&self) -> Option<Advice> {
        let (map, island) = self.terrain.as_ref().zip(self.active())?;
        if island.completed {
            return None;
        }
        let guide = Guide {
            map,
            island,
            level: map.level,
            here: Vec2::from_array(island.position),
        };
        guide
            .hands()
            .or_else(|| guide.sites())
            .or_else(|| guide.goals())
            .or_else(|| guide.quest())
            .or_else(|| guide.deliveries())
            .or_else(|| guide.reserve())
    }
}

impl Island {
    /// Goods worth watching in the status bar: basics plus everything open goals ask for.
    pub fn watched_goods(&self, level: Level) -> Vec<Good> {
        let mut goods = vec![Good::Wood, Good::Stone, Good::Food];
        let mut watch = |good: Good| {
            if !goods.contains(&good) {
                goods.push(good);
            }
        };
        for site in self.construction.iter().flatten() {
            for good in Good::ALL.into_iter().filter(|&good| site.missing(good) > 0) {
                watch(good);
            }
        }
        if let Some(order) = level.order(self.orders_delivered) {
            for (good, _) in order.goods {
                watch(good);
            }
        }
        if self.quest == Quest::Offering
            && let Some((good, _)) = level.offering()
        {
            watch(good);
        }
        watch(level.reserve().0);
        for goal in level
            .goals()
            .filter(|goal| self.progress(*goal) < goal.count)
        {
            let tier = self
                .lots
                .iter()
                .flatten()
                .filter(|property| property.kind == goal.kind && property.tier < goal.tier)
                .map(|property| property.tier + 1)
                .max()
                .unwrap_or(1);
            for (good, _) in goal.kind.cost(tier) {
                watch(good);
            }
        }
        for lot in 0..self.lots.len() {
            if self.assignment(Task::Produce(lot)) > 0
                && let Some(recipe) = self.recipe_for(level, lot)
            {
                for (good, _) in recipe.need {
                    watch(good);
                }
            }
        }
        goods.truncate(MAX_STOCK_ROW);
        goods.sort_by_key(|&good| good as usize);
        goods
    }

    /// Finished rows of the island's goal list, and how many rows there are.
    pub fn milestones(&self, level: Level) -> (usize, usize) {
        let (stock, count) = level.reserve();
        let rows: Vec<bool> = level
            .goals()
            .map(|goal| self.progress(goal) >= goal.count)
            .chain([self.amount(stock) >= count])
            .chain(level.plan().map(|_| self.quest == Quest::Finished))
            .chain(
                (level.order_count() > 0).then_some(self.orders_delivered >= level.order_count()),
            )
            .chain(
                level
                    .work_order()
                    .map(|_| level.work_order_delivered(self.orders_delivered)),
            )
            .collect();
        (rows.iter().filter(|&&done| done).count(), rows.len())
    }
}

struct Guide<'a> {
    map: &'a Terrain,
    island: &'a Island,
    level: Level,
    here: Vec2,
}

impl Guide<'_> {
    fn name(&self, good: Good) -> &'static str {
        good.name(self.level.region)
    }

    fn hands(&self) -> Option<Advice> {
        let island = self.island;
        if let Some(load) = island.cargo {
            let needs = |lot: usize| island.site(lot).is_some_and(|s| s.missing(load.kind) > 0);
            let site = island
                .building_load
                .filter(|&lot| needs(lot))
                .or_else(|| (0..island.lots.len()).find(|&lot| needs(lot)));
            let units = load.units;
            let name = self.name(load.kind);
            return match site {
                Some(lot) => Advice::new(
                    "hammer",
                    format!("Bring {units} {name} zur Baustelle."),
                    Some(Target::Plot(lot)),
                ),
                None if load.kind.uses_storage() && island.free_storage() == 0 => {
                    Advice::new("storehouse", FULL_STORE, Some(Target::Hall))
                }
                None => Advice::new(
                    "storehouse",
                    format!("Bring {units} {name} zum Rathaus."),
                    Some(Target::Hall),
                ),
            };
        }
        if island.shipment.is_some() {
            return Advice::new(
                "storehouse",
                "Bring die Lieferung zum Außenposten.",
                Some(Target::Outpost),
            );
        }
        if let Some(slot) = island.encounters.carrying()
            && let Some(spec) = island.encounter(self.level, slot)
        {
            let person = spec.author.split(" · ").next().unwrap_or(spec.author);
            return Advice::new(
                spec.icon(),
                format!("Bring die Fracht zu {person}."),
                Some(Target::Encounter(slot)),
            );
        }
        if island.market.packed.is_some() {
            return Advice::new(
                "sea_trade",
                "Bring die Handelsfracht zum Handelshafen.",
                Some(Target::TradeHarbor),
            );
        }
        None
    }

    fn sites(&self) -> Option<Advice> {
        let island = self.island;
        for lot in 0..island.construction.len() {
            let Some(site) = island.site(lot) else {
                continue;
            };
            if site.ready() {
                let busy = island
                    .builders
                    .iter()
                    .any(|builder| builder.lot == lot && builder.walk.work > 0.0);
                if busy {
                    continue;
                }
                return Advice::new(
                    "hammer",
                    "Alles liegt bereit. Wähle „Mitbauen“.",
                    Some(Target::Plot(lot)),
                );
            }
            let mut incoming = [0u16; Good::ALL.len()];
            for load in island
                .builders
                .iter()
                .filter(|builder| builder.lot == lot && !builder.returning)
                .filter_map(|builder| builder.walk.cargo)
            {
                incoming[load.kind as usize] += load.units;
            }
            let carriers =
                site.crew > 0 && (island.site_workers(lot) > 0 || island.available_staff() > 0);
            for good in Good::ALL {
                let short = site.missing(good).saturating_sub(incoming[good as usize]);
                if short == 0 {
                    continue;
                }
                if island.amount(good) > 0 {
                    if carriers {
                        continue;
                    }
                    return Advice::new(
                        "storehouse",
                        "Hol Baumaterial am Rathaus.",
                        Some(Target::Hall),
                    );
                }
                if let Some(advice) = self.obtain(good, short, "die Baustelle", 0) {
                    return Some(advice);
                }
                break;
            }
        }
        None
    }

    fn goals(&self) -> Option<Advice> {
        let island = self.island;
        for goal in self.level.goals() {
            // A goal with a site under way is covered by `sites`.
            if island.progress(goal) >= goal.count || self.site_for(goal.kind).is_some() {
                continue;
            }
            if let Some(advice) = self.build(goal.kind, goal.tier) {
                return Some(advice);
            }
        }
        None
    }

    /// Plans a new building or the next upgrade towards `tier`.
    fn build(&self, kind: Build, tier: u8) -> Option<Advice> {
        let island = self.island;
        let region = self.level.region;
        let upgrade = (0..island.lots.len())
            .filter(|&lot| {
                island.lots[lot].is_some_and(|p| p.kind == kind && p.tier < tier)
                    && island.site(lot).is_none()
            })
            .max_by_key(|&lot| island.lots[lot].map_or(0, |p| p.tier));
        if let Some(lot) = upgrade {
            let next = island.lots[lot].map_or(1, |p| p.tier + 1);
            if island.locked(self.level, kind, next) {
                return self.unlock(kind);
            }
            return Advice::new(
                "hammer",
                format!("Baue „{}“ auf Stufe {next} aus.", kind.name(region)),
                Some(Target::Plot(lot)),
            );
        }
        if island.locked(self.level, kind, 1) {
            return self.unlock(kind);
        }
        match self.free_plot(kind) {
            Some(lot) => Advice::new(
                "hammer",
                format!("Plane „{}“ auf dem markierten Bauplatz.", kind.name(region)),
                Some(Target::Plot(lot)),
            ),
            None => Advice::new(
                "hammer",
                format!(
                    "Kein Bauplatz frei für „{}“. Reiß ein Gebäude ab.",
                    kind.name(region)
                ),
                None,
            ),
        }
    }

    fn site_for(&self, kind: Build) -> Option<usize> {
        (0..self.island.construction.len()).find(|&lot| {
            self.island
                .site(lot)
                .is_some_and(|site| site.property.kind == kind)
        })
    }

    fn free_plot(&self, kind: Build) -> Option<usize> {
        let island = self.island;
        let level = self.level;
        let fishery = Build::Harbor(Kind::Fishery);
        (0..island.lots.len())
            .filter(|&lot| {
                island.lots[lot].is_none() && island.site(lot).is_none() && kind.fits(level, lot)
            })
            .min_by(|&a, &b| {
                let key = |lot: usize| {
                    (
                        level.coastal(lot) && kind != fishery,
                        self.map.plots[lot].distance_squared(self.here),
                    )
                };
                let (a, b) = (key(a), key(b));
                a.0.cmp(&b.0).then(a.1.total_cmp(&b.1))
            })
    }

    fn unlock(&self, kind: Build) -> Option<Advice> {
        if kind == Build::Monument {
            if !self.level.has_trading() || self.island.market.blueprint {
                return None;
            }
            if self.island.market.marks >= PLAN_PRICE {
                return Advice::new(
                    "adventure_blueprint",
                    "Kauf den Sonderbauplan im Handelshafen.",
                    Some(Target::TradeHarbor),
                );
            }
            return self.earn_marks(0);
        }
        if self.level.plan() == Some(kind) {
            return self.quest();
        }
        None
    }

    fn quest(&self) -> Option<Advice> {
        self.level.plan()?;
        match self.island.quest {
            Quest::Finished => None,
            Quest::Unheard => Advice::new(
                "shrine",
                format!("Besuche „{}“ für den Bauplan.", self.level.landmark),
                Some(Target::Shrine),
            ),
            Quest::Searching => Advice::new(
                "adventure_blueprint",
                "Hol das Fundstück vom markierten Hang.",
                Some(Target::Relic),
            ),
            Quest::Carrying => Advice::new(
                "adventure_blueprint",
                "Bring das Fundstück zum Heiligtum.",
                Some(Target::Shrine),
            ),
            Quest::Offering => {
                let (good, count) = self.level.offering()?;
                if self.island.amount(good) >= count {
                    Advice::new(
                        "adventure_craft",
                        format!("Gib {count} {} am Heiligtum ab.", self.name(good)),
                        Some(Target::Shrine),
                    )
                } else {
                    self.obtain(good, count, "das Heiligtum", 0)
                }
            }
        }
    }

    fn deliveries(&self) -> Option<Advice> {
        let island = self.island;
        let order = self.level.order(island.orders_delivered)?;
        if island
            .courier
            .as_ref()
            .is_some_and(|courier| courier.order.is_some())
        {
            return None;
        }
        if island.order_ready(self.level) {
            return Advice::new(
                "ferry",
                "Pack die Lieferung im Rathaus.",
                Some(Target::Hall),
            );
        }
        let (good, count) = order
            .goods
            .into_iter()
            .find(|&(good, count)| island.amount(good) < count)?;
        self.obtain(good, count, "die Lieferung", 0)
    }

    fn reserve(&self) -> Option<Advice> {
        let (good, count) = self.level.reserve();
        self.obtain(good, count, "den Vorrat", 0)
    }

    /// How to get `need` units of `good` into the hall.
    fn obtain(&self, good: Good, need: u16, purpose: &str, depth: u8) -> Option<Advice> {
        let island = self.island;
        let missing = need.saturating_sub(island.amount(good));
        if missing == 0 || depth > 4 {
            return None;
        }
        match good {
            Good::Masterwork => return self.masterwork(depth),
            Good::Imports if island.market.marks >= IMPORT_PRICE => {
                return Advice::new(
                    "sea_trade",
                    "Kauf Baustoffe im Handelshafen.",
                    Some(Target::TradeHarbor),
                );
            }
            Good::Imports => return self.earn_marks(depth),
            _ => {}
        }
        if let Some(offer) = self.market_offer(good) {
            return Advice::new(
                "sea_trade",
                format!("Kauf {} {} im Handelshafen.", offer.units, self.name(good)),
                Some(Target::TradeHarbor),
            );
        }
        if good.uses_storage() && island.free_storage() == 0 {
            return Advice::new("storehouse", FULL_STORE, Some(Target::Hall));
        }
        if Good::RAW.contains(&good)
            && let Some(id) = island.available_near(self.map, good, self.here)
        {
            return Advice::new(
                good.icon(self.level.region),
                format!("Sammle {missing} {} für {purpose}.", self.name(good)),
                Some(Target::Deposit(id)),
            );
        }
        self.produce(good, depth)
    }

    fn produce(&self, good: Good, depth: u8) -> Option<Advice> {
        let island = self.island;
        let level = self.level;
        let region = level.region;
        let makes = |kind: Build, tier: u8| {
            kind.recipe(tier, region)
                .is_some_and(|recipe| recipe.output.0 == good)
        };
        let workshop = (0..island.lots.len())
            .filter(|&lot| {
                island
                    .recipe_for(level, lot)
                    .is_some_and(|recipe| recipe.output.0 == good)
            })
            .max_by_key(|&lot| {
                (
                    island.assignment(Task::Produce(lot)),
                    island.batch(lot).is_some(),
                )
            });
        if let Some(lot) = workshop {
            return self.operate(lot, good, depth);
        }
        if let Some(lot) = (0..island.lots.len()).find(|&lot| {
            island
                .site(lot)
                .is_some_and(|site| makes(site.property.kind, site.property.tier))
        }) {
            let name = island.site(lot)?.property.kind.name(region);
            return Advice::new(
                "hammer",
                format!("Bau „{name}“ fertig, dort entsteht {}.", self.name(good)),
                Some(Target::Plot(lot)),
            );
        }
        let plan = Build::all(level)
            .into_iter()
            .filter(|&plan| {
                makes(plan, 1) && !island.locked(level, plan, 1) && self.free_plot(plan).is_some()
            })
            .min_by_key(|plan| plan.cost(1).iter().map(|&(_, units)| units).sum::<u16>())?;
        Advice::new(
            "hammer",
            format!("Plane „{}“ für {}.", plan.name(region), self.name(good)),
            self.free_plot(plan).map(Target::Plot),
        )
    }

    /// Keeps one workshop going: staff, inputs, storage, then helping out.
    fn operate(&self, lot: usize, good: Good, depth: u8) -> Option<Advice> {
        let island = self.island;
        let level = self.level;
        let property = island.lots[lot]?;
        let workshop = property.kind.name(level.region);
        let recipe = island.recipe_for(level, lot)?;
        let batch = island.batch(lot);
        if island.assignment(Task::Produce(lot)) == 0 && batch.is_none_or(|batch| batch.paused) {
            if island.available_staff() == 0 {
                return Advice::new(
                    "adventure_labor",
                    "Alle Bürger sind beschäftigt. Stell einen unter „Bürger“ frei.",
                    None,
                );
            }
            return Advice::new(
                "age_adult",
                format!("„{workshop}“ braucht einen Bürger."),
                Some(Target::Plot(lot)),
            );
        }
        if batch.is_some_and(|batch| batch.left == 0.0) {
            return Advice::new("storehouse", FULL_STORE, Some(Target::Hall));
        }
        if batch.is_none() {
            if let Some(&(input, units)) = recipe
                .need
                .iter()
                .find(|&&(input, units)| island.amount(input) < units)
            {
                let purpose = self.name(recipe.output.0);
                return self.obtain(input, units, purpose, depth + 1);
            }
            if !island.room_for_recipe(&recipe) {
                return Advice::new("storehouse", FULL_STORE, Some(Target::Hall));
            }
            if recipe.output.0 == Good::Fish && !island.fish_available(self.map, lot) {
                return Advice::new("fishery", "Die Fischgründe sind erschöpft.", None);
            }
        }
        let fishing = batch.is_some_and(|batch| batch.fishing.is_some());
        Advice::new(
            good.icon(level.region),
            if fishing {
                format!(
                    "„{workshop}“ fängt {}. Das Boot ist unterwegs.",
                    self.name(good)
                )
            } else {
                format!("„{workshop}“ stellt {} her. Hilf mit!", self.name(good))
            },
            Some(Target::Plot(lot)),
        )
    }

    fn masterwork(&self, depth: u8) -> Option<Advice> {
        let island = self.island;
        let level = self.level;
        let order = level.work_order()?;
        let name = order.workshop.name(level.region);
        let Some(lot) = (0..island.lots.len())
            .filter(|&lot| island.lots[lot].is_some_and(|p| p.kind == order.workshop))
            .max_by_key(|&lot| island.lots[lot].map_or(0, |p| p.tier))
        else {
            if let Some(site) = self.site_for(order.workshop) {
                return Advice::new(
                    "hammer",
                    format!("Bau „{name}“ fertig für den Meisterauftrag."),
                    Some(Target::Plot(site)),
                );
            }
            return self.build(order.workshop, 1);
        };
        if island.base_crafted < order.base_needed && !island.makes_masterwork(lot) {
            return self.operate(lot, order.base, depth);
        }
        if island.lots[lot].is_some_and(|p| p.tier < order.tier) {
            if self.site_for(order.workshop).is_some() {
                return None;
            }
            return self.build(order.workshop, order.tier);
        }
        if !island.makes_masterwork(lot) {
            return Advice::new(
                Good::Masterwork.icon(level.region),
                format!("Stell „{name}“ auf {} um.", self.name(Good::Masterwork)),
                Some(Target::Plot(lot)),
            );
        }
        self.operate(lot, Good::Masterwork, depth)
    }

    /// A market load of `good` the island can afford. Marks stay saved for the
    /// landmark where the island asks for one.
    fn market_offer(&self, good: Good) -> Option<Purchase> {
        let level = self.level;
        if !level.has_trading()
            || good.uses_storage()
            || level.goals().any(|goal| goal.kind == Build::Monument)
        {
            return None;
        }
        level
            .shop()
            .into_iter()
            .find(|offer| offer.good == good && self.island.market.marks >= offer.marks)
    }

    fn earn_marks(&self, depth: u8) -> Option<Advice> {
        let island = self.island;
        let offer = self.level.export(Export::Regular)?;
        if island.market.crossing.is_some() {
            return Advice::new(
                "sea_trade",
                "Das Handelsschiff ist unterwegs. Danach hast du mehr Handelsmarken.",
                None,
            );
        }
        if island.amount(offer.good) >= offer.units {
            return Advice::new(
                "sea_trade",
                "Pack Handelsfracht im Rathaus für Handelsmarken.",
                Some(Target::Hall),
            );
        }
        self.obtain(offer.good, offer.units, "die Handelsfracht", depth + 1)
    }
}
