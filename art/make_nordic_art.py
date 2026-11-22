"""Bake the Nordic campaign's chart paintings, icons and departure sound."""
from math import cos, exp, pi, sin
from pathlib import Path
from random import Random
from struct import pack
import wave

from make_ui import Art

INK = "#3d5259"
DARK = "#59463b"
OAK = "#9a7455"
GOLD = "#e6b578"
RED = "#b96752"
BLUE = "#597f92"
MOSS = "#829766"
LINEN = "#f3e0b3"
FROST = "#cee0d9"


def house(a, x, y, w, front=OAK, sod=MOSS):
    a.box((x-w*.82, y-3, x+w*.83, y+4), "#9e9e8c", 2)
    a.box((x-w*.74, y-w*.52, x+w*.72, y-2), front, 2)
    for dx in [-.63, -.43, -.22, 0, .20, .41, .61]:
        a.line([(x+dx*w, y-w*.50), (x+dx*w, y-1)], DARK, 1)
    roof = [(x-w, y-w*.47), (x-w*.65, y-w*.72), (x-w*.40, y-w*.90),
            (x+w*.36, y-w*.89), (x+w*.72, y-w*.71), (x+w, y-w*.48),
            (x+w*.55, y-w*.39), (x-w*.55, y-w*.39)]
    a.poly(roof, sod)
    a.line([(x-w*.95, y-w*.48), (x-w*.53, y-w*.40), (x+w*.55, y-w*.40),
            (x+w*.94, y-w*.50)], DARK, 2)
    a.line([(x-w*.36, y-w*.87), (x+w*.35, y-w*.86)], "#a9b181", 2)
    a.box((x-w*.20, y-w*.30, x+w*.18, y), DARK, 1)
    for dx in [-.48, .49]:
        a.box((x+(dx-.10)*w, y-w*.29, x+(dx+.10)*w, y-w*.16), GOLD, 1)


def pine(a, x, foot, h, foliage=BLUE, frost=False):
    a.line([(x, foot-h*.75), (x, foot+1)], DARK, 1)
    for i in [1.0, .77, .54]:
        a.poly([(x, foot-h*i), (x+h*(.32-i*.10), foot-h*(i-.40)),
                (x, foot-h*(i-.45)), (x-h*(.35-i*.11), foot-h*(i-.40))], foliage)
        if frost:
            a.poly([(x, foot-h*i), (x+h*.095, foot-h*(i-.20)),
                    (x-h*.083, foot-h*(i-.23))], FROST)


def fish(a, x, y, w, color):
    a.poly([(x-w*.28,y), (x-w*.68,y-w*.29), (x-w*.67,y+w*.23), (x-w*.28,y+w*.09),
            (x+w*.05,y+w*.29), (x+w*.57,y+w*.19), (x+w*.74,y+w*.04),
            (x+w*.53,y-w*.20), (x+w*.09,y-w*.26)], color)
    a.oval((x+w*.42, y-w*.1, x+w*.53, y+w*.018), INK)
    a.line([(x-w*.23,y+w*.065), (x+w*.35,y+w*.12)], LINEN, 1)


def boat(a, x, y, s, cloth=RED, sail=True):
    a.poly([(x-s, y-s*.21), (x-s*.67,y-s*.05), (x+s*.60,y-s*.05),
            (x+s,y-s*.26), (x+s*.78,y+s*.13), (x+s*.53,y+s*.24),
            (x-s*.54,y+s*.22), (x-s*.82,y+s*.13)], DARK)
    a.line([(x-s*.87,y-s*.12), (x-s*.59,y+s*.02), (x+s*.61,y+s*.02),
            (x+s*.88,y-s*.13)], GOLD, 2)
    if sail:
        a.line([(x, y), (x,y-s*1.63)], OAK, 2)
        a.poly([(x-s*.66,y-s*1.40), (x+s*.60,y-s*1.38),
                (x+s*.68,y-s*.43), (x-s*.58,y-s*.43)], LINEN)
        for dx in [-.45, -.07, .34]:
            a.poly([(x+(dx-.14)*s,y-s*1.39), (x+(dx+.06)*s,y-s*1.39),
                    (x+(dx+.14)*s,y-s*.43), (x+(dx-.07)*s,y-s*.43)], cloth)
        a.line([(x-s*.73,y-s*1.38), (x+s*.68,y-s*1.38)], DARK, 2)


def artwork(name):
    a = Art()
    a.oval((12,10,84,83), "#dadfbe")
    a.oval((16,15,80,80), "#eef0d1")
    a.oval((12,76,84,87), "#aebd9e")
    if name == "longhouse":
        house(a, 48, 76, 40)
    elif name == "fishery":
        house(a, 34, 71, 25, RED, BLUE)
        a.line([(49, 75), (49, 45), (82, 45), (82, 75)], DARK, 4)
        for k in range(3):
            fish(a, 57+k*10, 57+k%2*4, 7, BLUE)
        a.line([(18, 81), (77, 81)], BLUE, 3)
    elif name == "smokehouse":
        house(a, 44, 75, 35, DARK, INK)
        a.box((64, 31, 74, 63), "#8b8f80", 2)
        a.box((61, 29, 77, 34), DARK, 1)
        a.line([(69,24), (68,18), (73,10)], "#9aa396", 2)
        fish(a, 48, 72, 18, RED)
    elif name == "woodlot":
        a.line([(16, 76), (16, 41), (51, 32), (81, 45), (81, 77)], DARK, 4)
        a.poly([(12, 42), (48, 27), (85, 43), (82, 47), (47, 34), (15, 47)], MOSS)
        for row in range(3):
            for col in range(4-row):
                x, y = 27+col*14+row*6, 70-row*12
                a.oval((x-8, y-8, x+8, y+8), DARK)
                a.oval((x-6, y-6, x+6, y+6), GOLD)
                a.oval((x-3, y-3, x+3, y+3), OAK)
    elif name == "boatyard":
        a.line([(20,74),(20,50),(49,31),(76,49),(76,74)], DARK, 6)
        a.poly([(13, 48),(48,25),(85,49),(80,53),(48,33),(17,53)], BLUE)
        boat(a, 49, 74, 30, sail=False)
    elif name == "sea_trade":
        a.line([(11,79),(33,78),(45,82),(66,78),(86,78)], BLUE, 3)
        boat(a,48,69,33)
    elif name == "weaver":
        a.line([(20,78),(20,30),(76,30),(76,79)], DARK, 5)
        for n,c in enumerate([RED, BLUE, MOSS]):
            x=24+n*17
            a.box((x,35,x+14,72+n%2*6), c, 1)
            a.line([(x+6,41),(x+9,49),(x+6,56),(x+2,49),(x+6,41)], LINEN, 1)
    elif name == "feast_hall":
        house(a, 48, 75, 42, RED)
        a.line([(16, 61), (14, 49), (21, 38)], OAK, 4)
        a.line([(78, 61), (81, 49), (73, 38)], OAK, 4)
        a.oval((39, 63, 57, 79), RED)
        a.poly([(44,75), (52,59), (52,68), (58,77), (40,77)], GOLD)
    elif name == "smoked_fish":
        fish(a,48,53,42,RED)
        a.line([(33,31),(32,24),(38,16)], "#95a79c", 2)
        a.line([(55,30),(53,25),(59,17)], "#95a79c", 2)
    elif name == "woven_cloth":
        a.poly([(19,36),(65,27),(80,37),(66,77),(24,72),(32,58)], BLUE)
        a.line([(25,45),(63,36),(73,42),(63,70),(28,66)], LINEN, 3)
        a.poly([(49,44),(58,51),(50,61),(42,53)], GOLD)
    elif name == "hearth_bowl":
        a.poly([(19,54),(30,76),(65,77),(79,55)], DARK)
        a.oval((17,45,80,64), OAK)
        a.oval((20,47,77,60), "#d19055")
        for x,y,col in [(32,54,GOLD),(48,51,LINEN),(57,55,MOSS),(63,52,RED)]:
            a.oval((x-4,y-2,x+4,y+2), col)
        a.line([(44,39),(40,33),(46,25)], "#9fae9e",2)
    elif name.startswith("wear_"):
        if name in {"wear_fjord","wear_krapp","wear_winter"}:
            color={"wear_fjord":BLUE,"wear_krapp":RED,"wear_winter":MOSS}[name]
            a.poly([(38,25),(49,29),(59,25),(74,28),(80,53),(66,60),(66,78),
                    (28,78),(29,60),(15,51),(22,30)], color)
            a.line([(38,26),(48,36),(59,26)], LINEN, 4)
            a.box((29,55,66,61), DARK, 2)
            for x in [38,59]:
                a.oval((x-5,36,x+5,46), GOLD)
        elif name == "wear_wool":
            a.poly([(36,25),(58,20),(71,35),(71,58),(80,70),(54,78),(27,74),
                    (18,63),(28,53),(28,38)], "#8a7ca2")
            a.oval((35,34,61,59), DARK)
            a.line([(30,58),(43,66),(56,66),(68,56)], LINEN, 3)
        elif name == "wear_sailor":
            a.poly([(19,63),(23,43),(36,32),(59,31),(74,47),(78,63)], BLUE)
            a.box((17,57,80,69), OAK, 3)
            for x in [33,48,62]:
                a.line([(47,32),(x,58)], LINEN, 2)
        elif name == "wear_braid":
            for i in range(11):
                x=20+i*5.7
                a.poly([(x,44),(x+10,50),(x,61),(x-6,55)], [RED,GOLD,LINEN][i%3])
        elif name == "wear_amber_brooch":
            a.oval((25,27,71,75), GOLD)
            a.oval((32,33,64,69), DARK)
            a.poly([(48,34),(62,46),(58,62),(35,62),(33,47)], "#d68e51")
            a.line([(37,49),(48,38),(57,48)], LINEN, 3)
        elif name == "wear_woven_bag":
            a.line([(30,55),(38,23),(66,30),(67,51)], OAK, 5)
            a.box((26,47,70,78), BLUE, 6)
            for i in range(5):
                a.line([(28,51+i*5),(66,56+i*5)], [GOLD,LINEN][i%2], 1)
            a.oval((44,51,56,63), RED)
    a.save(name)


def paint(index):
    a = Art(230, 112)
    r = Random(90547+index*551)
    climate=index//6
    sky,sea,far,near,soil,plants = [
        ("#b9d2cf","#779a9c","#8ea8a4","#779896","#b6aa8a","#617f70"),
        ("#c2d8d6","#7394a1","#9cb4b8","#74959c","#c2ccc1","#58796e"),
        ("#cdd4ba","#7a9fa0","#9aaf9e","#839989","#afaa86","#547e6a"),
        ("#d7cbb4","#7f9b9c","#b1b2a0","#96a59c","#c2aa86","#758975"),
    ][climate]
    a.box((0,0,230,112), sea)
    a.box((0,0,230,40), sky)
    moon=35+index*21%168
    a.oval((moon,7,moon+19,26),"#e9dfb0")
    for layer,col in enumerate([far,near]):
        base=38+layer*10
        step=31+r.randrange(10)
        for x in range(-26,244,step):
            peak=(x+r.randrange(8,25),base-r.randrange(8,30))
            a.poly([(x-9,base+4),peak,(x+step+18,base+6),(x+step,base+9),(x,base+8)],col)
            if climate in (0,1) and (x+index+layer)%3:
                px,py=peak
                a.poly([(px,py),(px-9,py+9),(px-1,py+8),(px+3,py+11),
                        (px+11,py+9)], "#dee7de")
    for _ in range(34):
        x,y=r.randrange(223),r.randrange(47,111)
        a.line([(x,y),(x+r.randrange(2,13),y)],"#a0bbb2",1)
    c=96+r.randrange(-12,12)
    w=66+r.randrange(-9,19)
    shore=[(c-w,78),(c-w*.67,59),(c-w*.13,51),(c+w*.53,56),
           (c+w,71),(c+w*.96,87),(c+12,99),(c-w*.62,92)]
    a.poly([(x+1,y+5) for x,y in shore], "#54777b")
    a.poly(shore, soil)
    a.poly([(x, y-2) for x,y in shore[:5]]+[(c+27,82),(c-33,86)], "#8ba181")
    for n in range(13):
        x,y=c+r.randrange(-49,54),r.randrange(61,84)
        if abs(x-c) < 22 and n%2:
            continue
        pine(a,x,y,r.randrange(10,22),plants,climate==1)
    if index%4==0:
        house(a,c-13,76,32)
        a.box((c+35,59,c+45,75),"#92998a",2)
        a.line([(c+38,61),(c+38,69),(c+43,61)],RED,1)
    elif index%4==1:
        house(a,c+21,77,27,RED)
        house(a,c-25,68,15,OAK,BLUE)
        for k in range(4):
            fish(a,c-31+k*7,78,6,BLUE)
    elif index%4==2:
        house(a,c+13,72,25,OAK,BLUE)
        a.line([(c-39,65),(c-19,54),(c,65),(c,76)],DARK,3)
        boat(a,c-19,75,17,sail=False)
    else:
        house(a,c-12,77,31,RED,"#77936e")
        a.line([(c+27,80),(c+27,66),(c+52,66),(c+52,80)],DARK,2)
        a.box((c+32,68,c+38,81),BLUE)
        a.box((c+43,68,c+48,82),RED)
    a.line([(c-2,81),(c+9,90),(c+13,99)],OAK,5)
    boat(a,177+index*11%32,92+(index%3)*3,16,RED if climate%2==0 else BLUE)
    a.save(f"island_{72+index}")


def departure():
    rate = 22050
    out = Path(__file__).resolve().parents[1] / "assets/audio/cast_off.wav"
    with wave.open(str(out), "wb") as track:
        track.setparams((1, 2, rate, 0, "NONE", "not compressed"))
        rng = Random(424938)
        frames = bytearray()
        for i in range(round(1.72*rate)):
            t=i/rate
            hull=0.24*rng.uniform(-1,1)*min(1,t/.01)*exp(-t/.055)
            signal=0.0
            for start, freq in [(0.063,146.83),(.28,220.0)]:
                if t>start:
                    hold=t-start
                    amp=min(hold/.085,1)*exp(-hold/.26)
                    signal+=amp*(sin(2*pi*freq*hold)+.27*sin(4*pi*freq*hold)+.14*sin(6*pi*freq*hold))
            frames+=pack("<h", round((signal*.55+hull)*8500))
        track.writeframes(frames)


if __name__=="__main__":
    for icon in ["longhouse","fishery","smokehouse","woodlot","boatyard","sea_trade",
                 "weaver","feast_hall","smoked_fish","woven_cloth","hearth_bowl",
                 "wear_fjord","wear_krapp","wear_winter","wear_wool","wear_sailor",
                 "wear_braid","wear_amber_brooch","wear_woven_bag"]:
        artwork(icon)
    for i in range(24):
        paint(i)
    departure()
