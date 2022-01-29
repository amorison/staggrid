#include<stdio.h>
#include<stdint.h>
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

bool grid_at_walls() {
    double xs[5] = {-0.5, 0., 0.5, 1.0, 1.5};
    int ierr;
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5, &ierr);
    if (!grid) return false;
    uintptr_t length;
    double* walls = grid_c_at(grid, POSITION_WALLS, &length, &ierr);
    if (!walls || ierr != 0 || length != 2) return false;
    bool check = walls[0] == 0. && walls[1] == 1.0;
    free(walls);
    return check;
}

bool grid_at_centers() {
    double xs[5] = {-0.5, 0., 0.5, 1.0, 1.5};
    int ierr;
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5, &ierr);
    if (!grid) return false;
    uintptr_t length;
    double* centers = grid_c_at(grid, POSITION_CENTERS, &length, &ierr);
    if (!centers || ierr != 0 || length != 3) return false;
    bool check = centers[0] == -0.5 && centers[1] == 0.5 && centers[2] == 1.5;
    free(centers);
    return check;
}

bool grid_at_invalid_position() {
    double xs[5] = {-0.5, 0., 0.5, 1.0, 1.5};
    int ierr;
    struct Grid1D* grid = grid_c_create(1, 1, xs, 5, &ierr);
    if (!grid) return false;
    uintptr_t length;
    double* centers = grid_c_at(grid, 2, &length, &ierr);
    if (!centers && ierr == 1 && length == 0) return true;
    if (centers) free(centers);
    return false;
}

int main(void) {

    check(&grid_create, "grid_create\0");
    check(&grid_invalid, "grid_invalid\0");
    check(&grid_span, "grid_span\0");
    check(&grid_at_walls, "grid_at_walls\0");
    check(&grid_at_centers, "grid_at_centers\0");
    check(&grid_at_invalid_position, "grid_at_invalid_position\0");

    return !result;
}
