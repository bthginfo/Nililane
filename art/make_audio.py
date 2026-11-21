"""Synthesize short game cues and a seamless Aegean shore loop."""

from array import array
from math import exp, pi, sin, sqrt
from pathlib import Path
import random
import wave

RATE = 22050
ROOT = Path(__file__).resolve().parents[1] / "assets/audio"
ROOT.mkdir(parents=True, exist_ok=True)


def render(name, duration, synth):
    data = array("h", (round(max(-0.98, min(0.98, synth(i / RATE))) * 32767)
                       for i in range(int(duration * RATE))))
    with wave.open(str(ROOT / (name + ".wav")), "wb") as f:
        f.setnchannels(1)
        f.setsampwidth(2)
        f.setframerate(RATE)
        f.writeframes(data.tobytes())


def struck(t, start, note, decay, volume):
    t -= start
    if t < 0:
        return 0
    attack = min(1, t * 250)
    return attack * volume * exp(-t * decay) * (
        sin(2 * pi * note * t) * 0.72 + sin(2 * pi * note * 2.19 * t) * 0.23
    )


def motif(name, length, notes):
    render(name, length, lambda t: sum(struck(t, *n) for n in notes))


motif("build_place", 0.49, [(0, 179, 14, 0.26), (0.05, 258, 15, 0.18),
    (0.11, 391, 12, 0.12), (0.17, 523, 8, 0.13)])

def assembly():
    """Four short wood and mallet strikes synchronized with the scaffold."""
    taps = [(0.016, 0.80, 139), (0.167, 0.65, 190), (0.423, 0.72, 156),
            (0.511, 0.40, 208)]
    length = 0.78
    samples = array("f", [0.0]) * int(length * RATE)
    rng = random.Random(205)
    for start, gain, frequency in taps:
        offset = int(start * RATE)
        low = 0.0
        for j in range(int(0.18 * RATE)):
            t = j / RATE
            low = 0.66 * low + 0.34 * rng.uniform(-1.0, 1.0)
            w = 2 * pi * frequency * t
            body = sin(w + 2 * pi * 38 * t * exp(-t * 180)) * exp(-t * 34)
            wood = (sin(w * 2.51) * 0.24 + sin(w * 4.1) * 0.11) * exp(-t * 79)
            click = low * exp(-130 * t) * 0.33
            samples[offset + j] += gain * (body * 0.46 + wood + click) * min(1.0, t * 2200)
    render("tier_assembly", length, lambda t: samples[round(t * RATE)])


assembly()

motif("market_sale", 0.33, [(0, 787, 20, 0.125), (0.022, 1175, 15, 0.102)])
motif("ferry_bell", 0.87, [(0, 392, 5, 0.168), (0.08, 523, 5, 0.15),
    (0.20, 587, 6, 0.11)])
motif("nili_aid", 0.90, [(0, 523, 8, 0.17), (0.082, 660, 8, 0.18),
    (0.17, 784, 7, 0.19), (0.28, 1046, 7, 0.14)])


rng = random.Random(1701)
dust = [(rng.uniform(0.1, 1.0), rng.randrange(65, 5600) / 15, rng.random() * 2 * pi)
        for _ in range(56)]
murmur = [(rng.uniform(0.02, 0.10), rng.randrange(20, 150) / 15, rng.random() * 2 * pi)
          for _ in range(12)]


def shore(t):
    surge = (0.46 + 0.40 * sin(2 * pi * t / 5 - 1.3) + 0.13 * sin(2 * pi * t / 3)) ** 2
    hiss = sum(amplitude * sin(2 * pi * freq * t + phase) / sqrt(freq)
               for amplitude, freq, phase in dust)
    ripple = sum(amplitude * sin(2 * pi * freq * t + phase) for amplitude, freq, phase in murmur)
    return hiss * (0.07 + 0.16 * surge) + ripple * (0.06 + 0.08 * surge)


render("shore_loop", 15, shore)

# Lyre and reed theme for the Roman campaign, with a frame drum in later phrases.
# Integer beat placement keeps the turn back to the first bar seamless.
def roman_theme():
    import subprocess

    beat = 0.71
    length = 96 * beat
    samples = int(length * RATE)
    music = array("f", [0]) * samples
    rng = random.Random(41)
    chords = [(50, 57, 62, 65), (48, 55, 60, 64), (46, 53, 58, 62), (45, 52, 57, 61),
              (50, 57, 62, 65), (53, 60, 65, 69), (48, 55, 60, 67), (45, 52, 57, 65)]
    motif_notes = [(74, 69, 72), (67, 69, 64), (65, 69, 70), (69, 65, 64),
                   (62, 65, 69), (72, 69, 67), (64, 67, 71), (69, 64, 61)]

    def put(where, number, volume, duration, instrument):
        f = 440 * 2 ** ((number - 69) / 12)
        start = int(where * beat * RATE)
        for j in range(int(duration * RATE)):
            t = j / RATE
            w = 2 * pi * f * t
            if instrument == "pluck":
                note = (sin(w) + 0.29 * sin(w * 2.004) + 0.12 * sin(w * 3.01)) * exp(-t * 4.4)
                envelope = min(1.0, t * 165) * min(1.0, (duration - t) * 9)
            elif instrument == "reed":
                vibrato = 0.47 * sin(2 * pi * 5.0 * t) * min(1.0, t * 2)
                note = (sin(w + vibrato) + 0.14 * sin(w * 2.0 + vibrato)) * exp(-t * 0.72)
                envelope = min(1.0, t * 13) * min(1.0, (duration - t) * 5.5)
            else:
                note = (sin(w / (1 + t * 1.1)) * 0.68 + (rng.random() - 0.5) * 0.24) * exp(-t * 21)
                envelope = min(1.0, t * 200) * min(1.0, (duration - t) * 23)
            music[(start + j) % samples] += note * envelope * volume

    for bar in range(24):
        chord = chords[bar % len(chords)]
        for i, index in enumerate((0, 2, 1, 3, 1, 2)):
            put(bar * 4 + i * 0.65, chord[index], 0.104 - (i % 2) * 0.027, 1.38, "pluck")
        for i, n in enumerate(motif_notes[bar % len(motif_notes)]):
            if bar not in (0, 7, 14, 23) and not (i == 1 and bar % 4 == 0):
                put(bar * 4 + i * 1.0 + 0.39, n - (12 if bar < 6 else 0),
                    0.066 if i == 2 else 0.086, 1.15, "reed")
        if 4 <= bar <= 21:
            put(bar * 4, 40, 0.13, 0.30, "drum")
            put(bar * 4 + 2.35, 47, 0.084, 0.24, "drum")
    render("roman_quay", length, lambda t: music[min(samples - 1, int(t * RATE))])
    subprocess.run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-y",
                    "-i", str(ROOT / "roman_quay.wav"), "-c:a", "libvorbis", "-q:a", "5",
                    str(ROOT / "roman_quay.ogg")], check=True)
    (ROOT / "roman_quay.wav").unlink()


roman_theme()

# A cyclic noise bed, with a separate slower roll under the drops.
def rain_loop():
    import subprocess

    rng = random.Random(733)
    length = 9.0
    samples = int(length * RATE)
    raw = array("f", (rng.uniform(-1, 1) for _ in range(samples)))
    hiss = 0.0
    body = 0.0
    rain = array("f")
    for j in range(samples * 2):
        x = raw[j % samples]
        hiss += (x - hiss) * 0.24
        body += (x - body) * 0.026
        if j >= samples:
            t = (j - samples) / RATE
            pulse = 0.74 + 0.18 * sin(2 * pi * t / 3.0) + 0.08 * sin(2 * pi * t / length)
            roll = sin(2 * pi * t * 209 / length) * (0.5 + 0.5 * sin(2 * pi * t / length)) ** 3
            rain.append(pulse * (0.30 * hiss + 0.41 * body) + 0.014 * roll)
    render("harbor_rain", length, lambda t: rain[min(samples - 1, int(t * RATE))])
    subprocess.run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-y",
                    "-i", str(ROOT / "harbor_rain.wav"), "-c:a", "libvorbis", "-q:a", "4",
                    str(ROOT / "harbor_rain.ogg")], check=True)
    (ROOT / "harbor_rain.wav").unlink()


rain_loop()
