c++ -O3 -Wall -std=c++17 -fPIC -shared -o src/cppmodulelib.so src/cppmodule.cpp
cp src/*.so .
cargo build