"""Synthesize the soft interface cues: focus tick, report stars and goal check.

The cues follow make_audio.py: mono 22.05 kHz, struck partials with a short
attack, quiet enough to sit under the harbor music. Run from the repository
root with `python3 art/make_ui_feedback.py`.
"""

from pathlib import Path
import wave

import numpy as np

RATE = 22050
ROOT = Path(__file__).resolve().parents[1] / "assets/audio"


def write(name, samples):
    peak = np.max(np.abs(samples))
    if peak > 0.98:
        samples = samples * (0.98 / peak)
    # Short fades keep the start and end free of clicks.
    fade = min(len(samples) // 4, int(0.004 * RATE))
    samples[:fade] *= np.linspace(0.0, 1.0, fade)
    samples[-fade:] *= np.linspace(1.0, 0.0, fade)
    data = np.round(samples * 32767).astype("<i2")
    with wave.open(str(ROOT / f"{name}.wav"), "wb") as f:
        f.setnchannels(1)
        f.setsampwidth(2)
        f.setframerate(RATE)
        f.writeframes(data.tobytes())


def timeline(duration):
    return np.arange(int(duration * RATE)) / RATE


def struck(t, start, note, decay, volume, bright=0.23):
    local = t - start
    on = local >= 0
    local = np.where(on, local, 0.0)
    attack = np.minimum(1.0, local * 260)
    tone = np.sin(2 * np.pi * note * local) * 0.72 + np.sin(2 * np.pi * note * 2.19 * local) * bright
    return np.where(on, attack * volume * np.exp(-local * decay) * tone, 0.0)


def tick():
    """A dry wooden tick for moving the focus between buttons."""
    t = timeline(0.055)
    rng = np.random.default_rng(311)
    noise = rng.uniform(-1, 1, t.size)
    smooth = np.convolve(noise, np.ones(5) / 5, mode="same")
    body = np.sin(2 * np.pi * 1530 * t) * np.exp(-t * 120) * 0.55
    knock = np.sin(2 * np.pi * 690 * t) * np.exp(-t * 90) * 0.32
    click = smooth * np.exp(-t * 420) * 0.35
    write("ui_tick", (body + knock + click) * 0.42)


def star():
    """A glassy chime. The game raises its pitch for each earned star."""
    t = timeline(0.95)
    shimmer = 1 + 0.003 * np.sin(2 * np.pi * 5.2 * t)
    tone = (
        struck(t * shimmer, 0.0, 1046.5, 5.2, 0.30, 0.18)
        + struck(t, 0.0, 1568.0, 7.5, 0.10, 0.10)
        + struck(t, 0.012, 2093.0, 9.0, 0.06, 0.05)
    )
    sparkle = struck(t, 0.045, 3136.0, 16.0, 0.035, 0.0)
    write("ui_star", tone + sparkle)


def goal():
    """Two rising plucks and a soft fifth when a harbor goal is complete."""
    t = timeline(0.78)
    tone = (
        struck(t, 0.0, 659.3, 9.0, 0.20)
        + struck(t, 0.085, 880.0, 7.5, 0.21)
        + struck(t, 0.17, 1318.5, 6.0, 0.13, 0.12)
        + struck(t, 0.17, 987.8, 6.5, 0.08, 0.10)
    )
    write("ui_goal", tone)


tick()
star()
goal()
