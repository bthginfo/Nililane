use super::{Chapter, Goal::*, Region};
use crate::errands::Job;
use crate::model::{Kind, Kind::*, Service::*};

const fn prefix<const N: usize>(plans: &[Kind]) -> [Kind; N] {
    let mut result = [plans[0]; N];
    let mut i = 0;
    while i < N {
        result[i] = plans[i];
        i += 1;
    }
    result
}

const AEGEAN: &[Kind] = &[
    House,
    Well,
    Bakery,
    Grove,
    Potter,
    Stoa,
    Taverna,
    HerbGarden,
    Inn,
    Apiary,
    Dyer,
    Storehouse,
    Press,
    Shrine,
    FerryOffice,
];
const HARBOR: &[Kind] = &[
    House,
    Well,
    Bakery,
    Grove,
    Potter,
    Taverna,
    Inn,
    HerbGarden,
    Storehouse,
    Mill,
    Aqueduct,
    Baths,
    GardenCourt,
    Forum,
    Scribe,
    Villa,
];
const INLAND_SEA: &[Kind] = &[
    Machiya,
    Well,
    RicePaddy,
    RiceKitchen,
    Temple,
    TeaGarden,
    TeaHouse,
    MarketGarden,
    PaperWorkshop,
    Inn,
    Storehouse,
    CeramicStudio,
    SilkShop,
];
const FJORD: &[Kind] = &[
    Longhouse,
    Well,
    Fishery,
    Smokehouse,
    Woodlot,
    Boatyard,
    Weaver,
    Sheepfold,
    FeastHall,
    Storehouse,
    Cooperage,
    SignalTower,
];

const GREEK_HIVES: [Kind; 10] = prefix(AEGEAN);
const GREEK_TABLES: [Kind; 8] = prefix(AEGEAN);
const GREEK_GUESTS: [Kind; 9] = prefix(AEGEAN);
const GREEK_DYES: [Kind; 11] = prefix(AEGEAN);
const GREEK_STORES: [Kind; 12] = prefix(AEGEAN);
const GREEK_OIL: [Kind; 13] = prefix(AEGEAN);
const GREEK_TEMPLES: [Kind; 14] = prefix(AEGEAN);
const ROMAN_WATER: [Kind; 12] = prefix(HARBOR);
const ROMAN_GARDENS: [Kind; 13] = prefix(HARBOR);
const ROMAN_WRITING: [Kind; 15] = prefix(HARBOR);
const MILLS: &[Kind] = &[
    House, Well, Bakery, Grove, Potter, Stoa, Taverna, Inn, HerbGarden, Storehouse, Mill,
];
const JAPAN_GARDENS: [Kind; 8] = prefix(INLAND_SEA);
const JAPAN_PAPER: [Kind; 11] = prefix(INLAND_SEA);
const JAPAN_CLAY: [Kind; 12] = prefix(INLAND_SEA);
const NORDIC_SHEEP: [Kind; 8] = prefix(FJORD);
const NORDIC_HEARTH: [Kind; 10] = prefix(FJORD);
const NORDIC_CASKS: [Kind; 11] = prefix(FJORD);

pub(super) const fn revise(ch: Chapter, stage: usize) -> Chapter {
    match ch.region {
        Region::Greece => greece(ch, stage),
        Region::Rome => rome(ch, stage),
        Region::Japan => japan(ch, stage),
        Region::Nordic => nordic(ch, stage),
        Region::Oasis => super::oasis::revise(ch, stage),
    }
}

const fn greece(mut ch: Chapter, stage: usize) -> Chapter {
    ch.building_set = match stage {
        3 => Some(&GREEK_TABLES),
        4 => Some(&GREEK_GUESTS),
        5..=7 => Some(&GREEK_HIVES),
        8..=9 => Some(&GREEK_DYES),
        10..=11 => Some(&GREEK_STORES),
        12..=13 => Some(&GREEK_OIL),
        14..=16 => Some(&GREEK_TEMPLES),
        17.. => Some(AEGEAN),
        _ => ch.building_set,
    };
    match stage {
        5 => {
            ch.name = "Eine Tafel für Kos";
            ch.ferry += 2;
            ch.walk = 4;
            ch.greeting = "Mara deckt die Tische. Aus der Backstube nebenan soll das Brot zur Taverne kommen.";
            ch.goals = [Pair(Taverna, Bakery, 1), Serve(Meze, 28), Specials(6)];
            ch.locals = &[Bread, Meze, Pottery, Shade, Meze];
            ch.guests = &[Meze, Bread, Meze, Bed, Pottery, Meze, Honey];
            ch.hint = "Eine Taverne braucht Brot und Oliven. Passe ihre Gerichte an die Gäste an.";
        }
        6 => {
            ch.name = "Honig am hölzernen Kai";
            ch.greeting = "Thaleia bringt die ersten Waben vom Hang hinunter zur Gasse.";
            ch.hint = "Eine Imkerei neben Backstube oder Taverne erhöht dort den Vorrat.";
            ch.goals = [
                Build(Apiary, 1),
                Serve(Honey, 27),
                Pair(Apiary, HerbGarden, 1),
            ];
            ch.guests = &[Honey, Bread, Shade, Meze, Honey, Pottery, Honey, Honey];
        }
        7 => {
            ch.name = "Die lange Nacht von Kythnos";
            ch.greeting = "Nach der späten Fähre duftet es am Kai nach warmem Brot und Honig.";
            ch.hint = "Heute kommen mehr Gäste zum Essen als zur Übernachtung. Halte eine warme Tafel und Honig für den Heimweg bereit.";
            ch.locals = &[Bread, Meze, Honey, Shade, Honey];
            ch.guests = &[Meze, Honey, Bread, Meze, Pottery, Honey, Meze, Shade];
            ch.goals = [Serve(Honey, 31), MarketDay(53), Route(Job::Welcome, 3)];
        }
        8 => {
            ch.guests = &[
                DyedCloth, Pottery, Bread, Shade, Meze, Pottery, Bed, DyedCloth,
            ];
        }
        9 => {
            ch.goals = [Residents(11), Pair(House, Grove, 2), GoodDay(80)];
            ch.hint = "Ausgebaute Wohnhöfe bringen Nachbarn. Sorg dafür, dass sie ihre Wünsche in der Nähe erfüllen können.";
        }
        10 => {
            ch.name = "Die Farben von Ikaria";
            ch.greeting = "Ion probiert neue Farbbäder. Am Anleger warten Siebe und Stoffe auf einen trockenen Platz.";
            ch.hint = "Für volle Farbbäder braucht die Färberei Wasser. Die Leute kaufen gern passende Stoffe.";
            ch.guests = &[
                DyedCloth, Honey, DyedCloth, Pottery, Meze, DyedCloth, Bread, Shade,
            ];
            ch.goals = [Build(Dyer, 1), Serve(DyedCloth, 42), Route(Job::Water, 4)];
        }
        11 => {
            ch.goals = [Serve(Meze, 46), FeastGuests(17), Specials(15)];
        }
        14 => {
            ch.name = "Die Düfte der Nacht";
            ch.greeting = "Händler und Nachbarn kommen erst nach Sonnenuntergang. Im Licht der Öllampen glänzen die Tonkrüge.";
            ch.hint =
                "Nachts fragen die Gäste nach Öl, krappfarbenen Stoffen und einem warmen Essen.";
            ch.guests = &[
                Oil, DyedCloth, Meze, Oil, Pottery, DyedCloth, Honey, Meze, Oil,
            ];
            ch.locals = &[Oil, Meze, Honey, Oil, Bread, DyedCloth];
            ch.goals = [Serve(Oil, 73), Specials(24), MarketDay(78)];
        }
        17 => {
            ch.guests = &[
                Offering, Oil, DyedCloth, Pottery, Meze, Offering, DyedCloth, Honey,
            ];
            ch.goals = [Upgrade(Dyer, 3), Variety(6), FeastGuests(21)];
        }
        18 => {
            ch.name = "Das Lotsenhaus am Kap";
            ch.greeting = "Doros kennt die ruhigen Buchten. Am Kap fehlt ihm noch ein fester Platz für die Fährpläne.";
            ch.hint =
                "Ein Lotsenhaus bis zu zwei Bauplätze vom Anleger entfernt bringt mehr Fährgäste.";
            ch.goals = [Build(FerryOffice, 1), FerryDay(17), Route(Job::Welcome, 6)];
        }
        19 => {
            ch.goals = [Upgrade(Apiary, 4), Serve(Offering, 82), Serve(Honey, 66)];
            ch.guests = &[Honey, Offering, Oil, Meze, Honey, Pottery, Shade, Offering];
            ch.hint = "Aus Gartenkräutern und Honig wird ein gutes Marktangebot. Plane genug Schalen für die Gaben ein.";
        }
        21 => {
            ch.name = "Lampen an den Brunnen von Kythira";
            ch.greeting = "Eleni wechselt die Öllampen aus. Zwischen den Häusern fließt nachts noch frisches Wasser.";
            ch.hint = "Die späten Gäste kommen vor allem für Öl und Tongefäße. Mit Wasseraufträgen hältst du die Werkstätten in Betrieb.";
            ch.guests = &[
                Pottery, Oil, Pottery, Offering, Honey, Oil, Meze, Shade, Oil,
            ];
            ch.locals = &[Bread, Pottery, Oil, Shade, Honey, Oil];
            ch.goals = [Upgrade(Well, 4), Route(Job::Water, 8), Serve(Pottery, 103)];
        }
        23 => {
            ch.goals = [Residents(16), Variety(8), FerryDay(24)];
            ch.guests = &[Offering, Honey, Meze, DyedCloth, Oil, Bread, Shade, Pottery];
            ch.locals = &[Bread, Honey, Meze, Oil, Offering, Pottery, Shade, DyedCloth];
            ch.hint = "Die Feier braucht acht verschiedene Angebote. Ein Lotsenhaus, Nachschub und mehrere Wohnhöfe helfen.";
        }
        _ => {}
    }
    ch
}

const fn rome(mut ch: Chapter, stage: usize) -> Chapter {
    ch.building_set = match stage {
        2 => Some(MILLS),
        3 => Some(&ROMAN_WATER),
        4..=6 => Some(&ROMAN_GARDENS),
        7..=8 => Some(&ROMAN_WRITING),
        9.. => Some(HARBOR),
        _ => ch.building_set,
    };
    match stage {
        2 => {
            ch.name = "Die Mühle an der Via Appia";
            ch.greeting =
                "Flavia hört die Mühlsteine schon vom Kai. Die Öfen warten auf frisches Mehl.";
            ch.hint = "Eine Getreidemühle versorgt Backstuben bis zu zwei Bauplätze entfernt.";
            ch.goals = [Pair(Mill, Bakery, 1), Serve(Bread, 55), Specials(10)];
            ch.guests = &[Bread, Meze, Bread, Pottery, Bread, Bread, Bed, Shade];
        }
        5 => ch.goals = [Route(Job::Quay, 5), FerryDay(15), Water(38)],
        7 => {
            ch.name = "Die Bäder nach Sonnenuntergang";
            ch.greeting = "In den Arkaden brennen Lampen. Nach der späten Überfahrt ziehen die Gäste direkt zum Bad.";
            ch.hint = "Die Nachtgäste wollen vor allem ins Bad und zur warmen Tafel. Ein guter Preis hält genug Plätze für sie offen.";
            ch.guests = &[Bath, Meze, Bath, Bath, Bed, Meze, Bread, Bath];
            ch.locals = &[Bath, Bread, Meze, Bath, Pottery, Meze];
            ch.goals = [Serve(Bath, 96), MarketDay(89), GoodDay(83)];
        }
        8 => {
            ch.name = "Schreiber auf dem Forum";
            ch.greeting = "Junia legt Wachstafeln aus. Neben dem Marktplatz braucht sie einen Raum für die Schreibstube.";
            ch.hint = "Ein Forum in Reichweite gibt der Schreibstube zusätzliche Kundschaft und mehr Vorrat.";
            ch.guests = &[
                Tablets, Trade, Pottery, Bread, Trade, Tablets, Bath, Meze, Tablets,
            ];
            ch.goals = [Pair(Scribe, Forum, 1), Serve(Tablets, 64), Serve(Trade, 78)];
        }
        10 => {
            ch.guests = &[Tablets, Bath, Bread, Trade, Pottery, Meze, Tablets, Bed];
            ch.goals = [Variety(6), Specials(34), MarketDay(103)];
        }
        14 => {
            ch.name = "Lampen an der Stadttreppe";
            ch.greeting = "Tertia liest die späten Ankünfte von der Wachstafel. Am Kai bleiben die Türen offen.";
            ch.hint = "Die Nachtgäste fragen nach Wachstafeln, Essen und Badeplätzen. Bring Wasser bis tief in die Schicht.";
            ch.guests = &[
                Tablets, Bath, Meze, Tablets, Bath, Bread, Trade, Bed, Tablets,
            ];
            ch.locals = &[Meze, Bath, Bread, Tablets, Bath, Meze];
            ch.goals = [Route(Job::Water, 8), Serve(Tablets, 97), GoodDay(85)];
        }
        15 => {
            ch.name = "Der Gartenhof von Cumae";
            ch.greeting = "Livia sucht einen Ort für die Bänke und Wasserbecken. Sie hat einen Bauplatz neben dem Aquädukt im Auge.";
            ch.hint = "Am Aquädukt hat der Gartenhof mehr Plätze. Am Forum füllt die Schreibstube mehr Wachstafeln für den Festtag ab.";
            ch.locals = &[Bread, Shade, Meze, Bath, Pottery, Shade];
            ch.guests = &[Shade, Bath, Meze, Tablets, Bread, Trade, Tablets, Shade];
            ch.goals = [
                Pair(GardenCourt, Aqueduct, 1),
                Serve(Tablets, 61),
                FeastGuests(23),
            ];
        }
        17 => ch.goals = [Upgrade(Inn, 4), Route(Job::Welcome, 10), FerryDay(22)],
        19 => {
            ch.guests = &[Tablets, Pottery, Bath, Meze, Bread, Tablets, Trade, Shade];
            ch.goals = [Upgrade(Scribe, 4), Serve(Pottery, 133), Serve(Tablets, 119)];
        }
        21 => {
            ch.name = "Der Nachtmarkt von Trastevere";
            ch.greeting = "Unter den Lampen bleiben die Tavernen und Schreibstuben offen. Flavia erwartet viele Stammgäste.";
            ch.hint = "An den späten Ständen lohnt sich ein Angebot, das zu den Stammgästen passt.";
            ch.locals = &[Bread, Meze, Tablets, Shade, Bath, Meze, Tablets];
            ch.guests = &[Meze, Tablets, Trade, Bath, Shade, Tablets, Meze, Bread];
            ch.goals = [MarketDay(128), Specials(45), Serve(Meze, 155)];
        }
        22 => ch.goals = [Residents(16), Upgrade(GardenCourt, 4), GoodDay(87)],
        _ => {}
    }
    ch
}

const fn japan(mut ch: Chapter, stage: usize) -> Chapter {
    ch.building_set = match stage {
        3 => Some(&JAPAN_GARDENS),
        4 => Some(&JAPAN_PAPER),
        5..=8 => Some(&JAPAN_CLAY),
        9.. => Some(INLAND_SEA),
        _ => ch.building_set,
    };
    match stage {
        3 => {
            ch.hint = "Ein Brunnen am Reisfeld steigert die Ernte. Der Gemüsegarten versorgt Reisküche und Teehaus bis zu zwei Plätze weit.";
            ch.goals = [
                Pair(RiceKitchen, MarketGarden, 1),
                Harvest(19),
                Serve(Rice, 36),
            ];
        }
        5 => {
            ch.guests = &[
                Rice, Tea, Lantern, Prayer, Bed, Pottery, Rice, Lantern, Pottery,
            ];
        }
        7 => {
            ch.name = "Die Nachtfähre in der Strömung";
            ch.greeting = "Die ersten Lampions weisen vom Anleger zur Teestube. Bei aufkommendem Wind muss der Kai frei bleiben.";
            ch.hint = "Lass Platz für die späten Gäste. Sie suchen Tee, Lampions und einen Ort für ein Gebet.";
            ch.locals = &[Rice, Tea, Prayer, Lantern, Tea, Lantern];
            ch.guests = &[Tea, Lantern, Prayer, Bed, Lantern, Prayer, Tea, Rice];
            ch.goals = [Route(Job::Quay, 4), Serve(Lantern, 38), FerryDay(12)];
        }
        8 => {
            ch.hint = "Mit kräftiger Glasur und passenden Schalen zieht die neue Keramikwerkstatt Gäste an. Brunnenwasser verbessert den Vorrat.";
            ch.guests = &[
                Pottery, Rice, Tea, Pottery, Bed, Lantern, Prayer, Pottery, Tea,
            ];
            ch.goals = [Build(CeramicStudio, 1), Serve(Pottery, 44), Specials(24)];
        }
        10 => {
            ch.goals = [
                Upgrade(PaperWorkshop, 3),
                Pair(PaperWorkshop, Temple, 1),
                MarketDay(80),
            ]
        }
        12 => ch.goals = [Route(Job::Welcome, 6), GoodDay(86), FerryDay(18)],
        14 => {
            ch.name = "Seide für die Abendfähre";
            ch.greeting = "Hana faltet den ersten gewebten Stoff unter den Papierlaternen. Daneben dampft schon der Tee.";
            ch.hint = "Der Gemüsegarten liefert Pflanzen zum Färben. Die Nachtgäste kaufen Stoff, trinken Tee und nehmen Lampions mit.";
            ch.guests = &[
                Textiles, Tea, Lantern, Bed, Tea, Textiles, Lantern, Textiles, Prayer, Rice,
            ];
            ch.locals = &[Tea, Rice, Textiles, Prayer, Lantern, Tea];
            ch.goals = [
                Pair(SilkShop, MarketGarden, 1),
                Serve(Textiles, 64),
                MarketDay(97),
            ];
        }
        16 => {
            ch.goals = [
                Upgrade(TeaGarden, 4),
                Pair(TeaHouse, TeaGarden, 2),
                Serve(Tea, 81),
            ];
        }
        18 => {
            ch.guests = &[
                Pottery, Prayer, Lantern, Tea, Pottery, Rice, Pottery, Textiles, Lantern, Bed,
            ];
            ch.goals = [
                Pair(Storehouse, CeramicStudio, 1),
                Route(Job::Freight, 6),
                Serve(Pottery, 92),
            ];
        }
        21 => {
            ch.name = "Papierlichter über Mukaishima";
            ch.greeting = "Die späte Fähre gleitet an beleuchteten Schaufenstern vorbei. Yui zählt die Lampions im Laden.";
            ch.hint = "Verschiedene Gäste mögen unterschiedliche Lampions und Stoffe. Passende Angebote zählen heute besonders.";
            ch.guests = &[
                Lantern, Textiles, Tea, Prayer, Pottery, Lantern, Bed, Textiles, Rice, Tea,
            ];
            ch.locals = &[Tea, Rice, Lantern, Prayer, Textiles, Pottery, Tea];
            ch.goals = [GoodDay(89), Specials(40), Variety(6)];
        }
        _ => {}
    }
    ch
}

const fn nordic(mut ch: Chapter, stage: usize) -> Chapter {
    ch.building_set = match stage {
        3 => Some(&NORDIC_SHEEP),
        4 => Some(&NORDIC_HEARTH),
        5..=10 => Some(&NORDIC_CASKS),
        11.. => Some(FJORD),
        _ => ch.building_set,
    };
    match stage {
        3 => {
            ch.hint = "Eine Schafweide versorgt die Weberei bis zu zwei Plätze weit. Im Tagesplan siehst du, wer warme Schals sucht.";
            ch.goals = [Pair(Sheepfold, Weaver, 1), Serve(Textiles, 32), Specials(9)];
        }
        6 => {
            ch.guests = &[
                HearthMeal, Casks, SmokedFish, Textiles, HearthMeal, SmokedFish, Casks,
            ];
        }
        7 => {
            ch.name = "Fackeln über Senja";
            ch.greeting = "Fischer tragen Proviant im Fackelschein zum Boot. Die Fassbinder hoffen, noch vor der Abfahrt fertig zu werden.";
            ch.hint = "Eine Fassbinderei neben dem Bootshaus spart eine Portion Proviant je Fahrt. Am Kai brauchen Gäste warmes Essen.";
            ch.guests = &[
                Casks, HearthMeal, SmokedFish, HearthMeal, Casks, Textiles, SmokedFish, Casks,
            ];
            ch.locals = &[SmokedFish, HearthMeal, Casks, Textiles, HearthMeal, Casks];
            ch.goals = [Pair(Boatyard, Cooperage, 1), Serve(Casks, 48), Voyages(5)];
        }
        9 => ch.goals = [Upgrade(Weaver, 3), Serve(Textiles, 53), Specials(31)],
        12 => ch.goals = [Residents(14), Upgrade(Sheepfold, 3), Serve(Textiles, 73)],
        14 => {
            ch.name = "Der Herd bleibt warm";
            ch.greeting = "Nach der letzten Fähre rücken die Gäste in der Festhalle zusammen. Draußen warten Fässer für die Überfahrt.";
            ch.hint = "Die Nachtgäste essen am Herd, kaufen Fässer und suchen warme Wolle. Nutze einen Festtag für die volle Halle.";
            ch.guests = &[
                HearthMeal, Casks, SmokedFish, HearthMeal, Textiles, Casks, HearthMeal, Textiles,
                SmokedFish,
            ];
            ch.locals = &[
                HearthMeal, SmokedFish, HearthMeal, Textiles, Casks, SmokedFish,
            ];
            ch.goals = [FeastGuests(24), Upgrade(FeastHall, 4), MarketDay(126)];
        }
        16 => {
            ch.name = "Das erste Signalfeuer";
            ch.greeting = "Leif kennt die Felsen vor dem Hafen. Ein sichtbares Feuer soll den Händlern beim Einlaufen helfen.";
            ch.hint = "Bis zu zwei Plätze vom Bootshaus entfernt erhöht ein Signalfeuer den Handelserlös.";
            ch.goals = [
                Pair(Boatyard, SignalTower, 1),
                CargoIncome(168),
                FerryDay(19),
            ];
        }
        17 => {
            ch.guests = &[
                HearthMeal, Casks, Textiles, SmokedFish, HearthMeal, Casks, Textiles, Casks,
                SmokedFish,
            ];
            ch.goals = [Serve(HearthMeal, 104), GoodDay(88), Variety(4)];
        }
        18 => ch.goals = [Upgrade(SignalTower, 4), CargoIncome(295), Specials(34)],
        21 => {
            ch.name = "Die Feuer von Karmøy";
            ch.greeting = "Über der dunklen See stehen die ersten Feuer. In der Gasse werden die letzten Stiefel und Vorratsfässer verladen.";
            ch.hint = "Halte Fang und Proviant in Reserve. Nachtgäste wollen Fässer, Stoff und einen Platz am Herd.";
            ch.guests = &[
                Casks, HearthMeal, Textiles, SmokedFish, Casks, HearthMeal, Casks, SmokedFish,
                Textiles,
            ];
            ch.locals = &[
                HearthMeal, SmokedFish, Casks, Textiles, SmokedFish, HearthMeal,
            ];
            ch.goals = [CargoIncome(355), Catch(43), Serve(Casks, 105)];
        }
        23 => {
            ch.guests = &[
                Casks, Textiles, HearthMeal, SmokedFish, Textiles, HearthMeal, Casks, SmokedFish,
            ];
            ch.goals = [Voyages(15), CargoIncome(430), FeastGuests(34)];
        }
        _ => {}
    }
    ch
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{CHAPTERS, REGION_SIZE, Shift};

    #[test]
    fn each_campaign_introduces_buildings_and_has_three_night_markets() {
        for region in Region::ALL {
            let start = region.start();
            let chapters = &CHAPTERS[start..start + REGION_SIZE];
            assert_eq!(
                chapters.iter().filter(|c| c.shift == Shift::Night).count(),
                3
            );
            assert!(chapters[23].allowed().len() > chapters[4].allowed().len());
            for pair in chapters.windows(2) {
                assert_ne!(
                    pair[0].goals.map(|g| g.label()),
                    pair[1].goals.map(|g| g.label())
                );
            }
        }
        assert!(
            CHAPTERS
                .iter()
                .flat_map(|chapter| chapter.goals)
                .filter(|goal| matches!(goal, Serve(Shade, _)))
                .count()
                <= 1
        );
        assert_eq!(Shift::Night.clock(1.0), "06:00");
        assert_eq!(Shift::Long.clock(1.0), "21:30");
    }
}
