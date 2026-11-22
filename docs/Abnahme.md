# Spielreview und Abnahme · Nililane 2.3.1

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
| Rezepte und Sortimente pro Laden | Preise und Produktionsketten für Wasser, Oliven und Warenvorräte. Ein wählbares Angebot innerhalb eines Ladens ist offen. |
| Straßengestaltung | Unterschiedliche Küsten, Bauplätze und sichtbare Ausbauten. Frei platzierbare Bepflanzung und Dekoration ist offen. |
| Kleine Aufgaben während des Tages | Nili übernimmt Lieferungen, Gästeführung und Hilfe nach Unwettern. |
| Freies Bauen | Nach dem Kapitelabschluss kann man im Hafen weiterspielen. Ein eigener Einstieg ohne Kampagnenziele ist offen. |

[Spielbeschreibung des Minami-Lane-Teams](https://doottinygames.itch.io/minami-lane)

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

## Prüfungen

- Formatierung, Clippy mit Warnungen als Fehler und 16 Rust-Tests
  einschließlich der drei Einstiegsinseln und des erneuten Ladens während
  einer Bitte vom Kai. Die Prüfung der Fährmodelle misst den Abstand von
  Rumpf und Rudern zum Steg bei vier Wellenstärken.
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
