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
  `people.rs` baut römische und griechische Figuren mit einem gemeinsamen
  Skelett. `construction.rs` steuert Gerüst, Ausbauteile und Staub.
  `work.rs` und `actors.rs` gestalten Nilis Bewegungen, Requisiten und Punktspuren.
  `clothes.rs` erzeugt Nilis Gewänder, Hüte und Accessoires.
  `pets.rs` enthält die Modelle und Bewegungen der Katzen und Hunde.
  `late_upgrades.rs` ergänzt die Ausbauten auf Stufe 5 und 6.
  `sky.rs` zeichnet Wolken, Regenschlieren und Sturmwellen.
- Die Bildtafeln und die meisten Spielsymbole in `ui/` kommen aus `art/make_ui.py`.
  Dazu gehören die Gästehinweise, das Gästebuch, die Controller-Symbole und das Budgetsymbol.
  Das Feuersymbol und die Garderobenbilder wurden für Nililane 2.4 gezeichnet.
  `art/make_audio.py` erzeugt das Meeresrauschen und die Klangmotive
  für Bau, Ausbau, Verkauf, Hafenglocke und Nilis Hilfe sowie das Regenbett und
  die römische Musik mit Leier, Flöte und Rahmentrommel.
- `art/make_harbor_music.py` komponiert die 48 Hafenthemen und ihre
  Abendfassungen. `art/make_street_audio.py` erzeugt die Ereignismotive
  und Klänge für Tiere und Straßenaktionen. `art/make_street_art.py`
  zeichnet die Altersgruppen, Straßensymbole und neuen Kleidungsbilder.
- DM Sans und DM Serif Text stehen unter der SIL Open Font License.
  Die Lizenztexte liegen bei den Schriften in `fonts/`.
