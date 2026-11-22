# Asset-Herkunft

Nililane baut auf Hippogroves Nilpferd, Ägäis-Architektur, Musik und
einzelnen UI-Klängen auf. Die Modelle von Kiefer und Stein kamen über
Hippogrove aus Waldwinkel. [Hippogrove-Revision](https://github.com/bthginfo/Hippogrove/tree/37742c2ceec53c80b88902dac3c5ca7dad93a1a8) · [Waldwinkel](https://github.com/bthginfo/waldwinkel)

- `models/hippo.glb` ist Nilis Körpermodell. Vier neu gebaute Gliedmaßen
  und Bevy-Animationen bilden sein Laufspiel.
- `models/villages/aegean.glb` liefert Wohnhof und Lagergebäude.
  `models/works/aegean.glb` liefert den Leuchtturm.
- Die 3D-Modelle für Backstube, Brunnen, Töpferei, Stoa, Hain, Taverne,
  Gasthaus, Figuren, Boote und Ufer entstehen in `src/world/`.
  `roman.rs` enthält die römische Architektur und das andere Tiberufer.
  `ships.rs` baut Rümpfe, Takelage, Segel, Ladung und Bugwellen.
  `people.rs` baut die Figuren aller fünf Regionen mit einem gemeinsamen
  Skelett. `construction.rs` steuert Gerüst, Ausbauteile und Staub.
  `work.rs` und `actors.rs` gestalten Nilis Bewegungen, Requisiten und Punktspuren.
  `clothes.rs` erzeugt Nilis Gewänder, Hüte und Accessoires.
  `pets.rs` enthält die Modelle und Bewegungen der Katzen und Hunde.
  Die jeweiligen Gebäudemodule enthalten alle sechs Ausbaustufen. `lanterns.rs` baut die Lichtquellen der Nachtinseln.
  `sky.rs` zeichnet Wolken, Regenschlieren und Sturmwellen.
- Die Bildtafeln und die meisten Spielsymbole in `ui/` kommen aus `art/make_ui.py`.
  Dazu gehören die Gästehinweise, das Gästebuch, die Controller-Symbole und das Budgetsymbol.
  Das Feuersymbol und die Garderobenbilder wurden für Nililane 2.4 gezeichnet.
  `art/make_audio.py` erzeugt das Meeresrauschen und die Klangmotive
  für Bau, Ausbau, Verkauf, Hafenglocke und Nilis Hilfe sowie das Regenbett und
  die römische Musik mit Leier, Flöte und Rahmentrommel.
- Die 120 Hafenstücke und ihre Abendfassungen sind in `art/harbor_scores.py`
  ausgeschrieben. `art/make_harbor_music.py` spielt die Partituren mit FluidSynth
  und dem SoundFont GeneralUser GS 2.0.3 von S. Christian Collins ein.
  Im Spiel liegen die fertigen Vorbis-Aufnahmen. [SoundFont und Lizenz](https://schristiancollins.com/generaluser.php)
- `art/make_street_audio.py` erzeugt Klänge für Tiere und Straßenaktionen.
  `art/make_street_art.py` zeichnet die Altersgruppen, Straßensymbole und
  neuen Kleidungsbilder.
- DM Sans und DM Serif Text stehen unter der SIL Open Font License.
  Die Lizenztexte liegen bei den Schriften in `fonts/`.

## Japan und die nordischen Häfen

`src/world/japan.rs` baut Tempel, Reisfelder, Holzhäuser und die Küste.
Als Vorbilder dienten die historische Hafenstadt Tomonoura und die
Insel- und Tempellandschaften der Inlandsee. Die buddhistische Tempelhalle
und die Torii an der Küste sind getrennte Bauwerke. [Stadt Fukuyama](https://www.city.fukuyama.hiroshima.jp/soshiki/kanko/85749.html) · [Miyajima, Japan National Tourism Organization](https://www.japan.travel/en/spot/ma_179/)

`src/world/nordic.rs` baut Langhäuser, Handwerksbauten und Fjordlandschaften.
Die Modelle greifen Grasdächer, Holzbau und den Schiffbau der Wikingerzeit
auf. Die Gäste tragen auch wollene Mäntel und Fibeln. [Lofotr Viking Museum](https://www.museumnord.no/en/our-venues/lofotr-viking-museum/) · [Viking Ship Museum Roskilde](https://www.vikingeskibsmuseet.dk/en/visit-the-museum/exhibitions/the-five-viking-ships/skuldelev-1) · [Dänisches Nationalmuseum](https://en.natmus.dk/historical-knowledge/denmark/prehistoric-period-until-1050-ad/the-viking-age/the-people/clothes-and-jewellery/)

`art/make_japan_art.py` und `art/make_nordic_art.py` zeichnen die
Hafenkarten, Waren- und Kleidungsbilder und erzeugen den Glockenklang
und das Signal zur Abfahrt. Die neuen Hafenstücke verwenden für Japan
Zither und Bambusflöte, für die nordischen Häfen Fidel, Blockflöte und Hackbrett.


## Oasenreiche

Fiktive Häfen und Karawanenorte mit Lehmhöfen, Gewölben, Palmen und
Wasserstollen. `src/world/oasis.rs` gestaltet Gelände und Gebäude,
`art/make_oasis_art.py` zeichnet Kartenbilder, Symbole und Nilis
Kleidungsbilder. `art/harbor_scores.py` enthält die neuen Partituren.

## Abenteuerkampagne

Waldwinkel liefert die zusätzlich eingebundenen Erzmodelle
`resource_copper.glb` und `resource_crystal.glb`. Die Abenteuerlandschaft,
Rathäuser, Werkstätten und neuen Outfitmodelle entstehen aus den
geometrischen Bausteinen von Nililane. [Waldwinkel, Quellstand](https://github.com/bthginfo/Waldwinkel/tree/148987b069b250a249f3216f47aedfbc3636985e)

`art/make_adventure_art.py` zeichnet die Gebietsbilder, Materialsymbole und
25 Vorschauen für die Garderobe. Die fünf Abenteuerpartituren und ihre
Inselarrangements stehen in `art/make_adventure_music.py`. Die Aufnahme
nutzt denselben SoundFont wie die Hafenmusik. Die Sammelklänge entstehen
in `art/make_adventure_sfx.py`.
