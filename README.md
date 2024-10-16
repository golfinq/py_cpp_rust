# Using C++ in Rust, using Python

This repo showcases using C++ code inside rust, using python as an intermediary. This is obviously the best way to do this.

## Building
`source build.sh`

Here are the lines for `build.sh`:
```sh
c++ -O3 -Wall -std=c++17 -fPIC -shared -o src/cppmodulelib.so src/cppmodule.cpp
cp src/*.so .
cargo build
```

## Running
`cargo run`, if the outputted `.so` library isn't in the directory being used to run the rust binary - it won't work.