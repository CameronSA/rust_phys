# Rust Phys

A proof of concept physics engine, written in rust.

# Building From Source

In order to compile ```flo_draw```, clang must be installed, otherwise the error: 

```
cargo:warning=couldn't execute `llvm-config --prefix` 
```

will be thrown. For more information, click [here](https://www.unadulterated-faff.com/articles/2020/02/04/fixing-rust-compilation-issues-caused-by-missing-packages-part-2.html)

The same is true for the GBM buffer management library.

On ubuntu:

```
sudo apt-get install libclang-dev 
sudo apt-get install libgbm-dev
```