import ctypes
cppmodule = ctypes.CDLL("./cppmodulelib.so")

def add(i: int, j: int) -> int:
    return cppmodule.add(i, j)


if __name__ == "__main__":
    print(add(9, 10))