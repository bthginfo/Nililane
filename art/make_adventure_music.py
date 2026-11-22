"""Render five expedition themes and their island arrangements as loopable Vorbis."""
import argparse
from concurrent.futures import ProcessPoolExecutor
from dataclasses import replace
from pathlib import Path
from subprocess import run

from harbor_scores import Song
from make_harbor_music import RATE, arrange, master, synthesize

OUT = Path(__file__).resolve().parents[1] / "assets/audio/abenteuer"
THEMES = [
    Song("Griechenland", "Über dem Olivental", 89, 60, "mixolydian", "6/8", "lyre", "1471 7461 4565",
         "13--52 4--364 5--786 5-2--1",
         "3-56-8 6--4-6 5-7-24 1-----",
         "46-565 6---8- 6-54-3 2-47--"),
    Song("Rom", "Der ältere Weg", 79, 60, "major", "3/4", "chamber", "1451 6411 2465",
         "1--35- 6-56-- 2-45-2 1--35-",
         "1-34-5 6-8-65 3---21 1-----",
         "4-646- 7--865 6-4-5- 2-4-7-"),
    Song("Japan", "Der erste Kranich", 87, 60, "dorian", "4/4", "bamboo", "1471 1461 3745",
         "1---34-- 5---8--- 6--5-34- 1-----..",
         "1-3--5-- 6-5---34 5-8-65-- 1-------",
         "3--56--- 8---5--- 6--34--5 2-----.."),
    Song("Norden", "Lichter am Fjordhang", 88, 57, "dorian", "6/8", "fjord", "1471 1451 4675",
         "1-3-5- 4--6-4 7-654- 3---1-",
         "1-345- 6-58-6 5--2f- 1-----",
         "46-86- 6-8--9 7-6-57 5---2-"),
    Song("Oasenreiche", "Wind am Dattelhain", 83, 58, "hijaz", "7/8", "oasis", "1471 1461 6475",
         "12-3-41 4-6--54 7-6-4-2 1----..",
         "1-234-5 5-6-86- 6-5-4-2 1------",
         "6--86-5 4-6-5-4 7--6-4- 2-4--5-"),
]
ENSEMBLES = [
    ("lyre", "reed", "trail"),
    ("chamber", "pavilion", "courtyard"),
    ("bamboo", "koto", "festival"),
    ("fjord", "hearth", "sail"),
    ("oasis", "lute", "caravan"),
]


def score_for(region, stage):
    theme = THEMES[region]
    # Island arrangements vary in register, tempo, instrumental color and phrase order.
    forms = [(theme.call, theme.answer), (theme.answer, theme.call), (theme.call, theme.bridge)]
    call, answer = forms[stage % 3]
    chord_rows = theme.chords.split()
    chord_order = [(0, 1, 2), (1, 0, 2), (0, 2, 1)][stage % 3]
    bridge = theme.answer if stage % 3 == 2 else theme.bridge
    return replace(
        theme, call=call, answer=answer, bridge=bridge,
        chords=" ".join(chord_rows[index] for index in chord_order),
        tonic=theme.tonic + [-2, 0, 2, 3][stage // 3],
        bpm=theme.bpm + [-4, 4, 11, 0, 6, -2, 8, 3, -3, 12, -6, 5][stage],
        colors=ENSEMBLES[region][(stage // 2) % 3],
    )


def render(region, stage, soundfont):
    evening = stage is None
    theme = THEMES[region]
    score = theme if evening else score_for(region, stage)
    length, voices, events = arrange(score, evening)
    audio = master(synthesize(soundfont, length, voices, events), evening)
    name = f"nacht-{region}.ogg" if evening else f"{region * 12 + stage:02}.ogg"
    title = f"{theme.title} (Nacht)" if evening else f"{theme.title} · {stage + 1}"
    run([
        "ffmpeg", "-hide_banner", "-loglevel", "error", "-y", "-f", "f32le", "-ar", str(RATE),
        "-ac", "2", "-i", "pipe:0", "-c:a", "libvorbis", "-b:a", "52k", "-metadata", f"title={title}",
        "-metadata", "album=Nililane · Abenteuer", str(OUT / name),
    ], input=audio.astype("<f4").tobytes(), check=True)
    print(name, f"{len(audio) / RATE:.1f} s", title, flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--soundfont", required=True, type=Path)
    parser.add_argument("--jobs", type=int, default=3)
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    with ProcessPoolExecutor(max_workers=args.jobs) as pool:
        jobs = [pool.submit(render, region, stage, args.soundfont) for region in range(5) for stage in [None, *range(12)]]
        for job in jobs:
            job.result()
