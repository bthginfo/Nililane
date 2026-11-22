"""Paint the Oases' chart, building and wardrobe icons."""
from math import cos, sin, tau
from random import Random

from make_ui import Art, CREAM, GOLD, INK
from make_market_art import jar

SAND = "#d4a475"
SHADE = "#a87057"
BONE = "#f3d9a5"
JADE = "#58a194"
LAPIS = "#4b92a8"
PLUM = "#976287"
PINK = "#bf796a"
LEAF = "#64864f"


def palm(a, x, y, s, night=False):
    a.line([(x,y), (x+s*.12,y-s*.62), (x+s*.29,y-s)], "#875d4b", max(1, round(s*.12)))
    x, y = x+s*.29, y-s
    for i in range(7):
        dx = (i-3)*s*.24
        bend = s*(abs(i-3)*.075-.38)
        a.poly([(x,y), (x+dx*.58,y+bend-s*.13), (x+dx,y+bend+s*.20),
                (x+dx*.45,y+bend+s*.105)], ["#5c8271", "#719762", LEAF][(i+int(night))%3])
    a.oval((x-s*.15,y-s*.08,x+s*.14,y+s*.15), GOLD)


def facade(a, x, y, w, h, trim, dome=False, night=False):
    face, side, top = ("#987575","#735d65","#be927a") if night else (BONE,SHADE,SAND)
    dx,dy = w*.24,w*.17
    a.poly([(x+w,y-h),(x+w+dx,y-h-dy),(x+w+dx,y-dy),(x+w,y)],side)
    a.box((x,y-h,x+w,y),face)
    a.poly([(x,y-h),(x+dx,y-h-dy),(x+w+dx,y-h-dy),(x+w,y-h)],top)
    a.line([(x,y-h+2),(x+w,y-h+2),(x+w+dx,y-h-dy+2)],trim,2)
    for i in range(2):
        left=x+w*(.24+i*.36)
        cy=y-h*.33
        a.poly([(left-w*.103,y),(left-w*.103,cy),(left,cy-w*.125),
                (left+w*.103,cy),(left+w*.103,y)],GOLD if night else INK)
    if dome:
        center=x+w*.60
        y0=y-h-dy*.5
        a.box((center-w*.28,y0-w*.075,center+w*.28,y0+w*.06),face)
        a.poly([(center-w*.28,y0-w*.047), (center-w*.24,y0-w*.26), (center-w*.12,y0-w*.36),
                (center,y0-w*.48),(center+w*.15,y0-w*.36),(center+w*.26,y0-w*.23),
                (center+w*.29,y0-w*.045)],trim)
        a.line([(center,y0-w*.48),(center,y0-w*.55)],GOLD,2)


def paint(index):
    night=index in (7,14,21)
    r=Random(5817+index*1823)
    a=Art(230,112)
    sky,water,dunes,sand,stone=[
        ("#ffe4b0","#6aa6a3","#d6aa7c","#efc58b","#cf9a73"),
        ("#eacac0","#509ba0","#c59981","#d9b18e","#c68571"),
        ("#ffd9a8","#579c99","#ce976f","#e5b784","#bd8867"),
        ("#e5d2a4","#599ea3","#c6a181","#e5c692","#c29474"),
    ][(index//6+index)%4]
    if night:
        sky,water,dunes,sand,stone=("#3d5178","#355f76","#655a75","#bb977a","#8a6a68")
    a.box((0,0,230,112), water)
    a.box((0,0,230,53),sky)
    sun=22+index*31%173
    a.oval((sun,6,sun+19,25), "#f8df9f")
    if night:
        a.oval((sun+7,3,sun+22,19),sky)
        for _ in range(23):
            x,y=r.randrange(10,225),r.randrange(4,32)
            a.oval((x,y,x+1,y+1), BONE)
    for phase in (0,1):
        points=[(x,38+phase*7-r.randint(1,23)) for x in range(-23,255,20+phase*11)]
        a.poly(points+[(250,48+phase*6),(-25,48+phase*6)],dunes if phase==0 else stone)
    for i in range(38):
        x=r.randrange(12,225)
        y=r.randrange(54,111)
        a.line([(x,y),(x+r.randrange(2,14),y)],"#8ab9b0" if not night else "#4c7184",1)
    w=69+r.randint(-10,17); c=107+r.randint(-11,13)
    island=[(c-w,77),(c-w*.81,59),(c-w*.27,50),(c+w*.53,53),(c+w*.92,62),
            (c+w,78),(c+w*.47,95),(c-14,99),(c-w*.74,89)]
    a.poly([(x+2,y+5) for x,y in island],stone)
    a.poly(island,sand)
    a.poly([(c-w*.82,76),(c-15,60),(c+w*.79,68),(c+w*.43,80),(c-21,87)], "#98a076" if not night else "#777d68")
    for i in range(7+index%5):
        x=c+r.randint(round(-w*.77), round(w*.82))
        y=r.randrange(61,89)
        palm(a,x,y,r.randrange(9,20),night)
    color=[JADE,LAPIS,PLUM,"#b97456"][index%4]
    for x,y,w0,h,domed in [(c-39,75,31,19,index%4==0),(c-9,78,37,24,True),
                          (c+30,81,22,15,index%3!=0)]:
        facade(a,x,y,w0,h,color,domed,night)
    if index%3:
        a.poly([(c-10,92),(c+23,89),(c+42,81),(c+11,84)],LAPIS)
        a.line([(c-4,89),(c+25,86)],JADE,1)
    x=166+(index*19)%35; y=97-index%4*3
    a.poly([(x-21,y-1),(x-12,y+4),(x+13,y+4),(x+22,y-1)],SHADE)
    a.line([(x+2,y-1),(x+2,y-19)],INK,1)
    a.poly([(x-8,y-23),(x+19,y-7),(x-7,y-6)], BONE)
    a.line([(x-13,y-2),(x+15,y-2)],JADE,1)
    a.save(f"island_{96+index}")


def icon(key):
    a=Art()
    a.oval((12,10,84,84),"#d6c19a")
    a.oval((15,15,81,80),"#f1dfb7")
    if key == "oasis_home":
        facade(a,24,75,46,36,LAPIS,False)
        facade(a,34,43,24,12,JADE,False)
    elif key == "cistern":
        a.oval((19,47,80,76),SHADE)
        a.oval((19,40,80,68),SAND)
        a.oval((25,43,74,61),LAPIS)
        a.line([(26,54),(54,52),(64,54)],JADE,2)
        jar(a,63,40,PINK,.66)
    elif key == "date_garden":
        palm(a,44,75,40)
        palm(a,25,72,24)
        a.oval((49,65,73,79),SHADE)
        for i in range(5):
            x=52+i*4.3
            a.oval((x,66-i%3*3,x+5,70-i%3*3),GOLD)
    elif key == "spice_bazaar":
        a.poly([(17,41),(29,26),(70,26),(78,43)],PLUM)
        for i in range(3):
            x=27+i*16
            a.poly([(x,27),(x+7,27),(x+10,44),(x+2,44)],CREAM)
        a.line([(25,42),(25,75),(72,75),(72,42)],SHADE,4)
        for x,hue in [(30,PINK),(48,GOLD),(64,LEAF)]:
            a.poly([(x-9,65),(x,54),(x+9,65),(x+6,73),(x-6,73)],hue)
    elif key == "caravanserai":
        facade(a,19,77,52,38,JADE,True)
        a.poly([(37,77),(37,57),(48,44),(59,57),(59,77)],LAPIS)
        a.poly([(41,77),(41,58),(48,50),(55,58),(55,77)],INK)
    elif key == "qanat":
        a.box((29,30,61,61),SHADE,4)
        a.oval((26,26,64,45),SAND)
        a.oval((32,28,57,41),INK)
        a.poly([(37,50),(56,53),(72,76),(21,76)],SAND)
        a.poly([(41,51),(53,53),(63,72),(29,72)],LAPIS)
        a.line([(42,57),(48,66),(58,69)],JADE,2)
    elif key in ("glassworks","perfumer"):
        jar(a,35,59,LAPIS if key == "glassworks" else PLUM,1.35)
        jar(a,63,66,JADE if key == "glassworks" else PINK,.9)
        a.line([(31,48),(28,59),(29,66)],CREAM,2)
        if key == "perfumer":
            a.line([(52,48),(52,32),(62,24)],LEAF,2)
            for i in range(4):
                a.oval((54+i*2,30-i*2,66+i*2,35-i*2),PINK)
    elif key == "moon_garden":
        a.oval((21,58,76,78),LAPIS)
        a.oval((21,25,50,54),GOLD)
        a.oval((31,19,57,44),"#f1dfb7")
        palm(a,61,70,24,True)
    elif key == "carpet_hall":
        a.line([(20,75),(20,33),(76,33),(76,75)],SHADE,4)
        for i,col in enumerate([PLUM,LAPIS,PINK]):
            x=24+i*17
            a.box((x,35,x+15,73),col,2)
            a.poly([(x+8,44),(x+13,54),(x+8,63),(x+2,54)],GOLD)
    elif key == "caravan":
        a.line([(30,60),(25,75),(43,63),(44,76),(61,59),(65,72)],SHADE,4)
        a.poly([(21,55),(26,47),(34,40),(40,42),(45,49),(51,42),(58,50),
                (64,46),(66,35),(70,29),(81,33),(77,40),(71,40),(68,54),(63,61),
                (54,64),(33,65),(22,59)],SHADE)
        a.box((38,46,56,60),PLUM,2)
        a.line([(39,56),(54,56)],GOLD,2)
    elif key == "wear_lapis":
        a.poly([(35,29),(47,33),(60,29),(74,46),(64,53),(66,73),(30,73),(32,53),(22,46)],LAPIS)
        a.line([(35,37),(47,51),(59,37)],GOLD,3)
        a.line([(32,59),(66,59)],PLUM,4)
    elif key == "wear_dune_wrap":
        a.oval((21,32,76,56),SHADE)
        a.oval((23,28,74,49),CREAM)
        a.poly([(55,46),(72,46),(72,70),(64,72)],BONE)
        a.line([(26,43),(48,48),(68,41)],LAPIS,4)
    elif key == "wear_glass_pendant":
        a.line([(32,29),(34,46),(48,59),(62,46),(65,29)],SHADE,3)
        a.poly([(48,48),(59,59),(48,74),(37,59)],LAPIS)
        a.poly([(48,52),(52,60),(46,66),(42,60)],JADE)
    a.save(key)


if __name__ == "__main__":
    for index in range(24):
        paint(index)
    for key in ("oasis_home cistern date_garden spice_bazaar caravanserai qanat glassworks perfumer "
                "moon_garden carpet_hall caravan wear_lapis wear_dune_wrap wear_glass_pendant").split():
        icon(key)
