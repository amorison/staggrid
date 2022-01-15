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

bool grid_from_slice() {
    double xs[3] = {1.0, 1.5, 2.3};
    struct Grid1D* grid = grid_c_from_slice(xs, 3);
    if (grid) {
        grid_c_destroy(grid);
        return true;
    }
    return false;
}

bool grid_invalid() {
    double xs[3] = {1.0, 3.5, 2.3};
    struct Grid1D* grid = grid_c_from_slice(xs, 3);
    if (grid) {
        grid_c_destroy(grid);
        return false;
    }
    return true;
}

bool grid_span() {
    double xs[3] = {1.0, 1.5, 2.0};
    struct Grid1D* grid = grid_c_from_slice(xs, 3);
    if (grid) {
        double span = grid_c_span(grid);
        grid_c_destroy(grid);
        return span == 1.;
    }
    return false;
}

int main(void) {

    check(&grid_from_slice, "grid_from_slice\0");
    check(&grid_invalid, "grid_invalid\0");
    check(&grid_span, "grid_span\0");

    return !result;
}
