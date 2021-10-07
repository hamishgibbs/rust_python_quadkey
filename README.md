# rust_python_quadkey

A simple library to connect Python to a Rust library and benchmark Rust vs. Python performance for a simple trigonometric calculation.

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
