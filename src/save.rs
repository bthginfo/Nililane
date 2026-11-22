use std::{fs, io, path::PathBuf};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::{
    campaign::CHAPTERS,
    coach::ISLANDS,
    cosmetics::{Cosmetic, Wardrobe},
    model::{MAX_LEVEL, Origin, Phase, Town, is_open},
};

const VERSION: u32 = 2;

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub struct Record {
    pub stars: u8,
    pub best_day: u32,
    #[serde(default)]
    pub best_income: u32,
}

#[derive(Resource, Deserialize, Serialize)]
pub struct Campaign {
    version: u32,
    pub records: Vec<Record>,
    pub town: Option<Town>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tutorial: Option<Town>,
    pub music: bool,
    pub sounds: bool,
    pub large_ui: bool,
    #[serde(default, skip_serializing_if = "Wardrobe::is_empty")]
    pub wardrobe: Wardrobe,
}

impl Default for Campaign {
    fn default() -> Self {
        Self {
            version: VERSION,
            records: vec![Record::default(); CHAPTERS.len()],
            town: None,
            tutorial: None,
            music: true,
            sounds: true,
            large_ui: false,
            wardrobe: Wardrobe::default(),
        }
    }
}

impl Campaign {
    pub fn load() -> Self {
        let Ok(file) = fs::File::open(path()) else {
            return Self::default();
        };
        Self::decode(file).unwrap_or_default()
    }

    pub(crate) fn decode(input: impl io::Read) -> Option<Self> {
        let mut progress: Self = serde_json::from_reader(input).ok()?;
        if !(1..=VERSION).contains(&progress.version)
            || !progress.town.as_ref().is_none_or(valid_town)
        {
            return None;
        }
        if progress
            .tutorial
            .as_ref()
            .is_some_and(|town| town.chapter >= ISLANDS || !valid_town(town))
        {
            progress.tutorial = None;
        }
        if progress.version == 1
            && let Some(town) = &mut progress.town
        {
            if let Some(tally) = town
                .last_day
                .as_ref()
                .filter(|tally| tally.responses() >= 12)
            {
                town.totals.best_mood = tally.satisfaction().unwrap_or(0);
            }
            if let Phase::Open(day) = &mut town.phase {
                day.tally.ferry_served = day
                    .visits
                    .iter()
                    .filter(|visit| visit.origin == Origin::Ferry && visit.shop.is_some())
                    .count() as u32;
                if let Some(lot) = day.favor_at {
                    day.nili_at = crate::errands::Stop::Parcel(lot);
                }
            }
        }
        for town in progress.town.iter_mut().chain(&mut progress.tutorial) {
            let chapter = town.chapter;
            let adjust = |visits: &mut [crate::model::Visit]| {
                for (index, visit) in visits.iter_mut().enumerate() {
                    let age = visit.age(index).on_island(chapter);
                    if !visit.age(index).available(chapter) {
                        visit.age = Some(age);
                    }
                }
            };
            if let Phase::Open(day) = &mut town.phase {
                adjust(&mut day.visits);
            }
            adjust(&mut town.last_visits);
        }
        progress.version = VERSION;
        progress.records.resize(CHAPTERS.len(), Record::default());
        Some(progress)
    }

    pub fn save(&self) -> io::Result<()> {
        use io::Write;
        let file = path();
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        let temporary = file.with_extension("tmp");
        let mut output = fs::File::create(&temporary)?;
        serde_json::to_writer(&mut output, self)?;
        output.flush()?;
        output.sync_all()?;
        fs::rename(temporary, file)
    }

    pub fn unlocked(&self, chapter: usize) -> bool {
        chapter < CHAPTERS.len() && (chapter == 0 || self.records[chapter - 1].stars > 0)
    }

    pub fn active_town(&self) -> Option<&Town> {
        self.tutorial.as_ref().or(self.town.as_ref())
    }

    pub fn active_town_mut(&mut self) -> Option<&mut Town> {
        self.tutorial.as_mut().or(self.town.as_mut())
    }

    pub fn in_tutorial(&self) -> bool {
        self.tutorial.is_some()
    }

    pub fn enter_town(&mut self, town: Town, tutorial: bool) {
        if tutorial {
            self.tutorial = Some(town);
        } else {
            self.tutorial = None;
            self.town = Some(town);
        }
    }

    pub fn leave_tutorial(&mut self) -> bool {
        self.tutorial.take().is_some()
    }

    pub fn record(&mut self, chapter: usize, day: u32, stars: u8) {
        if self.in_tutorial() {
            return;
        }
        let record = &mut self.records[chapter];
        if stars > record.stars || (stars == record.stars && day < record.best_day) {
            record.stars = stars;
            record.best_day = day;
        }
    }

    pub fn record_income(&mut self, chapter: usize, income: u32) {
        if !self.in_tutorial() {
            let record = &mut self.records[chapter];
            record.best_income = record.best_income.max(income);
        }
    }

    pub fn travel_bonus(&self, next: usize) -> u32 {
        next.checked_sub(1)
            .and_then(|prior| self.records.get(prior))
            .filter(|record| record.stars > 0)
            .map_or(0, |record| travel_bonus(next, record.best_income))
    }

    pub fn available_stars(&self) -> u16 {
        let earned = self
            .records
            .iter()
            .map(|record| u16::from(record.stars.min(3)))
            .sum::<u16>();
        earned.saturating_sub(self.wardrobe.spent())
    }

    pub fn equip(&mut self, item: Cosmetic) -> bool {
        self.wardrobe.equip(item, self.available_stars())
    }
}

pub fn travel_bonus(next: usize, income: u32) -> u32 {
    CHAPTERS.get(next).map_or(0, |chapter| {
        (income.saturating_mul(15) / 100).min((chapter.coins / 3).clamp(12, 36))
    })
}

fn valid_town(town: &Town) -> bool {
    let Some(chapter) = CHAPTERS.get(town.chapter) else {
        return false;
    };
    if town.day == 0
        || !town.lots.iter().enumerate().all(|(i, b)| {
            b.is_none_or(|b| is_open(chapter, i) && (1..=MAX_LEVEL).contains(&b.level))
        })
    {
        return false;
    }
    let valid_slot = |slot| is_open(chapter, usize::from(slot));
    if !town
        .last_visits
        .iter()
        .all(|visit| visit.resolved && valid_visit(visit, chapter))
    {
        return false;
    }
    if let Phase::Open(day) = &town.phase {
        return (0.0..chapter.day_seconds()).contains(&day.seconds)
            && day.visits.iter().all(|visit| valid_visit(visit, chapter))
            && day.favor_at.is_none_or(valid_slot)
            && day.nili_at.valid(town, chapter)
            && day.nili.is_none_or(crate::errands::NiliControl::valid)
            && day
                .route
                .is_none_or(|route| route.valid(town, chapter, day.seconds, day.nili.is_some()));
    }
    true
}

fn valid_visit(visit: &crate::model::Visit, chapter: &crate::campaign::Chapter) -> bool {
    let valid_slot = |slot| is_open(chapter, usize::from(slot));
    (0.0..=chapter.day_seconds()).contains(&visit.at)
        && visit.shop.is_none_or(&valid_slot)
        && visit.guided_to.is_none_or(&valid_slot)
        && visit.declined_at.is_none_or(&valid_slot)
        && match visit.origin {
            Origin::Home(slot) => valid_slot(slot),
            Origin::Ferry => true,
        }
}

fn path() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        if let Some(dir) = bevy::window::ANDROID_APP
            .get()
            .and_then(|app| app.internal_data_path())
        {
            return dir.join("nililane-v1.json");
        }
    }
    if let Some(path) = std::env::var_os("NILILANE_SAVE_DIR") {
        return PathBuf::from(path).join("nililane-v1.json");
    }
    desktop_dir().join("nililane").join("nililane-v1.json")
}

fn desktop_dir() -> PathBuf {
    if let Some(path) =
        std::env::var_os("XDG_DATA_HOME").or_else(|| std::env::var_os("LOCALAPPDATA"))
    {
        return path.into();
    }
    if let Some(path) = std::env::var_os("HOME") {
        return PathBuf::from(path).join(".local/share");
    }
    PathBuf::from(".")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Kind, Price};

    #[test]
    fn wardrobe_spends_only_newly_earned_stars_and_can_rewear_items() {
        use Cosmetic::*;
        let mut progress = Campaign::default();
        assert!(!progress.equip(Sea));
        progress.record(0, 3, 3);
        assert!(progress.equip(Sea));
        assert_eq!(progress.available_stars(), 0);
        assert_eq!(progress.records[0].stars, 3);
        assert!(progress.unlocked(1));
        assert!(progress.equip(Sea));
        assert!(!progress.wardrobe.wearing(Sea));
        assert!(progress.equip(Sea));
        progress.record(1, 4, 2);
        progress.record(1, 3, 3);
        assert!(!progress.equip(Straw));
        progress.record(2, 4, 1);
        assert!(progress.equip(Straw));
        let data = serde_json::to_vec(&progress).unwrap();
        let progress = Campaign::decode(data.as_slice()).unwrap();
        assert_eq!(progress.available_stars(), 0);
        assert!(progress.wardrobe.wearing(Sea));
        assert!(progress.wardrobe.wearing(Straw));
        assert!(progress.unlocked(3));
    }

    #[test]
    fn a_controlled_delivery_runs_late_and_is_paid_once_after_reload() {
        use crate::errands::{Job, ManualStage, NiliControl};
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert!(town.build(chapter, 8, Kind::Well));
        assert!(town.open(chapter));
        town.tick(chapter, 3.0);
        let route = town.dispatch(chapter, Job::Water, 7).unwrap();
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.nili = Some(NiliControl {
            position: [-1.6, 0.8],
            yaw: 0.0,
            stage: ManualStage::Delivery,
        });
        let due = route.finish + 0.5;
        assert!(!town.tick(chapter, due - 3.0));
        let progress = Campaign {
            town: Some(town),
            ..Campaign::default()
        };
        let data = serde_json::to_vec(&progress).unwrap();
        let mut town = Campaign::decode(data.as_slice()).unwrap().town.unwrap();
        assert!(town.finish_controlled(chapter).is_none());
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        assert!((day.seconds - due).abs() < 0.001);
        let nili = day.nili.as_mut().unwrap();
        assert_eq!(nili.position, [-1.6, 0.8]);
        nili.stage = ManualStage::Unload(1.0);
        assert!(town.finish_controlled(chapter).is_some());
        assert!(town.finish_controlled(chapter).is_none());
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        assert!((day.seconds - due).abs() < 0.001);
        assert_eq!(day.tally.routes[Job::Water as usize], 1);
        assert!(day.route.is_none());
    }

    #[test]
    fn repeating_the_tutorial_preserves_the_campaign_across_reloads() {
        let mut progress = Campaign::default();
        progress.record(0, 4, 2);
        let mut town = Town::new(1, &CHAPTERS[1]);
        assert!(town.build(&CHAPTERS[1], 8, Kind::Well));
        assert!(town.open(&CHAPTERS[1]));
        town.tick(&CHAPTERS[1], 2.0);
        progress.enter_town(town, false);
        let campaign = serde_json::to_value(&progress).unwrap();

        for (chapter, rule) in CHAPTERS.iter().enumerate().take(ISLANDS) {
            progress.enter_town(Town::new(chapter, rule), true);
            let town = progress.active_town_mut().unwrap();
            assert_eq!(town.chapter, chapter);
            assert_eq!(town.lessons, 0);
            town.lessons |= crate::coach::Moment::Build.bit();
            progress.record(chapter, 2, 3);

            let data = serde_json::to_vec(&progress).unwrap();
            progress = Campaign::decode(data.as_slice()).unwrap();
            let town = progress.active_town().unwrap();
            assert_eq!(town.chapter, chapter);
            assert_eq!(town.lessons, crate::coach::Moment::Build.bit());
        }
        assert!(!progress.unlocked(2));
        assert!(progress.leave_tutorial());
        assert_eq!(serde_json::to_value(&progress).unwrap(), campaign);

        progress.enter_town(Town::new(0, &CHAPTERS[0]), true);
        let town = progress.active_town().unwrap();
        assert_eq!(town.chapter, 0);
        assert_eq!(town.day, 1);
        assert_eq!(town.lessons, 0);
    }

    #[test]
    fn a_paused_lesson_and_an_unfinished_favor_survive_a_reload() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        let seen = crate::coach::Moment::Ferry.bit();
        town.lessons |= seen;
        assert!(town.build(chapter, 2, Kind::Bakery));
        assert!(town.open(chapter));
        town.tick(chapter, 9.1);
        assert!(town.aid(chapter, 2));
        let progress = Campaign {
            town: Some(town),
            ..Campaign::default()
        };
        let data = serde_json::to_vec(&progress).unwrap();
        let mut town = Campaign::decode(data.as_slice()).unwrap().town.unwrap();
        assert_eq!(town.lessons, seen);
        town.tick(chapter, 60.0);
        assert_eq!(town.totals.favors, 1);
        assert_eq!(town.totals.routes_today(), 0);
    }

    #[test]
    fn version_one_open_day_keeps_ids_inventory_and_progress() {
        let mut progress = Campaign::default();
        progress.record(0, 3, 3);
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert!(town.build(chapter, 8, Kind::Well));
        assert!(town.set_price(7, Price::Fair));
        assert!(town.open(chapter));
        town.tick(chapter, 29.0);
        let route = town.favor_plan(chapter, 8).unwrap();
        assert!(town.aid(chapter, 8));
        town.tick(chapter, route.finish - route.started + 0.02);
        progress.town = Some(town.clone());
        let mut old = serde_json::to_value(&progress).unwrap();
        old["version"] = 1.into();
        old["town"]["lots"].as_array_mut().unwrap().truncate(12);
        old["town"].as_object_mut().unwrap().remove("lessons");
        let old_day = old["town"]["phase"]["Open"].as_object_mut().unwrap();
        old_day
            .get_mut("stock")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .truncate(12);
        old_day.remove("nili_at");
        old_day.remove("route");
        old_day
            .get_mut("tally")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove("routes");
        let data = serde_json::to_vec(&old).unwrap();
        let resumed = Campaign::decode(data.as_slice()).unwrap();
        assert_eq!(resumed.version, 2);
        assert!(resumed.unlocked(1));
        assert_eq!(resumed.records[0].stars, 3);
        let town2 = resumed.town.unwrap();
        assert_eq!(town.lots, town2.lots);
        let (Phase::Open(before), Phase::Open(after)) = (town.phase, town2.phase) else {
            unreachable!()
        };
        assert_eq!(before.stock, after.stock);
        assert_eq!(before.seconds, after.seconds);
        assert_eq!(after.nili_at, crate::errands::Stop::Parcel(8));
        assert_eq!(after.favor_at, Some(8));
    }
}
