"""Draw the new Nililane icons in the same palette as the rest of the HUD."""

from make_ui import Art, INK, CREAM, GOLD, TERRA, DEEP, SEA, BLUE, LEAF, DARK_LEAF, bread


def portrait(name, skin, hair, robe, child=False, elder=False):
    a = Art()
    if child:
        a.oval((15, 13, 82, 81), "#D4A061")
        a.oval((25, 21, 73, 69), hair)
        a.oval((28, 29, 70, 67), skin)
        a.poly([(25, 40), (32, 23), (58, 23), (70, 33), (52, 38), (46, 33), (30, 46)], hair)
        a.box((22, 67, 74, 89), robe, 9)
    else:
        a.oval((18, 11, 80, 75), hair)
        a.oval((23, 21, 75, 71), skin)
        a.poly([(22, 42), (24, 25), (37, 17), (62, 20), (76, 36),
                (67, 33), (42, 28), (28, 44)], hair)
        a.box((15, 68, 81, 89), robe, 10)
    a.box((43, 62, 55, 73), skin, 4)
    a.oval((37, 46, 43, 52), INK)
    a.oval((56, 46, 62, 52), INK)
    a.poly([(30, 86), (37, 69), (51, 75), (38, 87)], CREAM)
    if elder:
        a.line([(31, 45), (43, 45), (46, 51), (53, 51), (56, 45), (67, 45)], BLUE, 2)
        a.poly([(31, 53), (44, 63), (52, 63), (66, 53), (62, 68),
                (49, 76), (37, 69)], CREAM)
    else:
        a.line([(44, 58), (49, 61), (54, 58)], DEEP, 2)
    a.save(name)


portrait("age_child", "#D79C70", "#593B36", "#6AA98C", child=True)
portrait("age_adult", "#BB845E", "#403839", "#6494B6")
portrait("age_senior", "#D3A47B", "#DCD5BA", "#B37786", elder=True)


def pet(name, cat):
    a = Art()
    coat = "#D99253" if cat else "#B87B59"
    if cat:
        for x in (14, 57):
            a.poly([(x, 50), (x + 4, 9), (x + 26, 34), (x + 27, 56)], coat)
            a.poly([(x + 7, 36), (x + 8, 18), (x + 21, 34)], "#E3A3A3")
    else:
        a.box((8, 35, 29, 75), DEEP, 9)
        a.box((67, 35, 88, 75), DEEP, 9)
    a.oval((17, 23, 79, 79), coat)
    a.oval((27, 49, 69, 79), "#F7D39C")
    for x in (32, 56):
        a.oval((x, 45, x + 8, 55), INK)
        a.oval((x + 3, 46, x + 6, 49), CREAM)
    a.poly([(41, 58), (55, 58), (48, 65)], "#A9656B" if cat else INK)
    a.line([(34, 66), (41, 70), (48, 65), (55, 70), (62, 66)], DEEP, 2)
    if cat:
        for y in (58, 64, 70):
            a.line([(12, y - 4), (30, y), (38, y)], CREAM, 2)
            a.line([(58, y), (66, y), (84, y - 4)], CREAM, 2)
    else:
        a.box((30, 76, 67, 84), SEA, 3)
        a.oval((44, 78, 53, 90), GOLD)
    a.save(name)


pet("cat", True)
pet("dog", False)

a = Art()
for x, y, c in [(30, 60, TERRA), (54, 22, GOLD), (67, 57, LEAF)]:
    a.poly([(x-13,y+12),(x-17,y-4),(x-6,y-15),(x+20,y-17),(x+15,y+2),(x+1,y+17)], c)
    a.line([(x-17,y+17),(x+15,y-11)], DEEP, 2)
a.save("leaves")

a = Art()
a.poly([(13, 51), (24, 21), (63, 22), (80, 42), (69, 77), (30, 84)], "#CFB990")
a.poly([(24, 21), (63, 22), (74, 40), (56, 52), (48, 74), (18, 51)], CREAM)
a.line([(24, 38), (50, 49), (58, 26)], "#B69365", 3)
a.poly([(58, 64), (74, 53), (88, 59), (86, 75), (66, 82)], TERRA)
a.save("scraps")

a = Art()
a.poly([(10, 52), (47, 31), (86, 43), (85, 70), (55, 87), (13, 73)], TERRA)
a.poly([(10, 52), (47, 31), (86, 43), (53, 64)], "#9D694C")
for x, y in [(28, 50), (49, 49), (65, 44)]:
    a.line([(x, y+6), (x+1, y-15), (x+5, y-27)], DARK_LEAF, 4)
    a.oval((x-10,y-22,x+4,y-12), LEAF)
    a.oval((x+1,y-18,x+17,y-9), "#A4B780")
a.save("herb_garden")

for name, cloth, edge in [
    ("olive", "#789657", "#DDC879"), ("linen", "#E8D1A5", "#A85A6D"),
    ("coral", "#CB696A", "#FFCD9C"), ("night", "#58659E", "#B8C9E3"),
    ("mosaic", "#5DAF9F", "#EBAD72"),
]:
    a = Art()
    a.poly([(20, 29),(36, 13),(52, 17),(65, 16),(80, 34),(72, 53),(67, 50),
            (77, 79),(54, 87),(20, 82),(26, 52),(17, 53),(12, 39)], cloth)
    a.poly([(32, 22),(43, 42),(66, 30),(71, 38),(46, 56),(30, 65),(26, 58),(38, 48)], edge)
    a.line([(24, 73), (43, 78), (67, 74)], edge, 3)
    a.oval((31, 22, 43, 34), GOLD)
    a.save("wear_" + name)

for name, c, trim in [
    ("sunbonnet", "#EAC67A", "#CB6D87"), ("indigo", "#7C8FCD", "#505F9C"),
    ("diadem", GOLD, SEA), ("wreath", LEAF, "#D4839E"), ("headband", "#BF686D", CREAM),
]:
    a = Art()
    if name in ("sunbonnet", "indigo"):
        a.oval((8, 46, 89, 76), trim)
        a.poly([(21, 52), (27, 24), (48, 17), (71, 28), (78, 54), (58, 64), (36, 60)], c)
        a.line([(23, 52), (44, 59), (67, 55), (77, 50)], trim, 6)
    else:
        a.line([(12, 51),(27, 63),(49, 69),(69, 64),(84, 51)], c, 10)
        for i, x in enumerate((20,33,47,62,76)):
            y = 53 + 10 * (i in (1,2,3))
            a.oval((x-7,y-9,x+7,y+4), trim)
        if name == "diadem":
            a.poly([(34,60),(49,32),(64,60),(48,69)], c)
            a.poly([(42,58),(49,43),(58,57),(49,65)], trim)
    a.save("wear_" + name)

for name, c in [
    ("satchel", "#B88256"), ("shell", "#E9A99B"), ("pearls", CREAM), ("brooch", SEA),
    ("scarf", "#CB698A"), ("belt", "#C4904D"), ("basket", "#D7A767"), ("amulet", "#E8A257"),
]:
    a = Art()
    if name in ("satchel", "basket"):
        a.line([(21, 53), (23, 29), (44, 14), (62, 24), (70, 51)], DEEP, 6)
        a.poly([(14,46),(83,46),(75,79),(48,86),(21,79)], c)
        a.line([(19,55), (47,64), (78,54)], GOLD, 5)
        if name == "basket":
            for x in (29, 43, 58, 68):
                a.line([(x,56), (x-2,79)], DEEP, 2)
        else:
            a.box((42,59,54,72), GOLD, 2)
    elif name in ("scarf", "belt"):
        a.poly([(11,34),(28,48),(65,45),(85,31),(80,49),(61,66),(42,63),(28,67),(16,54)], c)
        a.oval((46,45,63,63), GOLD)
        if name == "scarf":
            a.poly([(49,52),(41,78),(24,88),(36,57)], c)
        else:
            a.box((37,45,55,63), CREAM, 4)
    elif name == "brooch":
        a.oval((21,21,75,75), GOLD)
        a.poly([(48,26),(69,48),(48,69),(29,48)], c)
        a.poly([(48,26),(48,69),(29,48)], "#82C3AA")
    else:
        points = [(13,31),(20,47),(31,60),(47,67),(63,60),(76,48),(83,31)]
        a.line(points, "#BC885C", 4)
        for x,y in points:
            a.oval((x-4,y-4,x+4,y+4), CREAM if name == "pearls" else GOLD)
        a.poly([(35,68),(47,58),(63,67),(60,79),(47,88),(34,80)], c)
        a.line([(37,72),(46,64),(56,71)], CREAM, 3)
    a.save("wear_" + name)
