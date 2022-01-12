staggrid
========

**This project is experimental and should not be used in production yet.**

This library aims at providing simple and sound APIs to manipulate staggered
grids in finite volume solvers.

The library itself is implemented in Rust and exposes APIs for:

- Python, to ease post-processing of simulation results;
- C, to allow you to readily use this library in existing simulation software.

Build the Python package
------------------------

You can install staggrid as any regular Python package:

    $ python3 -m pip install .

and then import it in a Python script:

    import staggrid

Call the C api
--------------

You will need [cbindgen](https://github.com/eqrion/cbindgen) to generate a
header file automatically.  Install it with:

    cargo install --force cbindgen

Examples of how to call the C API are provided in the `tests_c_api/`
directory.  You can build it with the following commands:

    # build the Rust crate and generate a header file
    $ cargo build
    $ cbindgen --output tests_c_api/staggrid.h --lang c

    # build the tests
    $ cd tests_c_api
    $ cmake -B build
    $ cmake --build build

    # run the tests
    $ cd build
    $ ctest build
