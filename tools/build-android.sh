#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

sdk=${ANDROID_SDK_ROOT:-${ANDROID_HOME:-}}
if [[ -z "$sdk" ]]; then
    printf 'ANDROID_HOME muss auf das Android-SDK zeigen.\n' >&2
    exit 1
fi
ndk=${ANDROID_NDK_HOME:-"$sdk/ndk/28.2.13676358"}
case "$(uname -s)" in
    Linux) host=linux-x86_64 ;;
    Darwin) host=darwin-x86_64 ;;
    *) printf 'Die Android-Paketierung läuft unter Linux oder macOS.\n' >&2; exit 1 ;;
esac
llvm="$ndk/toolchains/llvm/prebuilt/$host/bin"
tools="$sdk/build-tools/36.0.0"
platform="$sdk/platforms/android-36/android.jar"
out="$PWD/build/android"
manifest=packaging/AndroidManifest.xml
variant=${NILILANE_VARIANT:-tablet}
base="$out"
read -r -a architectures <<< "${NILILANE_ABIS:-arm64-v8a x86_64}"
case "$variant" in
    tablet)
        version=$(sed -n 's/.*android:versionName="\([^"]*\)".*/\1/p' "$manifest")
        apk="$base/Nililane-$version.apk"
        ;;
    phone)
        version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml)
        out="$base/phone"
        mkdir -p "$out"
        sed -e 's/package="game.nililane"/package="game.nililane.phone"/' \
            -e 's/android:versionCode="[0-9]*"/android:versionCode="11"/' \
            -e "s/android:versionName=\"[^\"]*\"/android:versionName=\"$version\"/" \
            -e 's/android:label="Nililane"/android:label="Nililane Handy"/' \
            -e 's/android:name=".GameActivity"/android:name="game.nililane.GameActivity"/' \
            "$manifest" > "$out/AndroidManifest.xml"
        manifest="$out/AndroidManifest.xml"
        apk="$base/Nililane-$version-Handy.apk"
        ;;
    *) printf 'Unbekannte Android-Variante %s\n' "$variant" >&2; exit 1 ;;
esac
mkdir -p "$out/classes" "$out/native"
if [[ ! -x "$tools/aapt2" || ! -f "$platform" || ! -x "$llvm/llvm-strip" ]]; then
    printf 'SDK Platform 36, Build Tools 36.0.0 und NDK r28c werden benötigt.\n' >&2
    exit 1
fi

for abi in "${architectures[@]}"; do
    case "$abi" in
        arm64-v8a) target=aarch64-linux-android ;;
        x86_64) target=x86_64-linux-android ;;
        *) printf 'Unbekannte Architektur %s\n' "$abi" >&2; exit 1 ;;
    esac
    env_name=${target//-/_}
    linker="$llvm/${target}26-clang"
    env "CARGO_TARGET_${env_name^^}_LINKER=$linker" "CC_${env_name}=$linker" \
        "CXX_${env_name}=${linker}++" "AR_${env_name}=$llvm/llvm-ar" \
        cargo build --release --locked --no-default-features --lib --target "$target"
    mkdir -p "$out/native/$abi"
    "$llvm/llvm-strip" --strip-unneeded -o "$out/native/$abi/libnililane.so" \
        "${CARGO_TARGET_DIR:-target}/$target/release/libnililane.so"
done

"$tools/aapt2" compile --dir packaging/res -o "$out/resources.zip"
"$tools/aapt2" link --manifest "$manifest" -I "$platform" -o "$out/linked.apk" "$out/resources.zip"
java -m jdk.compiler/com.sun.tools.javac.Main -source 8 -target 8 -Xlint:-options -bootclasspath "$platform" \
    -d "$out/classes" packaging/java/game/nililane/GameActivity.java
"$tools/d8" --min-api 26 --lib "$platform" --output "$out" "$out/classes/game/nililane/GameActivity.class"

python3 - "$out" "${architectures[@]}" <<'PYRSA'
import sys
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZIP_STORED, ZipFile

root = Path(sys.argv[1])
with ZipFile(root / "linked.apk", "a") as apk:
    apk.write(root / "classes.dex", "classes.dex", compress_type=ZIP_DEFLATED)
    for abi in sys.argv[2:]:
        apk.write(root / "native" / abi / "libnililane.so", f"lib/{abi}/libnililane.so",
                  compress_type=ZIP_DEFLATED, compresslevel=6)
    for album in ("harbors", "abenteuer"):
        for track in sorted(Path("assets/audio", album).glob("*.ogg")):
            apk.write(track, f"assets/audio/{album}/{track.name}", compress_type=ZIP_STORED)
PYRSA

if [[ -n "${NILILANE_KEYSTORE:-}" ]]; then
    key="$NILILANE_KEYSTORE"
    alias=${NILILANE_KEY_ALIAS:-nililane}
    password="file:${NILILANE_KEY_PASSWORD_FILE:?Dateipfad zum Keystore-Passwort fehlt}"
else
    key="$out/nililane-test.jks"
    alias=nililane-test
    password=pass:android
    if [[ ! -f "$key" ]]; then
        keytool -genkeypair -keystore "$key" -storetype PKCS12 -alias "$alias" \
            -storepass android -keypass android -keyalg RSA -keysize 2048 -validity 10000 \
            -dname 'CN=Nililane Test, O=Nililane'
    fi
fi
"$tools/zipalign" -f -P 16 4 "$out/linked.apk" "$out/aligned.apk"
"$tools/apksigner" sign --ks "$key" --ks-key-alias "$alias" --ks-pass "$password" \
    --v4-signing-enabled false --out "$apk" "$out/aligned.apk"
"$tools/apksigner" verify --verbose "$apk"
"$tools/zipalign" -c -P 16 4 "$apk"
printf 'APK %s\n' "$apk"
