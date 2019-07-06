# Test snippets

This directory contains two sets of test snippets which can be run in
Python.  The `snippets/` directory contains functional tests, and the
`benchmarks/` directory contains snippets for use in benchmarking
RustPython's performance.

## Generates the test for not implemented methods

run using cpython not_impl_gen.py it automatically generate a
test snippet to check not yet implemented methods

## Running with CPython + RustPython

One way to run these snippets is by using CPython to do the parsing and
compilation to bytecode. When this is done, run the bytecode with rustpython.

## Running with RustPython

The other option is to run all snippets with RustPython.
