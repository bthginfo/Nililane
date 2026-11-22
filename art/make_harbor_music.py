"""Render Nililane's written chamber score as loopable Vorbis recordings.

Requires NumPy, pyfluidsynth, libfluidsynth, FFmpeg, and GeneralUser GS 2.0.3.
Pass the path to the soundfont with --soundfont. The game ships the mixdowns.
"""

import argparse
from concurrent.futures import ProcessPoolExecutor
from dataclasses import dataclass
from pathlib import Path
import re
from subprocess import run

import fluidsynth
import numpy as np

from harbor_scores import SCORES, Song


RATE = 32_000
OUT = Path(__file__).resolve().parents[1] / "assets/audio/harbors"
MODES = {
    "major": (0, 2, 4, 5, 7, 9, 11),
    "minor": (0, 2, 3, 5, 7, 8, 10),
    "dorian": (0, 2, 3, 5, 7, 9, 10),
    "mixolydian": (0, 2, 4, 5, 7, 9, 10),
    "lydian": (0, 2, 4, 6, 7, 9, 11),
}
DEGREES = {symbol: degree for degree, symbol in enumerate("def123456789", start=-3)}


@dataclass(frozen=True)
class Voice:
    program: int
    velocity: int
    gate: float
    volume: int
    pan: int


# The programs are the zero-based General MIDI numbers.
NYLON = Voice(24, 82, 1.22, 110, 51)
HARP = Voice(46, 81, 1.22, 99, 76)
FLUTE = Voice(73, 75, 0.91, 104, 64)
CLARINET = Voice(71, 69, 0.93, 103, 69)
NYLON_BACK = Voice(24, 68, 1.38, 87, 47)
HARP_BACK = Voice(46, 60, 1.24, 84, 84)
BASS = Voice(32, 68, 0.88, 83, 64)
CELLO = Voice(42, 63, 0.87, 64, 59)
PIZZ = Voice(45, 60, 0.72, 70, 56)
BRUSHES = Voice(40, 58, 0.9, 67, 70)

KOTO = Voice(107, 79, 1.19, 106, 56)
SHAKUHACHI = Voice(77, 74, 0.94, 101, 68)
SHAMISEN = Voice(106, 67, 1.12, 88, 45)
KOTO_BACK = Voice(107, 61, 1.21, 85, 79)
FIDDLE = Voice(110, 72, 0.92, 100, 60)
RECORDER = Voice(74, 67, 0.97, 100, 69)
DULCIMER = Voice(15, 69, 1.25, 91, 59)
DULCIMER_BACK = Voice(15, 55, 1.29, 78, 80)

# Lead, answering voice, plucked accompaniment. Quiet voices also swap at dusk.
ENSEMBLES = {
    "lyre": (FLUTE, HARP, NYLON_BACK),
    "reed": (CLARINET, NYLON, HARP_BACK),
    "trail": (NYLON, FLUTE, HARP_BACK),
    "guitar": (FLUTE, NYLON, NYLON_BACK),
    "chamber": (CLARINET, HARP, NYLON_BACK),
    "pavilion": (CLARINET, HARP, NYLON_BACK),
    "courtyard": (NYLON, FLUTE, HARP_BACK),
    "koto": (KOTO, SHAKUHACHI, SHAMISEN),
    "bamboo": (SHAKUHACHI, KOTO, KOTO_BACK),
    "festival": (KOTO, SHAKUHACHI, KOTO_BACK),
    "fjord": (FIDDLE, RECORDER, DULCIMER_BACK),
    "hearth": (RECORDER, DULCIMER, HARP_BACK),
    "sail": (FIDDLE, DULCIMER, DULCIMER_BACK),
}
METERS = {
    "4/4": (8, (0, 2, 4, 6)),
    "3/4": (6, (0, 2, 4)),
    "6/8": (6, (0, 3)),
    "7/8": (7, (0, 3, 5)),
}


@dataclass(frozen=True)
class Measure:
    melody: str
    root: int


def phrases(score: Song):
    length, _ = METERS[score.meter]
    result = []
    for melody, chord_row in zip((score.call, score.answer, score.bridge), score.chords.split(), strict=True):
        bars = melody.split()
        if len(bars) != 4 or len(chord_row) != 4:
            raise ValueError(f"{score.harbor}: a phrase needs four bars")
        phrase = []
        for bar, chord in zip(bars, chord_row, strict=True):
            if len(bar) != length or not re.fullmatch(r"([def1-9][-.]*|\.)+", bar):
                raise ValueError(f"{score.harbor}: invalid bar {bar!r} for {score.meter}")
            phrase.append(Measure(bar, int(chord) - 1))
        result.append(phrase)
    return result


def midi_pitch(score: Song, degree: int):
    octave, interval = divmod(degree, 7)
    return score.tonic + octave * 12 + MODES[score.mode][interval]


def voicing(score: Song, root: int, former: tuple[int, ...]):
    """Keep inner parts near their preceding pitches when the harmony moves."""
    chord = [midi_pitch(score, root + interval) % 12 for interval in (0, 2, 4)]
    candidates = sorted(pitch for pitch in range(46, score.tonic + 3) if pitch % 12 in chord)
    return min(
        (
            (low, middle, high)
            for low, middle, high in zip(candidates, candidates[1:], candidates[2:])
            if {low % 12, middle % 12, high % 12} == set(chord)
        ),
        key=lambda notes: sum(abs(a - b) for a, b in zip(notes, former))
        + abs(notes[-1] - (score.tonic - 2)) * 0.44,
    )


@dataclass(frozen=True, order=True)
class Event:
    frame: int
    velocity: int
    channel: int
    key: int


def arrange(score: Song, evening: bool):
    call, answer, bridge = phrases(score)
    size, accents = METERS[score.meter]
    bpm = score.bpm * (0.91 if evening else 1.0)
    cell_seconds = 30 / bpm
    # A, B and A again. The last four bars are a quiet cadence in the day version.
    coda = answer if not evening else []
    measures = call + answer + bridge + bridge + call + answer + coda
    length = round(len(measures) * size * cell_seconds * RATE)
    events = []
    lead, reply, picking = ENSEMBLES[score.colors]
    if evening:
        lead, reply = reply, lead
        if score.colors in ("koto", "bamboo", "festival"):
            picking = KOTO_BACK if lead is SHAKUHACHI else SHAMISEN
        else:
            picking = NYLON_BACK if picking is HARP_BACK else HARP_BACK
        if score.colors in ("fjord", "hearth", "sail"):
            lead, reply, picking = RECORDER, DULCIMER, DULCIMER_BACK
    bass = PIZZ if score.colors in ("pavilion", "chamber") else BASS
    voices = {0: lead, 1: reply, 2: picking, 3: bass, 4: CELLO, 9: BRUSHES}
    former = (score.tonic - 12, score.tonic - 8, score.tonic - 4)

    def put(channel, key, at, duration, accent=0, push=0.0):
        voice = voices[channel]
        start = round((at * cell_seconds + push) * RATE)
        stop = round((at * cell_seconds + push + duration * cell_seconds) * RATE)
        events.append(Event(start, max(29, min(112, voice.velocity + accent)), channel, key))
        events.append(Event(stop, 0, channel, key))

    for bar, measure in enumerate(measures):
        start = bar * size
        section = bar // 8
        phrase_bar = bar % 4
        chord = voicing(score, measure.root, former)
        former = chord
        root = midi_pitch(score, measure.root)
        while root > 49:
            root -= 12
        fifth = root + 7 if root <= 45 else root - 5
        contrasting = section == 1
        coda_bar = section == 3
        # Interest comes from phrasing and changing who plays, with the written melody intact.
        first_pass = bar < 8
        retreat = -8 if coda_bar else -3 if evening else 0
        phrase_dynamics = (-3, 0, 2, -4)[phrase_bar] + retreat
        channel = 1 if (contrasting and bar < 12) or coda_bar else 0
        if not first_pass and section == 2 and phrase_bar >= 2:
            channel = 1

        for match in re.finditer(r"([def1-9])(-*)", measure.melody):
            at = match.start()
            weight = 4 if at in accents else -3
            if at == 0 and phrase_bar == 0:
                weight += 4
            degree = DEGREES[match[1]]
            pitch = midi_pitch(score, degree)
            use_lower_register = evening and pitch > 73 and voices[channel] is HARP
            if use_lower_register:
                pitch -= 12
            duration = min(len(match[0]) * voices[channel].gate, len(match[0]) + 0.24)
            # The bass enters first. The lead sits a few milliseconds behind the pluck.
            put(channel, pitch, start + at, duration, weight + phrase_dynamics,
                push=0.006 if at in accents else 0.012)

        put(3, root, start, (size if evening or coda_bar else size * 0.52) - 0.24,
            phrase_dynamics - 1)
        if not evening and not coda_bar:
            if bar % 2 == 1 and phrase_bar != 3:
                next_root = midi_pitch(score, measures[bar + 1].root) if bar + 1 < len(measures) else score.tonic
                while next_root > 50:
                    next_root -= 12
                put(3, next_root, start + size - 1.03, 0.79, -10)
            else:
                put(3, fifth, start + accents[-1], 1.34, -6 + retreat)

        if evening:
            # Sparse, ringing shapes leave room for the lower-register evening lead.
            positions = ((0.09, 0), (1.65, 1), (size / 2 + 0.33, 2), (size - 1.02, 1))
        elif score.colors in ("trail", "courtyard", "festival", "sail") and not coda_bar:
            positions = tuple((at + 0.24, degree) for at, degree in enumerate(
                (0, 1, 2, 1, 0, 2, 1, 2)[:size]
            ))
        elif contrasting:
            positions = ((0.10, 0), (1.11, 1), (size / 2 + 0.12, 2), (size - 1.1, 1))
        else:
            positions = ((0.12, 0), (1.11, 2), (size / 2, 1), (size - 1.13, 2))
        for onset, degree in positions:
            tone = chord[degree]
            volume = phrase_dynamics - (3 if evening else 0) - (5 if degree == 0 else 0)
            if bar % 2 == 0 and degree == 2 and not evening and not coda_bar:
                tone += 12 if tone + 12 < score.tonic + 9 else 0
            put(2, tone, start + onset, min(2.1, size - onset - 0.06), volume)
        if contrasting or (section == 2 and (phrase_bar < 2 or evening)):
            # An unobtrusive bowed inner voice connects the sections.
            sustain = chord[1] if bass is BASS else chord[0] + 12
            put(4, sustain, start + 0.13, size - 0.40, phrase_dynamics - 8)

        active_rhythm = score.colors in ("trail", "courtyard", "guitar", "festival", "sail")
        if active_rhythm and not evening and not coda_bar and 2 <= bar < len(measures) - 5:
            # Low strokes and soft brushes, with no tuned pattern over the score.
            put(9, 36, start, 0.46, 0)
            put(9, 38, start + accents[-1] + 0.05, 0.62, -8)
            if contrasting or bar % 2 == 0:
                put(9, 42, start + accents[1] + 0.04, 0.39, -18)
    return length, voices, sorted(events)


def synthesize(soundfont: Path, length: int, voices, events):
    synth = fluidsynth.Synth(
        gain=0.43, samplerate=RATE, channels=16,
        **{"synth.chorus.active": False, "synth.reverb.active": True},
    )
    try:
        bank = synth.sfload(str(soundfont), update_midi_preset=0)
        if bank < 0:
            raise ValueError(f"FluidSynth could not load {soundfont}")
        synth.set_reverb(0.36, 0.56, 55.0, 0.29)
        for channel, voice in voices.items():
            result = synth.program_select(channel, bank, 128 if channel == 9 else 0, voice.program)
            if result != 0:
                raise ValueError(f"SoundFont preset {voice.program} is unavailable")
            synth.cc(channel, 7, voice.volume)
            synth.cc(channel, 10, voice.pan)
            synth.cc(channel, 91, 29 if channel in (0, 1) else 21)

        # Put the final cadence's tail underneath the first beat at the loop point.
        tail_frames = RATE * 3
        pcm = np.empty((length + tail_frames, 2), dtype=np.float32)
        played = 0

        def advance(frame):
            nonlocal played
            if frame > played:
                frames = synth.get_samples(frame - played).reshape(-1, 2)
                pcm[played:frame] = frames.astype(np.float32) / 32768.0
                played = frame

        # Repeated notes may be re-articulated before the earlier gate ends.
        # Keep the newest note active if the earlier one is still releasing.
        active = {}
        for event in events:
            advance(event.frame)
            pair = (event.channel, event.key)
            if event.velocity:
                if pair in active:
                    synth.noteoff(*pair)
                synth.noteon(*pair, event.velocity)
                active[pair] = active.get(pair, 0) + 1
            elif active.get(pair, 0) == 1:
                synth.noteoff(*pair)
                del active[pair]
            elif pair in active:
                active[pair] -= 1
        advance(length + tail_frames)
        pcm[:tail_frames] += pcm[length:] * np.linspace(1.0, 0.0, tail_frames)[:, None]
        return pcm[:length]
    finally:
        synth.delete()


def master(pcm, evening: bool):
    measurement = run(
        ["ffmpeg", "-hide_banner", "-nostats", "-f", "f32le", "-ar", str(RATE), "-ac", "2",
         "-i", "pipe:0", "-af", "ebur128=framelog=verbose", "-f", "null", "-"],
        input=pcm.astype("<f4").tobytes(), capture_output=True, check=True,
    ).stderr.decode()
    measured = float(re.findall(r"I:\s+(-?\d+\.\d+) LUFS", measurement)[-1])
    target = -20.5 if evening else -19.0
    gain = 10 ** ((target - measured) / 20)
    # A fixed gain preserves the players' written dynamics.
    return pcm * min(gain, 0.76 / np.max(np.abs(pcm)))


def write_score(index: int, evening: bool, soundfont: Path, out: Path):
    score = SCORES[index]
    length, voices, events = arrange(score, evening)
    pcm = master(synthesize(soundfont, length, voices, events), evening)
    suffix = "-abend" if evening else ""
    title = f"{score.title} (Abendfassung)" if evening else score.title
    run([
        "ffmpeg", "-hide_banner", "-loglevel", "error", "-y",
        "-f", "f32le", "-ar", str(RATE), "-ac", "2", "-i", "pipe:0",
        "-c:a", "libvorbis", "-b:a", "64k",
        "-metadata", f"title={title}", "-metadata", f"location={score.harbor}",
        "-metadata", "album=Nililane · Hafenmusik",
        str(out / f"{index:02}{suffix}.ogg"),
    ], input=pcm.astype("<f4").tobytes(), check=True)
    print(f"{index:02}{suffix:6} {score.harbor:14} {len(pcm) / RATE:5.1f} s  {title}", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--soundfont", required=True, type=Path)
    parser.add_argument("--out", type=Path, default=OUT)
    parser.add_argument("--evening", action="store_true", help="only render the evening arrangements")
    parser.add_argument("--day", action="store_true", help="only render the day arrangements")
    parser.add_argument("--jobs", type=int, default=3)
    parser.add_argument("indices", nargs="*", type=int, metavar="HARBOR")
    args = parser.parse_args()
    if args.evening and args.day:
        parser.error("--day and --evening are mutually exclusive")
    if any(not 0 <= index < len(SCORES) for index in args.indices):
        parser.error(f"Harbor numbers must be between 0 and {len(SCORES) - 1}")
    if not args.soundfont.is_file():
        parser.error(f"SoundFont not found: {args.soundfont}")
    args.out.mkdir(parents=True, exist_ok=True)
    variants = [True] if args.evening else [False] if args.day else [False, True]
    with ProcessPoolExecutor(max_workers=args.jobs) as pool:
        jobs = [
            pool.submit(write_score, i, evening, args.soundfont, args.out)
            for i in args.indices or range(len(SCORES)) for evening in variants
        ]
        for job in jobs:
            job.result()


if __name__ == "__main__":
    main()
