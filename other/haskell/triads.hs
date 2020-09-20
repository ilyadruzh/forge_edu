triads n = [(x,y,z) | let ns = [1 .. n], 
 x <- ns, y <- ns, z <- ns, x*x+y*y == z*z]