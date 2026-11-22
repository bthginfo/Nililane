"""Draw Nililane's game icons and island charts as antialiased PNGs."""

from pathlib import Path
import math
import random
from PIL import Image, ImageDraw

OUT = Path(__file__).resolve().parents[1] / "assets/ui"
OUT.mkdir(parents=True, exist_ok=True)
INK = "#294B51"
CREAM = "#FFF1CD"
GOLD = "#E8AE59"
TERRA = "#C77053"
DEEP = "#95513F"
SEA = "#549C9D"
BLUE = "#4C718C"
LEAF = "#7F9B65"
DARK_LEAF = "#51745C"


class Art:
    def __init__(self, width=96, height=96):
        self.size = width, height
        self.canvas = Image.new("RGBA", (width * 3, height * 3))
        self.draw = ImageDraw.Draw(self.canvas)

    def poly(self, points, fill):
        self.draw.polygon([(int(x * 3), int(y * 3)) for x, y in points], fill=fill)

    def box(self, xy, fill, radius=0):
        self.draw.rounded_rectangle([int(x * 3) for x in xy], radius=radius * 3, fill=fill)

    def oval(self, xy, fill):
        self.draw.ellipse([int(x * 3) for x in xy], fill=fill)

    def line(self, points, fill, width=3):
        self.draw.line([(int(x * 3), int(y * 3)) for x, y in points],
                       fill=fill, width=width * 3, joint="curve")

    def save(self, name):
        self.canvas.resize(self.size, Image.Resampling.LANCZOS).save(OUT / (name + ".png"), optimize=True)


def house(a, roof=TERRA, shutters=SEA, base=CREAM):
    a.poly([(15, 44), (46, 23), (76, 42), (76, 77), (15, 77)], base)
    a.poly([(76, 42), (86, 34), (86, 68), (76, 77)], "#D6AA78")
    a.poly([(8, 44), (47, 17), (82, 36), (76, 45), (47, 29), (20, 48)], roof)
    a.poly([(47, 17), (61, 10), (94, 30), (82, 36)], DEEP if roof == TERRA else BLUE)
    a.box((44, 51, 59, 77), INK, 2)
    a.box((21, 52, 33, 64), shutters, 1)
    a.box((63, 51, 72, 64), shutters, 1)
    a.box((11, 75, 77, 81), "#B99C76", 2)


def arch(a):
    a.box((14, 73, 83, 81), "#CBA577", 2)
    a.box((19, 64, 79, 73), CREAM, 2)
    for x in (25, 44, 63):
        a.box((x, 36, x + 8, 66), CREAM, 2)
        a.box((x + 6, 38, x + 10, 64), "#DCBD8C")
    a.box((17, 28, 81, 38), BLUE, 2)
    a.poly([(13, 28), (49, 12), (86, 28)], TERRA)
    a.poly([(32, 27), (49, 20), (65, 27)], CREAM)


def jug(a, off=0, tint=TERRA):
    a.poly([(40 + off, 20), (57 + off, 20), (58 + off, 37),
            (71 + off, 49), (68 + off, 67), (58 + off, 80), (39 + off, 80),
            (29 + off, 66), (27 + off, 50), (40 + off, 36)], tint)
    a.poly([(52 + off, 20), (57 + off, 20), (58 + off, 37),
            (71 + off, 49), (68 + off, 67), (58 + off, 80), (53 + off, 79), (60 + off, 55)], DEEP)
    a.line([(38 + off, 34), (24 + off, 39), (22 + off, 49), (32 + off, 58)], DEEP, 5)
    a.line([(58 + off, 34), (72 + off, 39), (75 + off, 49), (65 + off, 58)], DEEP, 5)
    a.box((37 + off, 17, 61 + off, 24), GOLD, 2)
    a.line([(30 + off, 56), (39 + off, 52), (49 + off, 60), (59 + off, 51), (66 + off, 55)], CREAM, 3)


def bread(a):
    a.poly([(12, 62), (17, 46), (34, 35), (57, 25), (77, 27), (86, 38),
            (84, 55), (73, 68), (55, 76), (32, 79), (17, 73)], "#AD6241")
    a.poly([(12, 61), (19, 43), (40, 32), (58, 25), (77, 26), (85, 38),
            (74, 55), (46, 70), (26, 74)], GOLD)
    for x, y in [(35, 57), (50, 48), (64, 39)]:
        a.line([(x - 7, y - 9), (x + 3, y + 5)], CREAM, 4)


def leaf(a):
    a.line([(32, 77), (48, 53), (65, 28)], DARK_LEAF, 6)
    for x, y, sign in [(45, 55, -1), (53, 48, 1), (58, 39, -1), (65, 32, 1)]:
        a.poly([(x, y), (x + sign * 15, y - 5), (x + sign * 22, y - 19),
                (x + sign * 8, y - 14)], LEAF if sign < 0 else DARK_LEAF)
    a.oval((43, 55, 54, 70), "#38453E")
    a.oval((52, 30, 61, 44), "#38453E")


def star(a, color=GOLD, middle=(48, 48), radius=34):
    points = []
    for j in range(10):
        ang = math.pi * (j / 5 - 0.5)
        r = radius if j % 2 == 0 else radius * 0.44
        points.append((middle[0] + math.cos(ang) * r, middle[1] + math.sin(ang) * r))
    a.poly(points, color)


def guest_face(a, mood):
    # Broad features and different mouth shapes stay legible at HUD size.
    palettes = {
        "guest_happy": ("#A9E2B7", "#57A28F", "#DEFFE3"),
        "guest_mixed": ("#FFE1A1", "#DC9953", "#FFF4D3"),
        "guest_down": ("#FFC2AE", "#CC735F", "#FFE4D7"),
        "guest_wait": ("#BFE1DF", "#6FA6A9", "#E2FAF2"),
    }
    face, side, light = palettes[mood]
    a.oval((7, 9, 89, 91), "#172F33")
    a.oval((11, 10, 85, 86), side)
    a.oval((13, 10, 81, 78), face)
    a.line([(23, 25), (29, 20), (39, 17)], light, 3)
    a.oval((24, 47, 36, 53), side)
    a.oval((60, 47, 72, 53), side)
    for x in (33, 55):
        a.oval((x, 34, x + 8, 44), INK)
    if mood == "guest_happy":
        a.poly([(33, 52), (62, 52), (58, 61), (51, 66), (42, 66), (36, 61)], INK)
        a.line([(37, 54), (58, 54)], light, 2)
    elif mood == "guest_down":
        a.line([(34, 62), (40, 55), (47, 53), (54, 55), (61, 62)], INK, 5)
        a.line([(31, 32), (40, 29)], INK, 3)
        a.line([(55, 29), (65, 32)], INK, 3)
    elif mood == "guest_mixed":
        a.line([(37, 57), (57, 57)], INK, 5)
    else:
        for x in (35, 45, 55):
            a.oval((x, 54, x + 5, 59), INK)


def footprints(a):
    for x, y in [(26, 48), (59, 19)]:
        a.poly([(x, y + 23), (x - 5, y + 10), (x, y), (x + 10, y - 4),
                (x + 20, y + 3), (x + 17, y + 16), (x + 9, y + 26)], BLUE)
        a.box((x - 4, y + 16, x + 12, y + 23), SEA, 2)
        a.line([(x, y + 6), (x + 12, y + 10)], CREAM, 3)


def empty_crate(a):
    a.poly([(11, 32), (47, 17), (86, 31), (50, 47)], "#E5B682")
    a.poly([(19, 33), (47, 23), (77, 33), (50, 44)], "#6B483D")
    a.poly([(13, 36), (49, 49), (49, 78), (14, 66)], "#AD6E4E")
    a.poly([(51, 49), (84, 36), (83, 66), (51, 78)], "#C8905E")
    a.line([(25, 43), (25, 70)], GOLD, 6)
    a.line([(72, 43), (72, 70)], GOLD, 6)
    a.line([(25, 55), (49, 64), (75, 55)], DEEP, 3)


def guest_book(a):
    a.box((9, 20, 86, 78), "#38645F", 8)
    a.poly([(14, 19), (33, 14), (48, 20), (63, 14), (82, 19),
            (82, 68), (63, 65), (48, 73), (32, 65), (14, 69)], CREAM)
    a.line([(48, 23), (48, 68)], GOLD, 3)
    a.oval((19, 27, 42, 52), "#92C4A8")
    a.oval((24, 35, 28, 39), INK)
    a.oval((33, 35, 37, 39), INK)
    a.line([(25, 43), (30, 47), (35, 43)], INK, 2)
    for y in (30, 39, 48, 57):
        a.line([(55, y + 1), (75, y - 2)], "#B58C66", 2)


def make_icon(name):
    a = Art()
    if name == "house":
        house(a)
    elif name == "inn":
        house(a, BLUE, GOLD)
        a.box((35, 48, 66, 56), "#D9A95E", 2)
        a.box((38, 43, 46, 48), CREAM, 2)
    elif name == "well":
        a.oval((14, 59, 84, 81), "#B3976D")
        a.box((14, 54, 84, 67), "#D0B888", 7)
        a.oval((14, 43, 84, 64), CREAM)
        a.oval((22, 48, 76, 60), SEA)
        for x in (23, 66):
            a.box((x, 20, x + 7, 57), "#AD7C56", 3)
        a.box((18, 16, 79, 23), INK, 2)
        a.line([(48, 23), (48, 44)], INK, 3)
        a.box((41, 39, 55, 50), "#C98F60", 3)
    elif name in ("bread", "bakery"):
        if name == "bakery":
            house(a, TERRA, SEA)
            a.box((17, 43, 75, 55), SEA, 2)
            for x in (24, 40, 56, 69):
                a.box((x, 43, x + 5, 55), CREAM, 1)
        else:
            bread(a)
    elif name in ("olive", "grove"):
        if name == "grove":
            a.line([(47, 79), (50, 40), (60, 29)], "#91654B", 7)
            for x, y, tint in [(28, 43, LEAF), (43, 27, DARK_LEAF), (60, 35, "#9AAA70"), (71, 43, LEAF)]:
                a.poly([(x - 18, y + 3), (x - 14, y - 11), (x + 1, y - 16),
                        (x + 16, y - 4), (x + 17, y + 9), (x - 5, y + 14)], tint)
                a.oval((x, y, x + 5, y + 6), INK)
            a.box((24, 76, 73, 81), GOLD, 3)
        else:
            leaf(a)
    elif name == "potter":
        jug(a)
    elif name == "stoa":
        arch(a)
    elif name in ("taverna", "meze"):
        a.oval((12, 20, 86, 78), "#C68E61")
        a.oval((12, 16, 86, 71), CREAM)
        a.oval((21, 23, 77, 63), SEA)
        for x, y in [(31, 35), (41, 48), (61, 43), (50, 29)]:
            a.oval((x - 7, y - 4, x + 7, y + 5), GOLD if x % 3 else TERRA)
        a.line([(60, 61), (50, 53), (62, 50)], DARK_LEAF, 3)
    elif name == "bed":
        a.box((14, 30, 22, 74), INK, 2)
        a.box((74, 53, 83, 74), INK, 2)
        a.box((17, 43, 81, 64), SEA, 4)
        a.box((22, 37, 42, 52), CREAM, 4)
        a.box((45, 43, 79, 60), TERRA, 3)
    elif name == "coin":
        a.oval((15, 21, 83, 82), "#9E653D")
        a.oval((12, 12, 80, 73), GOLD)
        a.oval((21, 20, 71, 65), "#BA7B44")
        a.oval((25, 23, 67, 62), "#F7D380")
        a.line([(38, 54), (47, 36), (56, 31)], "#A36C48", 4)
        a.oval((54, 26, 62, 34), "#A36C48")
    elif name == "star":
        star(a, DEEP, (49, 51))
        star(a)
    elif name == "sun":
        a.oval((25, 25, 71, 71), GOLD)
        for i in range(8):
            t = i * math.pi / 4
            a.line([(48 + 29 * math.cos(t), 48 + 29 * math.sin(t)),
                    (48 + 39 * math.cos(t), 48 + 39 * math.sin(t))], GOLD, 5)
    elif name == "water":
        a.poly([(48, 11), (69, 42), (74, 59), (67, 75), (49, 83),
                (32, 78), (22, 62), (26, 46)], SEA)
        a.poly([(48, 11), (68, 42), (74, 59), (67, 75), (49, 83), (55, 58)], BLUE)
        a.line([(34, 49), (33, 61), (39, 68)], CREAM, 5)
    elif name in ("cloud", "rain", "storm"):
        dim = "#798C94" if name == "storm" else "#BBCEBD"
        a.oval((17, 35, 58, 63), dim)
        a.oval((30, 22, 69, 61), dim)
        a.oval((51, 33, 83, 62), dim)
        a.box((18, 46, 82, 61), dim, 7)
        if name != "cloud":
            for x in (28, 66):
                a.line([(x + 2, 68), (x - 4, 81)], SEA, 5)
        if name == "storm":
            a.poly([(47, 63), (55, 63), (53, 71), (60, 70), (45, 90), (48, 77), (42, 78)], GOLD)
        elif name == "rain":
            a.line([(53, 67), (48, 81)], BLUE, 5)
    elif name == "wind":
        for i in range(3):
            x, y = 17 + i * 5, 29 + 17 * i
            a.line([(x, y), (64 - i * 4, y), (74 - i * 4, y - 8),
                    (70 - i * 4, y - 14)], SEA, 6)
    elif name in ("ferry", "chart"):
        a.poly([(11, 60), (85, 60), (74, 76), (26, 76)], DEEP)
        a.poly([(15, 60), (82, 60), (74, 66), (25, 66)], GOLD)
        a.line([(47, 17), (47, 61)], INK, 4)
        a.poly([(47, 19), (74, 54), (47, 54)], CREAM)
        a.poly([(45, 24), (22, 54), (45, 54)], SEA)
        a.line([(12, 82), (25, 79), (44, 83), (61, 79), (84, 82)], SEA, 3)
    elif name in ("book", "guide"):
        a.box((18, 17, 78, 78), DEEP, 5)
        a.box((28, 18, 78, 74), CREAM, 4)
        a.line([(38, 33), (63, 33)], TERRA, 4)
        a.line([(38, 46), (65, 46)], SEA, 3)
        a.line([(38, 58), (58, 58)], SEA, 3)
        a.box((21, 20, 27, 73), GOLD, 2)
    elif name == "hammer":
        a.line([(27, 75), (61, 39)], "#997457", 14)
        a.poly([(43, 30), (58, 16), (80, 36), (67, 52)], BLUE)
        a.poly([(49, 25), (58, 16), (80, 36), (73, 40)], SEA)
    elif name == "lock":
        a.line([(33, 45), (33, 31), (39, 22), (55, 22), (63, 31), (63, 46)], INK, 7)
        a.box((24, 42, 74, 76), GOLD, 6)
        a.oval((44, 52, 54, 62), INK)
    elif name == "check":
        a.oval((15, 15, 82, 82), SEA)
        a.line([(30, 48), (43, 60), (68, 34)], CREAM, 7)
    elif name == "storehouse":
        a.poly([(10, 44), (46, 17), (85, 41), (85, 79), (10, 79)], "#BE825A")
        a.poly([(6, 44), (46, 15), (89, 42), (81, 49), (46, 30), (15, 49)], DEEP)
        a.box((30, 48, 64, 80), INK, 2)
        a.box((36, 56, 58, 79), GOLD, 3)
        a.line([(36, 66), (58, 66)], DEEP, 3)
    elif name == "press":
        a.oval((16, 52, 82, 80), DEEP)
        a.oval((15, 40, 81, 70), "#CAB88C")
        a.oval((23, 46, 72, 64), DARK_LEAF)
        a.line([(48, 15), (48, 55)], "#9B6C49", 9)
        a.line([(22, 30), (76, 31)], DEEP, 8)
        a.oval((53, 54, 60, 63), GOLD)
    elif name == "shrine":
        arch(a)
        star(a, GOLD, (48, 51), 16)
    elif name == "aqueduct":
        a.box((7, 19, 89, 32), TERRA, 2)
        a.box((7, 26, 89, 31), CREAM)
        for x in (7, 35, 63):
            a.line([(x, 77), (x, 47), (x + 5, 34), (x + 12, 30),
                    (x + 19, 34), (x + 24, 47), (x + 24, 77)], TERRA, 8)
        a.line([(13, 79), (85, 79)], SEA, 4)
    elif name == "baths":
        a.oval((17, 37, 79, 83), "#B78467")
        a.oval((12, 29, 84, 73), CREAM)
        a.oval((20, 34, 77, 68), SEA)
        for x in (34, 49, 62):
            a.line([(x, 46), (x + 5, 37), (x, 28), (x + 4, 18)], BLUE, 3)
    elif name == "forum":
        a.box((9, 28, 88, 75), CREAM, 3)
        a.box((7, 23, 90, 35), TERRA, 2)
        for x in (16, 42, 67):
            a.box((x, 47, x + 14, 74), DEEP, 5)
            a.box((x, 62, x + 14, 73), GOLD, 2)
        a.box((6, 73, 91, 81), BLUE, 2)
    elif name == "villa":
        house(a, TERRA, DEEP)
        a.box((22, 58, 72, 79), CREAM, 3)
        a.box((30, 62, 67, 74), SEA, 2)
        a.line([(36, 64), (58, 65)], "#C2E2CD", 2)
    elif name in ("guest_happy", "guest_mixed", "guest_down", "guest_wait"):
        guest_face(a, name)
    elif name == "guest_steps":
        footprints(a)
    elif name == "guest_empty":
        empty_crate(a)
    elif name == "guest_book":
        guest_book(a)
    elif name == "mood":
        a.oval((13, 13, 83, 83), "#6AA58F")
        a.oval((32, 34, 40, 44), INK)
        a.oval((56, 34, 64, 44), INK)
        a.line([(29, 54), (37, 61), (48, 64), (60, 61), (69, 53)], CREAM, 5)
    elif name == "nili":
        a.oval((7, 10, 89, 89), "#EDD0A0")
        a.oval((16, 17, 84, 83), "#9C8899")
        a.oval((16, 15, 36, 36), "#927B90")
        a.oval((65, 14, 84, 36), "#927B90")
        a.oval((22, 19, 32, 32), "#C493A0")
        a.oval((69, 18, 79, 30), "#C493A0")
        a.poly([(24, 71), (73, 70), (67, 85), (39, 89)], TERRA)
        a.oval((18, 45, 83, 76), "#B29BA7")
        for x in (31, 65):
            a.oval((x, 39, x + 5, 46), INK)
        for x in (40, 57):
            a.oval((x, 51, x + 6, 56), "#796775")
        a.line([(38, 66), (48, 69), (62, 65)], "#745E72", 2)
        a.box((35, 61, 42, 69), CREAM, 2)
        a.box((62, 60, 69, 68), CREAM, 2)
    a.save(name)


ICONS = ("house well bakery grove potter stoa taverna inn bread olive meze bed coin star "
         "sun water wind ferry chart book guide hammer lock check nili storehouse press shrine aqueduct baths forum villa mood cloud rain storm "
         "guest_happy guest_mixed guest_down guest_wait guest_steps guest_empty guest_book").split()
for item in ICONS:
    make_icon(item)


def island_card(index):
    a = Art(230, 112)
    rng = random.Random(437 + index * 75)
    roman = index >= 24
    tide = (["#7EA495", "#809E9A", "#87A79C", "#8EAFA4", "#799D99"] if roman
            else ["#73ADAE", "#6FA6A3", "#78ABB4", "#70A9AF", "#80AAA3"])[index % 5]
    a.box((0, 0, 230, 112), tide)
    a.oval((172, 14, 197, 39), "#ECD39B")
    for n in range(30):
        x, y = rng.randrange(230), rng.randrange(25, 111)
        a.line([(x, y), (x + rng.randrange(2, 16), y)], "#96C1B7", 1)
    center = rng.randrange(69, 107)
    a.poly([(center - 59, 85), (center - 43, 69), (center - 21, 60),
            (center + 17, 64), (center + 58, 72), (center + 82, 89), (center + 29, 101),
            (center - 33, 95)], "#468687")
    a.poly([(center - 60, 82), (center - 38, 61), (center - 10, 50),
            (center + 35, 60), (center + 67, 72), (center + 77, 85),
            (center + 28, 95), (center - 31, 90)], "#E7CA99")
    a.poly([(center - 44, 72), (center - 22, 53), (center + 9, 49),
            (center + 45, 63), (center + 36, 78), (center - 21, 83)], "#AEAC79")
    if roman:
        a.box((center - 46, 46, center + 52, 54), TERRA)
        for x in range(center - 45, center + 48, 18):
            a.line([(x, 74), (x, 62), (x + 3, 55), (x + 8, 53),
                    (x + 13, 55), (x + 16, 62), (x + 16, 74)], DEEP, 4)
    for n in range(3 + index % 3):
        x = center - 30 + n * 15
        y = 60 + rng.randrange(-7, 8)
        a.poly([(x, y), (x + 12, y - 6), (x + 13, y + 9), (x, y + 13)], CREAM)
        a.poly([(x - 2, y), (x + 4, y - 9), (x + 14, y - 6), (x + 12, y - 2)], TERRA if n % 3 else BLUE)
        a.box((x + 3, y + 4, x + 7, y + 9), SEA, 1)
    for n in range(5):
        x = center + rng.randrange(-41, 61)
        y = rng.randrange(72, 88)
        a.poly([(x, y - 14), (x + 4, y - 4), (x + 1, y + 1), (x - 4, y - 3)], DARK_LEAF)
    boat = (index * 13) % 80 + 136
    a.poly([(boat, 97), (boat + 17, 97), (boat + 11, 102), (boat + 4, 102)], DEEP)
    a.poly([(boat + 8, 80), (boat + 8, 96), (boat + 17, 96)], CREAM)
    a.save("island_" + str(index))


for i in range(48):
    island_card(i)

# A continuous fade lets the live 3D street show through the edge of the title panel.
fade = Image.new("RGBA", (384, 2))
for x in range(fade.width):
    alpha = round(240 * (1 - x / (fade.width - 1)) ** 1.55)
    for y in range(2):
        fade.putpixel((x, y), (11, 36, 40, alpha))
fade.save(OUT / "title_fade.png", optimize=True)
