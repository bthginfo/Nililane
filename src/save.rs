use std::{fs, io, path::PathBuf};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::{
    campaign::CHAPTERS,
    model::{LOTS, MAX_LEVEL, Phase, Town, is_open},
};

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub struct Record {
    pub stars: u8,
    pub best_day: u32,
}

#[derive(Resource, Deserialize, Serialize)]
pub struct Campaign {
    version: u32,
    pub records: Vec<Record>,
    pub town: Option<Town>,
    pub music: bool,
    pub sounds: bool,
    pub large_ui: bool,
}

impl Default for Campaign {
    fn default() -> Self {
        Self {
            version: 1,
            records: vec![Record::default(); CHAPTERS.len()],
            town: None,
            music: true,
            sounds: true,
            large_ui: false,
        }
    }
}

impl Campaign {
    pub fn load() -> Self {
        let Ok(file) = fs::File::open(path()) else {
            return Self::default();
        };
        let Ok(mut progress) = serde_json::from_reader::<_, Self>(file) else {
            return Self::default();
        };
        if progress.version != 1 || !progress.town.as_ref().is_none_or(valid_town) {
            return Self::default();
        }
        progress.records.resize(CHAPTERS.len(), Record::default());
        progress
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

    pub fn record(&mut self, chapter: usize, day: u32, stars: u8) {
        let record = &mut self.records[chapter];
        if stars > record.stars || (stars == record.stars && day < record.best_day) {
            *record = Record {
                stars,
                best_day: day,
            };
        }
    }
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
    if let Phase::Open(day) = &town.phase {
        return day.seconds.is_finite()
            && day
                .visits
                .iter()
                .all(|v| v.shop.is_none_or(|slot| usize::from(slot) < LOTS))
            && day.favor_at.is_none_or(|slot| usize::from(slot) < LOTS);
    }
    true
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
