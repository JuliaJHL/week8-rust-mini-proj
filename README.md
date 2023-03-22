# A Rust CLI tool: file converter
This is a Rust CLI tool that can convert the type of image files. It takes in three command line arguments: the path to the input image, the path to the output image, and the desired format of the output image. 

## Project Setup
1. clone the repo:
```
https://github.com/JuliaJHL/week8-rust-mini-proj.git
```
2. cd into the project:
```
cd week8-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run <input_path> <output_path> [output_format]
```

## examples
There is a `Lenna.png` in the input folder. I convert it to `Lenna.jpeg` and store it in the output folder.
![week8](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/week8.png)
You can check the results in the `input` and `output` folders.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
