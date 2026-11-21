# Nililane

Nililane ist ein 3D-Straßenmanagementspiel in Rust und Bevy. Mit dem Nilpferd Nili
reist du zuerst durch das antike Griechenland und danach durch das antike
Rom. Du planst die Gasse am Morgen, versorgst Nachbarn und Fährgäste und hilfst
tagsüber dort, wo sich die Lieferungen stauen.

**[Nililane 1.0.0 für Android herunterladen](https://raw.githubusercontent.com/bthginfo/Nililane/41b99f3b4f3aba070bc5998357d921c89215eba4/Nililane-1.0.0.apk)** · [SHA-256](https://raw.githubusercontent.com/bthginfo/Nililane/41b99f3b4f3aba070bc5998357d921c89215eba4/Nililane-1.0.0.apk.sha256) · [Veröffentlichung](https://github.com/bthginfo/Nililane/releases/tag/v1.0.0)

![Nili vor der Backstube und dem Brunnen in Naxos](docs/bilder/naxos.jpg)

![Ein römisches Viertel mit Aquädukt, Thermen und Fährschiffen im Sturm](docs/bilder/capua.jpg)

## Auf dem Tablet spielen

Die signierte APK benötigt Android 8 oder neuer, Vulkan 1.1 und ein Gerät
mit ARM64- oder x86_64-Prozessor. Öffne die APK auf dem Tablet und installiere
sie. Nililane läuft offline und öffnet sich im Querformat.

Kopple den Xbox-Controller über die Android-Bluetooth-Einstellungen oder
schließe ihn per USB an. Unter **Erste Reise** führt Nili dich durch den
ersten Bau, die Eröffnung der Gasse und eine Hafenbitte. Im Haupt- und Pausenmenü
gibt es eine bebilderte **Spielhilfe**. Unter **Einstellungen** kannst du
die Oberfläche vergrößern sowie Musik und Spielklänge separat schalten.

| Xbox-Controller | Aktion |
| --- | --- |
| Steuerkreuz oder linker Stick | Bauplatz oder Menüaktion wählen |
| A | Bestätigen, Grundstück öffnen oder Nili helfen lassen |
| B | Zurück, während des Spiels pausieren |
| X | Aufträge ansehen |
| Y | Tag eröffnen, während des Tages zwischen 1× und 2× wechseln |
| RB | Hafenbuch mit Nachfrage, Vorräten und Hafenbitte öffnen |
| Rechter Stick, LT und RT | Kamera schwenken und zoomen |
| LB | Kamera zurücksetzen |
| Start | Pausenmenü öffnen |

Touch und Maus wählen Bauplätze und Schaltflächen. Mit der Tastatur bedienen
Pfeiltasten oder WASD die Auswahl, Enter bestätigt, Escape geht zurück.
X, Y, P, Q und E entsprechen X, Y, Start, LB und RB.

## Die Reise

- **Griechenland** beginnt in Naxos und endet am Hafenrat von Korinth. In
  24 Kapiteln kommen Speicher, Ölpressen und Heiligtümer zu den ersten
  Werkstätten, Höfen und Tavernen hinzu.
- **Rom** schaltet sich nach Griechenland frei. In 24 weiteren Kapiteln
  verbinden Aquädukte, Thermen, Foren und Villen die Gassen von Ostia bis
  Rom. Eigene Bauten, Kleidung, Musik und Schiffe prägen die zweite Reise.
- Die 15 Gebäudetypen lassen sich bis Stufe 4 ausbauen. Für Läden stehen
  drei Preise zur Wahl. Die Ausbauvorschau zeigt, wie sich Wasser,
  Vorräte und Bewohnerzahlen unter dem Tagesplan verändern.
- Im Hafenbuch stehen Nachfrage, Vorräte, Zufriedenheit und die Vorhersage
  für drei Tage. Nordwind, Regen und Sturm beeinflussen die See, den
  Fährverkehr und die Wasserreserven. Mit Hafenbeschlüssen kannst du
  Wasser holen, zusätzliche Fährgäste einladen oder Nachbarn bevorzugen.
- Am Hafen warten drei Aufträge. Richttag und Hafenbitten bringen
  Zusatzsterne. Nach dem Abschluss kannst du bleiben, weiterreisen
  oder einen Hafen für mehr Sterne erneut besuchen.
- Bauentscheidungen, laufende Tage und Sternrekorde werden lokal gespeichert.
  Bei Normaltempo ist jede Reise inklusive Planung auf rund acht Stunden
  ausgelegt. Dein Bautempo, die Tempo-Taste und Wiederholungen verändern
  die tatsächliche Spielzeit.

## Aus dem Quellcode starten

Mit Rust Stable und auf Linux mit Vulkan, X11, `libudev-dev` und
`libasound2-dev`.

```sh
cargo run --locked
cargo test --locked
```

Zum Paketieren sind JDK 17 oder neuer, Python 3, SDK Platform 36, Build Tools
36.0.0 und NDK r28c erforderlich. Die SDK-Installation muss über
`ANDROID_HOME` oder `ANDROID_SDK_ROOT` erreichbar sein.

```sh
rustup target add aarch64-linux-android x86_64-linux-android
tools/build-android.sh
```

Die APK liegt in `build/android/`. Der Standardbuild erzeugt eine lokal
signierte Test-APK. Für ein Paket mit eigenem Veröffentlichungszertifikat
verwende `NILILANE_KEYSTORE` und `NILILANE_KEY_PASSWORD_FILE` sowie bei
Bedarf `NILILANE_KEY_ALIAS`. Ein Update von `game.nililane` braucht
denselben Signaturschlüssel.

[Asset-Herkunft](assets/QUELLEN.md) · [Tests und Spielreview](docs/Abnahme.md)
