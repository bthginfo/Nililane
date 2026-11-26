//! One short „So geht’s“ line for every harbor objective, written for young players.

use super::{Chapter, Goal, age_plural};
use crate::{
    campaign_orders::shop_for,
    model::{Age, Assortment, Building, Calling, Kind},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Genus {
    Masculine,
    Feminine,
    Neuter,
    Plural,
}

fn genus(kind: Kind) -> Genus {
    use Genus::*;
    use Kind::*;
    match kind {
        House | Well | Grove | Storehouse | Aqueduct | HerbGarden | TeaGarden | Temple
        | Woodlot | GardenCourt | MarketGarden | DateGarden | SpiceBazaar | Qanat | MoonGarden => {
            Masculine
        }
        Inn | Shrine | Forum | RicePaddy | TeaHouse | Machiya | Longhouse | Boatyard
        | FerryOffice | SignalTower | OasisHome => Neuter,
        Baths => Plural,
        _ => Feminine,
    }
}

/// „den Brunnen“, „die Backstube“, „das Gasthaus“.
fn the(kind: Kind) -> String {
    let article = match genus(kind) {
        Genus::Masculine => "den",
        Genus::Feminine | Genus::Plural => "die",
        Genus::Neuter => "das",
    };
    format!("{article} {}", kind.name())
}

/// „im Speicher“, „in der Backstube“, „im Gasthaus“.
fn inside(kind: Kind) -> String {
    let article = match genus(kind) {
        Genus::Masculine | Genus::Neuter => "im",
        Genus::Feminine => "in der",
        Genus::Plural => "in den",
    };
    format!("{article} {}", kind.name())
}

/// „einen Brunnen“, „eine Backstube“, „ein Gasthaus“.
fn one(kind: Kind) -> String {
    match genus(kind) {
        Genus::Masculine => format!("einen {}", kind.name()),
        Genus::Feminine => format!("eine {}", kind.name()),
        Genus::Neuter => format!("ein {}", kind.name()),
        Genus::Plural => kind.name().to_owned(),
    }
}

/// „ihn“, „sie“, „es“ for a building just named.
fn it(kind: Kind) -> &'static str {
    match genus(kind) {
        Genus::Masculine => "ihn",
        Genus::Feminine | Genus::Plural => "sie",
        Genus::Neuter => "es",
    }
}

/// „zweimal“, „dreimal“, „12-mal“.
fn times(count: u32) -> String {
    match count {
        2 => "zweimal".into(),
        3 => "dreimal".into(),
        4 => "viermal".into(),
        _ => format!("{count}-mal"),
    }
}

/// „Kindern“, „Erwachsenen“, „älteren Gästen“.
fn age_dative(age: Age) -> &'static str {
    match age {
        Age::Child => "Kindern",
        Age::Adult => "Erwachsenen",
        Age::Senior => "älteren Gästen",
    }
}

/// The first offer each age group likes, ready from the first level on.
fn first_favorite(age: Age) -> Assortment {
    match age {
        Age::Child => Assortment::Colorful,
        Age::Adult => Assortment::Hearty,
        Age::Senior => Assortment::Classic,
    }
}

/// A shop of this harbor whose offer can be changed and whose wares the guests want.
fn favorite_shop(chapter: &Chapter) -> Option<Kind> {
    chapter
        .starts
        .iter()
        .map(|&(_, kind)| kind)
        .chain(chapter.allowed().iter().copied())
        .find(|&kind| {
            Assortment::supported(kind)
                && kind.service().is_some_and(|service| {
                    chapter.locals.contains(&service) || chapter.guests.contains(&service)
                })
        })
}

fn home(chapter: &Chapter) -> Kind {
    chapter
        .starts
        .iter()
        .map(|&(_, kind)| kind)
        .chain(chapter.allowed().iter().copied())
        .find(|&kind| Building::new(kind).residents() > 0)
        .unwrap_or(Kind::House)
}

pub(super) fn hint(goal: Goal, chapter: &Chapter) -> String {
    match goal {
        Goal::Build(kind, 1) => format!("Tippe auf einen freien Bauplatz und baue {}.", one(kind)),
        Goal::Build(kind, count) => format!(
            "Baue {} {}, am besten an verschiedenen Enden der Gasse.",
            times(count),
            one(kind)
        ),
        Goal::Serve(service, _) => shop_for(service, chapter).map_or_else(
            || "Halte das Angebot bereit und den Preis fair, dann kaufen mehr Gäste.".to_owned(),
            |shop| {
                format!(
                    "Baue {} nah am Kai und bei den Häusern. Ein fairer Preis und Nachschub helfen.",
                    one(shop)
                )
            },
        ),
        Goal::Renown(_) => {
            "Jeder zufriedene Gast bringt Ansehen. Lieblingsangebote und Hilfe am Kai bringen mehr."
                .into()
        }
        Goal::Help(_) => "Jeden Tag gibt es eine Bitte vom Kai. Folge mit Nili der Markierung.".into(),
        Goal::Earn(_) => {
            "Jeder Verkauf bringt Münzen. Volle Läden und passende Preise bringen mehr.".into()
        }
        Goal::FeastGuests(_) => {
            "Am Festtag kommen mehr Gäste. Der Tagesplan zeigt ihn an, füll vorher die Regale."
                .into()
        }
        Goal::Variety(count) => format!(
            "Verkaufe an einem Tag {count} verschiedene Waren. Jeder Laden braucht dafür Vorrat."
        ),
        Goal::MarketDay(_) => {
            "Zählt an einem einzigen Tag. Ein Festtag und volle Regale bringen am meisten.".into()
        }
        Goal::Water(_) => {
            "Baue Brunnen und baue sie aus. Jede Wassertour mit Nili bringt zwei Krüge mehr."
                .into()
        }
        Goal::Harvest(_) => {
            "Baue Brunnen neben die Reisfelder und baue die Felder aus. Zählt an einem Tag.".into()
        }
        Goal::Bell(_) => Calling::Bell.hint().into(),
        Goal::Catch(_) => {
            "Baue die Fischerhütte aus. Bei Wind und Sturm fangen die Fischer weniger.".into()
        }
        Goal::Voyages(_) => {
            "Geh mit Nili zum Bootshaus und belade ein Schiff mit Räucherfisch.".into()
        }
        Goal::CargoIncome(_) => {
            "Schick Schiffe vom Bootshaus. Fahrten übers offene Meer bringen mehr Silber.".into()
        }
        Goal::Caravans(_) => {
            "Geh mit Nili zur Karawanserei. Halte Datteln und freie Krüge bereit.".into()
        }
        Goal::CaravanIncome(_) => {
            "Schick Karawanen von der Karawanserei. Die Gewürzstraße bringt am meisten.".into()
        }
        Goal::SpiceRoutes(_) => {
            "Die Gewürzstraße braucht Datteln, Gewürze, fünf freie Krüge und einen Qanat ab Stufe 2."
                .into()
        }
        Goal::Route(job, _) => {
            format!("Tippe auf Nili und wähle „{}“ ({}).", job.name(), job.purpose())
        }
        Goal::RouteVariety(count) => {
            format!("Probiere {count} verschiedene Touren mit Nili aus. Jede Sorte zählt einmal.")
        }
        Goal::FerryDay(_) => {
            "Baue Läden nah am Anleger. Ein Fährtag im Hafenplan bringt drei Gäste mehr.".into()
        }
        Goal::GoodDay(_) => {
            "Genug Vorrat, kurze Wege und faire Preise. Zählt am Abend ab 12 Rückmeldungen.".into()
        }
        Goal::Specials(_) => {
            "Wähle morgens im Laden ein Angebot für Kinder, Erwachsene oder ältere Gäste.".into()
        }
        Goal::Residents(_) => {
            let home = home(chapter);
            format!(
                "Baue {} und baue {} aus. Jede Stufe bringt neue Nachbarn.",
                one(home),
                it(home)
            )
        }
        Goal::Upgrade(kind, level) => format!(
            "Tippe auf {} und wähle „Ausbauen“, bis Stufe {level} erreicht ist.",
            the(kind)
        ),
        Goal::Pair(first, second, count) => {
            let base = format!("Baue {} direkt neben {}.", one(first), one(second));
            if count > 1 {
                format!("{base} Das brauchst du {}.", times(count))
            } else {
                base
            }
        }
        Goal::Produce(..) => {
            "Läden mit passenden Nachbarn stellen jeden Morgen mehr her. Bau sie aus.".into()
        }
        Goal::Sale(order, false) => format!(
            "Lass {} beim Grundangebot „{}“, bis es oft genug verkauft ist.",
            the(order.shop),
            Assortment::Standard.label(order.shop)
        ),
        Goal::TradeUpgrade(order) => format!(
            "Baue {} bis Stufe {} aus. Dann gibt es „{}“.",
            the(order.shop),
            order.special.tier(),
            order.special.label(order.shop)
        ),
        Goal::Sale(order, true) => {
            let offer = order.special.label(order.shop);
            match order.special.flavor().filter(|_| order.favorite_only) {
                Some(age) => format!(
                    "Wähle morgens „{offer}“ {}. Es zählt, wenn {} es kaufen.",
                    inside(order.shop),
                    age_plural(age)
                ),
                None => format!("Wähle morgens „{offer}“ {}.", inside(order.shop)),
            }
        }
        Goal::Dogs(_) => {
            "In der ersten Schichthälfte kommt ein Hund. Lauf mit Nili hin und streichle ihn."
                .into()
        }
        Goal::Clean(_) => "Laub und Unrat liegen in der Gasse. Sammle sie mit Nili ein.".into(),
        Goal::GuestCare(age, minimum, _) => format!(
            "Halte die Lieblingswaren für {} bereit. Zählt ab {minimum} zufriedenen {} am Tag.",
            age_plural(age),
            age_dative(age)
        ),
        Goal::FavoriteMenu(age, count) => format!(
            "Biete {} an einem Tag in {count} verschiedenen Läden ihr Lieblingsangebot an.",
            age_dative(age)
        ),
        Goal::Rebuild(order) => format!(
            "Reiß {} ab und baue dort {}. Bau {} danach bis Stufe {} aus.",
            the(order.old),
            one(order.new),
            it(order.new),
            order.tier
        ),
        Goal::Streak(_) => {
            "Niemand darf leer ausgehen, sonst beginnt die Serie neu. Volle Regale helfen.".into()
        }
        Goal::TourDay(count) => format!(
            "Schick Nili an einem Tag auf {count} verschiedene Touren, etwa Nachschub und Wasser."
        ),
        Goal::Rush(_) => {
            "Bei einem Zwischenfall läutet die Glocke. Lauf sofort mit Nili zur Markierung.".into()
        }
        Goal::FanDay(age, _) => favorite_shop(chapter).map_or_else(
            || format!("Wähle morgens im Laden das Angebot für {}.", age_plural(age)),
            |shop| {
                let offer = first_favorite(age).label(shop);
                let (verb, extra) = match age {
                    Age::Child => ("lieben", " Ausgebaute Wohnhäuser bringen Familien."),
                    _ => ("mögen", ""),
                };
                format!(
                    "{} {verb} „{offer}“. Wähle es morgens {}.{extra}",
                    age.group(),
                    inside(shop)
                )
            },
        ),
        Goal::SteadyMood(_) => {
            "Halte an zwei Abenden hintereinander viele Gäste glücklich. Der schwächere Tag zählt."
                .into()
        }
        Goal::Call(calling, _) => calling.hint().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        campaign::{CHAPTERS, harbor_stage},
        errands::Job,
    };

    #[test]
    fn every_objective_explains_itself_in_one_short_line() {
        for (index, chapter) in CHAPTERS.iter().enumerate() {
            for goal in chapter.objectives(index) {
                let hint = goal.hint(chapter);
                assert!(hint.ends_with('.'), "{}: {hint}", chapter.district);
                assert!(hint.chars().count() <= 115, "{}: {hint}", chapter.district);
                assert!(!hint.contains("  ") && !hint.contains('{'), "{hint}");
            }
        }
        let rhodos = &CHAPTERS[3];
        assert_eq!(
            Goal::FanDay(Age::Child, 2).hint(rhodos),
            "Kinder lieben „Honigfladen“. Wähle es morgens in der Backstube. \
             Ausgebaute Wohnhäuser bringen Familien."
        );
        assert_eq!(
            Goal::Pair(Kind::Bakery, Kind::Well, 1).hint(rhodos),
            "Baue eine Backstube direkt neben einen Brunnen."
        );
        assert_eq!(
            Goal::Upgrade(Kind::Inn, 2).hint(rhodos),
            "Tippe auf das Gasthaus und wähle „Ausbauen“, bis Stufe 2 erreicht ist."
        );
    }

    #[test]
    fn new_goal_kinds_only_ask_for_what_the_harbor_offers() {
        for (index, chapter) in CHAPTERS.iter().enumerate() {
            let stage = harbor_stage(index);
            let builds = |kind: Kind| {
                chapter.allowed().contains(&kind)
                    || chapter.starts.iter().any(|&(_, start)| start == kind)
            };
            let goals: Vec<_> = chapter.objectives(index).collect();
            let removed = |kind: Kind| {
                goals
                    .iter()
                    .any(|goal| matches!(goal, Goal::Rebuild(order) if order.old == kind))
            };
            for &goal in &goals {
                let fits = match goal {
                    Goal::Call(calling, _) => chapter
                        .allowed()
                        .iter()
                        .any(|&kind| Calling::of(kind) == Some(calling)),
                    Goal::FanDay(age, _) => {
                        age.available(index) && favorite_shop(chapter).is_some()
                    }
                    Goal::Rush(_) => stage >= 3,
                    Goal::TourDay(count) => {
                        let kinds = Job::ALL.iter().filter(|job| job.unlocked(index)).count();
                        kinds as u32 >= count && u32::from(Job::limit(index)) >= count
                    }
                    Goal::Bell(_) => builds(Kind::Temple),
                    Goal::Voyages(_) => builds(Kind::Boatyard),
                    Goal::Caravans(_) => builds(Kind::Caravanserai) && !removed(Kind::Caravanserai),
                    _ => true,
                };
                assert!(fits, "{}: {}", chapter.district, goal.label());
            }
        }
    }
}
