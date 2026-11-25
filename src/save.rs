use std::{fs, io, path::PathBuf};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::{
    campaign::{CHAPTERS, previous_harbor},
    coach::ISLANDS,
    cosmetics::{Cosmetic, Wardrobe},
    model::{MAX_LEVEL, Origin, Phase, Town, is_open},
};

const VERSION: u32 = 2;

/// The loudest volume step (100 %); step 0 is off.
pub const LOUDEST: u8 = 4;

/// A volume control in the settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    /// Harbor, menu and adventure music.
    Music,
    /// The sea, the shore and the rain.
    Ambience,
    /// Interface and game cues.
    Cues,
}

impl Channel {
    pub const ALL: [Self; 3] = [Self::Music, Self::Ambience, Self::Cues];

    pub fn name(self) -> &'static str {
        match self {
            Self::Music => "Musik",
            Self::Ambience => "Umgebung",
            Self::Cues => "Klänge",
        }
    }
}

/// The amplitude factor of a volume step. The steps follow loudness rather
/// than amplitude, so 50 % sounds about half as loud instead of barely
/// quieter: −16, −9, −4 and 0 dB.
pub fn gain(step: u8) -> f32 {
    const DECIBELS: [f32; LOUDEST as usize] = [-16.0, -9.0, -4.0, 0.0];
    match step.min(LOUDEST) {
        0 => 0.0,
        step => 10f32.powf(DECIBELS[usize::from(step) - 1] / 20.0),
    }
}

/// How a volume step reads in the settings.
pub fn volume_label(step: u8) -> String {
    match step.min(LOUDEST) {
        0 => "Aus".to_owned(),
        step => format!("{} %", u32::from(step) * 100 / u32::from(LOUDEST)),
    }
}

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    adventures: Vec<usize>,
    pub town: Option<Town>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    moored: Vec<Town>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tutorial: Option<Town>,
    /// The on/off switches of older versions. They are still written, so an
    /// older version keeps playing what the player hears now; the volume
    /// steps below take precedence once set.
    music: bool,
    sounds: bool,
    pub large_ui: bool,
    #[serde(default, skip_serializing_if = "Wardrobe::is_empty")]
    pub wardrobe: Wardrobe,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    music_volume: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ambience_volume: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sound_volume: Option<u8>,
}

impl Default for Campaign {
    fn default() -> Self {
        Self {
            version: VERSION,
            records: vec![Record::default(); CHAPTERS.len()],
            adventures: Vec::new(),
            town: None,
            moored: Vec::new(),
            tutorial: None,
            music: true,
            sounds: true,
            large_ui: false,
            wardrobe: Wardrobe::default(),
            music_volume: None,
            ambience_volume: None,
            sound_volume: None,
        }
    }
}

impl Campaign {
    pub fn load() -> Self {
        Self::load_from(&path())
    }

    fn load_from(file: &std::path::Path) -> Self {
        let Ok(bytes) = fs::read(file) else {
            return Self::default();
        };
        Self::decode(bytes.as_slice()).unwrap_or_else(|| {
            // Keep the unreadable file so the next save cannot erase it for good.
            if let Err(error) = fs::copy(file, backup_path(file)) {
                bevy::log::warn!("Nililane: {error}");
            }
            Self::default()
        })
    }

    pub(crate) fn decode(input: impl io::Read) -> Option<Self> {
        let mut progress: Self = serde_json::from_reader(input).ok()?;
        if !(1..=VERSION).contains(&progress.version) {
            return None;
        }
        // A damaged harbor must not cost the whole campaign: repair it or let it go.
        progress.town = progress.town.take().and_then(repair_town);
        progress.tutorial = progress
            .tutorial
            .take()
            .filter(|town| town.chapter < ISLANDS)
            .and_then(repair_town);
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
        let current = progress.town.as_ref().map(|town| town.chapter);
        let mut seen = [false; crate::campaign::CHAPTER_COUNT];
        progress.moored = std::mem::take(&mut progress.moored)
            .into_iter()
            .filter_map(repair_town)
            .filter(|town| {
                if Some(town.chapter) == current || seen[town.chapter] {
                    return false;
                }
                seen[town.chapter] = true;
                true
            })
            .collect();
        for town in progress
            .town
            .iter_mut()
            .chain(&mut progress.tutorial)
            .chain(&mut progress.moored)
        {
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
                if day.supplied.is_none() {
                    day.supplied = Some(std::array::from_fn(|lot| day.stock_capacity(lot)));
                }
            }
            adjust(&mut town.last_visits);
        }
        progress.version = VERSION;
        progress.records.resize(CHAPTERS.len(), Record::default());
        progress
            .adventures
            .retain(|&index| index < crate::adventure::LEVELS);
        progress.adventures.sort_unstable();
        progress.adventures.dedup();
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

    /// The volume step (0 = off, `LOUDEST` = 100 %) of a channel. Saves from
    /// before the volume steps map their switches: on is 100 %, off is off.
    /// The old sound switch covered both the ambience and the cues.
    pub fn volume(&self, channel: Channel) -> u8 {
        let (step, switch) = match channel {
            Channel::Music => (self.music_volume, self.music),
            Channel::Ambience => (self.ambience_volume, self.sounds),
            Channel::Cues => (self.sound_volume, self.sounds),
        };
        step.unwrap_or(if switch { LOUDEST } else { 0 })
            .min(LOUDEST)
    }

    pub fn set_volume(&mut self, channel: Channel, step: u8) {
        let mut steps = Channel::ALL.map(|channel| self.volume(channel));
        steps[channel as usize] = step.min(LOUDEST);
        let [music, ambience, cues] = steps;
        self.music_volume = Some(music);
        self.ambience_volume = Some(ambience);
        self.sound_volume = Some(cues);
        self.music = music > 0;
        self.sounds = ambience > 0 || cues > 0;
    }

    /// Moves a channel one step louder or quieter; false at either end.
    pub fn step_volume(&mut self, channel: Channel, louder: bool) -> bool {
        let step = self.volume(channel);
        let next = if louder {
            (step + 1).min(LOUDEST)
        } else {
            step.saturating_sub(1)
        };
        self.set_volume(channel, next);
        next != step
    }

    /// One step louder, from 100 % back to off; a tap on the control does this.
    pub fn cycle_volume(&mut self, channel: Channel) {
        let step = self.volume(channel);
        self.set_volume(channel, if step >= LOUDEST { 0 } else { step + 1 });
    }

    pub fn unlocked(&self, chapter: usize) -> bool {
        let cleared = |index| {
            self.records
                .get(index)
                .is_some_and(|record: &Record| record.stars > 0)
        };
        chapter < CHAPTERS.len()
            && (chapter == 0
                || cleared(chapter)
                || previous_harbor(chapter).is_some_and(cleared)
                || chapter.checked_sub(1).is_some_and(cleared))
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
            let chapter = town.chapter;
            if let Some(prior) = self.town.replace(town)
                && prior.chapter != chapter
                && !matches!(&prior.phase, Phase::Report(result) if result.stars > 0)
            {
                self.moored.retain(|town| town.chapter != prior.chapter);
                self.moored.push(prior);
            }
        }
    }

    pub fn in_progress(&self, chapter: usize) -> bool {
        self.town.as_ref().is_some_and(|town| {
            town.chapter == chapter
                && !matches!(&town.phase, Phase::Report(result) if result.stars > 0)
        }) || self.moored.iter().any(|town| town.chapter == chapter)
    }

    pub fn take_moored(&mut self, chapter: usize) -> Option<Town> {
        let index = self
            .moored
            .iter()
            .position(|town| town.chapter == chapter)?;
        Some(self.moored.swap_remove(index))
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
        [previous_harbor(next), next.checked_sub(1)]
            .into_iter()
            .flatten()
            .filter_map(|prior| self.records.get(prior))
            .filter(|record| record.stars > 0)
            .map(|record| travel_bonus(next, record.best_income))
            .max()
            .unwrap_or(0)
    }

    pub fn reward_adventure(&mut self, island: usize) -> bool {
        if self.adventures.contains(&island) {
            return false;
        }
        self.adventures.push(island);
        true
    }

    pub fn available_stars(&self) -> u16 {
        let earned = self
            .records
            .iter()
            .map(|record| u16::from(record.stars.min(3)))
            .sum::<u16>();
        (earned + 2 * self.adventures.len() as u16).saturating_sub(self.wardrobe.spent())
    }

    pub fn equip(&mut self, item: Cosmetic) -> bool {
        self.cosmetic_available(item) && self.wardrobe.equip(item, self.available_stars())
    }

    pub fn cosmetic_available(&self, item: Cosmetic) -> bool {
        self.wardrobe.owns(item)
            || item
                .requires_adventure()
                .is_some_and(|index| self.adventures.contains(&index))
            || item.requires_chapter().is_none_or(|chapter| {
                self.records
                    .get(chapter)
                    .is_some_and(|record| record.stars > 0)
            })
    }
}

#[cfg(test)]
mod japan_progress_tests {
    use super::*;
    use crate::campaign::{JAPAN_START, NORDIC_START};

    #[test]
    fn finishing_twelve_harbors_opens_the_next_region_and_the_bonus_route() {
        use crate::campaign::{CORE_HARBORS, REGION_SIZE, Region, next_harbor};
        let mut save = Campaign::default();
        for region in Region::ALL {
            let start = region.start();
            let last = start + CORE_HARBORS - 1;
            assert!(!save.unlocked(last + 1));
            let onward = next_harbor(last);
            assert_eq!(
                onward,
                (region != Region::Oasis).then_some(start + REGION_SIZE)
            );
            assert!(onward.is_none_or(|index| !save.unlocked(index)));
            for chapter in start..=last {
                assert!(save.unlocked(chapter));
                save.record(chapter, 8, 1);
            }
            save.record_income(last, 1450);
            assert!(save.unlocked(last + 1));
            assert!(!save.unlocked(last + 2));
            if let Some(index) = onward {
                assert!(save.unlocked(index));
                assert!(save.travel_bonus(index) > 0);
            }
            assert_eq!(next_harbor(start + REGION_SIZE - 1), None);
        }
    }

    #[test]
    fn switching_routes_keeps_an_unfinished_harbor_and_market_after_reloading() {
        use crate::campaign::{CORE_HARBORS, REGION_SIZE};
        let mut save = Campaign::default();
        save.record(CORE_HARBORS - 1, 8, 1);
        let bonus = CORE_HARBORS;
        let mut rome = Town::new(REGION_SIZE, &CHAPTERS[REGION_SIZE]);
        assert!(rome.open(&CHAPTERS[REGION_SIZE]));
        rome.tick(&CHAPTERS[REGION_SIZE], 15.0);
        save.enter_town(rome, false);
        save.enter_town(Town::new(bonus, &CHAPTERS[bonus]), false);
        assert!(save.in_progress(REGION_SIZE));
        let bytes = serde_json::to_vec(&save).unwrap();
        let mut resumed = Campaign::decode(bytes.as_slice()).unwrap();
        let town = resumed.take_moored(REGION_SIZE).unwrap();
        assert!(matches!(&town.phase, Phase::Open(day) if day.seconds == 15.0));
        resumed.enter_town(town, false);
        assert!(resumed.in_progress(bonus));
    }

    #[test]
    fn an_old_campaign_opens_japan_after_rome_and_keeps_outfits_locked_until_their_islands() {
        let mut old = Campaign::default();
        old.records.truncate(JAPAN_START);
        old.record(0, 5, 3);
        old.record(1, 5, 3);
        assert!(old.equip(Cosmetic::Sea));
        let load = |campaign: &Campaign| {
            let json = serde_json::to_vec(campaign).unwrap();
            Campaign::decode(&json[..]).unwrap()
        };

        let locked = load(&old);
        assert_eq!(locked.records.len(), CHAPTERS.len());
        assert!(!locked.unlocked(JAPAN_START));
        assert!(locked.wardrobe.wearing(Cosmetic::Sea));

        old.record(JAPAN_START - 1, 18, 3);
        let mut ready = load(&old);
        assert!(ready.unlocked(JAPAN_START));
        assert!(!ready.unlocked(JAPAN_START + 1));
        assert!(ready.available_stars() >= Cosmetic::IndigoKosode.cost());
        assert!(!ready.equip(Cosmetic::IndigoKosode));
        ready.record(JAPAN_START, 4, 2);
        assert!(ready.unlocked(JAPAN_START + 1));
        assert!(ready.equip(Cosmetic::IndigoKosode));
        assert!(!ready.equip(Cosmetic::Haori));
        assert!(!ready.unlocked(NORDIC_START));
        ready.record(NORDIC_START - 1, 17, 3);
        assert!(ready.unlocked(NORDIC_START));
        assert!(!ready.equip(Cosmetic::FjordTunic));
        ready.record(NORDIC_START, 4, 3);
        assert!(ready.equip(Cosmetic::FjordTunic));
        assert!(ready.wardrobe.owns(Cosmetic::IndigoKosode));
        assert!(ready.equip(Cosmetic::IndigoKosode));
        let upgraded = load(&ready);
        assert!(upgraded.wardrobe.wearing(Cosmetic::IndigoKosode));
        assert!(upgraded.wardrobe.owns(Cosmetic::FjordTunic));
        assert_eq!(upgraded.records[JAPAN_START - 1].best_day, 18);
    }
}

pub fn travel_bonus(next: usize, income: u32) -> u32 {
    CHAPTERS.get(next).map_or(0, |chapter| {
        (income.saturating_mul(15) / 100).min((chapter.coins / 3).clamp(12, 36))
    })
}

fn backup_path(file: &std::path::Path) -> PathBuf {
    let mut name = file.file_name().unwrap_or_default().to_os_string();
    name.push(".bak");
    file.with_file_name(name)
}

// Keeps a harbor whose day went wrong: its buildings, coins and totals survive and
// the damaged parts restart from the morning plan.
fn repair_town(mut town: Town) -> Option<Town> {
    if valid_town(&town) {
        return Some(town);
    }
    let chapter = CHAPTERS.get(town.chapter)?;
    town.day = town.day.max(1);
    for (lot, building) in town.lots.iter_mut().enumerate() {
        if building.is_some_and(|b| !is_open(chapter, lot) || !(1..=MAX_LEVEL).contains(&b.level)) {
            *building = None;
        }
    }
    if town
        .voyage
        .is_some_and(|voyage| !voyage.valid(&town, chapter))
    {
        town.voyage = None;
    }
    if !town
        .last_visits
        .iter()
        .all(|visit| visit.resolved && valid_visit(visit, chapter))
    {
        town.last_visits.clear();
    }
    if matches!(town.phase, Phase::Open(_)) && !valid_town(&town) {
        town.phase = Phase::Plan;
    }
    valid_town(&town).then_some(town)
}

fn valid_town(town: &Town) -> bool {
    let Some(chapter) = CHAPTERS.get(town.chapter) else {
        return false;
    };
    if town.day == 0
        || town
            .voyage
            .is_some_and(|voyage| !voyage.valid(town, chapter))
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

pub(crate) fn path() -> PathBuf {
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
    fn a_damaged_harbor_is_repaired_without_losing_records_or_outfits() {
        let mut progress = Campaign::default();
        progress.record(0, 3, 3);
        progress.record(1, 4, 2);
        assert!(progress.equip(Cosmetic::Sea));
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert!(town.build(chapter, 8, Kind::Well));
        assert!(town.open(chapter));
        town.tick(chapter, 5.0);
        progress.town = Some(town);
        let mut damaged = serde_json::to_value(&progress).unwrap();
        damaged["town"]["phase"]["Open"]["seconds"] = 9999.0.into();
        let data = serde_json::to_vec(&damaged).unwrap();
        let repaired = Campaign::decode(data.as_slice()).unwrap();
        assert_eq!(repaired.records[0].stars, 3);
        assert_eq!(repaired.records[1].stars, 2);
        assert!(repaired.wardrobe.wearing(Cosmetic::Sea));
        let town = repaired.town.as_ref().unwrap();
        assert!(town.planning());
        assert_eq!(town.lots[8].unwrap().kind, Kind::Well);

        damaged["town"]["chapter"] = 9999.into();
        let data = serde_json::to_vec(&damaged).unwrap();
        let without = Campaign::decode(data.as_slice()).unwrap();
        assert!(without.town.is_none());
        assert_eq!(without.records[0].stars, 3);
        assert!(without.wardrobe.wearing(Cosmetic::Sea));
    }

    #[test]
    fn an_unreadable_save_is_backed_up_before_starting_fresh() {
        let dir = std::env::temp_dir().join(format!("nililane-backup-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("nililane-v1.json");
        fs::write(&file, b"{\"version\": 2, \"records\": [").unwrap();
        let progress = Campaign::load_from(&file);
        assert_eq!(progress.available_stars(), 0);
        let backup = dir.join("nililane-v1.json.bak");
        assert_eq!(fs::read(&backup).unwrap(), fs::read(&file).unwrap());
        fs::remove_dir_all(&dir).unwrap();
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

#[cfg(test)]
mod settings_tests {
    use super::*;

    fn decode(json: serde_json::Value) -> Campaign {
        Campaign::decode(serde_json::to_vec(&json).unwrap().as_slice()).unwrap()
    }

    /// A save as the versions before the volume steps wrote it.
    fn switches(music: bool, sounds: bool) -> serde_json::Value {
        let mut old = serde_json::to_value(Campaign::default()).unwrap();
        old["music"] = music.into();
        old["sounds"] = sounds.into();
        for field in ["music_volume", "ambience_volume", "sound_volume"] {
            assert!(old.get(field).is_none(), "{field} is written by default");
        }
        old
    }

    #[test]
    fn old_switches_map_to_full_volume_or_off() {
        let quiet = decode(switches(false, true));
        assert_eq!(quiet.volume(Channel::Music), 0);
        assert_eq!(quiet.volume(Channel::Ambience), LOUDEST);
        assert_eq!(quiet.volume(Channel::Cues), LOUDEST);

        let silent = decode(switches(true, false));
        assert_eq!(silent.volume(Channel::Music), LOUDEST);
        assert_eq!(silent.volume(Channel::Ambience), 0);
        assert_eq!(silent.volume(Channel::Cues), 0);

        let fresh = Campaign::default();
        for channel in Channel::ALL {
            assert_eq!(fresh.volume(channel), LOUDEST);
        }
    }

    #[test]
    fn volume_steps_survive_a_reload_and_keep_the_old_switches_in_step() {
        let mut progress = decode(switches(true, true));
        progress.set_volume(Channel::Music, 2);
        progress.set_volume(Channel::Cues, 0);
        progress.set_volume(Channel::Ambience, 1);
        let json = serde_json::to_value(&progress).unwrap();
        // An older version reads only the switches.
        assert_eq!(json["music"], true);
        assert_eq!(json["sounds"], true);
        let loaded = decode(json);
        assert_eq!(loaded.volume(Channel::Music), 2);
        assert_eq!(loaded.volume(Channel::Ambience), 1);
        assert_eq!(loaded.volume(Channel::Cues), 0);

        let mut muted = loaded;
        muted.set_volume(Channel::Music, 0);
        muted.set_volume(Channel::Ambience, 0);
        let json = serde_json::to_value(&muted).unwrap();
        assert_eq!(json["music"], false);
        assert_eq!(json["sounds"], false);
    }

    #[test]
    fn a_damaged_volume_is_clamped_and_steps_stop_at_both_ends() {
        let mut old = switches(true, true);
        old["music_volume"] = 9.into();
        let mut progress = decode(old);
        assert_eq!(progress.volume(Channel::Music), LOUDEST);
        assert!(!progress.step_volume(Channel::Music, true));
        assert!(progress.step_volume(Channel::Music, false));
        assert_eq!(progress.volume(Channel::Music), LOUDEST - 1);
        progress.set_volume(Channel::Cues, 0);
        assert!(!progress.step_volume(Channel::Cues, false));
        // A tap counts up and wraps from 100 % to off.
        let mut seen = Vec::new();
        for _ in 0..=LOUDEST {
            progress.cycle_volume(Channel::Cues);
            seen.push(progress.volume(Channel::Cues));
        }
        assert_eq!(seen, [1, 2, 3, 4, 0]);
    }

    #[test]
    fn volume_steps_rise_by_loudness_and_read_as_percent() {
        assert_eq!(gain(0), 0.0);
        assert_eq!(gain(LOUDEST), 1.0);
        assert_eq!(gain(LOUDEST + 3), 1.0);
        for step in 1..=LOUDEST {
            assert!(gain(step) > gain(step - 1));
        }
        // Half loudness is close to −10 dB, well below half the amplitude.
        assert!((0.3..0.4).contains(&gain(2)), "{}", gain(2));
        assert!(gain(1) > 0.1, "25 % must stay audible");
        let labels: Vec<_> = (0..=LOUDEST).map(volume_label).collect();
        assert_eq!(labels, ["Aus", "25 %", "50 %", "75 %", "100 %"]);
    }
}
