"""Generate short, dry gathering sounds for the expedition inventory."""
from pathlib import Path
import wave

import numpy as np

RATE = 22_050
OUT = Path(__file__).resolve().parents[1] / "assets/audio/abenteuer"


def effect(name, notes, hits, seed):
    rng = np.random.default_rng(seed)
    t = np.arange(round(RATE * 0.44)) / RATE
    noise = rng.uniform(-1, 1, len(t))
    rustle = np.convolve(noise, np.ones(13) / 13, "same")
    mix = np.zeros_like(t)
    for start, level, decay in hits:
        age = np.maximum(0, t - start)
        envelope = (1 - np.exp(-age * 960)) * np.exp(-age * decay)
        ring = sum(np.sin(2 * np.pi * pitch * age) * strength for pitch, strength in notes)
        mix += (ring * 0.62 + rustle * 0.49 + noise * 0.19) * level * envelope
    mix = np.tanh(mix * 1.6) * 0.77
    with wave.open(str(OUT / f"{name}.wav"), "wb") as wav:
        wav.setparams((1, 2, RATE, 0, "NONE", "not compressed"))
        wav.writeframes((mix * 32767).astype("<i2").tobytes())


if __name__ == "__main__":
    effect("wood", [(122, 0.24), (278, 0.19), (468, 0.13)], [(0.012, 0.8, 35), (0.12, 0.42, 42)], 39)
    effect("stone", [(264, 0.36), (531, 0.24), (851, 0.13)], [(0.010, 0.76, 43), (0.075, 0.41, 75)], 72)
    effect("forage", [(412, 0.09), (284, 0.10)], [(0.035, 0.58, 31), (0.12, 0.67, 21)], 12)
