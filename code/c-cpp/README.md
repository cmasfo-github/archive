
I mainly use CMake, for the cross-platform build.

How to Run: (if CMake is installed)

```
mkdir build
cd build
cmake .. (process cmake system, not built yet)
cmake --build . (this is building)
```

If project name is hello:

```
# Linux
./hello
# Windows (depends on build config)
./Debug/hello.exe (debug build)
./Release/hello.exe (release build)
```
