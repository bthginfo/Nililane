use serde::{Deserialize, Serialize};

use crate::{
    campaign::{Chapter, Goal},
    errands::{Blocked, JOB_TYPES, Job, Route, Stop},
};

pub use crate::layout::{LOTS, distance};
pub const DAY_SECONDS: f32 = 55.0;
pub const MAX_LEVEL: u8 = 4;
pub const SERVICES: usize = 9;
pub const BUILDINGS: usize = 15;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Service {
    Bread,
    Pottery,
    Shade,
    Meze,
    Bed,
    Oil,
    Offering,
    Bath,
    Trade,
}

impl Service {
    pub const ALL: [Self; SERVICES] = [
        Self::Bread,
        Self::Pottery,
        Self::Shade,
        Self::Meze,
        Self::Bed,
        Self::Oil,
        Self::Offering,
        Self::Bath,
        Self::Trade,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Bread => "Brot",
            Self::Pottery => "Keramik",
            Self::Shade => "Rast",
            Self::Meze => "Meze",
            Self::Bed => "Herberge",
            Self::Oil => "Olivenöl",
            Self::Offering => "Weihegaben",
            Self::Bath => "Thermen",
            Self::Trade => "Markt",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    House,
    Well,
    Bakery,
    Grove,
    Potter,
    Stoa,
    Taverna,
    Inn,
    Storehouse,
    Press,
    Shrine,
    Aqueduct,
    Baths,
    Forum,
    Villa,
}

impl Kind {
    pub const ALL: [Self; BUILDINGS] = [
        Self::House,
        Self::Well,
        Self::Bakery,
        Self::Grove,
        Self::Potter,
        Self::Stoa,
        Self::Taverna,
        Self::Inn,
        Self::Storehouse,
        Self::Press,
        Self::Shrine,
        Self::Aqueduct,
        Self::Baths,
        Self::Forum,
        Self::Villa,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::House => "Wohnhof",
            Self::Well => "Brunnen",
            Self::Bakery => "Backstube",
            Self::Grove => "Olivenhain",
            Self::Potter => "Töpferei",
            Self::Stoa => "Stoa",
            Self::Taverna => "Taverne",
            Self::Inn => "Gasthaus",
            Self::Storehouse => "Speicher",
            Self::Press => "Ölpresse",
            Self::Shrine => "Heiligtum",
            Self::Aqueduct => "Aquädukt",
            Self::Baths => "Thermen",
            Self::Forum => "Forum",
            Self::Villa => "Villa",
        }
    }

    pub fn cost(self) -> u32 {
        match self {
            Self::House => 21,
            Self::Well => 13,
            Self::Bakery => 19,
            Self::Grove => 16,
            Self::Potter => 24,
            Self::Stoa => 24,
            Self::Taverna => 31,
            Self::Inn => 34,
            Self::Storehouse => 32,
            Self::Press => 34,
            Self::Shrine => 39,
            Self::Aqueduct => 45,
            Self::Baths => 49,
            Self::Forum => 46,
            Self::Villa => 41,
        }
    }

    pub fn service(self) -> Option<Service> {
        match self {
            Self::Bakery => Some(Service::Bread),
            Self::Potter => Some(Service::Pottery),
            Self::Stoa => Some(Service::Shade),
            Self::Taverna => Some(Service::Meze),
            Self::Inn => Some(Service::Bed),
            Self::Press => Some(Service::Oil),
            Self::Shrine => Some(Service::Offering),
            Self::Baths => Some(Service::Bath),
            Self::Forum => Some(Service::Trade),
            _ => None,
        }
    }

    pub fn water_user(self) -> bool {
        matches!(self, Self::Bakery | Self::Potter | Self::Inn | Self::Baths)
    }

    pub fn water_source(self) -> bool {
        matches!(self, Self::Well | Self::Aqueduct)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Price {
    Fair,
    #[default]
    Regular,
    Fine,
}

impl Price {
    pub const ALL: [Self; 3] = [Self::Fair, Self::Regular, Self::Fine];

    pub fn label(self) -> &'static str {
        match self {
            Self::Fair => "Günstig",
            Self::Regular => "Normal",
            Self::Fine => "Hoch",
        }
    }

    pub fn coins(self, service: Service) -> u32 {
        let base = match service {
            Service::Bread => 3,
            Service::Pottery => 5,
            Service::Shade => 3,
            Service::Meze => 5,
            Service::Bed => 6,
            Service::Oil => 6,
            Service::Offering => 6,
            Service::Bath => 7,
            Service::Trade => 5,
        };
        match self {
            Self::Fair => base - 1,
            Self::Regular => base,
            Self::Fine => base + 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Building {
    pub kind: Kind,
    pub level: u8,
    pub price: Price,
}

impl Building {
    pub const fn new(kind: Kind) -> Self {
        Self {
            kind,
            level: 1,
            price: Price::Regular,
        }
    }

    pub fn upgrade_cost(self) -> u32 {
        self.kind.cost() / 2 + 4 * u32::from(self.level)
    }

    pub fn refund(self) -> u32 {
        let investments: u32 = (1..self.level)
            .map(|level| Self { level, ..self }.upgrade_cost())
            .sum();
        (self.kind.cost() + investments) / 2
    }

    pub fn residents(self) -> u16 {
        match self.kind {
            Kind::House => 2 + u16::from(self.level),
            Kind::Villa => 2 + u16::from(self.level) * 2,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Cloud,
    Wind,
    Heat,
    Feast,
    Rain,
    Storm,
}

impl Weather {
    pub fn name(self) -> &'static str {
        match self {
            Self::Clear => "Sonne",
            Self::Cloud => "Wolken",
            Self::Storm => "Sturm",
            Self::Wind => "Wind",
            Self::Heat => "Hitze",
            Self::Feast => "Festtag",
            Self::Rain => "Regen",
        }
    }

    pub fn ferry_change(self) -> i16 {
        match self {
            Self::Wind => -2,
            Self::Rain => -3,
            Self::Storm => -5,
            Self::Feast => 4,
            _ => 0,
        }
    }

    pub fn swell(self) -> f32 {
        match self {
            Self::Clear | Self::Heat => 0.18,
            Self::Cloud | Self::Feast => 0.32,
            Self::Rain => 0.58,
            Self::Wind => 0.78,
            Self::Storm => 1.0,
        }
    }

    pub fn rainfall(self) -> u16 {
        match self {
            Self::Rain => 4,
            Self::Storm => 5,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Favor {
    Jugs,
    Shade,
    Cargo,
}

impl Favor {
    pub fn name(self) -> &'static str {
        match self {
            Self::Jugs => "Wasser verteilen",
            Self::Shade => "Schatten spenden",
            Self::Cargo => "Ware vom Kai holen",
        }
    }

    pub fn note(self) -> &'static str {
        match self {
            Self::Jugs => "+2 Vorrat für Läden an der Quelle",
            Self::Shade => "+2 Plätze je Stoa und Therme",
            Self::Cargo => "+3 Vorrat für einen Laden",
        }
    }

    pub fn allows(self, kind: Kind) -> bool {
        match self {
            Self::Jugs => kind.water_source(),
            Self::Shade => matches!(kind, Kind::Grove | Kind::Stoa | Kind::Baths),
            Self::Cargo => kind.service().is_some(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    Home(u8),
    Ferry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Miss {
    NoShop,
    Walk,
    Price,
    Empty,
}

impl Miss {
    pub const ALL: [Self; 4] = [Self::NoShop, Self::Walk, Self::Price, Self::Empty];

    pub fn label(self) -> &'static str {
        match self {
            Self::NoShop => "Angebot fehlt",
            Self::Walk => "Weg zu weit",
            Self::Price => "Zu teuer",
            Self::Empty => "Ausverkauft",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::NoShop => "Baue einen Laden mit diesem Angebot.",
            Self::Walk => "Baue näher bei den Wohnhöfen oder am Kai.",
            Self::Price => "Stell morgen den Preis niedriger ein.",
            Self::Empty => "Schick Nili mit Nachschub oder bau den Laden aus.",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Visit {
    pub origin: Origin,
    pub wants: Service,
    pub budget: Price,
    pub at: f32,
    pub resolved: bool,
    pub shop: Option<u8>,
    #[serde(default)]
    pub miss: Option<Miss>,
    #[serde(default)]
    pub guided_to: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declined_at: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pleased,
    Missed(Miss),
}

impl Visit {
    pub fn verdict(&self) -> Option<Verdict> {
        if !self.resolved {
            return None;
        }
        if self.shop.is_some() {
            return Some(Verdict::Pleased);
        }
        self.miss.map(Verdict::Missed)
    }
}

pub enum GuestTip {
    Shop(usize),
    Build(usize, Kind),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Tally {
    #[serde(deserialize_with = "read_service_counts")]
    pub served: [u32; SERVICES],
    #[serde(deserialize_with = "read_service_counts")]
    pub missed: [u32; SERVICES],
    #[serde(default)]
    pub gaps: [u32; 4],
    pub income: u32,
    pub reputation: u32,
    pub favors: u32,
    #[serde(default)]
    pub feast_peak: u32,
    #[serde(default)]
    pub widest_day: u32,
    #[serde(default)]
    pub record_income: u32,
    #[serde(default)]
    pub peak_water: u32,
    #[serde(default)]
    pub routes: [u32; JOB_TYPES],
    #[serde(default)]
    pub ferry_served: u32,
    #[serde(default)]
    pub best_ferry_day: u32,
    #[serde(default)]
    pub best_mood: u32,
}

impl Tally {
    pub fn routes_today(&self) -> u32 {
        self.routes.iter().sum()
    }

    pub fn served_total(&self) -> u32 {
        self.served.iter().sum()
    }

    pub fn responses(&self) -> u32 {
        self.served_total() + self.missed.iter().sum::<u32>()
    }

    pub fn satisfaction(&self) -> Option<u32> {
        satisfaction_percent(self.served_total(), self.responses())
    }

    fn add(&mut self, other: &Self) {
        for service in Service::ALL {
            let i = service as usize;
            self.served[i] += other.served[i];
            self.missed[i] += other.missed[i];
        }
        self.income += other.income;
        self.reputation += other.reputation;
        self.favors += other.favors;
        for (gap, added) in self.gaps.iter_mut().zip(other.gaps) {
            *gap += added;
        }
        self.feast_peak = self.feast_peak.max(other.feast_peak);
        self.widest_day = self.widest_day.max(other.widest_day);
        self.record_income = self.record_income.max(other.record_income);
        self.peak_water = self.peak_water.max(other.peak_water);
        for (total, count) in self.routes.iter_mut().zip(other.routes) {
            *total += count;
        }
        self.ferry_served += other.ferry_served;
        self.best_ferry_day = self.best_ferry_day.max(other.best_ferry_day);
        self.best_mood = self.best_mood.max(other.best_mood);
    }
}

pub fn satisfaction_percent(served: u32, total: u32) -> Option<u32> {
    let total = u64::from(total);
    (total > 0).then(|| ((u64::from(served) * 100 + total / 2) / total) as u32)
}

fn read_service_counts<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<[u32; SERVICES], D::Error> {
    let values = Vec::<u32>::deserialize(deserializer)?;
    let mut result = [0; SERVICES];
    for (target, value) in result.iter_mut().zip(values) {
        *target = value;
    }
    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Day {
    pub seconds: f32,
    #[serde(default)]
    pub water: u16,
    #[serde(deserialize_with = "extend_plot_array")]
    pub stock: [u16; LOTS],
    pub visits: Vec<Visit>,
    pub tally: Tally,
    pub favor: Favor,
    pub favor_at: Option<u8>,
    #[serde(default)]
    pub nili_at: Stop,
    #[serde(default)]
    pub route: Option<Route>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub tally: Tally,
    pub grant: u32,
    pub stars: u8,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Plan,
    Open(Box<Day>),
    Report(Report),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarborPlan {
    #[default]
    Regular,
    Ferry,
    Water,
    Neighbors,
}

impl HarborPlan {
    pub const ALL: [Self; 4] = [Self::Regular, Self::Ferry, Self::Water, Self::Neighbors];

    pub fn name(self) -> &'static str {
        match self {
            Self::Regular => "Normal",
            Self::Ferry => "Fährtag",
            Self::Water => "Wasserfahrt",
            Self::Neighbors => "Nachbarschaft",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    pub chapter: usize,
    pub day: u32,
    pub coins: u32,
    #[serde(deserialize_with = "extend_plot_array")]
    pub lots: [Option<Building>; LOTS],
    pub totals: Tally,
    pub phase: Phase,
    #[serde(default)]
    pub harbor: HarborPlan,
    #[serde(default)]
    pub last_day: Option<Tally>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub last_visits: Vec<Visit>,
}

#[derive(Default)]
pub struct Production {
    pub stock: [u16; LOTS],
    pub water: u16,
    pub olives: u16,
    pub used_water: u16,
    pub used_olives: u16,
}

pub fn is_open(chapter: &Chapter, lot: usize) -> bool {
    chapter.layout.contains(lot)
}

fn extend_plot_array<'de, D, T>(deserializer: D) -> Result<[T; LOTS], D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default + Copy,
{
    let values = Vec::<T>::deserialize(deserializer)?;
    if values.len() != crate::layout::ORIGINAL_COLUMNS * 2 && values.len() != LOTS {
        return Err(serde::de::Error::custom("unexpected number of plots"));
    }
    let mut plots = [T::default(); LOTS];
    plots[..values.len()].copy_from_slice(&values);
    Ok(plots)
}

impl Town {
    pub fn new(chapter: usize, rule: &Chapter) -> Self {
        let mut lots = [None; LOTS];
        for &(lot, kind) in rule.starts {
            lots[lot] = Some(Building::new(kind));
        }
        Self {
            chapter,
            day: 1,
            coins: rule.coins,
            lots,
            totals: Tally::default(),
            phase: Phase::Plan,
            harbor: HarborPlan::Regular,
            last_day: None,
            last_visits: Vec::new(),
        }
    }

    pub fn weather(&self, chapter: &Chapter) -> Weather {
        self.weather_ahead(chapter, 0)
    }

    pub fn weather_ahead(&self, chapter: &Chapter, days: usize) -> Weather {
        chapter.weather[(self.day.saturating_sub(1) as usize + days) % chapter.weather.len()]
    }

    pub fn planning(&self) -> bool {
        matches!(self.phase, Phase::Plan)
    }

    pub fn feedback(&self) -> Option<&Tally> {
        match &self.phase {
            Phase::Open(day) => Some(&day.tally),
            Phase::Report(report) => Some(&report.tally),
            Phase::Plan => self.last_day.as_ref(),
        }
    }

    pub fn guest_history(&self) -> &[Visit] {
        if let Phase::Open(day) = &self.phase {
            &day.visits
        } else {
            &self.last_visits
        }
    }

    pub fn guest_tip(&self, chapter: &Chapter, guest: usize) -> Option<GuestTip> {
        let visit = self.guest_history().get(guest)?;
        match (visit.verdict()?, &self.phase) {
            (_, Phase::Report(_)) => None,
            (Verdict::Missed(Miss::Price | Miss::Empty), _) => {
                let lot = usize::from(visit.declined_at.or(visit.guided_to)?);
                self.lots[lot]
                    .is_some_and(|b| b.kind.service() == Some(visit.wants))
                    .then_some(GuestTip::Shop(lot))
            }
            (Verdict::Missed(Miss::NoShop | Miss::Walk), Phase::Plan) => {
                let kind = chapter
                    .allowed()
                    .iter()
                    .copied()
                    .find(|kind| kind.service() == Some(visit.wants))?;
                let origin = match visit.origin {
                    Origin::Home(lot) => usize::from(lot),
                    Origin::Ferry => chapter.layout.dock(),
                };
                chapter
                    .layout
                    .lots()
                    .filter(|&lot| self.lots[lot].is_none())
                    .filter(|&lot| {
                        distance(origin, lot)
                            <= walking_range(&self.lots, visit.origin, lot, chapter, false)
                    })
                    .min_by_key(|&lot| distance(origin, lot))
                    .map(|lot| GuestTip::Build(lot, kind))
            }
            _ => None,
        }
    }

    pub fn count(&self, kind: Kind) -> u32 {
        self.lots
            .iter()
            .flatten()
            .filter(|building| building.kind == kind)
            .count() as u32
    }

    pub fn next_favor(&self, chapter: &Chapter) -> Favor {
        chapter.favors[(self.day.saturating_sub(1) as usize) % chapter.favors.len()]
    }

    pub fn visitors(&self, chapter: &Chapter) -> (u16, u16) {
        let residents = self
            .lots
            .iter()
            .flatten()
            .map(|b| self.home_visits(*b))
            .sum();
        let ferry = chapter
            .ferry
            .saturating_add_signed(self.weather(chapter).ferry_change());
        let ferry = match self.harbor {
            HarborPlan::Regular => ferry,
            HarborPlan::Ferry => ferry + 3,
            HarborPlan::Water | HarborPlan::Neighbors => ferry.saturating_sub(2),
        };
        (residents, ferry)
    }

    fn home_visits(&self, building: Building) -> u16 {
        let residents = building.residents();
        residents + u16::from(residents > 0 && self.harbor == HarborPlan::Neighbors)
    }

    pub fn set_harbor(&mut self, chapter: &Chapter, rule: HarborPlan) -> bool {
        if !self.planning() || !chapter.harbor_plans() {
            return false;
        }
        self.harbor = rule;
        true
    }

    pub fn build(&mut self, chapter: &Chapter, lot: usize, kind: Kind) -> bool {
        if !self.planning()
            || !is_open(chapter, lot)
            || self.lots[lot].is_some()
            || self.coins < kind.cost()
            || !chapter.allowed().contains(&kind)
        {
            return false;
        }
        self.lots[lot] = Some(Building::new(kind));
        self.coins -= kind.cost();
        true
    }

    pub fn upgrade(&mut self, lot: usize) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::as_mut) else {
            return false;
        };
        if building.level >= MAX_LEVEL || self.coins < building.upgrade_cost() {
            return false;
        }
        self.coins -= building.upgrade_cost();
        building.level += 1;
        true
    }

    pub fn set_price(&mut self, lot: usize, price: Price) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::as_mut) else {
            return false;
        };
        if building.kind.service().is_none() {
            return false;
        }
        building.price = price;
        true
    }

    pub fn remove(&mut self, lot: usize) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::take) else {
            return false;
        };
        self.coins += building.refund();
        true
    }

    pub fn neighboring(&self, lot: usize, kind: Kind, range: usize) -> bool {
        self.lots.iter().enumerate().any(|(other, building)| {
            building.is_some_and(|b| b.kind == kind)
                && other != lot
                && distance(lot, other) <= range
        })
    }

    pub fn production(&self, chapter: &Chapter) -> Production {
        let weather = self.weather(chapter);
        let heat = weather == Weather::Heat;
        let mut plan = Production {
            water: chapter.spring + weather.rainfall(),
            ..Production::default()
        };
        for (lot, building) in self.lots.iter().enumerate() {
            let Some(building) = building else { continue };
            match building.kind {
                Kind::Well => plan.water += 4 + u16::from(building.level) * 4,
                Kind::Grove => plan.olives += 2 + u16::from(building.level) * 3,
                Kind::Aqueduct => {
                    plan.water += 10
                        + u16::from(building.level) * 5
                        + u16::from(self.neighboring(lot, Kind::Well, 1)) * 4;
                }
                _ => {}
            }
        }
        plan.water = match self.harbor {
            HarborPlan::Ferry => plan.water.saturating_sub(2),
            HarborPlan::Water => plan.water + 4,
            _ => plan.water,
        };
        // The closest workshops get their jars first. The stable plot order breaks ties.
        let mut workshops: Vec<_> = self
            .lots
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(lot, b)| b.filter(|b| b.kind.water_user()).map(|b| (lot, b)))
            .collect();
        let water_distance = |lot| {
            self.lots
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    b.filter(|b| b.kind.water_source())
                        .map(|_| distance(lot, i))
                })
                .min()
                .unwrap_or(LOTS)
        };
        // Nearby jobs in the same range retain street order from morning to morning.
        workshops.sort_by_key(|(lot, _)| water_distance(*lot).min(3));
        for (lot, b) in workshops {
            let level = u16::from(b.level);
            let cost = level + u16::from(heat) + 2 * u16::from(b.kind == Kind::Baths);
            let stocked = plan.water - plan.used_water >= cost;
            if stocked {
                plan.used_water += cost;
            }
            plan.stock[lot] = match (b.kind, stocked) {
                (Kind::Bakery, true) => 4 + level * 3,
                (Kind::Bakery, false) => 2 + level,
                (Kind::Potter | Kind::Inn, true) => 3 + level * 2,
                (Kind::Baths, true) => {
                    4 + level * 3 + 2 * u16::from(self.neighboring(lot, Kind::Aqueduct, 2))
                }
                _ => 1 + level,
            };
        }

        for (lot, b) in self.lots.iter().copied().enumerate() {
            let Some(b) = b else { continue };
            let level = u16::from(b.level);
            match b.kind {
                Kind::Stoa => plan.stock[lot] = 4 + level * 2,
                Kind::Forum => plan.stock[lot] = 5 + level * 3,
                Kind::Taverna => {
                    let bakery = self.nearest_stock(lot, Kind::Bakery, 2, &plan);
                    plan.stock[lot] = match bakery {
                        Some(source) if plan.olives - plan.used_olives >= 2 => {
                            plan.stock[source] -= 2;
                            plan.used_olives += 2;
                            3 + level * 3 + u16::from(distance(lot, source) <= 1)
                        }
                        _ => 1 + level,
                    };
                }
                Kind::Shrine => {
                    plan.stock[lot] = match self.nearest_stock(lot, Kind::Potter, 2, &plan) {
                        Some(source) => {
                            plan.stock[source] -= 2;
                            4 + level * 2
                        }
                        None => 1 + level,
                    };
                }
                _ => {}
            }
        }
        for (lot, b) in self.lots.iter().enumerate() {
            let Some(b) = b else { continue };
            if b.kind == Kind::Press {
                let level = u16::from(b.level);
                let olives = (2 + level).min(plan.olives - plan.used_olives);
                plan.used_olives += olives;
                plan.stock[lot] = 1 + olives * 2 + u16::from(self.neighboring(lot, Kind::Grove, 1));
            }
            if b.kind.service().is_some() {
                let supported = self
                    .lots
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| distance(lot, *i) <= 2)
                    .filter_map(|(_, b)| b.filter(|b| b.kind == Kind::Storehouse))
                    .map(|b| u16::from(b.level))
                    .max()
                    .unwrap_or(0);
                plan.stock[lot] += supported;
            }
        }
        plan
    }

    fn nearest_stock(
        &self,
        lot: usize,
        kind: Kind,
        amount: u16,
        plan: &Production,
    ) -> Option<usize> {
        self.lots
            .iter()
            .enumerate()
            .filter(|(i, source)| {
                source.is_some_and(|b| b.kind == kind) && plan.stock[*i] >= amount
            })
            .min_by_key(|(i, _)| distance(lot, *i))
            .map(|(i, _)| i)
    }

    pub fn visit_plan(&self, chapter: &Chapter) -> Vec<Visit> {
        let (residents, ferry) = self.visitors(chapter);
        let count = usize::from(residents + ferry);
        let mut visitors = Vec::with_capacity(count);
        let weather = self.weather(chapter);
        let offset = (self.day as usize - 1) * 3;
        for (lot, b) in self.lots.iter().enumerate() {
            let Some(b) = b.filter(|b| b.residents() > 0) else {
                continue;
            };
            for local in 0..usize::from(self.home_visits(b)) {
                let index = visitors.len() + offset;
                let heat_shade = weather == Weather::Heat
                    && local == 1
                    && !self.neighboring(lot, Kind::Grove, 1)
                    && chapter.allowed().contains(&Kind::Stoa);
                visitors.push(Visit {
                    origin: Origin::Home(lot as u8),
                    wants: if heat_shade {
                        Service::Shade
                    } else {
                        chapter.locals[index % chapter.locals.len()]
                    },
                    budget: if b.kind == Kind::Villa && local % 3 != 0 {
                        Price::Fine
                    } else if local == 0 {
                        Price::Fair
                    } else {
                        Price::Regular
                    },
                    at: 0.0,
                    resolved: false,
                    shop: None,
                    miss: None,
                    guided_to: None,
                    declined_at: None,
                });
            }
        }
        for n in 0..usize::from(ferry) {
            let budget = if weather == Weather::Feast || n % 3 == 1 {
                Price::Fine
            } else if n % 5 == 4 {
                Price::Fair
            } else {
                Price::Regular
            };
            visitors.push(Visit {
                origin: Origin::Ferry,
                wants: chapter.guests[(n + offset) % chapter.guests.len()],
                budget,
                at: 0.0,
                resolved: false,
                shop: None,
                miss: None,
                guided_to: None,
                declined_at: None,
            });
        }
        let total = count.saturating_sub(1).max(1) as f32;
        for (i, visit) in visitors.iter_mut().enumerate() {
            visit.at = 6.0 + (i as f32 / total) * (chapter.day_seconds() - 12.0);
        }
        visitors
    }

    pub fn forecast(&self, chapter: &Chapter) -> [u16; SERVICES] {
        let mut demand = [0; SERVICES];
        for visit in self.visit_plan(chapter) {
            demand[visit.wants as usize] += 1;
        }
        demand
    }

    pub fn open(&mut self, chapter: &Chapter) -> bool {
        if !self.planning() {
            return false;
        }
        let production = self.production(chapter);
        self.phase = Phase::Open(Box::new(Day {
            seconds: 0.0,
            water: production.water,
            stock: production.stock,
            visits: self.visit_plan(chapter),
            tally: Tally::default(),
            favor: self.next_favor(chapter),
            favor_at: None,
            nili_at: Stop::Quay,
            route: None,
        }));
        true
    }

    pub fn dispatch(&mut self, chapter: &Chapter, job: Job, lot: usize) -> Result<Route, Blocked> {
        let route = job.plan(self, chapter, lot)?;
        let Phase::Open(day) = &mut self.phase else {
            return Err(Blocked::Morning);
        };
        day.route = Some(route);
        Ok(route)
    }

    pub fn can_aid(&self, lot: usize) -> bool {
        let Phase::Open(day) = &self.phase else {
            return false;
        };
        day.seconds >= 8.0
            && day.route.is_none()
            && day.favor_at.is_none()
            && self
                .lots
                .get(lot)
                .and_then(|b| *b)
                .is_some_and(|b| day.favor.allows(b.kind))
    }

    pub fn aid(&mut self, lot: usize) -> bool {
        if !self.can_aid(lot) {
            return false;
        }
        let Phase::Open(day) = &mut self.phase else {
            return false;
        };
        match day.favor {
            Favor::Jugs => {
                for (neighbor, building) in self.lots.iter().enumerate() {
                    if building.is_some_and(|b| b.kind.water_user()) && distance(lot, neighbor) <= 3
                    {
                        day.stock[neighbor] += 2;
                    }
                }
            }
            Favor::Shade => {
                for (neighbor, building) in self.lots.iter().enumerate() {
                    if building.is_some_and(|b| matches!(b.kind, Kind::Stoa | Kind::Baths)) {
                        day.stock[neighbor] += 2;
                    }
                }
            }
            Favor::Cargo => day.stock[lot] += 3,
        }
        day.tally.income += 6;
        day.tally.reputation += 2;
        day.tally.favors += 1;
        day.favor_at = Some(lot as u8);
        day.nili_at = Stop::Parcel(lot as u8);
        true
    }

    pub fn tick(&mut self, chapter: &Chapter, elapsed: f32) -> bool {
        let Phase::Open(day) = &mut self.phase else {
            return false;
        };
        let until = (day.seconds + elapsed).min(chapter.day_seconds());
        if let Some(route) = day.route.filter(|route| route.finish <= until) {
            resolve_visits(&self.lots, day, chapter, route.finish);
            finish_route(&self.lots, day, route);
        }
        resolve_visits(&self.lots, day, chapter, until);
        day.seconds = until;
        if day.seconds < chapter.day_seconds() {
            return false;
        }
        let mut tally = day.tally.clone();
        self.last_visits = std::mem::take(&mut day.visits);
        tally.widest_day = tally.served.iter().filter(|&&served| served > 0).count() as u32;
        tally.record_income = tally.income;
        tally.peak_water = u32::from(day.water);
        tally.best_ferry_day = tally.ferry_served;
        if tally.responses() >= 12 {
            tally.best_mood = tally.satisfaction().unwrap_or(0);
        }
        if self.weather(chapter) == Weather::Feast {
            tally.feast_peak = tally.served_total();
        }
        self.totals.add(&tally);
        self.last_day = Some(tally.clone());
        let grant = 6 + u32::from(tally.served_total() == 0) * 3;
        self.coins += tally.income + grant;
        let stars = if chapter
            .goals
            .iter()
            .all(|&goal| self.goal(goal).0 >= goal.target())
        {
            1 + u8::from(self.day <= chapter.par)
                + u8::from(self.totals.favors >= chapter.favor_goal)
        } else {
            0
        };
        self.phase = Phase::Report(Report {
            tally,
            grant,
            stars,
        });
        true
    }

    pub fn advance(&mut self) -> bool {
        if !matches!(self.phase, Phase::Report(_)) {
            return false;
        }
        self.day += 1;
        self.phase = Phase::Plan;
        self.harbor = HarborPlan::Regular;
        true
    }

    pub fn goal(&self, goal: Goal) -> (u32, u32) {
        let value = match goal {
            Goal::Build(kind, _) => self.count(kind),
            Goal::Serve(service, _) => self.totals.served[service as usize],
            Goal::Renown(_) => self.totals.reputation,
            Goal::Help(_) => self.totals.favors,
            Goal::Earn(_) => self.totals.income,
            Goal::FeastGuests(_) => self.totals.feast_peak,
            Goal::Variety(_) => self.totals.widest_day,
            Goal::MarketDay(_) => self.totals.record_income,
            Goal::Water(_) => self.totals.peak_water,
            Goal::Route(job, _) => self.totals.routes[job as usize],
            Goal::RouteVariety(_) => self.totals.routes.iter().filter(|&&n| n > 0).count() as u32,
            Goal::FerryDay(_) => self.totals.best_ferry_day,
            Goal::GoodDay(_) => self.totals.best_mood,
            Goal::Upgrade(kind, _) => self
                .lots
                .iter()
                .flatten()
                .filter(|b| b.kind == kind)
                .map(|b| u32::from(b.level))
                .max()
                .unwrap_or(0),
            Goal::Pair(first, second, _) => self
                .lots
                .iter()
                .enumerate()
                .filter(|(lot, b)| {
                    b.is_some_and(|b| b.kind == first) && self.neighboring(*lot, second, 1)
                })
                .count() as u32,
        };
        (value, goal.target())
    }
}

fn resolve_visits(lots: &[Option<Building>; LOTS], day: &mut Day, chapter: &Chapter, until: f32) {
    let mut due: Vec<_> = day
        .visits
        .iter()
        .enumerate()
        .filter(|(_, visit)| !visit.resolved && visit.at <= until)
        .map(|(i, visit)| (i, visit.at))
        .collect();
    if due.len() > 1 {
        due.sort_by(|a, b| a.1.total_cmp(&b.1));
    }
    for (index, _) in due {
        let visit = &day.visits[index];
        let service = visit.wants as usize;
        let result = target(lots, day, visit, chapter);
        let visit = &mut day.visits[index];
        visit.resolved = true;
        match result {
            Ok(shop) => {
                let b = lots[shop].expect("a sale requires a building");
                day.stock[shop] -= 1;
                day.tally.served[service] += 1;
                day.tally.income += b.price.coins(visit.wants);
                day.tally.reputation +=
                    1 + u32::from(b.price == Price::Fair && visit.budget == Price::Fine);
                day.tally.ferry_served += u32::from(visit.origin == Origin::Ferry);
                visit.shop = Some(shop as u8);
            }
            Err(decline) => {
                day.tally.missed[service] += 1;
                day.tally.gaps[decline.reason as usize] += 1;
                visit.miss = Some(decline.reason);
                visit.declined_at = decline.lot;
            }
        }
    }
}

fn finish_route(lots: &[Option<Building>; LOTS], day: &mut Day, route: Route) {
    day.route = None;
    day.nili_at = route.delivery;
    day.tally.routes[route.job as usize] += 1;
    let lot = match route.delivery {
        Stop::Parcel(i) => usize::from(i),
        _ => {
            day.tally.income += 5;
            day.tally.reputation += 2;
            return;
        }
    };
    match route.job {
        Job::Freight => {
            day.stock[lot] += if matches!(route.pickup, Stop::Parcel(_)) {
                6
            } else {
                4
            };
        }
        Job::Water => {
            day.stock[lot] += 4;
            day.water += 2;
        }
        Job::Welcome => {
            if let Some(wants) = lots[lot].and_then(|b| b.kind.service()) {
                for delay in [0.6, 1.8] {
                    day.visits.push(Visit {
                        origin: Origin::Ferry,
                        wants,
                        budget: Price::Regular,
                        at: route.finish + delay,
                        resolved: false,
                        shop: None,
                        miss: None,
                        guided_to: Some(lot as u8),
                        declined_at: None,
                    });
                }
            }
        }
        Job::Shade => day.stock[lot] += 5,
        Job::Quay => {}
    }
    day.tally.income += 3;
    day.tally.reputation += 1 + u32::from(route.job != Job::Freight);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Decline {
    reason: Miss,
    lot: Option<u8>,
}

fn walking_range(
    lots: &[Option<Building>; LOTS],
    origin: Origin,
    lot: usize,
    chapter: &Chapter,
    quay_cleared: bool,
) -> usize {
    match origin {
        Origin::Home(_) => 4,
        Origin::Ferry => {
            chapter.walk
                + 2 * usize::from(quay_cleared)
                + usize::from(lots.iter().enumerate().any(|(i, b)| {
                    b.is_some_and(|b| b.kind == Kind::Forum) && distance(i, lot) <= 2
                }))
        }
    }
}

fn target(
    lots: &[Option<Building>; LOTS],
    day: &Day,
    visit: &Visit,
    chapter: &Chapter,
) -> Result<usize, Decline> {
    let origin = match visit.origin {
        Origin::Home(lot) => usize::from(lot),
        Origin::Ferry => chapter.layout.dock(),
    };
    let mut best = None;
    let mut declined = None;
    let quay_cleared = day.tally.routes[Job::Quay as usize] > 0;
    for (lot, building) in lots.iter().enumerate() {
        if visit.guided_to.is_some_and(|i| usize::from(i) != lot) {
            continue;
        }
        let Some(building) = building.filter(|b| b.kind.service() == Some(visit.wants)) else {
            continue;
        };
        let steps = distance(origin, lot);
        let range = walking_range(lots, visit.origin, lot, chapter, quay_cleared);
        let candidate = (steps as u32 * 2 + building.price.coins(visit.wants), lot);
        let failure = if visit.guided_to.is_none() && steps > range {
            Some(Miss::Walk)
        } else if building.price as u8 > visit.budget as u8 {
            Some(Miss::Price)
        } else if day.stock[lot] == 0 {
            Some(Miss::Empty)
        } else {
            None
        };
        if let Some(reason) = failure {
            let rank = (std::cmp::Reverse(reason as u8), candidate);
            if declined.is_none_or(|(prior, _)| rank < prior) {
                declined = Some((
                    rank,
                    Decline {
                        reason,
                        lot: Some(lot as u8),
                    },
                ));
            }
        } else if best.is_none_or(|score| candidate < score) {
            best = Some(candidate);
        }
    }
    best.map(|(_, lot)| lot).ok_or_else(|| {
        declined.map_or(
            Decline {
                reason: Miss::NoShop,
                lot: None,
            },
            |(_, decline)| decline,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{campaign::CHAPTERS, save::Campaign};
    use Kind::*;

    const PLANS: [&[(usize, Kind)]; 12] = [
        &[(2, Bakery), (8, Well)],
        &[(8, Well), (0, Grove)],
        &[(9, Potter), (8, Well)],
        &[(9, Stoa), (0, Grove), (8, Potter)],
        &[(8, Taverna), (9, Stoa), (10, Potter), (6, Bakery)],
        &[(10, Inn), (2, Grove), (4, Potter), (3, Taverna)],
        &[(6, Potter), (11, Potter), (10, Stoa), (1, Grove), (5, Inn)],
        &[(9, Potter), (11, Inn), (3, Grove)],
        &[
            (3, Grove),
            (9, Taverna),
            (0, House),
            (11, Potter),
            (5, Inn),
            (6, Bakery),
        ],
        &[
            (11, Inn),
            (1, Grove),
            (8, Taverna),
            (10, Potter),
            (6, Well),
            (5, Stoa),
        ],
        &[
            (11, Inn),
            (3, Grove),
            (9, Taverna),
            (10, Potter),
            (5, Stoa),
            (6, Well),
        ],
        &[
            (11, Inn),
            (9, Taverna),
            (3, Grove),
            (10, Stoa),
            (4, Potter),
            (6, Bakery),
        ],
    ];

    #[test]
    fn all_chapters_can_earn_three_stars_and_unlock_the_next() {
        let mut save = Campaign::default();
        for (i, (chapter, builds)) in CHAPTERS.iter().zip(PLANS).enumerate() {
            assert!(save.unlocked(i), "{}", chapter.district);
            let mut town = Town::new(i, chapter);
            let mut result = None;
            for _ in 1..=chapter.par {
                for &(lot, kind) in builds {
                    if town.lots[lot].is_none() && !town.build(chapter, lot, kind) {
                        break;
                    }
                }
                for goal in chapter.goals {
                    if let Goal::Upgrade(kind, level) = goal
                        && let Some(lot) = town.lots.iter().position(|b| {
                            b.is_some_and(|b| b.kind == kind && u32::from(b.level) < level)
                        })
                    {
                        town.upgrade(lot);
                    }
                }
                town = play_market(town, chapter, HarborPlan::Regular, Price::Fair);
                let Phase::Report(report) = &town.phase else {
                    unreachable!()
                };
                if report.stars == 3 {
                    let day = town.day;
                    save.record(i, day, report.stars);
                    result = Some(day);
                    break;
                }
                town.advance();
            }
            assert!(
                result.is_some(),
                "{} fehlte am Richttag: {:?}, {} Bitten, {} Gäste. Gebäude: {:?}",
                chapter.district,
                chapter.goals.map(|goal| (goal.label(), town.goal(goal))),
                town.totals.favors,
                town.totals.served_total(),
                town.lots,
            );
            eprintln!(
                "{}: drei Sterne an Tag {}",
                chapter.district,
                result.unwrap()
            );
        }
    }

    #[test]
    fn save_inside_day_preserves_inventory_prices_and_completed_favor() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.set_price(2, Price::Fair));
        assert!(town.open(chapter));
        town.tick(chapter, 12.0);
        assert!(town.aid(2));
        let serialized = serde_json::to_vec(&town).unwrap();
        let mut restored: Town = serde_json::from_slice(&serialized).unwrap();
        assert!(!restored.aid(2));
        assert_eq!(restored.lots[2].unwrap().price, Price::Fair);
        town.tick(chapter, DAY_SECONDS);
        restored.tick(chapter, DAY_SECONDS);
        assert_eq!(town.coins, restored.coins);
        assert_eq!(town.totals.served, restored.totals.served);
        assert_eq!(restored.totals.favors, 1);
    }

    #[test]
    fn complaints_remain_actionable_the_next_morning_after_a_reload() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.set_price(2, Price::Fine));
        assert!(town.open(chapter));
        assert!(town.tick(chapter, chapter.day_seconds()));
        let guest = town
            .guest_history()
            .iter()
            .position(|v| v.verdict() == Some(Verdict::Missed(Miss::Price)))
            .unwrap();
        assert_eq!(town.guest_history()[guest].declined_at, Some(2));
        assert!(town.guest_tip(chapter, guest).is_none());
        let mut town: Town = serde_json::from_slice(&serde_json::to_vec(&town).unwrap()).unwrap();
        assert!(town.advance());
        assert!(matches!(
            town.guest_tip(chapter, guest),
            Some(GuestTip::Shop(2))
        ));
        assert!(town.remove(2));
        assert!(town.guest_tip(chapter, guest).is_none());
        assert_eq!(
            town.guest_history()[guest].verdict(),
            Some(Verdict::Missed(Miss::Price))
        );
    }

    #[test]
    fn a_missing_shop_points_to_a_buildable_plot_within_walking_range() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.open(chapter));
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert!(town.advance());
        let guest = town
            .guest_history()
            .iter()
            .position(|v| matches!(v.origin, Origin::Home(1)))
            .unwrap();
        let Some(GuestTip::Build(lot, Bakery)) = town.guest_tip(chapter, guest) else {
            panic!("expected a recommendation for a bakery");
        };
        assert!(is_open(chapter, lot));
        assert!(distance(1, lot) <= 4);
        assert!(town.build(chapter, lot, Bakery));
    }

    #[test]
    fn tavern_uses_two_breads_and_rewards_an_adjacent_oven() {
        let chapter = &CHAPTERS[4];
        let mut town = Town::new(4, chapter);
        let bread = town.production(chapter).stock[7];
        assert!(town.build(chapter, 8, Taverna));
        let plan = town.production(chapter);
        assert_eq!(plan.stock[7], bread - 2);
        assert_eq!(plan.stock[8], 7);
        assert_eq!(plan.used_olives, 2);
        assert!(!town.build(chapter, 11, Bakery));
    }

    #[test]
    fn real_visits_use_budget_distance_and_stock() {
        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        assert!(town.build(chapter, 0, Potter));
        let visit = Visit {
            origin: Origin::Ferry,
            wants: Service::Pottery,
            budget: Price::Fair,
            at: 0.0,
            resolved: false,
            shop: None,
            miss: None,
            guided_to: None,
            declined_at: None,
        };
        let review = |town: &Town| {
            let mut market = town.clone();
            assert!(market.open(chapter));
            let Phase::Open(day) = &market.phase else {
                unreachable!()
            };
            target(&market.lots, day, &visit, chapter)
        };
        assert_eq!(
            review(&town),
            Err(Decline {
                reason: Miss::Walk,
                lot: Some(0)
            })
        );
        assert!(town.remove(0));
        assert!(town.build(chapter, 9, Potter));
        assert_eq!(
            review(&town),
            Err(Decline {
                reason: Miss::Price,
                lot: Some(9)
            })
        );
        assert!(town.set_price(9, Price::Fair));
        assert_eq!(review(&town), Ok(9));
        assert!(town.open(chapter));
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.stock[9] = 0;
        assert_eq!(
            target(&town.lots, day, &visit, chapter),
            Err(Decline {
                reason: Miss::Empty,
                lot: Some(9)
            })
        );
    }

    #[test]
    fn a_resumed_nili_route_delivers_once_to_the_chosen_plot() {
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert!(town.build(chapter, 8, Well));
        assert!(town.open(chapter));
        let route = town
            .dispatch(chapter, Job::Water, 7)
            .unwrap_or_else(|_| panic!("reachable water route"));
        assert!(matches!(
            town.dispatch(chapter, Job::Freight, 7),
            Err(Blocked::Busy)
        ));
        assert!(!town.tick(chapter, route.pickup_at + 0.2));
        let packed = serde_json::to_vec(&town).unwrap();
        let mut resumed: Town = serde_json::from_slice(&packed).unwrap();
        let elapsed = route.finish - route.pickup_at;
        for market in [&mut town, &mut resumed] {
            assert!(!market.tick(chapter, elapsed));
            let Phase::Open(day) = &market.phase else {
                unreachable!()
            };
            assert!(day.route.is_none());
            assert_eq!(day.tally.routes[Job::Water as usize], 1);
            assert_eq!(day.nili_at, Stop::Parcel(7));
            assert!(matches!(
                market.dispatch(chapter, Job::Water, 7),
                Err(Blocked::Done)
            ));
            assert!(market.tick(chapter, chapter.day_seconds()));
            assert_eq!(market.goal(Goal::Route(Job::Water, 2)), (1, 2));
        }
        assert_eq!(town.coins, resumed.coins);
        assert_eq!(town.totals.served, resumed.totals.served);
    }

    #[test]
    fn a_guided_party_uses_the_selected_shop_and_normal_prices() {
        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        assert!(town.build(chapter, 0, Potter));
        assert!(town.set_price(0, Price::Fine));
        assert!(town.open(chapter));
        let route = town
            .dispatch(chapter, Job::Welcome, 0)
            .unwrap_or_else(|_| panic!("reachable escort"));
        town.tick(chapter, route.finish + 2.0);
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let party: Vec<_> = day
            .visits
            .iter()
            .filter(|visit| visit.guided_to == Some(0))
            .collect();
        assert_eq!(party.len(), 2);
        assert!(party.iter().all(|visit| {
            visit.resolved && visit.origin == Origin::Ferry && visit.miss == Some(Miss::Price)
        }));
    }

    fn reference_layout(i: usize, chapter: &Chapter) -> Vec<(usize, Kind)> {
        use crate::campaign::{Goal::*, REGION_SIZE};

        if let Some(plan) = PLANS.get(i) {
            return plan.to_vec();
        }
        if i < REGION_SIZE {
            return [
                (6, Storehouse),
                (3, Grove),
                (9, Press),
                (5, Shrine),
                (8, Taverna),
                (4, Potter),
                (10, Stoa),
                (11, Inn),
            ]
            .into_iter()
            .filter(|(_, k)| chapter.allowed().contains(k))
            .collect();
        }
        let aqueduct = if chapter.district == "Tibur" { 1 } else { 4 };
        let mut plan = vec![
            (aqueduct, Aqueduct),
            (10, Baths),
            (5, Forum),
            (11, Inn),
            (9, Potter),
            (8, Taverna),
            (10, Stoa),
        ];
        if chapter.district == "Lavinium" {
            plan.push((3, Villa));
        } else if chapter.goals.iter().any(|g| matches!(g, Build(Baths, 2))) {
            plan.push((3, Baths));
        } else if chapter.goals.iter().any(|g| matches!(g, Build(Forum, 2))) {
            plan.push((6, Forum));
        } else if chapter.goals.iter().any(|g| matches!(g, Build(Bakery, 2))) {
            plan.push((6, Bakery));
        }
        if chapter.district == "Tibur" {
            plan.push((6, Potter));
        }
        plan.push((6, Storehouse));
        plan.into_iter()
            .filter(|(_, k)| chapter.allowed().contains(k))
            .collect()
    }

    fn play_market(mut town: Town, chapter: &Chapter, policy: HarborPlan, tariff: Price) -> Town {
        town.set_harbor(chapter, policy);
        for i in 0..LOTS {
            town.set_price(i, tariff);
        }
        assert!(town.open(chapter));
        town.tick(chapter, 9.0);
        let demand = town.forecast(chapter);
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let best = town
            .lots
            .iter()
            .enumerate()
            .filter_map(|(i, building)| {
                let b = building.filter(|b| day.favor.allows(b.kind))?;
                let deficit = b
                    .kind
                    .service()
                    .map_or(0, |s| demand[s as usize].saturating_sub(day.stock[i]));
                Some((deficit, i))
            })
            .max();
        if let Some((_, lot)) = best {
            assert!(town.aid(lot));
        }
        let distinct_required = chapter
            .goals
            .iter()
            .any(|g| matches!(g, Goal::RouteVariety(_)));
        let mut jobs = Job::ALL;
        jobs.sort_by_key(|&job| {
            let mandatory = chapter.goals.iter().any(|&g| {
                matches!(g, Goal::Route(candidate, count) if candidate == job && town.totals.routes[job as usize] < count)
            });
            let distinct = distinct_required && town.totals.routes[job as usize] == 0;
            (!mandatory, !distinct, job != Job::Quay)
        });
        for job in jobs {
            let Phase::Open(day) = &town.phase else { break };
            let targets = job.targets(&town, chapter);
            let selected = targets
                .iter()
                .filter_map(|&lot| {
                    let planned = job.plan(&town, chapter, lot).ok()?;
                    let service = town.lots[lot].and_then(|b| b.kind.service());
                    let stock = day.stock[lot];
                    let demand = service.map_or(0, |service| {
                        day.visits
                            .iter()
                            .filter(|v| !v.resolved && v.wants == service)
                            .count() as u16
                    });
                    let relevant = service.is_some_and(|s| {
                        chapter
                            .goals
                            .iter()
                            .any(|&g| matches!(g, Goal::Serve(kind, _) if kind == s))
                    });
                    let benefit = if job == Job::Welcome {
                        i32::from(stock.min(2)) * 20 + i32::from(relevant) * 12 - i32::from(demand)
                    } else {
                        i32::from(demand.saturating_sub(stock)) * 7 + i32::from(relevant)
                    };
                    Some((benefit, -(planned.finish as i32), lot))
                })
                .max();
            if let Some((_, _, lot)) = selected {
                let current = day.seconds;
                if let Ok(route) = town.dispatch(chapter, job, lot) {
                    town.tick(chapter, route.finish - current + 0.02);
                }
            }
        }
        town.tick(chapter, chapter.day_seconds());
        town
    }

    #[test]
    fn both_campaigns_have_reachable_objectives_and_distinct_city_plans() {
        use crate::campaign::REGION_SIZE;

        let mut save = Campaign::default();
        assert!(!save.unlocked(REGION_SIZE));
        let mut failures = Vec::new();
        for (i, chapter) in CHAPTERS.iter().enumerate() {
            if failures.is_empty() {
                assert!(save.unlocked(i));
            }
            assert!(
                chapter.starts.iter().all(|&(lot, kind)| {
                    is_open(chapter, lot) && chapter.allowed().contains(&kind)
                }),
                "invalid starter plots in {}",
                chapter.district
            );
            let mut town = Town::new(i, chapter);
            let layout = reference_layout(i, chapter);
            let mut result = None;
            for _ in 0..chapter.par + 8 {
                for &(lot, kind) in &layout {
                    if town.lots[lot].is_none() && !town.build(chapter, lot, kind) {
                        break;
                    }
                }
                for goal in chapter.goals {
                    if let Goal::Upgrade(kind, level) = goal
                        && let Some(lot) = town.lots.iter().position(|b| {
                            b.is_some_and(|b| b.kind == kind && u32::from(b.level) < level)
                        })
                    {
                        town.upgrade(lot);
                    }
                }
                for desired in [
                    Well, Aqueduct, Grove, House, Villa, Bakery, Potter, Press, Shrine, Baths, Inn,
                    Taverna, Forum, Stoa,
                ] {
                    for lot in 0..LOTS {
                        if town.day >= 3
                            && town.coins > 27
                            && town.lots[lot].is_some_and(|b| b.kind == desired)
                        {
                            town.upgrade(lot);
                        }
                    }
                }
                let policies = if chapter.harbor_plans() {
                    &HarborPlan::ALL[..]
                } else {
                    &[HarborPlan::Regular]
                };
                let mut best: Option<(f64, Town)> = None;
                for &policy in policies {
                    for tariff in Price::ALL {
                        let next = play_market(town.clone(), chapter, policy, tariff);
                        let progress: f64 = chapter
                            .goals
                            .iter()
                            .map(|&g| {
                                let (old, need) = town.goal(g);
                                let (new, _) = next.goal(g);
                                f64::from(new.min(need).saturating_sub(old.min(need)))
                                    / f64::from(need)
                            })
                            .sum();
                        let Phase::Report(report) = &next.phase else {
                            unreachable!()
                        };
                        let score = progress * 2400.0
                            + f64::from(report.tally.served_total())
                            + f64::from(report.tally.income) * 0.18;
                        if best.as_ref().is_none_or(|(prior, _)| score > *prior) {
                            best = Some((score, next));
                        }
                    }
                }
                town = best.expect("a policy is always available").1;
                let Phase::Report(report) = &town.phase else {
                    unreachable!()
                };
                if report.stars > 0 {
                    save.record(i, town.day, report.stars);
                    result = Some((town.day, report.stars));
                    break;
                }
                town.advance();
            }
            let Some((days, stars)) = result else {
                failures.push(format!(
                    "{}: {:?}",
                    chapter.district,
                    chapter.goals.map(|g| (g.label(), town.goal(g)))
                ));
                continue;
            };
            eprintln!(
                "{:02} {:<15} {days} / {} Tage, {stars} Sterne",
                i + 1,
                chapter.district,
                chapter.par
            );
        }
        assert!(
            failures.is_empty(),
            "Unfinished routes: {}",
            failures.join("\n")
        );
    }
}
