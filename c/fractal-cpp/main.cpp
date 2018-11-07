#include "pgraph.h"

#include <complex.h>

#define W 854

#define H 480

#define X0 427

#define Y0 240

#define PXL 0.1

#define EPS 0.1

#define PI5 0.628318530717959

//Изображение

void fractal1()

{

    image *img = create_image(W, H);

    for (int i = 0; i < W; i++)

        for (int j = 0; j < H; j++)

        {

            double x = (i - X0) * PXL;

            double y = (j - Y0) * PXL;

            complex double z = x + I * y;

            if (x || y)

            {

                double complex t;

                do

                {

                    t = z;

                    z = 0.8 * z + 0.2 * cpow(z, -4);

                }

                while (cabs(z - t) >= EPS);

                color col;

                switch ((int)(carg(z) / PI5))

                {

                case 0:

                    col = RED;

                    break;

                case 1:

                case 2:

                    col = LIME;

                    break;

                case 3:

                case 4:

                    col = BLUE;

                    break;

                case -3:

                case -4:

                    col = YELLOW;

                    break;

                case -1:

                case -2:

                    col = PURPLE;

                    break;
                }

                set_color(img, i, j, col);
            }
        }

    save_to_file(img, "result.bmp");

    free(img);
}

int main()

{

    fractal1();

    return 0;
}