"""Synthesize short street and animal sounds."""

from pathlib import Path
import wave

import numpy as np

OUT = Path(__file__).resolve().parents[1] / "assets/audio"
RATE = 24000
RNG = np.random.default_rng(2215)


def tone(t, hz):
    return np.sin(2 * np.pi * hz * t)


def time(seconds):
    return np.arange(int(seconds * RATE)) / RATE


def save(name, data):
    samples = np.int16(np.clip(data, -0.98, 0.98) * 32767)
    with wave.open(str(OUT / f"{name}.wav"), "wb") as sound:
        sound.setnchannels(1)
        sound.setsampwidth(2)
        sound.setframerate(RATE)
        sound.writeframes(samples.tobytes())


def tap(data, at, duration, hz, gain):
    t = time(duration)
    chord = tone(t, hz) + 0.22 * tone(t, hz * 2.01) + 0.12 * tone(t, hz * 3.0)
    env = np.minimum(1, t / 0.012) * np.exp(-7 * t / duration)
    at = int(at * RATE)
    data[at:at + len(t)] += chord * env * gain


t = time(0.48)
noise = RNG.normal(0, 0.48, len(t))
noise -= np.roll(noise, 1) * 0.54
env = np.minimum(t * 30, 1) * np.exp(-t * 10) * (0.38 + 0.62 * tone(t, 18) ** 2)
save("street_sweep", noise * env * 0.44)

t = time(0.78)
cat = tone(t, 86) * (0.37 + tone(t, 21) * 0.24) + tone(t, 182) * 0.12
save("street_purr", cat * np.minimum(t * 16, 1) * np.clip((0.78 - t) * 5, 0, 1) * 0.56)

t = time(0.58)
dog = np.zeros(len(t))
for at in [0.035, 0.24]:
    y = time(0.23)
    wavelet = np.sin(2 * np.pi * (351 * y - y ** 2 * 554))
    wavelet += 0.36 * RNG.normal(0, 1, len(y))
    start = int(at * RATE)
    dog[start:start + len(y)] += wavelet * np.minimum(y * 58, 1) * np.exp(-y * 18) * 0.27
save("street_woof", dog)

bell = np.zeros(len(time(0.74)))
tap(bell, 0.015, 0.54, 469, 0.39)
tap(bell, 0.17, 0.52, 347, 0.29)
save("street_bell", bell)
