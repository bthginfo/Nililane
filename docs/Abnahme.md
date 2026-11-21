# Spielreview und Abnahme · Nililane 2.0

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
der Auswahl am Rand. Die Zufriedenheit bleibt in der Hauptanzeige.
Das Hafenbuch zeigt bei geöffnetem Hafen zwei aktuelle Gästestimmen
zusammen mit den häufigsten Gründen für unerfüllte Wünsche.

## Prüfungen

- Formatierung, Clippy mit Warnungen als Fehler und neun Rust-Tests.
- Gerenderte Stichproben in Griechenland und Rom im Querformat.
- Android 15 mit Vulkan und simulierten Gamepad-Eingaben. Nach der
  Installation über 1.0.0 haben A und Y den vorhandenen Tag samt Bauplätzen,
  Brotpreis und Verkaufszahlen fortgesetzt. In Amorgos haben R3,
  Steuerkreuz und A einen Wasserlauf ausgewählt und zur Backstube
  geschickt. Der Betrieb erhielt die Lieferung einmal.
- APK-Signatur v2 und v3, 16-KiB-Ausrichtung beider Bibliotheken und
  SHA-256-Abgleich des GitHub-Downloads.
- Der Test auf einem physischen Android-Tablet steht noch aus.
