import pytest
import numpy as np
from staggrid import Grid1D
from staggrid.error import SingularGridError, NonMonotonicGridError


def test_grid_from_array():
    grid = Grid1D(np.array([1., 2]))
    assert grid.span() == 1.


def test_grid_singular():
    with pytest.raises(SingularGridError):
        Grid1D(np.array([1.]))


def test_grid_non_monotonic():
    with pytest.raises(NonMonotonicGridError):
        Grid1D(np.array([1., 3., 2.]))
