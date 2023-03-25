# deps-readme
- STATUS: IN PROGRESS

`cargo-deps-readme` is a Rust crate that simplifies the process of creating a README.md file containing a list of a crate's dependencies. This crate can be used to quickly generate a formatted and organized list of dependencies specified in a Cargo.toml file.

## Usage:

Add cargo-deps-readme to the [dev-dependencies] section of your crate's Cargo.toml file by adding the following line:

```
[dev-dependencies]
cargo-deps-readme = "0.1.0"
```
Run the following command to generate the README.md file:

```
deps-readme
```

This will create a new DEPENDECIES.md file with a list of all the dependencies specified in your Cargo.toml file.

The output will be a formatted list of dependencies in the following format:
```

- [crate-name](https://crates.io/crates/crate-name) - version: 0.1.0 - description: This crate does something useful
- [other-crate](https://crates.io/crates/other-crate) - version: 0.2.3 - description: Another useful crate

```

This crate is useful for crate authors who want to provide an easily accessible list of  dependencies for users and contributors to their project. It can also be used by developers who want to quickly understand the dependencies of a crate they are using or contributing to.
