"""Building and night-market symbols for Nililane's expanded catalog."""
from math import cos, sin, tau

from make_ui import Art, INK, CREAM, GOLD, TERRA, DEEP, SEA, BLUE, DARK_LEAF, LEAF


def shelter(a, tile=BLUE):
    a.box((23, 42, 72, 75), CREAM, 2)
    a.poly([(14, 46), (29, 29), (68, 29), (82, 46)], tile)
    a.line([(16, 47), (79, 47)], INK, 3)
    for x in [28, 67]:
        a.line([(x,49),(x,76)], DEEP, 5)


def jar(a, x, y, color, size=1.):
    a.poly([(x-7*size,y-9*size),(x+7*size,y-9*size),(x+13*size,y),
            (x+11*size,y+12*size),(x-9*size,y+12*size),(x-13*size,y)],color)
    a.box((x-7*size,y-11*size,x+7*size,y-7*size),DEEP,2)
    a.line([(x-9*size,y),(x-6*size,y+8*size)], CREAM, 2)


def canopy(a):
    a.line([(20, 75),(20, 37),(75, 37),(75, 75)],DEEP,4)
    a.poly([(15, 36),(25, 29),(70, 29),(81,36),(78,42),(16,42)],DARK_LEAF)


def draw(key):
    a = Art()
    if key == "moon":
        a.oval((17, 15, 79, 79), "#58658b")
        a.oval((23, 19, 74, 72), "#e7c886")
        a.oval((38, 11, 85, 60), "#58658b")
        for x,y in [(60,27),(71,19),(73,40)]:
            a.line([(x-3,y),(x+3,y)],CREAM,2)
            a.line([(x,y-3),(x,y+3)],CREAM,2)
        a.save(key)
        return
    a.oval((12, 10, 84, 84), "#e6ddbc")
    a.oval((15, 17, 81, 79), "#f0e7ce")
    a.oval((14, 77, 82, 86), "#cdc4a1")
    if key == "apiary":
        for x,y in [(31,60),(63,65)]:
            a.oval((x-14,y-15,x+14,y+13), GOLD)
            for off in [-8,-2,4]:
                a.line([(x-10,y+off),(x+10,y+off)],DEEP,2)
            a.oval((x-3,y+6,x+3,y+10),INK)
        a.line([(44, 68),(44, 77),(70,77)], DARK_LEAF)
    elif key == "dyer":
        canopy(a)
        for n,color in enumerate([TERRA,BLUE,"#835d9a"]):
            x=26+n*16
            a.poly([(x,43),(x+11,44),(x+13,68),(x,65)],color)
        for x,c in [(31,TERRA),(63,BLUE)]:
            a.oval((x-12,68,x+12,80),DEEP)
            a.oval((x-10,69,x+10,74),c)
    elif key == "ferry_office":
        shelter(a,SEA)
        a.box((53,50,72,73),BLUE,3)
        a.line([(69,34),(69,19),(77,24),(69,26)],TERRA,3)
        a.poly([(23,68),(48,68),(45,77),(28,77)],DEEP)
        a.poly([(36,46),(43,64),(30,64)],GOLD)
    elif key == "mill":
        a.oval((14,62,78,80),DEEP)
        a.oval((16,54,76,72),"#bbab96")
        a.oval((27,45,66,65), "#887869")
        a.line([(46,51),(46,36),(70,30)],DEEP,5)
        for x in [48,59,70]:
            a.line([(x-8,68),(x,57)],GOLD,3)
    elif key == "scribe":
        shelter(a,TERRA)
        a.box((29, 53, 67, 78), DEEP, 3)
        a.box((33, 56, 63, 74), GOLD, 2)
        for y in [60,65,70]:
            a.line([(39,y),(55,y)],DEEP,1)
        a.line([(51,64),(68,47)],CREAM,3)
    elif key == "garden_court":
        canopy(a)
        a.oval((20,61,76,81),INK)
        a.oval((25,64,71,77),SEA)
        a.line([(48, 74), (48,51),(39,57)],BLUE,3)
        for x in [25,73]:
            a.oval((x-9,45,x+9,67),LEAF)
    elif key == "market_garden":
        for n in range(3):
            y=38+n*13
            a.poly([(24,y),(69,y-4),(79,y+5),(16,y+10)],DEEP)
            for i in range(5):
                x=23+11*i
                a.oval((x-4,y-6,x+6,y+3),[DARK_LEAF,LEAF][(n+i)%2])
                a.oval((x+1,y-6,x+3,y-2),TERRA if n==1 else GOLD)
    elif key == "ceramic_studio":
        shelter(a,SEA)
        jar(a,30,66,BLUE,.82)
        jar(a,61,61,SEA,1.02)
        a.oval((43,67,54,76),CREAM)
    elif key == "silk_shop":
        shelter(a,"#695b9d")
        for n,color in enumerate(["#b276a7",SEA,TERRA]):
            x=28+n*15
            a.poly([(x,49),(x+11,49),(x+12,76),(x,73)],color)
            a.line([(x+2,56),(x+9,62)],CREAM,1)
    elif key == "sheepfold":
        for x,y,s in [(28,63,1.),(51,51,1.1),(67,70,.75)]:
            a.oval((x-12*s,y-8*s,x+11*s,y+7*s),CREAM)
            a.oval((x+5*s,y-4*s,x+14*s,y+5*s),DEEP)
            for dx in [-6,4]:
                a.line([(x+dx*s,y+5*s),(x+dx*s,y+10*s)],DEEP,2)
    elif key == "cooperage":
        shelter(a,TERRA)
        jar(a,38,65,GOLD,1.12)
        jar(a,63,67,"#b8875c",.93)
        for x,y in [(38,65),(63,66)]:
            for d in [-4,6]:
                a.line([(x-9,y+d),(x+9,y+d)],INK,2)
    elif key == "signal_tower":
        a.poly([(29,80),(36,38),(60,38),(69,80)],DEEP)
        for y in [46,57,68]:
            a.line([(34,y),(63,y)],GOLD,3)
        a.box((26,33,71,40),INK,2)
        a.poly([(37,32),(40,21),(53,13),(51,25),(62,21),(59,35)],TERRA)
        a.poly([(45,34),(52,23),(56,31),(54,35)],GOLD)
    a.save(key)


if __name__ == "__main__":
    for name in ["moon", "apiary", "dyer", "ferry_office", "mill", "scribe", "garden_court",
                 "market_garden", "ceramic_studio", "silk_shop", "sheepfold", "cooperage", "signal_tower"]:
        draw(name)
