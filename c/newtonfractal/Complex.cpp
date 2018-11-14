// Complex.cpp: The Complex class needed for Main.cpp.
// Aaron Burton
// Whitman College
// May 2009
#include <iostream>
#include <cassert>
#include "CoordPlane.h"
using namespace std;
CoordPlane::CoordPlane()
{
    _h_max = 800;
    _x_min = -1.5;
    _x_max = 1.5;
    _y_min = -1;
    _y_max = 1;
    _SquareAspect(); // sets _v_max
}
CoordPlane::CoordPlane(const int hmax, const double xmin,
                       const double xmax, const double ymin,
                       const double ymax)
{
    _h_max = hmax;
    _x_min = xmin;
    _x_max = xmax;
    _y_min = ymin;
    _y_max = ymax;
    _SquareAspect(); // sets _v_max
    assert(_x_min <= _x_max);
    assert(_y_min <= _y_max);
    assert(_h_max > 100);
}
double CoordPlane::h2x(const int h) const
{
    double Slope;
    Slope = (_x_max - _x_min) / (_h_max - 1);
    return Slope * h + _x_min;
}
double CoordPlane::v2y(const int v) const
{
    double Slope;
    Slope = (_y_max - _y_min) / (-1 * (_v_max - 1));
    return Slope * v + _y_max;
}
int CoordPlane::x2h(const double x) const
{
    double Slope;
    Slope = (_x_max - _x_min) / (_h_max - 1);
    return int((x - _x_min) / Slope);
}
int CoordPlane::y2v(const double y) const
{
    double Slope;
    Slope = (_y_min - _y_max) / (-1 * (_v_max - 1));
    return int((y - _y_min) / Slope);
}
int CoordPlane::HorizontalResolution() const
{
    return _h_max;
}
int CoordPlane::VerticalResolution() const
{
    return _v_max;
}
double CoordPlane::xLowerBound() const
{
    return _x_min;
}

double CoordPlane::yLowerBound() const
{
    return _y_min;
}