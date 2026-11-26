# Spielreview und Abnahme

## Grafik, Spielgefühl und Spiellogik 3.6.0

- Alle 114 Rust-Tests sind bestanden, ein Fixture-Test für Sichtproben ist
  ausgeklammert. Formatprüfung und Clippy mit `-D warnings` sind sauber. Die
  Kampagnenprüfung umfasst weiterhin alle 120 Häfen mit drei Sternen und alle
  60 Abenteuerinseln, jetzt mit gemischten Rohstoffgebieten, Besonderheiten
  und dem Markt im Handelshafen. Ein Test hält Boden, Pflanzen und Wasser
  jeder Abenteuerinsel unter 200 000 Vertices.
- Neue Tests prüfen unter anderem:
  - die Reparatur beschädigter Hafen- und Inselspielstände samt `.bak`-Kopie,
  - Tagesereignisse auf 120 Häfen über acht Tage,
  - Bürger, die aus fertigen Gebäuden geschoben werden,
  - die Rückkehr von Begegnungen und die Tragkraft mit Speichern,
  - die Preisvorschau gegen den echten Markttag in 24 Häfen,
  - Opfergabe und Herold sowie Einkäufe im Inselmarkt,
  - die Übernahme alter Lautstärke-Schalter in die neuen Stufen,
  - das Blättern durch Reiter, die Klangwarteschlange und das Einschwingen
    der Animationen.
- Sichtproben unter Linux mit Mesa-Lavapipe (Vulkan) im Format 1280 × 800:
  - Naxos bei Tag und Nacht, die Abendabrechnung, die Preiskarten der
    Backstube, die Einstellungen und die Kampagnenkarte,
  - die Salzgärten mit Siedlung, Küste, Nacht, Inselkarte samt Besonderheit
    und Handy-Vorschau.
  Das Meer zeigt facettierte Wellen, die Laternen ihre Lichthöfe.
- Nicht im Bild geprüft wurden Inselfanfare und Feuerwerk, Opfergabe und
  Herold, Matsuri und Thing sowie die Klänge, denn der Container hat kein
  Audiogerät. Die Klanglogik ist durch Tests abgedeckt.
- Ein letzter Durchgang hat beide HUDs aufgeräumt. Die Sichtproben zeigen
  offene Inselziele zuerst, eine schlanke Zufriedenheitskarte und Tipps, die
  das Zielpanel freilassen. Tests, Clippy und Formatprüfung blieben grün.
- `tools/build-android.sh` hat mit SDK 36, Build-Tools 36.0.0 und NDK r28c
  beide Pakete gebaut:
  - Tablet: `game.nililane`, versionCode 21, ARM64 und x86_64.
  - Handy: `game.nililane.phone`, versionCode 9, ARM64.
  Geprüft wurden `apksigner verify`, `zipalign -c -P 16 4` und die
  16-KiB-Ausrichtung (0x4000) der ELF-Segmente.
- Diese APKs tragen den lokalen Testschlüssel, nicht das bisherige
  Veröffentlichungszertifikat. Sie lassen sich deshalb nicht als Update über
  3.5.1 installieren. Mit dem Release-Keystore müssen sie neu signiert
  werden.
- Veröffentlicht sind die Pakete im `downloads`-Branch (Commit `f3d5961`)
  per Git LFS und in den Releases `v3.6.0` und `v3.6.0-phone`. Alle drei
  Download-Links lieferten HTTP 200 mit der erwarteten Dateigröße.
- Der Wasser-Shader verzichtet auf Schleifen über Arrays, weil Lavapipe daran
  scheiterte. Auf echten Android-Geräten und im Android-Emulator ist 3.6.0
  noch nicht geprüft; der Container hat keine Hardware-Virtualisierung.

## Handelsfracht und Arbeitsaufträge 3.5.1

- In allen fünf Gebieten wurden Importwaren am Anleger gekauft, mit Nili
  getragen, gespeichert, geladen und am Rathaus abgeliefert. Die Prüfung
  zählt die Fracht erst nach der Ablieferung zum Lagerbestand.
- Die Werkstatt pausiert nach dem letzten Stopp. Ein gespeicherter
  Arbeitsgang behält seine Zutaten und seinen Fortschritt. Die
  Fischerhütte setzt den Fang mit einem neu zugeteilten Bürger fort.
- Die offenen Menüs werden sofort aktualisiert, wenn eine Begegnung
  beginnt oder das Handelsschiff zurückkommt.
- Alle 69 Rust-Tests, die Formatprüfung und Clippy sind bestanden.
  Die Kampagnenprüfung umfasst 120 Häfen mit drei Sternen und
  alle 60 Abenteuerinseln.

## Lebendige Siedlungen und Küstenhandel 3.5.0

- Alle 67 automatisierten Tests sind bestanden. In der Kampagnenprüfung
  wurden alle 120 Häfen mit drei Sternen und alle 60 Abenteuerinseln
  abgeschlossen. In den fünf Abenteuerfinalen wurden Handelswaren hergestellt, zum
  Anleger getragen und verschifft. Aus dem Erlös kamen die Baupläne und
  die Baustoffe für die Gebäude auf Stufe zwei.
- In jedem Gebiet wurden die Begegnungen nach dem Start der Partie
  ausgelöst. Die Prüfungen laufen zu den Personen und ihren Zielen,
  verbrauchen die Fracht und schreiben die Belohnungen einmal gut.
  Beim Brand lassen sich Wasserstelle und Küste als Quelle verwenden.
- Die 30 Modelle der fünf Sondergebäude wurden auf eigenständige
  Ausbaustufen und ihre Bauplatzgrenzen geprüft. Eine weitere Prüfung
  erzeugt Arbeitsplätze für alle Gebäudetypen in jeder Kultur.
  Sichtproben in Capua und Sonnenbogen zeigen die Arbeitsplätze in der
  Gasse und im Abenteuer sowie die Auftrags- und Handelsansicht.
- Beide Android-Ausgaben ließen sich im Emulator als Update installieren.
  Die vorhandenen Spielstände waren direkt nach der Installation
  bytegleich. Die ARM64-Dateien sind mit dem bisherigen
  Veröffentlichungszertifikat signiert.

## Handelsaufträge und Handwerk 3.4.0

- Die Kampagnenprüfung hat alle 120 Häfen mit drei Sternen beendet. Dafür
  wurden Gebäude errichtet, Angebote angepasst und Nilis Aufgaben zu Fuß
  erledigt. Bei den Umbauaufträgen zählt die neue Ausbaustufe auf dem
  vorgesehenen Grundstück.
- Die Prüfung aller 60 Abenteuerinseln hat die neuen Grundwaren und
  Meisterstücke in Werkstätten hergestellt, die Ausbauten mit Baumaterial
  beliefert und die Fracht zum Außenposten gebracht. Eine Probe mit einem
  älteren Spielstand erhält die Vorräte und die laufende Baustelle.
- In Capua war „18/22 Portionen Meze“ vollständig zu sehen. Die Aufträge
  konnten im Tablet-Layout nebeneinander und im Handy-Layout untereinander
  gelesen werden. Im Laden ist das aktuelle Auftragsangebot markiert.
- Getestet wurden Lieferungen bei vollem und fast vollem Lager. Der
  Sammler hört sofort auf, und die Restfracht bleibt vor dem Rathaus.
  Auch der Abbruch mitten im Abbau und mit einer Ladung unterwegs wurde
  geprüft. **L3** öffnet und schließt das Lager, auch während eines Ausflugs.
- Alle 62 Rust-Tests, die Formatprüfung und Clippy mit `-D warnings`
  waren erfolgreich. Die beiden ARM64-Pakete verwenden die bestehenden
  Paketkennungen und das bisherige Veröffentlichungszertifikat.

## Lesbare Anzeigen 3.3.1

- Vorratsanzeigen wachsen anhand des tatsächlich gerenderten Textes. Die
  Prüfung umfasst alle 24 Angebote, längere Mengenangaben wie „18/22
  Portionen Meze“ und die Hinweise der Gäste auf Handy und Tablet.
- Anzeigen über Gebäuden, Sammelstellen und Zielen bleiben an allen vier
  Bildschirmrändern sichtbar.

## Baustellen und Lieferungen 3.3.0

- Beim Antippen von **Weg anzeigen** auf dem Tablet erschien die Entfernung
  zum Bauplatz. Die Spur nutzt begehbare Wege. Ein Zielzeichen bleibt auch
  über einem verdeckten Ziel sichtbar. Die Routenprüfung läuft um das Rathaus.
- Auf Kleions Ufer ließ sich ein Wohnhof vormerken, obwohl kein Holz im
  Lager lag. Bei 72 von 72 belegten Lagerplätzen legte Nili sechs Holz
  direkt auf der Baustelle ab. Anschließend holte sie Stein am Rathaus,
  brachte ihn zur Baustelle und baute den Wohnhof. Zwischen Abladen und
  Fertigstellung waren die Stapel und der Baufortschritt zu sehen.
- In der Simulation holen Bautrupps Material aus dem Lager und tragen es
  zum Bauplatz. Zwei Arbeiter stellen das Gebäude früher fertig als einer.
  Auf allen 60 Abenteuerinseln wurden Gebäude und Ausbauten über Baustellen
  errichtet und die Inselziele samt Lieferungen erfüllt. Alle 120 Hafeninseln
  bestehen die Kampagnenprüfung.
- Bürger können in allen fünf Gebieten zum Außenposten laufen. Ein Test
  speichert auf dem Hinweg, lädt erneut und prüft eine einzige Zustellung
  sowie die Rückkehr des Bürgers. Nili kann Lieferungen am Rathaus packen.
- Die Siedlungsbilanz nach Kleions Ufer zeigte Belegung, Einwohner und
  Vorräte. Das Ende der Insel und die Belohnung werden vor der Wahl zum
  Bleiben oder Weiterreisen gespeichert.
- Die x86_64-APKs liefen im Android-Emulator. Das Tablet-Update ließ den
  vorhandenen Abenteuer-Spielstand beim Installieren bytegleich. Das
  Einstellungsmenü dunkelte im Handyformat mit simuliertem Displayausschnitt
  die linke, rechte und untere Bildschirmkante ab. Die ARM64-Pakete sind
  mit dem bisherigen Android-Zertifikat signiert.

Alle 54 Rust-Tests, die Formatprüfung und Clippy mit `-D warnings` waren erfolgreich.

## Abenteuer-Logistik 3.2.0

- **Einstieg.** Auf Kleions Ufer zeigte der Abbau einen Arbeitsbalken
  und einen fallenden Baum. Nili trug zehn Holz zum Rathaus. Der
  Bestand stand vorher auf null und nach dem Abladen auf zehn. Der
  Stumpf blieb am Weg sichtbar.
- **Kampagne.** Die Spielsimulation beendet alle 60 Inseln mit
  Abbauwegen, Lagergrenzen, laufender Produktion, Bauplänen und 95
  Lieferungen. Die späteren Testpartien nutzen zusätzliche Speicher
  und sortieren Überschüsse aus. Alle 120 Hafeninseln bestehen
  weiterhin die Kampagnenprüfung.
- **Spielstände.** Ein voller Rohstoffspeicher nimmt nur die freie
  Menge auf. Nilis Restfracht bleibt erhalten. Der Test lädt einen
  Bürger auf dem Rückweg und einen laufenden Fischfang erneut. Beide
  Ladungen werden einmal eingelagert. Die älteren Vorräte lassen sich
  mit dem erweiterten Warenverzeichnis laden.
- **Handy-Ausgabe.** Die signierte APK verwendet dieselbe Paket-ID und
  dasselbe Zertifikat wie die vorherige Handy-Fassung. Das Update auf
  3.2.0 lief im Android-Emulator. Das Startmenü und der Einstieg in die
  Abenteuerkampagne reagierten auf Touch. Der Tablet-Spielstand im
  Emulator blieb bytegleich. Auf einem physischen S24 Ultra steht der
  Durchlauf noch aus.
- **Tablet-Ausgabe.** Im Android-Emulator im Format 800 × 600 ließ sich
  3.2.0 über 3.0.0 installieren. Der Hafen- und der Abenteuer-Spielstand
  waren unmittelbar nach dem Update bytegleich. Der virtuelle
  Xbox-Controller öffnete die Abenteuerkarte und setzte Kleions Ufer
  mit zehn bereits eingelagerten Einheiten Holz fort. Für die ARM64-APK
  wurden Paketkennung, Signatur und 16-KiB-Ausrichtung geprüft.

Alle 48 Rust-Tests und Clippy mit `-D warnings` sind erfolgreich.
[Abbauen und Abladen auf Kleions Ufer](bilder/handy-abenteuer.jpg)

## Handy-Ausgabe 3.1.0

- **Oberfläche.** Das Handy-Layout wurde im Querformat 1300 × 600 geprüft.
  In Naxos wurden Tagesplan, Gästebuch und Backstuben-Ausbau geöffnet.
  Auf Kleions Ufer blieb die Insel beim Gehen sichtbar und die Ortsaktion
  wechselte in Reichweite des Holzes zu „Sammeln“.
- **Steuerung.** Der erste Kontakt am Rand des Sticks hielt Nili an Ort
  und Stelle. Das Ziehen bewegte Nili und beim Loslassen blieb sie stehen.
  Rust-Tests prüfen die Auswahl mit dem zweiten Finger und das Wischen
  über einer Schaltfläche ohne versehentliche Auswahl.
- **Installation.** Der x86_64-Testbuild öffnete das Handy-Startmenü im
  Android-Emulator neben der Tablet-Ausgabe. Der Tablet-Spielstand war
  nach der Installation bytegleich. Das ARM64-Paket wurde auf Paket-ID,
  Architektur, Signatur und Ausrichtung geprüft. Der Praxistest auf einem
  physischen Galaxy S24 Ultra steht aus.

[Handy-Oberfläche auf Kleions Ufer](bilder/handy-abenteuer.jpg)

## Abenteuerkampagne 3.0.0

- **Einstieg.** Auf Kleions Ufer wurden Rohstoffe gesammelt, ein Wohnhof
  und ein Sägewerk gebaut und Bürger für die Produktion und zum Sammeln
  eingeteilt. Die Backstube schloss die erste Insel ab. Anschließend
  wurde das Tal der Oliven freigeschaltet und gestartet. Die Einführung öffnet die passende Baukategorie und gibt
  dem empfohlenen Bauplan den Controller-Fokus. Die Ortsaktion folgt
  dem markierten Tutorialziel, sobald Nili in Reichweite ist. Der
  Touch-Stick sitzt unten links, getrennt von den Vorräten.
  Die Einführung wurde auf der Karte erneut gestartet. Danach war der
  vorhandene Siedlungsspielstand bytegenau identisch. Die Gebietsschalter
  der Garderobe passen im Tabletformat in den Rahmen. Der Kauf und das
  Speichern der Maulbeertoga wurden im Spiel geprüft.
- **Spielentwicklung.** Der Kampagnentest löst alle 60 Inselziele mit
  Vorräten, Baukosten, Ausbau, Produktionsketten und Bauplanaufträgen.
  Er sucht mit den Spielkollisionen Wege zu den Rohstoffen und
  Auftragsorten und speichert jede abgeschlossene Insel für einen
  erneuten Ladevorgang. Überlappende Rohstoffplätze wurden korrigiert.
  Die 90 Werkstattmodelle aller fünf Gebiete passen in ihre Bauplätze
  und unterscheiden sich in jeder Stufe. Auch die 120 Häfen bleiben
  Teil der automatisierten Prüfung.
- **Spielkritik.** Spätere Inseln kombinieren Bergpfade, Warenlieferungen,
  Gebäudeausbau und andere Vorratsziele. Im Spiel wurde die Lieferung
  in Theras Aschefeldern abgegeben. Der Bestand sank um zwölf Mahlzeiten
  und der Bauplan wurde freigeschaltet. Die Tag- und Nachtansicht wurde
  in bebauten Siedlungen geprüft. Getrennte Vegetationsflächen lassen
  sich beim Rendern außerhalb der Kamera ausblenden.
- **Veröffentlichung.** Die signierten ARM64- und x86_64-APKs haben
  Paketversion 14 und verwenden das Zertifikat der bisherigen Version.
  Auf dem Android-Emulator im Tabletformat 800 × 600 wurden die
  Installation über 2.7.0, der Einstieg über Touch, das Gehen mit dem
  Touch-Stick, Sammeln mit der Xbox-A-Taste und die Inselkarte mit X
  geprüft. Das bisherige Hafen-Save blieb bytegenau erhalten.
  Der ARM64-Build wurde als Paket geprüft. Der Praxistest auf einem
  physischen Tablet steht aus.

Alle 41 Rust-Tests und Clippy mit `-D warnings` liefen erfolgreich.
Die 65 Abenteuer-Musikdateien wurden als verschiedene Stereoaufnahmen
im gepackten Vorbis-Format geprüft.

## Hafenreisen 2.7.0

Die Kampagnenprüfung baut und versorgt alle 120 Häfen durch ihre regulären
Bau-, Preis-, Ausbau- und Hafenaktionen. Sie steuert Nili zu Abholorten
und Zielen, setzt Preise und Angebote und lädt den Spielstand nach jedem
Hafenabschluss erneut. Alle Häfen erreichten drei Sterne.

Die Kampagnenkarte bietet nach zwölf Inseln den Weg ins nächste Gebiet
und die optionale Weiterfahrt in der bisherigen Region. Ein Test wechselt
mit einer noch laufenden Schicht zwischen den Routen und lädt beide
Fortschritte. Der Android-Emulator erreichte mit der A-Taste Ostia
unmittelbar nach Kallisti.

Die Modellprüfung rendert die sechs Stufen aller 67 regionalen
Gebäudevarianten. Sie hat unter anderem durchstoßene Dächer, überlagerte
Vordächer und ungestützte Aufbauten sichtbar gemacht. Die Ausbaustufen
haben jetzt durchgehende Fassaden und Dächer. Die Laufwege und die Maße
der Modelle wurden ebenfalls geprüft. Am Tabletformat war die
Vorratsanzeige mit „7/7 Laibe“ um 20:36 und 20:52 lesbar.
Ein eigener Test schließt einen kurzen Auftrag gegen Ende der Schicht ab.

## Frühere Abnahme · 2.4.0

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
