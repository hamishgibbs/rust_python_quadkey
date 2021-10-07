# rust_python_quadkey

A simple example of connecting Python to a Rust library and benchmarking Rust vs. Python performance for a trigonometric calculation.

**To Build the Rust Library:**

```{shell}
cargo build --release
```

**To import Rust in Python**

```{shell}
cp ./target/release/libquadkeys.dylib ./quadkeys.so
```

**Execute an example Python script:**

```{shell}
python main.py
```

**Benchmark Pure Python against Rust:**

```{shell}
python -m pytest main.py
```

### Resources
[Speed up your Python using Rust](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust)
[Rust bindings for the Python interpreter](https://rustrepo.com/repo/PyO3-PyO3-rust-foreign-function-interface)
[Py03](https://github.com/PyO3/pyo3)
[pyquadkey2](https://github.com/muety/pyquadkey2)
[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
