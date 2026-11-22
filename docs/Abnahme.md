# Spielreview und Abnahme · Nililane 2.4.0

## Einstieg als neuer Spieler

Auf Naxos zeigt der Einstieg den Bauplatz, den Preis und den Tagesstart.
Beim Eintreffen der Fähre und bei den ersten Gästestimmen hält die Uhr an.
Ein heller Ausschnitt markiert den jeweiligen Ort, daneben zeigt die
Hinweisbox die nächste Aktion. Auf Amorgos folgen der Brunnen und die
Wasserlieferung. Auf Delos folgen Töpferei, Nachschub und Ausbau.

Bei der Wahl einer Nili-Tour zeigen zwei Schilder Abholort und Ziel.
Die Punktspur und das Schild über Nili folgen der Arbeit. Krüge, Kisten,
Fahne, Zweige und Treibgut gehören zu unterschiedlichen Bewegungen.
Die Lieferung wird bei ihrer Ankunft einmal abgerechnet. Gesehene
Hinweise und laufende Touren werden gespeichert.

Das Tutorial lässt sich auf der Kampagnenkarte in beiden Regionen neu
starten. Die Übungsreise beginnt auf Naxos und speichert ihren Fortschritt
separat. Nach Delos oder über **Übung beenden** führt der Weg zurück in
die Kampagnenkarte.

## Veröffentlichung

Die signierte APK enthält ARM64 und x86_64 und verwendet dieselbe Paket-ID
und dasselbe Veröffentlichungszertifikat wie 1.0.0. Android 8 und Vulkan 1.1
sind die Mindestvoraussetzungen. Modelle, Schriften, Bilder und Musik liegen
im Offline-Paket. Die README führt zur APK und ihrer SHA-256-Prüfsumme.
[Asset-Herkunft](../assets/QUELLEN.md)

## Spielentwicklung

- 48 Häfen lassen sich mit regulären Bau-, Preis-, Ausbau- und Hafenaktionen
  abschließen. Die Kampagne schaltet jeden Hafen nach dem vorherigen frei.
  Ostia wird nach Korinth verfügbar.
- Eine 1.0-Gasse behält beim Laden ihre Parzellen, Preise, Vorräte,
  Tageszeit, Bitten vom Kai und Sternrekorde. Neue Bauplätze ergänzen die
  bisherigen Platznummern. Laufende Touren bleiben im Autosave erhalten
  und werden nach dem Laden genau einmal abgerechnet.
- Ein gegliederter Laufzyklus setzt die Füße der Gäste auf, abgerundete
  Wegpunkte geben die Blickrichtung vor. Die Figur läuft von der Wohnung
  oder Fähre zur Straße, dann zum konkreten Laden und wieder zurück.
  Baugerüste, Ausbauteile und Staub verschwinden nach der Animation.
- Die Referenzstrategien ergeben bei Normaltempo gut fünf Stunden
  geöffnete Gasse für Griechenland und gut sechs für Rom. Planung,
  Tourenauswahl, Berichte und Wiederholungen kommen dazu. Rund acht
  Stunden pro Reise bleiben das Ziel für einen ersten Durchlauf.

## Oberfläche und Bedienung

Die Kampagnenkarte zeigt Größe und Küstenform. Im Baumenü stehen Kosten
und Ertrag neben den Gebäudebildern. Die Preisstufen zeigen, wie viele
erwartete Gäste genug Geld haben. Der Ausbau zeigt Vorrat oder Kapazität
vorher und nachher. Wetterkarten zeigen Änderungen bei Wasser und
Fährgästen. Die Texte in allen 48 Häfen sind kürzer. Die Spielhilfe
verbindet Controller-Symbole mit kurzen Hinweisen und bebilderten Abläufen.

Die Zufriedenheitskarte zeigt Anteil und Anzahl erfüllter Wünsche. Die
Gesichter, Warenbilder und Texte tauchen über den Gästen auf, sobald sie
das Ziel erreicht haben. Über **View**, **F** oder per Tipp öffnet sich das
Gästebuch. Dort führen Filter zum jeweiligen Grund und Karten zum
betroffenen Laden. Zum Preis steht das Budget des Gastes. Gespeicherte
Stimmen stehen am Folgemorgen noch zur Verfügung. Die Bestandsübersicht
während des Tages berücksichtigt die noch anstehenden Wünsche.

## Abgleich mit Minami Lane

| Mechanik | Stand in Nililane |
| --- | --- |
| Einzelne Gästereaktionen als Entscheidungshilfe | Sprechblasen in der Gasse, Gründe und Gastbudget im Gästebuch, Sprung zum betroffenen Laden. |
| Sortimente und verschiedene Gästewünsche | Backstuben, Töpfereien und Tavernen zeigen erwartete Käufe der drei Altersgruppen und bieten passende Spezialitäten. |
| Straßengestaltung | Unterschiedliche Küsten, Bauplätze und sichtbare Ausbauten. Frei platzierbare Bepflanzung und Dekoration ist offen. |
| Kleine Aufgaben während des Tages | Nili wird bei Lieferungen, Gästeführung, Ofenbränden und Hilfe nach Unwettern direkt gesteuert. |
| Freies Bauen | Nach dem Kapitelabschluss kann man im Hafen weiterspielen. Ein eigener Einstieg ohne Kampagnenziele ist offen. |

[Spielbeschreibung des Minami-Lane-Teams](https://doottinygames.itch.io/minami-lane)

GamesRadar+ hebt die kleinen Tätigkeiten während des Tages und die
persönlichen Gästestimmen hervor. Games Asylum beschreibt, wie
unterschiedliche Geschmäcker die Auswahl im Laden beeinflussen.
Gamereactor nennt die Wiederholung von Bauen und Warten als Schwäche.
Nililane erweitert deshalb die Eingriffe am laufenden Tag und zeigt bei
Spezialitäten, für welche Altersgruppe sich eine Änderung lohnt.
[GamesRadar+](https://www.gamesradar.com/games/management/i-finally-tried-minami-lane-after-seeing-over-5-000-positive-reviews-and-within-4-hours-its-already-the-chillest-management-game-ive-ever-played/) · [Games Asylum](https://www.gamesasylum.com/2025/07/10/minami-lane-review/) · [Gamereactor](https://www.gamereactor.eu/minami-lane-1578993/)

## Kampagnenprüfung in 2.3

| Reise | Häfen mit drei Sternen | Spielstand geladen |
| --- | --- | --- |
| Griechenland | 24 von 24 | Nach jedem Hafen |
| Rom | 24 von 24 | Nach jedem Hafen |

Die Simulation beginnt jeden Hafen mit den vorgesehenen Gebäuden und
Münzen. Sie baut und erweitert Läden, stellt Preise ein, wählt Hafenpläne,
schickt Nili los und wertet den Abend aus. Der nächste Hafen öffnet sich
nach dem Abschluss. Die Prüfung umfasst die Aufträge, den Zeitbonus und
die Bitten vom Kai.

In Delos liefert die Töpferei auf Stufe 2 bei knappem Wasser zunächst
weniger Schalen. Der Einstieg markiert dann zuerst einen Brunnenplatz.
Die Ausbauvorschau zeigt solche Engpässe auch in den übrigen Häfen an.
Puteoli erreicht mit zwei Gasthäusern drei Sterne bis zum 23. Tag.
Der Hafentipp nennt die beiden Häuser und ihren Wasserbedarf.

## Kampagnenprüfung in 2.5

Die Kampagnenprüfung steuert Nili über den Laufkorridor und den Steg zu
beiden Übergabepunkten. Die Marktuhr läuft während Weg und Arbeit weiter.
Die Referenzpartien erreichen drei Sterne in allen 48 Häfen und laden
jeden abgeschlossenen Hafen erneut aus dem Spielstand. Weitere Prüfungen
decken Wetterwechsel, Gästezahlen, Spezialitätenerlös, Kräutergärten,
wiederholte Gästetouren und abgelaufene Aufträge ab.

Der Vergleich mit den Referenzpartien ergibt 206 Tage in Griechenland und
326 Tage in Rom. Das entspricht bei Normaltempo rund 6½ und 11½ Stunden
reiner Marktzeit. Dazu kommen Planung und Menüs. Die 4:3-Prüfung der
Gebäudeansicht zeigt selbst mit großer Oberfläche alle neun Angebote,
die Preise und die Zurück-Taste gleichzeitig. [Gebäudeansicht](bilder/angebote.jpg)

Die Tagesnotizen greifen die Rückmeldungen als Grundlage für die nächste
Planung auf. Die Referenz für Ladenentscheidungen und die Kombination
aus Bevölkerungs-, Zufriedenheits- und Umsatzzielen waren die Berichte
über Minami Lane. [Siliconera](https://www.siliconera.com/review-minami-lane-feels-neighborly/) · [Missionsübersicht](https://www.trueachievements.com/game/Minami-Lane/walkthrough/3)

## Musik und Auftragsstart in 2.5.2

Die 48 Partituren und ihre Abendfassungen sind als Vorbis-Aufnahmen im
Android-Paket enthalten. Alle 96 Dateien lassen sich dekodieren und sind
dem passenden Hafen zugeordnet. Die Prüfung misst Pegel und Loop-Übergänge.
Die Tagesstücke liegen bei rund −19 LUFS, die Abendfassungen bei −20,5 LUFS.

Im Android-Emulator hat ein Tipp auf die markierte Backstube von Naxos
den Auftrag geöffnet. Nilis Position blieb bei Auftragsbeginn erhalten
und die Hafenuhr lief weiter. Der Spielstand war nach der Installation
über Version 2.5.1 bytegleich. 28 Rust-Tests sind bestanden.

## Prüfungen

- Formatierung, Clippy mit Warnungen als Fehler und 20 Rust-Tests
  einschließlich der drei Einstiegsinseln und des erneuten Ladens während
  einer Bitte vom Kai. Die Prüfung der Fährmodelle misst den Abstand von
  Rumpf und Rudern zum Steg bei vier Wellenstärken.
- In 2.4 erreichen die Fußwege alle Gebäude und Anleger der 48 Häfen.
  Eine gespeicherte manuelle Lieferung hält die Hafenuhr an, lässt sich
  fortsetzen und wird einmal abgerechnet. In der Kampagnensimulation
  ließen sich erneut alle 48 Häfen mit drei Sternen abschließen.
- Im 4:3-Format wurden Ofenbrand und Kistenbergung ab dem letzten
  Arbeitsschritt gespielt. Beim Klick auf eine Backstube erschienen
  Vorrat, erwartete Gäste und Wasserbedarf. Die Auswahl der Honigfladen
  brachte in einem Vergleichslauf zwei Punkte zusätzliches Ansehen bei
  gleicher Verkaufszahl.
- In der Garderobe blieben nach Purpurtoga und Fischerhut von 36 freien
  Sternen 16 übrig. Beide Stücke erschienen am Modell. Die Sternrekorde
  beeinflussen die Freischaltung auch nach dem Kauf.
- Der Vergleich der Android-Aufnahmen im 4:3-Format zeigt ruhige Dachkanten
  und Schatten im Startmenü von 2.3.2. Menschen und Schiffe bewegen sich
  weiter. Die Aufnahmen stammen aus dem Vulkan-Emulator.
- Die Installation über 2.3.1 hat den Spielstand bytegenau erhalten.
  Auf der Kampagnenkarte startete eine simulierte Y-Taste Naxos an Tag 1.
  Der bestehende Naxos-Spielstand an Tag 3 blieb separat gespeichert.
- In der Desktop-Fassung ließ sich die Übung von der römischen
  Kampagnenkarte aus starten. Die aus der Simulation erzeugten
  Abschlussberichte führten von Naxos über Amorgos nach Delos und
  anschließend zurück zur Karte. Der reguläre Hafen und die
  Sternrekorde blieben bei jedem Übergang erhalten.
- Der Liegeplatz wurde bei Sonne auf Naxos und bei Sturm in Capua im
  4:3-Format geprüft. In Capua entfernte sich die Fähre mit gesetztem Segel,
  sobald der Landgang weg war. Im Titelbild ist der Steg mit der ganzen
  Fähre zu sehen. [Titelbild](bilder/faehre.jpg)
- Gerenderte Stichproben in Griechenland und Rom im Querformat. In Kythnos
  öffnete ein Tipp auf die Sprechblase die passende Stimme. Der Preisfilter
  führte zur Stoa und die Zurück-Taste zur ausgewählten Rückmeldung.
- Android 15 mit Vulkan und simulierten Gamepad-Eingaben. Nach der
  Installation über 1.0.0 haben A und Y den vorhandenen Tag samt Bauplätzen,
  Brotpreis und Verkaufszahlen fortgesetzt. In Amorgos haben R3,
  Steuerkreuz und A einen Wasserlauf ausgewählt und zur Backstube
  geschickt. Der Betrieb erhielt die Lieferung einmal.
- Für 2.1 wurde im Android-15-Emulator im 4:3-Format ein laufender Tag
  aus 2.0 fortgesetzt. Simulierte Gamepad-Eingaben öffneten mit View das
  Gästebuch tagsüber und in der Abendabrechnung. B führte zur Abrechnung
  zurück, A zum nächsten Morgen. Die Gästestimmen blieben gespeichert.
- Für 2.2 hat die Installation über 2.1 den Spielstand bytegenau erhalten.
  Im Android-15-Emulator im 4:3-Format führten simulierte Gamepad-Eingaben
  durch den bebilderten Einstieg, den Bau der Backstube, den günstigen
  Brotpreis, den Tagesstart und die erste Bitte vom Kai. In der Desktop-Prüfung
  lieferte Nili auf Amorgos einmal Wasser an die markierte Backstube.
  Die Vorratskarten von Capua blieben in der großen UI lesbar.
- In der Desktop-Fassung hielten die neuen Hinweise den Spieltag auf Naxos
  an. Auf Amorgos brachten die markierte Quelle und das Lieferziel Nili
  zur Backstube. In Delos zeigte die Ausbauvorschau im 4:3-Format mit
  großer Oberfläche zunächst 5 → 3 Schalen. Nach dem Brunnenbau zeigte
  sie 5 → 7 Schalen und der Ausbau gelang. In Capua lieferte Nili die
  Gästeführung, die Schattentour und das Kai-Räumen jeweils einmal ab.
- Für 2.3 öffnete die simulierte A-Taste im Android-15-Emulator den ersten
  Naxos-Hinweis im 4:3-Format. Die Prüfung in diesem Durchlauf reichte bis
  zu diesem Hinweis. Die Installation von 2.3.1 über 2.3.0 hat den
  vorhandenen Spielstand bytegenau erhalten.
- APK-Signatur v2 und v3, 16-KiB-Ausrichtung beider Bibliotheken und
  SHA-256-Abgleich des GitHub-Downloads.
- Der Test auf einem physischen Android-Tablet steht noch aus.

## Japan und die nordischen Häfen in 2.6.0

- **Erster Einstieg.** In Japan und den nordischen Häfen folgen die neuen
  Systeme über mehrere Kapitel. Die Tempelglocke erhält einen Marker in der
  Gasse und eine eigene Aktion in Reichweite von Nili. Die Bootshauskarten
  vergleichen Proviant, früheste Rückkehr und Silberertrag. An einem
  begehbaren Bootshaus war die erste verfügbare Abfahrt vorausgewählt und
  ließ sich direkt bestätigen. Die Hafenuhr lief währenddessen weiter.
- **Kampagne und Fortschritt.** 96 von 96 Häfen erreichen in den
  Referenzpartien drei Sterne. Nach der Abendabrechnung werden Spielstand und
  nächster Hafen geprüft. Japans geöffnete Gassen dauern bei Normaltempo
  8 Stunden und 14 Minuten, die nordischen 9 Stunden und 33 Minuten.
  Planung, Menüs und Wiederholungen kommen hinzu.
- **Simulation.** Die 31 Rust-Tests decken die manuelle Bewegung zu den
  Einsatzorten, Tempelbesuche, kostenfreies Gebet, die beiden Freischaltungen,
  Kleidung, Warenvorräte und bei Sturm verzögerte Handelsfahrten ab. Der
  Spielstand speichert Proviantabzug, Rückkehrtag und einmalige Auszahlung.
- **Oberfläche.** Im Querformat 4:3 zeigt die Garderobe Japan und Nordisch
  auf eigenen Karten. Gesperrte Artikel zeigen Sternpreis und Hafen.
  [Japan](bilder/japan.jpg) · [Nordische Häfen](bilder/nordische-haefen.jpg)
- **Android.** Die APKs für ARM64 und x86_64 enthalten je alle 192 Musikaufnahmen offline. Signatur und 16-KiB-Ausrichtung sind
  geprüft. Die Installation über 2.5.2 hat den vorhandenen Spielstand
  bytegleich erhalten. Im Emulator haben A-Tastendrücke von Rom nach
  Tomonoura und von Awajishima nach Borg geführt.
