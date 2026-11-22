"""Paint the expedition map art, inventory icons and new wardrobe previews."""
import math
import random

from make_ui import Art, BLUE, CREAM, GOLD, INK, LEAF, TERRA, arch, bread, house, jug, star


def bundle(a, plank=False):
    for x, y in [(21, 60), (35, 68), (38, 49)]:
        a.poly([(x, y), (x + 40, y - 35), (x + 53, y - 30), (x + 13, y + 9)], "#BD8053")
        a.poly([(x + 13, y + 9), (x + 53, y - 30), (x + 53, y - 17), (x + 13, y + 16)], "#925F43")
        if plank:
            a.line([(x + 6, y + 3), (x + 45, y - 32)], "#F3CB88", 3)
            a.poly([(x, y), (x + 13, y + 9), (x + 13, y + 16), (x, y + 8)], GOLD)
        else:
            a.oval((x - 5, y - 1, x + 16, y + 16), "#F0C083")
            a.oval((x + 2, y + 4, x + 10, y + 11), "#BD8053")


def stones(a, ore=False):
    for x, y, scale, tint in [(17, 63, 0.74, "#89949C"), (40, 46, 1.0, "#C2C8B8"), (63, 67, 0.67, "#989A94")]:
        def shape(points, color):
            a.poly([(x + dx * scale, y + dy * scale) for dx, dy in points], color)
        shape([(-13, 12), (-17, -5), (-8, -18), (15, -21), (29, -3), (24, 17), (5, 23)], tint)
        shape([(-13, 12), (-17, -5), (3, 3), (5, 23)], "#596D75")
        shape([(3, 3), (29, -3), (24, 17), (5, 23)], "#7B8A8E")
        if ore:
            shape([(-10, -7), (-5, -15), (9, -18), (17, -10), (12, 0), (2, 2)], "#F8AA71")
            shape([(17, -10), (19, -2), (10, 9), (2, 2), (12, 0)], "#C77A5C")


def brick(a, clay=False):
    base, light = ("#C18C68", "#E3B084") if clay else (TERRA, "#F09869")
    for x, y in [(12, 55), (47, 57), (30, 36)]:
        a.poly([(x, y), (x + 22, y - 11), (x + 43, y), (x + 21, y + 12)], light)
        a.poly([(x, y), (x + 21, y + 12), (x + 21, y + 28), (x, y + 16)], base)
        a.poly([(x + 21, y + 12), (x + 43, y), (x + 43, y + 16), (x + 21, y + 28)], "#98584B")
    if clay:
        for x, y in [(31, 62), (49, 54), (63, 66)]:
            a.oval((x, y, x + 4, y + 4), "#85594D")


def mountains(a, flag):
    a.poly([(8, 77), (31, 48), (52, 14), (85, 77)], "#779280")
    a.poly([(52, 14), (85, 77), (39, 77)], "#517561")
    a.poly([(33, 46), (52, 14), (66, 41), (56, 37), (51, 45), (43, 39)], CREAM)
    a.poly([(6, 78), (24, 64), (35, 73), (51, 63), (91, 78)], "#A9AF65")
    if flag:
        a.line([(66, 62), (75, 32)], INK, 5)
        a.poly([(75, 32), (93, 37), (71, 47)], GOLD)


def icon(name):
    a = Art()
    if name == "adventure":
        a.oval((4, 5, 92, 93), "#466970")
        a.oval((9, 8, 87, 85), "#579096")
        mountains(a, True)
    elif name == "hill":
        mountains(a, True)
    elif name in ("wood", "boards"):
        bundle(a, name == "boards")
    elif name in ("stone", "ore"):
        stones(a, name == "ore")
    elif name in ("clay", "bricks"):
        brick(a, name == "clay")
    elif name in ("food", "meals"):
        if name == "meals":
            a.oval((9, 27, 87, 80), "#C28A56")
            a.oval((12, 21, 84, 72), CREAM)
        else:
            a.line([(53, 45), (55, 21), (76, 16)], LEAF, 5)
        bread(a)
        a.oval((58, 55, 83, 77), "#BC5B56")
        a.oval((53, 52, 73, 71), "#E88474")
        a.poly([(62, 55), (64, 43), (75, 45), (68, 53)], LEAF)
    elif name == "tools":
        a.line([(28, 77), (65, 28)], "#995E47", 12)
        a.line([(28, 76), (61, 31)], "#D7A367", 6)
        a.poly([(40, 24), (54, 10), (84, 34), (74, 50)], "#A5BBB5")
        a.poly([(74, 50), (84, 34), (82, 41), (72, 58), (42, 34), (40, 24)], "#64858A")
    elif name == "blueprint":
        a.poly([(16, 21), (33, 16), (77, 29), (71, 70), (54, 83), (13, 72)], "#386C8E")
        a.line([(21, 23), (69, 36), (64, 69), (20, 66), (21, 23)], "#7FBDC3", 3)
        a.line([(29, 55), (43, 40), (61, 54), (58, 63), (32, 60), (29, 55)], CREAM, 3)
        a.line([(17, 22), (16, 71), (29, 75)], GOLD, 5)
        a.line([(77, 27), (75, 70)], BLUE, 6)
    elif name == "labor":
        for x, y, color in [(29, 33, "#C27D5C"), (68, 35, "#D2AB76"), (48, 44, "#EFB387")]:
            a.oval((x - 15, y + 9, x + 15, y + 35), BLUE if x == 48 else LEAF)
            a.oval((x - 11, y - 11, x + 11, y + 12), color)
            a.poly([(x - 12, y), (x - 11, y - 9), (x, y - 14), (x + 10, y - 7), (x + 12, y + 1), (x + 2, y - 4)], "#634940")
    elif name == "craft":
        jug(a, -9)
        jug(a, 13, "#6CB7B2")
    elif name == "cloth":
        a.poly([(12, 43), (50, 19), (80, 31), (75, 63), (34, 84), (14, 72)], "#AA6485")
        a.poly([(14, 43), (32, 46), (60, 31), (76, 33), (33, 60), (13, 57)], "#E5A57F")
        for x in range(21, 42, 5):
            a.line([(x, 67), (x - 5, 76)], CREAM, 2)
    elif name in ("sawmill", "brickyard", "forge"):
        house(a, {"sawmill": "#759167", "brickyard": "#C05E50", "forge": "#5E7893"}[name])
        if name == "sawmill":
            a.oval((56, 49, 86, 80), "#8FA8A5")
            a.oval((63, 56, 79, 73), INK)
            a.oval((68, 61, 74, 67), GOLD)
        else:
            a.box((72, 23, 84, 55), "#845D53", 2)
            a.box((69, 21, 87, 28), CREAM, 2)
            a.poly([(49, 72), (43, 64), (50, 54), (52, 62), (55, 64), (58, 60), (60, 69), (55, 75)], GOLD)
    a.save(f"adventure_{name}" if name != "adventure" else name)


def cover(index):
    a = Art(480, 190)
    rnd = random.Random(193 + index)
    tables = [
        ("#AED4C6", "#477F86", "#D2BE85", "#819666", "#648460", "#CB7357", "#F8D584"),
        ("#BBCDBE", "#598895", "#D2A674", "#889361", "#777951", "#B7604A", "#F1CB81"),
        ("#B4C9CB", "#5B8792", "#D2BC94", "#71946D", "#4D775C", "#A95148", "#F2C684"),
        ("#ADBFC2", "#4B7382", "#A9AB8A", "#607D6B", "#496C63", "#7D594B", "#F2D197"),
        ("#EDC69D", "#4E9AA0", "#DEAE75", "#C5885E", "#A26C58", "#7563A1", "#FFE0A5"),
    ]
    sky, sea, shore, grass, stone, roof, sun = tables[index]
    a.box((0, 0, 480, 190), sky)
    a.oval((369, 13, 414, 58), sun)
    a.box((0, 88, 480, 190), sea)
    for j in range(22):
        x = rnd.randrange(10, 480)
        y = rnd.randrange(90, 189)
        a.line([(x, y), (x + rnd.randint(12, 43), y - 1)], "#86AA9B", 1)
    a.poly([(17, 133), (65, 103), (106, 97), (138, 67), (197, 75), (264, 52), (316, 68), (369, 67), (433, 100),
            (453, 138), (424, 162), (335, 172), (259, 166), (190, 185), (103, 168), (56, 160)], shore)
    a.poly([(27, 130), (65, 105), (139, 70), (195, 77), (265, 55), (312, 70), (366, 69), (432, 103),
            (449, 127), (407, 145), (328, 151), (262, 149), (191, 174), (105, 158), (54, 151)], grass)
    for x, y, w, h in [(82, 121, 57, 60), (137, 109, 58, 72), (221, 119, 83, 95), (296, 113, 66, 70), (368, 129, 47, 49)]:
        a.poly([(x - w, y), (x - 12, y - h * 0.67), (x + 9, y - h), (x + 32, y - h * 0.42), (x + w, y)], stone)
        a.poly([(x + 9, y - h), (x + 32, y - h * 0.42), (x + w, y), (x + 3, y + 4)], grass)
        if index == 3:
            a.poly([(x - 18, y - h * 0.6), (x + 9, y - h), (x + 29, y - h * 0.46), (x + 13, y - h * 0.56), (x + 3, y - h * 0.53)], CREAM)
    a.line([(170, 180), (186, 154), (209, 142), (266, 145), (312, 126), (358, 123)], "#E1C99B", 5)
    for x, y in [(46, 139), (115, 138), (131, 154), (144, 116), (270, 122), (350, 111), (389, 125), (409, 132), (285, 151)]:
        if index == 4:
            a.line([(x, y), (x + 2, y - 22)], "#865D4F", 4)
            for offset in [-12, -6, 3, 11]:
                a.poly([(x + 2, y - 23), (x + offset, y - 26), (x + offset * 1.1, y - 16), (x + 2, y - 19)], "#568072")
        else:
            for j in range(1, 3):
                a.poly([(x - j * 6, y + j * 4 - 13), (x - 2, y - 29 + j * 3), (x + j * 5, y + j * 4 - 13)], "#4F755B")
    building = Art()
    if index == 0:
        arch(building)
    else:
        house(building, roof, BLUE)
    for x, y, size in [(199, 108, 58), (234, 127, 42), (319, 112, 35)]:
        inset = building.canvas.resize((size * 3, size * 3))
        a.canvas.paste(inset, (x * 3, y * 3), inset)
    star(a, sun, (437, 49), 7)
    a.save(f"adventure_cover_{index}")


CLOTHES = [
    ("aegean_chiton", "garment", "#408A99", "#E6C288"),
    ("olive_peplos", "garment", "#8E9B59", "#E6CA89"),
    ("myrtle_crown", "head", "#609477", "#F0D8A3"),
    ("folded_petasos", "head", "#5D7FA5", "#E0B87C"),
    ("shell_pin", "accessory", "#EDCDB9", "#82B4B7"),
    ("mulberry_toga", "garment", "#9B5575", "#F5D4AA"),
    ("aventin_tunic", "garment", "#D4A35B", "#42847B"),
    ("red_pileus", "head", "#A95851", "#F2C28C"),
    ("gold_laurel", "head", "#CB9C50", "#F9DF8E"),
    ("signet_cord", "accessory", "#CE9657", "#AE675C"),
    ("sakura_kosode", "garment", "#B46987", "#F8C9AD"),
    ("mino_cape", "garment", "#AB8456", "#EFD397"),
    ("river_kasa", "head", "#809475", "#BED8AE"),
    ("crane_pin", "head", "#F1DBB0", "#B67B62"),
    ("rice_pouch", "accessory", "#567890", "#DFA676"),
    ("sea_cloak", "garment", "#517586", "#D6BE9A"),
    ("tablet_dress", "garment", "#7D525F", "#E3B486"),
    ("fur_cap", "head", "#9B7054", "#E5C2A1"),
    ("knit_band", "head", "#796C9D", "#E0B889"),
    ("knot_pendant", "accessory", "#D1AE71", "#5B8A88"),
    ("oasis_caftan", "garment", "#368A81", "#F1C072"),
    ("moon_silk", "garment", "#78638F", "#E9CC98"),
    ("star_turban", "head", "#445B94", "#F3CC77"),
    ("caravan_veil", "head", "#CA825D", "#F9DFAC"),
    ("rose_flask", "accessory", "#B47591", "#E8BD79"),
]


def wardrobe_icon(name, category, base, trim):
    a = Art()
    a.oval((9, 72, 88, 85), "#315154")
    if category == "garment":
        long_sleeve = name in ("sakura_kosode", "sea_cloak", "oasis_caftan", "moon_silk", "mino_cape")
        sleeve = 86 if long_sleeve else 74
        a.poly([(29, 26), (39, 18), (57, 18), (68, 25), (sleeve, 53), (73, 61), (65, 47),
                (73, 79), (22, 79), (31, 48), (96 - sleeve, 57), (15, 49)], base)
        a.poly([(33, 38), (46, 59), (56, 76), (71, 79), (23, 79)], (*tuple(bytes.fromhex(base[1:])), 189))
        a.line([(27, 75), (71, 75)], trim, 5)
        a.line([(35, 25), (47, 46), (59, 23)], trim, 5)
        if "toga" in name or "chiton" in name:
            a.line([(27, 65), (64, 43), (66, 31)], trim, 7)
        if name in ("aventin_tunic", "olive_peplos", "tablet_dress", "oasis_caftan"):
            for x in range(37, 66, 9):
                a.poly([(x-4, 60), (x, 55), (x+4, 60), (x, 64)], trim)
        if name in ("sakura_kosode", "moon_silk"):
            for x, y in [(31, 57), (59, 63), (43, 70)]:
                star(a, trim, (x, y), 6)
        if name == "mino_cape":
            for x in range(24, 76, 6):
                a.line([(46 + (x - 46) * 0.57, 25), (x, 68)], trim, 2)
        if name == "sea_cloak":
            a.line([(26, 37), (35, 28), (48, 30), (62, 29), (71, 36)], trim, 9)
    elif name in ("myrtle_crown", "gold_laurel", "knit_band"):
        for step in range(18):
            angle = step / 18 * math.tau
            x = 48 + math.cos(angle) * 30
            y = 53 + math.sin(angle) * 17
            a.oval((x-7, y-4, x+7, y+4), base)
            if name == "knit_band" or step % 2 == 0:
                a.poly([(x-4, y-1), (x+5, y-7), (x+8, y-9), (x+7, y-2), (x+2, y+3)], trim)
    elif name in ("folded_petasos", "river_kasa"):
        a.oval((9, 55, 87, 73), trim)
        a.poly([(12, 57), (45, 28), (54, 28), (85, 58), (70, 64), (27, 64)], base)
        for x in range(23, 82, 12):
            a.line([(48, 30), (x, 60)], trim, 2)
        if name == "folded_petasos":
            a.poly([(65, 58), (74, 47), (88, 40), (84, 60), (75, 66)], trim)
    elif name == "crane_pin":
        a.line([(27, 82), (60, 34)], trim, 6)
        a.poly([(43, 51), (33, 42), (26, 23), (51, 33), (61, 37),
                (71, 17), (79, 23), (75, 35), (81, 39), (69, 42), (63, 53)], base)
        a.line([(59, 44), (38, 36), (30, 28)], trim, 3)
    elif category == "head":
        if name == "red_pileus":
            a.poly([(18, 68), (26, 45), (46, 21), (56, 17), (67, 26), (70, 52), (80, 69)], base)
            a.oval((17, 63, 81, 75), trim)
        elif name == "fur_cap":
            a.oval((18, 23, 79, 69), base)
            a.box((17, 53, 79, 70), trim, 3)
            a.oval((16, 61, 34, 84), trim)
            a.oval((62, 61, 81, 84), trim)
        else:
            a.oval((15, 28, 82, 67), base)
            a.line([(21, 56), (39, 39), (57, 30)], trim, 5)
            a.line([(43, 64), (60, 49), (76, 39)], trim, 4)
            if name == "star_turban":
                star(a, trim, (49, 54), 13)
            else:
                a.poly([(15, 56), (31, 65), (27, 83), (16, 80)], trim)
                a.poly([(70, 65), (81, 51), (86, 76), (72, 80)], trim)
    elif name == "shell_pin":
        a.oval((20, 31, 76, 76), trim)
        for x in range(23, 78, 10):
            a.line([(48, 64), (x, 42 - abs(x - 48) * 0.28)], base, 5)
    else:
        a.line([(23, 22), (35, 43), (49, 57), (64, 39), (73, 20)], trim, 5)
        if name == "rice_pouch":
            a.box((28, 52, 69, 78), base, 9)
            a.line([(28, 57), (63, 58), (70, 54)], trim, 5)
        elif name == "rose_flask":
            a.oval((32, 49, 69, 80), base)
            a.box((44, 39, 57, 51), trim, 2)
            star(a, trim, (50, 65), 9)
        elif name == "knot_pendant":
            for x, y in [(38, 60), (60, 60), (49, 75)]:
                a.oval((x-10, y-10, x+10, y+9), base)
                a.oval((x-5, y-5, x+5, y+5), trim)
        else:
            a.oval((28, 45, 70, 81), base)
            a.poly([(48, 49), (63, 64), (48, 77), (36, 62)], trim)
    a.save(f"wear_{name}")


if __name__ == "__main__":
    for key in ("adventure wood stone clay ore food boards bricks tools blueprint labor hill craft cloth meals sawmill brickyard forge").split():
        icon(key)
    for region in range(5):
        cover(region)
    for item in CLOTHES:
        wardrobe_icon(*item)
