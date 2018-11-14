// Complex.h: Header file for the Complex class.
// Aaron Burton
// Whitman College
// May 2009
#ifndef COMPLEX_H
#define COMPLEX_H
#include <iostream>
using namespace std;
// A simple complex number class
class Complex
{
    // friend functions and overloads
    friend double abs(const Complex &);
    friend Complex operator+(const Complex &, const Complex &);
    friend Complex operator-(const Complex &, const Complex &);
    friend Complex operator*(const Complex &, const Complex &);
    friend Complex operator/(const Complex &, const Complex &);
    friend Complex operator-(const Complex &); //negation
    friend Complex operator!(const Complex &); //conjugation
    friend ostream &operator<<(ostream &, const Complex &);

  public:
    // constructor, first arg sets real part, second arg sets imag part,
    // defaults are set to 0
    Complex(double = 0, double = 0);
    // returns the real part of the complex number
    double RealPart() const;
    // sets the real part of the complex number to the double arg
    void SetRealPart(double);
    // returns the imaginary part of the complex number
    double ImagPart() const;
    // sets the imaginary part of the complex number to the double arg
    void SetImagPart(double);

  private:
    // variables for holding the real and imaginary parts
    double _real, _imag;
};
#endif