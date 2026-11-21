# Spielreview und Abnahme

## Einstieg als neuer Spieler

Naxos beginnt am freien Bauplatz. Die erste verfügbare Backstube erhält den
Controllerfokus. Das Tutorial führt durch Bau, Preiswahl, Tagesstart
und Nilis Lieferung. Für spätere Systeme gibt es Hinweise am passenden Hafen
und die bebilderte Spielhilfe im Haupt- und Pausenmenü.

## Veröffentlichung

Ein Paket enthält ARM64 und x86_64. Das Paket ist für Android 8 und neuer
mit Vulkan 1.1 vorgesehen. Modelle, Schriften, Bilder und Töne liegen im
Paket. Android lädt ohne Netzwerkberechtigung. Die Release-Dateien enthalten
die APK und ihre SHA-256-Prüfsumme. Die Herkunft der übernommenen und neuen
Assets ist dokumentiert. [Asset-Herkunft](../assets/QUELLEN.md)

## Spielentwicklung

Der Simulationstest spielt alle 48 Kapitel mit regulären Bau-, Preis-, Ausbau-
und Hafenaktionen und prüft die Freischaltung von Rom. Die Nachfrage in
mehreren Häfen wurde nach den Durchläufen angepasst. Wasseraufträge zählen
die tatsächlich bereitgestellte Menge und behalten ihren erreichten Höchstwert
beim Wetterwechsel. Laufende Besuche, Vorräte, Preise und bereits erfüllte
Hafenbitten gehen ins lokale Autosave.

Die Referenzstrategien ergeben bei Normaltempo etwa sechs Stunden geöffnete
Gasse für Griechenland und gut sieben für Rom. Die Planung kommt dazu.
Acht Stunden pro Reise sind ein Designziel für einen ersten Durchlauf.

## Oberfläche und Bedienung

Menüs haben eine sichtbare Vorauswahl, eine eigene Fokusumrandung und
Bedienhinweise. Beim Rücksprung aus der Spielhilfe oder den Einstellungen
landet der Fokus bei der vorherigen Aktion. Die Hauptanzeige zeigt die
Zufriedenheit während des Tages. Das Hafenbuch erklärt die vier Ursachen
unerfüllter Wünsche. Die Ausbauvorschau zeigt aktuelle und geplante Mengen
mit den heutigen Vorräten. Die drei Wettertage stehen zusammen im Tagesplan.

## Prüfungen

- Rust-Tests für beide Kampagnen, Vorrat und Besuchsregeln, Lieferketten und
  das Wiederherstellen eines Spielstands während des Tages.
- Gerenderte Stichproben im Querformat mit 1280 × 800 und 1024 × 576
  sowie normaler und großer Oberfläche.
- Android 15 mit Vulkan und einem virtuellen Xbox Wireless Controller.
  Im ersten Tag wurden Backstube und Brunnen gebaut, der Brotpreis
  geändert und Nilis Lieferung erledigt. Die über GitHub geladene APK
  übernahm den Spielstand bytegleich und setzte ihn bis zur Abrechnung
  und zum zweiten Morgen fort.
- APK-Signatur v2 und v3, 16-KiB-Ausrichtung beider nativer Bibliotheken
  und SHA-256-Abgleich des GitHub-Downloads geprüft.
- Der physische Android-Tablet-Test mit Bluetooth oder USB steht noch aus.
