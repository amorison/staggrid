import pytest
import numpy as np
from staggrid import Grid1D, Position
from staggrid.error import (
    SingularGridError, NonMonotonicGridError, MissingPositionsGridError)


def test_grid_from_array():
    grid = Grid1D(1, 1, np.array([-0.5, 0., 0.5, 1., 1.5]))
    assert grid.span() == 1.


def test_grid_singular():
    with pytest.raises(SingularGridError):
        Grid1D(0, 1, np.array([]))


def test_grid_non_monotonic():
    with pytest.raises(NonMonotonicGridError):
        Grid1D(1, 1, np.array([-0.5, 0., 1.5, 1., 1.5]))


def test_grid_missing_points():
    with pytest.raises(MissingPositionsGridError):
        Grid1D(1, 1, np.array([-0.5, 0., 0.5, 1.]))


def test_grid_at_walls():
    positions = np.array([-0.5, 0., 0.5, 1., 1.5])
    grid = Grid1D(1, 1, positions)
    assert np.all(grid.at(Position.Walls) == positions[1::2])


def test_grid_at_centers():
    positions = np.array([-0.5, 0., 0.5, 1., 1.5])
    grid = Grid1D(1, 1, positions)
    assert np.all(grid.at(Position.Centers) == positions[::2])
