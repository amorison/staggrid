#include<stdio.h>
#include "staggrid.h"

int main() {
    double xs[3] = {1.0, 1.5, 2.3};
    //double xs[3] = {1.0, 3.5, 2.3};

    struct Grid1D* grid = grid_c_from_slice(xs, 3);

    if (!grid) {
        printf("Could not create grid\n");
        return 1;
    }

    double span = grid_c_span(grid);
    printf("Span = %f\n", span);
    grid_c_destroy(grid);
    return 0;
}
