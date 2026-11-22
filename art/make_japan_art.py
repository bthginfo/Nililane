"""Bake the Inland Sea paintings, wardrobe cards and temple chime."""
from math import cos, exp, pi, sin
from pathlib import Path
from random import Random
from struct import pack
import wave

from make_ui import Art

DARK = "#2b4652"
TILE = "#536779"
EDGE = "#8599a0"
TIMBER = "#664d45"
CEDAR = "#a97459"
PAPER = "#f8e8b9"
RICE = "#fff4d2"
RED = "#c5654d"
RUST = "#934b48"
GOLD = "#e9b966"
JADE = "#73955d"
TEAL = "#6caaab"
INDIGO = "#596fa3"
PINK = "#e6a0ae"


def roof(a, x, y, w, height, tile=TILE):
    a.poly([(x-w, y+2), (x-w*.75, y-height*.41), (x-w*.54, y-height),
            (x+w*.54, y-height), (x+w*.75, y-height*.41), (x+w, y+2),
            (x+w*.65, y+4), (x-w*.65, y+4)], DARK)
    a.poly([(x-w*.96, y), (x-w*.53, y-height), (x+w*.53, y-height),
            (x+w*.95, y), (x+w*.53, y+1), (x-w*.5, y+1)], tile)
    for i in [-.6, -.38, -.15, .15, .38, .6]:
        a.line([(x+i*w*.81, y-height+.5), (x+i*w*1.17, y)], EDGE, max(1, int(w/16)))
    a.line([(x-w*.57, y-height), (x+w*.57, y-height)], DARK, max(2, int(w/12)))


def hall(a, x, y, w, tier=1, paint=RED):
    a.box((x-w*.76, y-3, x+w*.76, y+3), "#b5a690", 2)
    a.box((x-w*.61, y-w*.57, x+w*.61, y-2), PAPER, 1)
    for pillar in [-.55, -.22, .22, .55]:
        a.box((x+(pillar-.046)*w, y-w*.59, x+(pillar+.046)*w, y), TIMBER, 1)
    a.box((x-w*.18, y-w*.34, x+w*.18, y-3), paint, 1)
    a.box((x-w*.68, y-w*.53, x+w*.68, y-w*.46), paint)
    roof(a, x, y-w*.53, w*.89, w*.25)
    for i in range(1, tier):
        top = y-w*(.80+i*.28)
        a.box((x-w*(.47-i*.08), top, x+w*(.47-i*.08), top+w*.25), paint)
        roof(a, x, top, w*(.70-i*.1), w*.19)


def bowl(a, x, y, w, fill, ceramic):
    a.poly([(x-w, y), (x+w, y), (x+w*.71, y+w*.46), (x+w*.38, y+w*.62),
            (x-w*.38, y+w*.62), (x-w*.75, y+w*.46)], ceramic)
    a.oval((x-w, y-w*.20, x+w, y+w*.24), DARK)
    a.oval((x-w*.86, y-w*.12, x+w*.86, y+w*.16), fill)
    a.line([(x-w*.69, y+w*.35), (x-w*.46, y+w*.49), (x+w*.20, y+w*.54)], EDGE, 3)


def lantern(a, x, y, r, color):
    a.oval((x-r, y-r*1.26, x+r, y+r*1.26), color)
    a.oval((x-r*.64, y-r*.99, x+r*.45, y+r*.56), GOLD)
    for off in [-.52, .02, .55]:
        a.line([(x-r*.80, y+r*off), (x-r*.30, y+r*(off+.13)),
                (x+r*.38, y+r*(off+.13)), (x+r*.85, y+r*off)], RUST, 1)
    a.box((x-r*.41, y-r*1.32, x+r*.41, y-r*1.17), TIMBER, 1)
    a.box((x-r*.31, y+r*1.13, x+r*.31, y+r*1.3), TIMBER, 1)


def make_icon(key):
    a = Art()
    a.oval((12, 10, 84, 82), "#e9deaa")
    a.oval((16, 14, 80, 78), "#f3e7bd")
    a.oval((13, 76, 85, 88), "#d1c18b")
    if key == "machiya":
        hall(a, 48, 78, 40, paint=INDIGO)
        a.box((25, 57, 71, 67), "#c48e67")
        for x in [30, 39, 48, 57, 66]:
            a.line([(x, 52), (x, 75)], TIMBER, 2)
    elif key == "temple":
        hall(a, 48, 79, 43, 3)
    elif key == "temple_bell":
        a.line([(22, 79), (22, 26), (75, 26), (75, 79)], TIMBER, 7)
        roof(a, 49, 27, 38, 12)
        a.box((44, 28, 54, 43), GOLD, 5)
        a.poly([(34, 46), (41, 38), (56, 38), (63, 46), (65, 61),
                (71, 70), (66, 75), (31, 75), (27, 70), (32, 60)], "#547d72")
        a.line([(36, 60), (37, 47), (43, 43)], "#9abe9e", 3)
        a.box((28, 68, 70, 75), "#38615d", 3)
        a.line([(58, 43), (65, 63), (70, 87)], CEDAR, 3)
    elif key == "rice_paddy":
        for n in range(3):
            y = 49 + n*12
            a.poly([(24-n*2, y), (72+n*2, y), (84, y+10), (11, y+10)], CEDAR)
            a.poly([(27-n*2, y+1), (69+n*2, y+1), (76, y+7), (20, y+7)], TEAL)
            for i in range(5):
                x = 27+i*10
                a.line([(x-3, y-5), (x, y+4), (x+4, y-6)], JADE, 2)
                a.line([(x, y-8), (x, y+2)], "#9caa5b", 2)
    elif key == "rice_kitchen":
        hall(a, 48, 72, 37, paint=INDIGO)
        a.box((22, 60, 75, 69), INDIGO)
        bowl(a, 49, 72, 17, RICE, INDIGO)
    elif key == "rice_bowl":
        a.oval((22, 36, 74, 63), RICE)
        for x, y in [(33, 44), (42, 38), (53, 41), (61, 45), (39, 50), (58, 51)]:
            a.oval((x-6,y-3,x+6,y+3), "#fff9e5")
        bowl(a, 48, 57, 30, RICE, INDIGO)
    elif key == "tea_garden":
        a.poly([(13, 74), (25, 45), (70, 37), (84, 73), (77, 82), (21, 83)], CEDAR)
        for i in range(3):
            for j in range(4):
                x, y = 29+j*13, 49+i*11-j*2
                a.oval((x-11, y-8, x+9, y+7), "#527653")
                a.oval((x-9, y-9, x+7, y+3), JADE)
                a.oval((x-4, y-7, x+2, y-4), "#a4b273")
    elif key == "tea_house":
        hall(a, 46, 72, 39, paint=JADE)
        bowl(a, 48, 74, 16, JADE, "#8faaa0")
    elif key == "tea_cup":
        bowl(a, 48, 59, 27, JADE, "#74928a")
        for x, y in [(46, 20), (56, 27), (37, 30)]:
            a.line([(x, y), (x+5, y+6), (x+1, y+15)], "#fcf9db", 3)
    elif key == "paper_workshop":
        hall(a, 47, 72, 39, paint=RED)
        for x, y, c in [(26, 62, RUST), (48, 68, RED), (70, 60, PINK)]:
            lantern(a, x, y, 9, c)
    elif key == "lampion":
        a.line([(48, 16), (48, 24)], TIMBER, 5)
        lantern(a, 48, 52, 24, RED)
        a.line([(48, 83), (48, 89)], GOLD, 3)
    elif key in {"wear_indigo_kosode", "wear_ahorn", "wear_haori"}:
        color, detail = {"wear_indigo_kosode": (INDIGO, "#bfd6dc"),
                         "wear_ahorn": (RED, GOLD), "wear_haori": ("#8a67a8", GOLD)}[key]
        a.poly([(39, 23), (47, 31), (57, 23), (70, 26), (83, 54), (66, 62),
                (62, 80), (31, 80), (29, 62), (12, 54), (25, 26)], color)
        a.line([(39, 24), (54, 53), (56, 80)], detail, 5)
        a.line([(57, 24), (41, 49)], detail, 4)
        a.box((30, 52, 65, 58), TIMBER, 2)
        for x, y in [(39, 68), (60, 70), (63, 37)]:
            a.oval((x-4, y-3, x+3, y+3), detail)
    elif key in {"wear_kasa", "wear_pilgrim"}:
        a.poly([(48, 25), (81, 58), (78, 68), (17, 68), (13, 58)], CEDAR)
        a.poly([(48, 24), (82, 58), (67, 66), (26, 65), (13, 58)], GOLD)
        for dx in [-26, -17, -7, 7, 17, 26]:
            a.line([(48, 25), (48+dx, 63)], PAPER, 2)
        if key == "wear_pilgrim":
            a.poly([(36, 66), (60, 66), (62, 84), (46, 80), (35, 85)], INDIGO)
    elif key == "wear_kanzashi":
        a.line([(25, 77), (59, 28)], GOLD, 5)
        a.line([(58, 36), (70, 65), (66, 78)], CEDAR, 2)
        a.oval((62, 73, 71, 83), RED)
        for n in range(7):
            turn = n*2*pi/7
            at = (56+cos(turn)*12, 34+sin(turn)*12)
            a.oval((at[0]-8, at[1]-8, at[0]+8, at[1]+8), PINK)
        a.oval((50, 28, 62, 40), GOLD)
    elif key == "wear_obi":
        a.box((22, 48, 76, 60), "#df9c9e", 4)
        a.poly([(49, 48), (23, 29), (17, 41), (24, 62), (50, 57),
                (69, 67), (82, 52), (72, 30)], "#b96380")
        a.line([(24, 37), (46, 50), (69, 38)], GOLD, 3)
        a.box((43, 45, 55, 59), PINK, 4)
    elif key == "wear_inro":
        a.line([(48, 19), (41, 30), (48, 45), (54, 30), (48, 19)], CEDAR, 3)
        a.oval((43, 27, 53, 37), GOLD)
        a.box((28, 40, 68, 81), TIMBER, 8)
        for y in [49, 59, 69]:
            a.line([(30, y), (48, y+3), (66, y)], GOLD, 2)
        a.poly([(48, 55), (54, 64), (48, 73), (42, 64)], PINK)
    else:
        raise ValueError(key)
    a.save(key)


def island(index):
    a = Art(230, 112)
    rng = Random(9041+index*1051)
    season = index // 6
    sky, sea, rim, grass, tree = [
        ("#ccdbcb", "#78a7a0", "#e6ca96", "#8fa77b", "#e19aa3"),
        ("#d0e3cd", "#78a9a8", "#deca95", "#88a978", "#729558"),
        ("#e3cfba", "#7babb1", "#e0be91", "#a2a37b", "#c67f60"),
        ("#d6dad3", "#73999f", "#dac4a2", "#889a76", "#b48d67"),
    ][season]
    a.box((0, 0, 230, 112), sea)
    a.box((0, 0, 230, 35), sky)
    moon = 151 + index * 17 % 63
    a.oval((moon, 5, moon+19, 24), "#f4e4b1")
    for layer, tint in enumerate(["#8aa89c", "#78998e"]):
        y = 29 + layer*10
        nodes = [(-6, y+10)]
        for x in range(0, 245, 12):
            nodes.append((x, y+rng.randint(-10, 6)))
        a.poly(nodes + [(230, y+10), (0, y+13)], tint)
    for _ in range(29):
        x, y = rng.randrange(220), rng.randrange(43, 112)
        a.line([(x, y), (x+rng.randrange(3, 13), y)], "#91b8ac", 1)
    center = 91 + rng.randint(-11, 11)
    radius = 65 + rng.randint(-10, 14)
    shape = [(center-radius, 76), (center-radius*.68, 61),
             (center-radius*.29, 53), (center+radius*.31, 55+index%5),
             (center+radius*.82, 62), (center+radius, 79), (center+21, 99), (center-38, 94)]
    a.poly([(x+2,y+5) for x,y in shape], "#63897c")
    a.poly(shape, rim)
    a.poly([(x,y-2) for x,y in shape[:6]]+[(center+28, 89), (center-29, 86)], grass)
    for n in range(12):
        x, y = center+rng.randint(-48, 53), rng.randint(56, 78)
        if abs(x-center) < 19 and n % 2:
            continue
        if n % 3:
            a.poly([(x, y-15), (x+4,y-5), (x+5,y), (x-5,y), (x-4,y-5)], "#527765")
            a.line([(x, y-3), (x, y+3)], TIMBER, 1)
        else:
            a.oval((x-5,y-9,x+6,y), tree)
            a.line([(x, y-2), (x, y+4)], TIMBER, 1)
    if index % 4 == 1:
        x = center+27
        for dx in [-11, 11]:
            a.line([(x+dx, 78), (x+dx*.88, 55)], RED, 4)
        a.line([(x-14, 60), (x+14, 60)], RED, 3)
        a.line([(x-17, 54), (x, 57), (x+17, 54)], DARK, 3)
        hall(a, center-24, 74, 19, 2)
    elif index % 4 == 2:
        hall(a, center-4, 71, 24+index%5, 3)
        for n in range(4):
            y = 74+n*3
            a.line([(center-7-n, y), (center+2+n, y)], RICE, 2)
    elif index % 4 == 3:
        for i in range(4):
            y = 61 + i*6
            a.poly([(center-47, y+3), (center-25, y-1), (center-9,y+2), (center-18,y+6), (center-49,y+8)], TEAL)
            a.line([(center-48, y+8), (center-24, y+8), (center-10, y+3)], GOLD, 1)
        hall(a, center+22, 76, 19, paint=JADE)
    else:
        hall(a, center-32, 75, 17)
        hall(a, center+13, 71, 25, 2 if index else 1, INDIGO)
        for n in range(3):
            x = center+25+n*8
            a.line([(x, 71), (x, 82)], CEDAR, 2)
            lantern(a, x, 73, 2.3, RED)
    for n in range(3):
        x, y = center-45+n*21, 86 + n % 2*4
        a.line([(x, y), (x+10, y-2)], PAPER, 1)
    x = 173 + index*9 % 29
    a.poly([(x-13, 94), (x+17, 94), (x+10, 100), (x-8, 100)], TIMBER)
    a.poly([(x-7, 77), (x+8, 77), (x+6, 94), (x-9, 93)], PAPER)
    a.line([(x-4, 78), (x-5, 93)], INDIGO, 2)
    a.line([(x-11, 94), (x+15, 94)], GOLD, 1)
    a.save(f"island_{48+index}")


def bell():
    output = Path(__file__).resolve().parents[1] / "assets/audio/temple_bell.wav"
    rate = 22050
    with wave.open(str(output), "wb") as sound:
        sound.setparams((1, 2, rate, 0, "NONE", "not compressed"))
        frames = bytearray()
        partials = [(147.0, .99, .68), (220.5, .55, .32), (309.7, .28, .49),
                    (422.6, .17, .33), (708.6, .12, .17), (960.1, .034, .15)]
        for i in range(round(rate*2.3)):
            t = i / rate
            strength = min(1.0, t/.03) * min(1.0, (2.3-t)/.35)
            sample = sum(amplitude * exp(-t/decay) * sin(2*pi*frequency*t)
                         for frequency, amplitude, decay in partials)
            frames += pack("<h", round(10500 * strength * sample))
        sound.writeframes(frames)


if __name__ == "__main__":
    for key in ["machiya", "temple", "temple_bell", "rice_paddy", "rice_kitchen", "rice_bowl",
                "tea_garden", "tea_house", "tea_cup", "paper_workshop", "lampion",
                "wear_indigo_kosode", "wear_ahorn", "wear_haori", "wear_kasa", "wear_pilgrim",
                "wear_kanzashi", "wear_obi", "wear_inro"]:
        make_icon(key)
    for index in range(24):
        island(index)
    bell()
