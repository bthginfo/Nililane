use serde::{Deserialize, Serialize};

use super::{Good, Island, Level};

pub const SLOTS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Arrival,
    Fire,
    Ruin,
    Fever,
    Festival,
    Relic,
}

#[derive(Clone, Copy)]
pub struct Encounter {
    pub event: Event,
    pub title: &'static str,
    pub author: &'static str,
    pub request: &'static str,
    pub cargo: Option<(Good, u16)>,
    pub reward: (Good, u16),
}

impl Encounter {
    pub fn icon(self) -> &'static str {
        match self.event {
            Event::Arrival => "ferry",
            Event::Fire => "fire",
            Event::Ruin => "hammer",
            Event::Fever => "herb_garden",
            Event::Festival => "star",
            Event::Relic => "shrine",
        }
    }

    fn work(self) -> f32 {
        match self.event {
            Event::Ruin => 9.0,
            Event::Fire => 2.7,
            Event::Fever => 3.5,
            Event::Festival => 4.0,
            Event::Arrival | Event::Relic => 1.4,
        }
    }

    pub fn action(self, phase: Status) -> &'static str {
        match phase {
            Status::Offered => "Mit der Person sprechen",
            Status::Seeking => match self.event {
                Event::Fire => "Wasser schöpfen",
                Event::Relic => "Fundstück aufnehmen",
                _ => "Fracht packen",
            },
            Status::Carrying => match self.event {
                Event::Fire => "Brand löschen",
                Event::Ruin => "Wiederaufbau beginnen",
                Event::Fever => "Vorräte übergeben",
                Event::Festival => "Festmahl bringen",
                Event::Relic => "Fundstück zurückbringen",
                Event::Arrival => "Ladung übergeben",
            },
            Status::Working => "Hilfe läuft",
            Status::Finished => "Erledigt",
            Status::Hidden => "",
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Hidden,
    Offered,
    Seeking,
    Carrying,
    Working,
    Finished,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Visit {
    pub status: Status,
    pub since: f32,
    /// Counts how often this place had a visitor; each round brings another event.
    #[serde(default)]
    pub round: u8,
}

/// A finished place gets a new visitor after this many seconds.
const REARM_SECONDS: f32 = 240.0;
/// A fire nobody answers goes out on its own after this many seconds.
const BURN_SECONDS: f32 = 150.0;
const FIRST_VISIT: [f32; SLOTS] = [24.0, 88.0, 156.0];

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Encounters {
    pub elapsed: f32,
    pub visits: [Visit; SLOTS],
}

impl Encounters {
    pub fn carrying(&self) -> Option<usize> {
        self.visits
            .iter()
            .position(|visit| visit.status == Status::Carrying)
    }

    pub fn changed(&mut self, slot: usize, status: Status) {
        self.visits[slot] = Visit {
            status,
            since: self.elapsed,
            round: self.visits[slot].round,
        };
    }

    pub fn progress(&self, slot: usize, spec: Encounter) -> f32 {
        ((self.elapsed - self.visits[slot].since) / spec.work()).clamp(0.0, 1.0)
    }
}

#[derive(Default)]
pub struct Changes {
    pub offered: Vec<usize>,
    pub finished: Vec<usize>,
    pub burned_out: Vec<usize>,
}

impl Island {
    /// The event currently waiting at `slot`, which changes from round to round.
    pub fn encounter(&self, level: Level, slot: usize) -> Option<Encounter> {
        level.encounter_in(slot, self.encounters.visits.get(slot)?.round)
    }

    pub fn update_encounters(&mut self, level: Level, dt: f32) -> Changes {
        let mut changes = Changes::default();
        if level.encounter_count() == 0 {
            return changes;
        }
        self.encounters.elapsed += dt;
        let built = self.lots.iter().flatten().count();
        let goals = level.goals().count();
        let met = level
            .goals()
            .filter(|goal| self.progress(*goal) >= goal.count)
            .count();
        for (slot, first) in FIRST_VISIT
            .into_iter()
            .enumerate()
            .take(level.encounter_count())
        {
            let spec = self.encounter(level, slot).unwrap();
            let visit = self.encounters.visits[slot];
            let waited = self.encounters.elapsed - visit.since;
            match visit.status {
                // Later visitors wait until the island has grown: slot k needs k/3 of the goals.
                Status::Hidden
                    if built >= 1
                        && met * SLOTS >= goals * slot
                        && self.encounters.elapsed >= first + level.stage as f32 * 2.0 =>
                {
                    self.encounters.changed(slot, Status::Offered);
                    changes.offered.push(slot);
                }
                Status::Offered | Status::Seeking
                    if spec.event == Event::Fire && waited >= BURN_SECONDS =>
                {
                    self.encounters.changed(slot, Status::Finished);
                    changes.burned_out.push(slot);
                }
                Status::Working if self.encounters.progress(slot, spec) >= 1.0 => {
                    self.encounters.changed(slot, Status::Finished);
                    self.add(spec.reward.0, spec.reward.1);
                    changes.finished.push(slot);
                }
                Status::Finished if waited >= REARM_SECONDS => {
                    let round = self.next_round(level, slot);
                    self.encounters.visits[slot] = Visit {
                        status: Status::Offered,
                        since: self.encounters.elapsed,
                        round,
                    };
                    changes.offered.push(slot);
                }
                _ => {}
            }
        }
        changes
    }

    /// The next round whose event differs from the last one here and from the other places.
    fn next_round(&self, level: Level, slot: usize) -> u8 {
        let previous = self.encounter(level, slot).map(|spec| spec.event);
        let others: Vec<_> = (0..level.encounter_count())
            .filter(|&other| other != slot)
            .filter_map(|other| self.encounter(level, other).map(|spec| spec.event))
            .collect();
        let round = self.encounters.visits[slot].round;
        (1..=6)
            .map(|step| round.wrapping_add(step))
            .find(|&next| {
                level.encounter_in(slot, next).is_some_and(|spec| {
                    Some(spec.event) != previous && !others.contains(&spec.event)
                })
            })
            .unwrap_or(round.wrapping_add(1))
    }

    pub fn encounter_pickup(&mut self, level: Level, slot: usize) -> bool {
        let Some(spec) = self.encounter(level, slot) else {
            return false;
        };
        if self.encounters.visits[slot].status != Status::Seeking || self.has_cargo() {
            return false;
        }
        if let Some((good, units)) = spec.cargo {
            if self.amount(good) < units {
                return false;
            }
            self.stores[good as usize] -= units;
        }
        self.encounters.changed(slot, Status::Carrying);
        true
    }

    pub fn return_encounter_cargo(&mut self, level: Level) -> bool {
        let Some(slot) = self.encounters.carrying() else {
            return false;
        };
        if let Some((good, count)) = self.encounter(level, slot).and_then(|spec| spec.cargo) {
            if good.uses_storage() && count > self.free_storage() {
                return false;
            }
            self.add(good, count);
        }
        self.encounters.changed(slot, Status::Seeking);
        true
    }
}

impl Level {
    pub fn encounter_count(self) -> usize {
        match self.stage {
            0 => 0,
            1..=2 => 1,
            3..=5 => 2,
            _ => SLOTS,
        }
    }

    /// The first visitor at `slot`; later rounds come from [`Island::encounter`].
    pub fn encounter(self, slot: usize) -> Option<Encounter> {
        self.encounter_in(slot, 0)
    }

    pub fn encounter_in(self, slot: usize, round: u8) -> Option<Encounter> {
        use Event::*;
        use Good::*;
        if slot >= self.encounter_count() {
            return None;
        }
        let region = self.region as usize;
        let event = [Arrival, Festival, Fire, Fever, Ruin, Relic]
            [(self.stage + region * 2 + slot * 2 + usize::from(round)) % 6];
        let count = 4 + (self.stage / 3) as u16;
        let (title, author, request, cargo, reward) = match event {
            Arrival => (
                [
                    "Ein Korb für die Nachbarinsel",
                    "Ein Fuhrwerk ohne Ladung",
                    "Ein Bote an der Bucht",
                    "Ein Gast aus dem nächsten Fjord",
                    "Eine Karawane auf Abwegen",
                ][region],
                [
                    "Philo · Schiffer",
                    "Sabina · Fuhrfrau",
                    "Sora · Bote",
                    "Runa · Händlerin",
                    "Mazin · Karawanenführer",
                ][region],
                [
                    "Philo braucht Bretter für die nächste Überfahrt.",
                    "Sabina fehlt Holz für ihren Wagen.",
                    "Sora nimmt Papier zur Schreibstube am anderen Ufer.",
                    "Runa braucht Bretter für einen beschädigten Landesteg.",
                    "Mazin sucht eine kleine Ladung Handelswaren.",
                ][region],
                Some(([Boards, Boards, Paper, Boards, Craft][region], count)),
                (Tools, 3 + count / 2),
            ),
            Fire => (
                [
                    "Feuer im Olivenlager",
                    "Funken am Tor",
                    "Das brennende Papierlager",
                    "Rauch am Bootsschuppen",
                    "Feuer am Dattelstand",
                ][region],
                [
                    "Myrto · Gärtnerin",
                    "Titus · Torwache",
                    "Naoko · Papiermacherin",
                    "Liv · Bootsbauerin",
                    "Rima · Marktfrau",
                ][region],
                "Hol Wasser. Wenn es keinen Brunnen gibt, führt der Weg zu einer flachen Stelle am Meer.",
                None,
                (Boards, count + 2),
            ),
            Ruin => (
                [
                    "Der eingestürzte Rastplatz",
                    "Ein Riss im Wachhaus",
                    "Das Tor nach dem Sturm",
                    "Der gefallene Holzschuppen",
                    "Die zerbrochenen Lehmbögen",
                ][region],
                [
                    "Doros · Steinmetz",
                    "Livia · Baumeisterin",
                    "Hana · Zimmerin",
                    "Leif · Zimmermann",
                    "Bahir · Maurer",
                ][region],
                "Bring Baumaterial und hilf beim Wiederaufbau. Die Arbeiten dauern einen Moment.",
                Some(([Boards, Bricks, Boards, Boards, Bricks][region], count + 2)),
                (Tools, count),
            ),
            Fever => (
                [
                    "Der kranke Olivenpflücker",
                    "Fieber in der Wache",
                    "Tee für die müde Pilgerin",
                    "Eine warme Schale für Astrid",
                    "Ein Rasttag für die Karawane",
                ][region],
                [
                    "Kleio · Nachbarin",
                    "Junia · Pflegerin",
                    "Aki · Reisgärtner",
                    "Astrid · Weberin",
                    "Yasmin · Nachbarin",
                ][region],
                "Bring etwas zu essen aus dem Lager. Die Erholung braucht einen ruhigen Moment.",
                Some((Meals, count)),
                (Cloth, count),
            ),
            Festival => (
                [
                    "Lichter für Demeter",
                    "Ein Fest für Flora",
                    "Das Fest der Papierlichter",
                    "Ein Feuer für den ersten Fang",
                    "Das Fest der Laternen",
                ][region],
                [
                    "Eleni · Festküche",
                    "Tertia · Veranstalterin",
                    "Yui · Lampionmacherin",
                    "Eira · Festküche",
                    "Nura · Gastgeberin",
                ][region],
                "Die Girlanden sind bereit. Bring Mahlzeiten zu den Tischen, damit das Fest anfangen kann.",
                Some((Meals, count + 3)),
                (Craft, count + 1),
            ),
            Relic => (
                [
                    "Ein Zeichen des Zeus",
                    "Die verschollene Flamme der Vesta",
                    "Ein Geschenk für Inari",
                    "Freyjas Bernsteinperle",
                    "Der Stern der Oase",
                ][region],
                [
                    "Bote des Zeus",
                    "Hüterin der Vesta",
                    "Hüter des Inari-Schreins",
                    "Bote der Freyja",
                    "Hüterin des Sterns",
                ][region],
                "Am markierten Hang liegt ein Fundstück. Bring es zum kleinen Heiligtum zurück.",
                None,
                (Tools, count + 2),
            ),
        };
        Some(Encounter {
            event,
            title,
            author,
            request,
            cargo,
            reward,
        })
    }
}
