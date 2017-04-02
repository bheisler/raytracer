This is a toy raytracer I wrote in rust to learn how raytracers work. I also
wrote a series of posts on it starting
[here](https://bheisler.github.io/post/writing-raytracer-in-rust/).

### Quickstart:

First, you'll need to have [cargo](https://rustup.rs/) installed. Then you can
clone the repository and run the raytracer:

    cd raytracer/app
    cargo run --release scenes/test.json out.png

You can modify the rendered scene by editing test.json. Enjoy!

### Python Quickstart:

This repository also contains code for exposing the raytracer through a C
interface to Python. If you'd like to define your scenes in python instead,
you can do this (note that on Unix-like OS's you may need to edit the path
in python/raytracer.py to point to the `*.so` file instead of a dll):

    cd raytracer/ffi
    cargo build --release
    cd ../python
    python runner.py
