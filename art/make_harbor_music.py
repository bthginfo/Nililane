"""Compose the 48 loopable harbor themes and their evening arrangements. Requires NumPy and FFmpeg with libvorbis."""

from dataclasses import dataclass
from pathlib import Path
from random import Random
from subprocess import run
import sys

import numpy as np


RATE = 24000
OUT = Path(__file__).resolve().parents[1] / "assets/audio/harbors"


@dataclass(frozen=True)
class Score:
    harbor: str
    bpm: int
    root: int
    mode: str
    meter: int
    motif: tuple[int, ...]


SCORES = [
    Score("Naxos", 92, 50, "ionian", 4, (0, 2, 4, 3, 2, 1, 0)),
    Score("Amorgos", 78, 45, "dorian", 3, (4, 2, 1, 3, 0, 2, -1)),
    Score("Delos", 89, 48, "mixolydian", 4, (0, 3, 2, 5, 4, 1, 2)),
    Score("Rhodos", 101, 52, "ionian", 4, (2, 4, 6, 5, 3, 4, 1)),
    Score("Samos", 85, 47, "dorian", 3, (3, 5, 6, 4, 2, 0, 1)),
    Score("Kos", 96, 50, "mixolydian", 4, (1, 3, 2, 4, 6, 5, 4)),
    Score("Paros", 83, 53, "ionian", 3, (0, 4, 6, 2, 5, 2, 1)),
    Score("Kythnos", 93, 48, "dorian", 4, (5, 3, 4, 2, 0, 2, 3)),
    Score("Thera", 76, 49, "aeolian", 4, (0, 3, 5, 4, 6, 2, 1)),
    Score("Milos", 94, 51, "mixolydian", 3, (2, 6, 4, 1, 3, 5, 2)),
    Score("Ikaria", 88, 46, "dorian", 3, (4, 6, 3, 5, 1, 2, 0)),
    Score("Kallisti", 80, 52, "aeolian", 4, (3, 1, 0, 4, 6, 5, 2)),
    Score("Andros", 102, 47, "mixolydian", 4, (1, 4, 5, 6, 3, 2, 1)),
    Score("Aigina", 86, 51, "ionian", 3, (4, 5, 2, 3, 6, 4, 0)),
    Score("Chios", 91, 48, "dorian", 4, (2, 0, 3, 6, 4, 5, 1)),
    Score("Epidauros", 74, 54, "aeolian", 3, (5, 3, 1, 2, 0, 4, 2)),
    Score("Skopelos", 99, 50, "ionian", 4, (0, 5, 3, 4, 1, 3, 2)),
    Score("Syros", 87, 45, "mixolydian", 4, (6, 4, 2, 3, 1, 5, 4)),
    Score("Tinos", 79, 49, "dorian", 3, (3, 0, 2, 5, 4, 6, 2)),
    Score("Serifos", 95, 53, "ionian", 4, (0, 1, 4, 2, 6, 5, 3)),
    Score("Kea", 90, 47, "aeolian", 4, (3, 6, 5, 3, 1, 0, 4)),
    Score("Kythira", 77, 50, "mixolydian", 3, (2, 3, 6, 4, 5, 1, 0)),
    Score("Zakynthos", 97, 48, "dorian", 4, (0, 2, 1, 4, 3, 6, 5)),
    Score("Korinth", 82, 51, "ionian", 4, (4, 6, 5, 2, 3, 1, 0)),
    Score("Ostia", 88, 45, "dorian", 4, (0, 3, 5, 6, 2, 4, 1)),
    Score("Portus", 98, 48, "mixolydian", 4, (3, 5, 2, 1, 6, 4, 1)),
    Score("Aricia", 76, 49, "ionian", 3, (4, 1, 3, 0, 5, 3, 2)),
    Score("Tibur", 84, 46, "aeolian", 3, (5, 4, 2, 6, 3, 1, 0)),
    Score("Praeneste", 95, 52, "ionian", 4, (0, 4, 5, 2, 3, 6, 4)),
    Score("Veii", 78, 47, "dorian", 4, (6, 5, 3, 2, 4, 1, 2)),
    Score("Tusculum", 90, 50, "mixolydian", 3, (2, 4, 1, 0, 6, 3, 4)),
    Score("Lanuvium", 87, 45, "ionian", 4, (1, 5, 6, 3, 2, 4, 0)),
    Score("Alba Longa", 73, 48, "aeolian", 3, (3, 6, 4, 2, 0, 5, 3)),
    Score("Fidenae", 96, 49, "dorian", 4, (0, 2, 5, 3, 6, 1, 2)),
    Score("Gabii", 91, 53, "mixolydian", 4, (6, 3, 4, 5, 1, 2, 0)),
    Score("Aquae Albulae", 75, 47, "ionian", 3, (0, 5, 4, 6, 2, 3, 1)),
    Score("Lavinium", 80, 46, "aeolian", 4, (4, 2, 0, 1, 5, 3, 6)),
    Score("Formiae", 99, 49, "mixolydian", 4, (2, 6, 3, 0, 4, 1, 3)),
    Score("Anxur", 87, 51, "dorian", 3, (5, 1, 4, 6, 3, 2, 0)),
    Score("Cumae", 77, 48, "aeolian", 4, (6, 4, 5, 3, 0, 2, 1)),
    Score("Neapolis", 103, 50, "ionian", 4, (3, 2, 4, 6, 5, 0, 4)),
    Score("Puteoli", 94, 45, "mixolydian", 4, (0, 3, 1, 5, 6, 4, 2)),
    Score("Capua", 85, 47, "dorian", 3, (4, 0, 6, 3, 2, 5, 1)),
    Score("Pompeii", 90, 49, "aeolian", 4, (2, 5, 1, 4, 6, 3, 0)),
    Score("Anio", 79, 52, "ionian", 3, (1, 0, 3, 6, 2, 4, 5)),
    Score("Trastevere", 97, 48, "mixolydian", 4, (5, 2, 6, 4, 3, 1, 0)),
    Score("Palatin", 83, 50, "dorian", 3, (3, 4, 6, 1, 0, 2, 5)),
    Score("Rom", 82, 45, "ionian", 4, (0, 4, 6, 5, 2, 3, 0)),
]

SCALES = {
    "ionian": (0, 2, 4, 5, 7, 9, 11),
    "dorian": (0, 2, 3, 5, 7, 9, 10),
    "mixolydian": (0, 2, 4, 5, 7, 9, 10),
    "aeolian": (0, 2, 3, 5, 7, 8, 10),
}

# Eight-bar phrases land on the tonic before switching orchestration.
PROGRESSIONS = [
    (0, 0, 3, 3, 1, 4, 3, 0),
    (0, 2, 5, 3, 0, 1, 4, 0),
    (0, 6, 3, 0, 5, 3, 4, 0),
    (0, 3, 1, 4, 5, 2, 4, 0),
    (0, 5, 3, 2, 1, 3, 4, 0),
    (0, 4, 5, 2, 3, 1, 4, 0),
]


def compose(index: int, score: Score, evening: bool = False) -> np.ndarray:
    roman = index >= 24
    beat = (67 if evening else 60) / score.bpm
    bars = 16 if evening else 28
    length = round(bars * score.meter * beat * RATE)
    mix = np.zeros((length, 2), dtype=np.float32)
    scale = SCALES[score.mode]
    family = (index + 2 * (index // 8)) % len(PROGRESSIONS)
    rng = Random(9087 + index)
    noise = np.random.default_rng(1059 + index)
    progression = PROGRESSIONS[family]

    def pitch(step: int, octave: int = 0) -> int:
        return score.root + scale[step % 7] + 12 * (step // 7 + octave)

    def note(at, key, duration, gain, pan, instrument):
        n = round(duration * RATE)
        t = np.arange(n, dtype=np.float64) / RATE
        frequency = 440 * 2 ** ((key - 69) / 12)
        phase = 2 * np.pi * frequency * t
        if instrument == "string":
            hardness = 0.23 if roman else 0.41
            body = (
                np.sin(phase) + 0.48 * np.sin(phase * 2.001)
                + hardness * np.sin(phase * 3.005) * np.exp(-t * 4.3)
                + 0.10 * np.sin(phase * 5.007) * np.exp(-t * 13)
                + 0.09 * np.sin(phase * 0.998) * np.exp(-t * 1.3)
            ) * np.exp(-t * (3.3 if roman else 4.5))
            attack, release = 0.025, 0.15
        elif instrument == "wind":
            wobble = np.sin(2 * np.pi * (4.3 + family * 0.14) * t) * np.minimum(t * 1.8, 0.29)
            overtone = 0.29 if roman else 0.16 + family * 0.015
            body = np.sin(phase + wobble) + overtone * np.sin(2 * phase + wobble)
            body += 0.08 * np.sin(3 * phase + wobble) * np.exp(-t * 2)
            body += 0.04 * np.sin(phase * 1.006)
            attack, release = 0.106, 0.21
        elif instrument == "drum":
            grain = noise.uniform(-1, 1, n)
            soft = np.convolve(grain, np.ones(9) / 9, mode="same")
            body = (np.sin(phase / (1 + t * 3.2)) * 0.78 + soft * 0.43) * np.exp(-t * 24)
            attack, release = 0.011, 0.08
        else:
            body = (
                np.sin(phase) + 0.13 * np.sin(phase * 2.76) * np.exp(-t * 5)
            ) * np.exp(-t * 6.9)
            attack, release = 0.015, 0.105
        envelope = np.minimum(t / attack, 1) ** 1.3
        envelope *= np.clip((duration - t) / release, 0, 1) ** 1.6
        voice = (body * envelope * gain).astype(np.float32)
        start = round(at * beat * RATE) % length
        left = np.sqrt(0.5 * (1 - pan))
        right = np.sqrt(0.5 * (1 + pan))
        stereo = voice[:, np.newaxis] * [left, right]
        first = min(n, length - start)
        mix[start:start + first] += stereo[:first]
        if first < n:
            mix[:n - first] += stereo[first:]

    meter = score.meter
    patterns = [(0, 0.75, 1.5, 2.35), (0, 0.58, 1.24, 2.12, 2.57),
                (0, 0.9, 1.78, 2.68), (0, 0.52, 1.43, 2.22)]
    melodic = [(0.18, 1.1, 2.33), (0.67, 1.31, 2.12), (0.14, 1.55, 2.43),
               (0.42, 1.6, 2.56), (0.26, 1.28, 2.02), (0.55, 1.1, 2.45)]
    for bar in range(bars):
        section = bar // 8
        phrase = bar % 8
        home = progression[phrase] if bar < 24 else (3, 1, 4, 0)[bar - 24]
        if section == 1 and phrase in (1, 2, 4):
            home = (home + (2 if phrase == 2 else 3)) % 7
        start = bar * meter
        voicing = (0, 4, 2, 7, 2) if family % 2 == 0 else (0, 4, 7, 2, 4)
        timing = patterns[(family + int(section == 1)) % len(patterns)]
        for j, onset in enumerate(timing):
            if (evening and j > 2) or (bar >= 26 and j > 1):
                break
            key = pitch(home + voicing[j], int(j > 0))
            note(start + onset * meter / 3, key, beat * (1.65 + 0.25 * (j % 2)),
                 rng.uniform(0.075, 0.108) * (1 if j == 0 else 0.84),
                 -0.31 if j % 2 else 0.13, "string")
        if phrase not in (3, 7) and bar < 27:
            melody = score.motif[phrase % len(score.motif)]
            response = score.motif[(phrase + 1 + family % 3) % len(score.motif)]
            degrees = (melody, response, 2 * melody - response)
            if section == 1:
                degrees = (response, melody + 2, score.motif[(phrase + 3) % len(score.motif)])
            for j, onset in enumerate(melodic[family]):
                if j == 2 and (phrase + index) % 3 == 0:
                    continue
                color = "bell" if evening and j != 1 else (
                    "wind" if (section + family) % 3 != 0 or j == 1 else "bell"
                )
                step = home + degrees[j]
                if step > 9:
                    step -= 7
                key = pitch(step, 1 if evening else (2 if not roman or family % 2 else 1))
                volume = 0.098 if color == "wind" else 0.063
                note(start + onset * meter / 3, key, beat * (0.80 + (j == 2) * 0.49),
                     volume * rng.uniform(0.82, 1.05), 0.34 if j != 1 else 0.19, color)
        if not evening and 4 <= bar < 25 and (roman or family % 2 == 1):
            note(start, 38 + family % 3, 0.29, 0.090, -0.13, "drum")
            if (bar + family) % 3 != 2:
                note(start + meter * 0.62, 46, 0.21, 0.047, 0.21, "drum")
        if bar in (7, 15, 23, 27):
            note(start + meter * 0.35, pitch(0 if bar != 15 else 4, 2),
                 beat * 1.8, 0.071, 0.19, "string")

    # Circular room reflections preserve the tails across the repeat point.
    dry = mix.copy()
    for delay, gain in [(0.071, 0.145), (0.127, 0.102), (0.204, 0.072), (0.31, 0.038)]:
        mix += np.roll(dry[:, ::-1], round(delay * RATE), axis=0) * gain
    rms = np.sqrt(np.mean(mix ** 2))
    peak = np.max(np.abs(mix))
    return mix * min(0.091 / rms, 0.78 / peak)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    args = sys.argv[1:]
    evening = "--evening" in args
    indices = list(map(int, (value for value in args if value != "--evening"))) or range(len(SCORES))
    for index in indices:
        score = SCORES[index]
        pcm = compose(index, score, evening)
        suffix = "-abend" if evening else ""
        run([
            "ffmpeg", "-hide_banner", "-loglevel", "error", "-y",
            "-f", "f32le", "-ar", str(RATE), "-ac", "2", "-i", "pipe:0",
            "-c:a", "libvorbis", "-q:a", "3.2", "-metadata", f"title={score.harbor}",
            "-metadata", "album=Nililane · Hafenreisen", str(OUT / f"{index:02}{suffix}.ogg"),
        ], input=pcm.astype("<f4").tobytes(), check=True)
        print(f"{index:02} {score.harbor:14} {len(pcm) / RATE:5.1f} s", flush=True)


if __name__ == "__main__":
    main()
