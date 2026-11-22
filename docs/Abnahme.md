# Spielreview und Abnahme · Nililane 2.1

## Einstieg als neuer Spieler

Naxos führt durch Bauen, Preiswahl, Tagesstart und die erste Hafenbitte. Der
Wasserlauf von Amorgos zeigt anschließend Nilis neue Touren. Das Menü erklärt
bei jeder Aufgabe die Voraussetzungen und den Nutzen. Die Zeit bleibt bei
der Zielwahl stehen. Auf der Insel markieren ein Ring und das HUD das Ziel.
Nili nimmt die Fracht sichtbar auf, und der Fortschritt steht neben dem
Tourensymbol.

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
  Tageszeit, Hafenbitten und Sternrekorde. Neue Bauplätze ergänzen die
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

Die Kampagnenkarte ergänzt die Hafenbilder um Größe, Küstenform und die
beiden Parzellenreihen. Im Grundstücksmenü stehen der nächste sichtbare
Ausbau und seine Wirkung. Bei gezoomter Kamera folgt der Ausschnitt
der Auswahl am Rand.

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

## Prüfungen

- Formatierung, Clippy mit Warnungen als Fehler und elf Rust-Tests.
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
- APK-Signatur v2 und v3, 16-KiB-Ausrichtung beider Bibliotheken und
  SHA-256-Abgleich des GitHub-Downloads.
- Der Test auf einem physischen Android-Tablet steht noch aus.
