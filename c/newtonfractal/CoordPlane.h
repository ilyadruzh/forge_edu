// CoordPlane.h: Header file for the Coordplane class.
// Aaron Burton
// Whitman College
// May 2009
#ifndef COORDPLANE_H
#define COORDPLANE_H
class CoordPlane
{
  public:
    // Default constructor, sets reasonable default values.
    CoordPlane();
    // Sets horizontal pixel resolution and corresponding x,y-coord plane,
    // h_max, x_min, x_max, y_min, y_max
    // v_max will be set by the private member function _SquareAspect()
    CoordPlane(const int, const double, const double, const double,
               const double);
    // converts horizontal pixel coord, to x value
    double h2x(const int) const;
    // converts vertical pixel coord, to y value
    double v2y(const int) const;
    // converts x value, to horizontal pixel coord
    int x2h(const double) const;
    // converts y value, to vertical pixel coord
    int y2v(const double) const;
    // Returns the current horizontal resolution.
    int HorizontalResolution() const;
    // Returns the current vertical resolution.
    int VerticalResolution() const;
    // Returns the current lower bound of the x range
    double xLowerBound() const;
    // Returns the current lower bound of the y range
    double yLowerBound() const;
    // Returns the current upper bound of the x range
    double xUpperBound() const;
    // Returns the current upper bound of the y range
    double yUpperBound() const;
    // Prints a synopsis of the current class data to the screen.
    void print() const;

  private:
    int _h_max, _v_max;
    double _x_min, _x_max, _y_min, _y_max;
    // Sets vertical resolution to correct aspect ratio
    void _SquareAspect();
};
#endif