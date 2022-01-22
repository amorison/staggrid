#include<stdio.h>
#include<stdbool.h>
#include "staggrid.h"

static bool result = true;
void check(bool (*fn_ptr)(void), char* msg) {
    if (!(*fn_ptr)()) {
        result = false;
        printf("%s failed!\n", msg);
    }
}

bool grid_create() {
    double xs[5] = {-0.5, 0., 0.5, 1.0, 1.5};
    int ierr;
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5, &ierr);
    if (grid) {
        grid_c_destroy(grid);
        return ierr == 0;
    }
    return false;
}

bool grid_invalid() {
    double xs[4] = {-0.5, 0., 0.5, 1.0};
    int ierr;
    struct Grid1D* grid = grid_c_create(1, 1, xs, 4, &ierr);
    if (grid) {
        grid_c_destroy(grid);
        return false;
    }
    // MissingPositions
    return ierr == 3;
}

bool grid_span() {
    double xs[5] = {-0.5, 0., 0.5, 1.0, 1.5};
    int ierr;
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5, &ierr);
    if (grid) {
        double span = grid_c_span(grid);
        grid_c_destroy(grid);
        return span == 1.;
    }
    return false;
}

int main(void) {

    check(&grid_create, "grid_create\0");
    check(&grid_invalid, "grid_invalid\0");
    check(&grid_span, "grid_span\0");

    return !result;
}
