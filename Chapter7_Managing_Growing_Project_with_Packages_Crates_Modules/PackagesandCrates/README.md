# Packages and Crates
## Crates
- A crate is the smallest amount of code that the Rust compiler considers at a time.
- Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
- Two forms
  - A binary crates: 
    - Binary crates are programs you can compile to an executable that you can run
    - Each must have a function called main that defines what happens when the executable runs. 
  - A library crates
    - Library crates don’t have a main function, and they don’t compile to an executable.
    - They define functionality intended to be shared with multiple projects.
- The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate

## Package
- A package is a bundle of one or more crates that provides a set of functionality.
- A package contains a Cargo.toml file that describes how to build those crates.
> Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.
- A package can contain as many binary crates as you like, but at most only one library crate. 
- A package must contain at least one crate, whether that’s a library or binary crate.

```rust
    //run new cargo project
    cargo new PachkageandCrates
```
- PachkageandCrates is a package
- src/main.rs is the crate root of a binary crate 
- src/lib.rs is the crate root file if package contain src/lib.rs