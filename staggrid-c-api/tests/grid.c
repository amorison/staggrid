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
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5);
    if (grid) {
        grid_c_destroy(grid);
        return true;
    }
    return false;
}

bool grid_invalid() {
    double xs[5] = {-0.5, 0., 0.5, 1.0};
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5);
    if (grid) {
        grid_c_destroy(grid);
        return false;
    }
    return true;
}

bool grid_span() {
    double xs[5] = {-0.5, 0., 0.5, 1.0, 1.5};
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5);
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
