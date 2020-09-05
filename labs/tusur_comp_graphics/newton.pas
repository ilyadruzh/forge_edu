program N3;
uses Graph, Crt;
type Complex = Record
                x : Real;
                y : Real;
               end;
const iter = 50;
      max  = 1e+6;
      min  = 1e-6;
var  z, t, d : Complex;
     y, x, p : Real;
   xc, yc, n : Integer;
     Cancel  : Boolean;
     gd, gm  : Integer;
     mx, my  : real;
 x0,xn,y0,yn : real;
begin
Cancel := False;
Randomize;
gd := Detect;
InitGraph(gd,gm,'c:\bp\bgi');
{дипазон по х,у}
x0:=-1;xn:=0.7;
y0:=-0.7;yn:=1.0;
{начало экранных координат, смещено влево и вверх}
xc := round(GetMaxX/(xn-x0));
yc := round(GetMaxY*(-y0)/(yn-y0));
{масштаб для перевода координат в экранные}
mx:=xc;
my:=yc/-y0;
y:=y0;{цикл по у}
repeat
 x:=x0; {цикл по х}
  repeat
   n := 0;
   z.x := x;
   z.y := y;
   d := z;
   while (sqr(z.x)+sqr(z.y) < max) and (sqr(d.x)+sqr(d.y) > min)
   and (n < iter) do
    begin
     t := z;
     {z^3 - 1}
     p := sqr(sqr(t.x)+sqr(t.y));
     z.x := 2/3*t.x + (sqr(t.x)-sqr(t.y))/(3*p);{ в этой строке ошибка}
     z.y := 2/3*t.y*(1-t.x/p);{}
     d.x := abs(t.x - z.x);
     d.y := abs(t.y - z.y);
     Inc(n);
     if keypressed then Cancel := true;
    end;
   PutPixel(xc + round(x*mx),yc + round(y*my),16 - (n mod 16));
   if cancel then exit;
   x:=x+0.005;{если 0.001 то картинка лучше, но рисует долго}
  until x>xn;
 y:=y+0.005;
until y>yn;
Readkey;
end.