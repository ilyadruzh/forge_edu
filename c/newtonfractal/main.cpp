// Main.cpp
// Aaron Burton
// Whitman College
// May 2009
#include <iostream>
#include <Magick++.h>
#include <cstdlib>
#include <string>
#include <iomanip>
#include "CoordPlane.h"
#include "Complex.h"
#define NUM_CELLS 5
#define HORIZONTALFIX 8
#define VERTICALFIX 3
#define NOTCOLORED 0
#define COLORED 1
using namespace std;
using namespace Magick;
string itos(int);

void OverLayGrid(Image &, const CoordPlane &);
void ZoomIn(Image &, CoordPlane &);
void DrawFractal(Image &, CoordPlane &);
int main()
{
    CoordPlane Plane;
    Image image;
    image = Image(Geometry(Plane.HorizontalResolution(),
                           Plane.VerticalResolution()),
                  "white");    // Creates a blank image.
    DrawFractal(image, Plane); // Draws the basins.
    OverLayGrid(image, Plane); // Overlays zoom-in grid.
    Plane.print();             // Prints coordinates
    image.display();           // Displays image
    ZoomIn(image, Plane);      // Zoom-in/save loop
}
// Most important function. Change this to draw the basins
// of attraction for different functions.
void DrawFractal(Image &Img, CoordPlane &CP)
{
    // Also color can be added here.
    int i, j, k;
    Complex Z, N;
    for (i = 0; i < CP.HorizontalResolution(); i++)
    {
        for (j = 0; j < CP.VerticalResolution(); j++)
        {
            Z = Complex(CP.h2x(i), CP.v2y(j)); //
            // Check First 25 iterates of N
            for (k = 1; k <= 25; k++)
            {
                Z = (2 * Z * Z * Z) / ((3 * Z * Z) - 1); // f(x)=z^3-z; N(z) for f(x).
                // Check distance between interates of N(z) and the roots.
                if ((Z.RealPart() + 1) * (Z.RealPart() + 1) + (Z.ImagPart() * Z.ImagPart()) <= .0001)
                {
                    Img.pixelColor(i, j, ColorRGB(1.0, 0, 0)); // root=-1
                    break;
                }
                if (((Z.RealPart() - 1) * (Z.RealPart() - 1)) + (Z.ImagPart() * Z.ImagPart()) <= .0001)
                {
                    Img.pixelColor(i, j, ColorRGB(0, 1.0, 0)); // root=1
                    break;
                }

                if ((Z.RealPart() * Z.RealPart()) + (Z.ImagPart() * Z.ImagPart()) <= .0001)
                {
                    Img.pixelColor(i, j, ColorRGB(0, 0, 1.0)); // root=0
                    break;
                }
                // Colors the point black if it does not converge.
                if (k == 25)
                    Img.pixelColor(i, j, ColorRGB(0, 0, 0));
            }
        }
    }
}
void OverLayGrid(Image &Img, const CoordPlane &CP)
{
    int i, j;
    double xinterval, yinterval;
    xinterval = (CP.HorizontalResolution() - 1) / NUM_CELLS;
    yinterval = (CP.VerticalResolution() - 1) / NUM_CELLS;
    for (i = 1; i <= NUM_CELLS - 1; i++)
    {
        Img.draw(DrawableLine(xinterval * i, 0, xinterval * i, CP.VerticalResolution()));
        Img.draw(DrawableLine(0, yinterval * i, CP.HorizontalResolution(),
                              yinterval * i));
    }
    for (i = 1; i <= NUM_CELLS * 2; i = i + 2)
    {
        for (j = 1; j <= NUM_CELLS * 2; j = j + 2)
        {
            Img.draw(DrawableText(xinterval * j / 2 - HORIZONTALFIX,
                                  yinterval * i / 2 + VERTICALFIX,
                                  itos((i / 2 * NUM_CELLS + j / 2) + 1)));
        }
    }
}
string itos(int x)
{
    string result;
    char num[100];
    sprintf(num, "%3i", x);
    result = num;
    return result;
}
string ftos(double x)
{
    string result;
    char num[100];
    sprintf(num, "%3.9f", x);
    result = num;
    return result;
}
void ZoomIn(Image &Img, CoordPlane &CP)
{
    int GetView(Image &, CoordPlane &);
    int CellNum, i, j;
    double x_min_new, x_max_new, y_min_new, y_max_new;
    double xinterval, yinterval;
    while (true)
    {
        CellNum = GetView(Img, CP);
        xinterval = (CP.xUpperBound() - CP.xLowerBound()) / NUM_CELLS;
        yinterval = (CP.yUpperBound() - CP.yLowerBound()) / NUM_CELLS;
        i = (CellNum - 1) / NUM_CELLS + 1;
        j = (CellNum - 1) % NUM_CELLS + 1;
        x_min_new = CP.xLowerBound() + (j - 1) * xinterval;
        x_max_new = CP.xLowerBound() + j * xinterval;
        y_min_new = CP.yUpperBound() - (i * yinterval);
        y_max_new = CP.yUpperBound() - ((i - 1) * yinterval);
        CP = CoordPlane(CP.HorizontalResolution(), x_min_new, x_max_new, y_min_new,
                        y_max_new);
        setprecision(16);
        Img = Image(Geometry(CP.HorizontalResolution(),
                             CP.VerticalResolution()),
                    "white");
        DrawFractal(Img, CP);
        CP.print();
        OverLayGrid(Img, CP);
        Img.display();
    }
}
int GetView(Image &Img2, CoordPlane &CP2)
{
    int Cell = 0;
    string c;
    while (Cell < 1 || Cell > NUM_CELLS * NUM_CELLS)
    {
        cout << endl
             << "Enter ’q’ to quit or ’s’ to save previous fractal"
             << endl;
        cout << "Enter a cell number to zoom-in on: ";
        // saves fractal as 1280x853
        if (cin.peek() =='s' || cin.peek() =='S')
        {
            CP2 = CoordPlane(1280, CP2.xLowerBound(), CP2.xUpperBound(),
                             CP2.yLowerBound(), CP2.yUpperBound());
            Img2 = Image(Geometry(CP2.HorizontalResolution(),
                                  CP2.VerticalResolution()),
                         "white");
            DrawFractal(Img2, CP2);
            c = "Aaron Burton [" + ftos(CP2.xLowerBound()) + "," +
                ftos(CP2.xUpperBound()) + "] X [" + ftos(CP2.yLowerBound()) +
                "," + ftos(CP2.yUpperBound()) + "]";
            Img2.draw(DrawableText(CP2.HorizontalResolution() - 450,
                                   CP2.VerticalResolution() - 15, c));
            Img2.write("fractal.png"); // File Name.
            cout << "Saved fractal.png";
            exit(0);
        }
        // quits on q
        if (cin.peek() =='q' || cin.peek() =='Q')
        {
            exit(0);
        }
        else
        {
            // get cell # to zoom-in on.
            cin >> Cell;
            if (cin.peek() =='\n')
            {
                c = cin.get();
            }
        }
    }
    return Cell;
}