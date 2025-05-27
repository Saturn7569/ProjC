<h1 align="center">ProjC</h1>

A C project and package manager

## Dependencies

Here is a list of all the dependencies *(crates)* in this project:

- Data structures - [serde](https://docs.rs/serde/latest/serde/)
- Command line arguments - [clap](https://docs.rs/clap/latest/clap/)
- Loading toml - [toml](https://docs.rs/toml/latest/toml/)
- Colorful text - [colored](https://docs.rs/colored/latest/colored/)

## Building from source

First, make sure you have [rust](https://www.rust-lang.org) installed.

Then download the source code, open the terminal and run:

```sh
cargo build --release
```

This will build a release version of the project and put it in the `target/release` directory.

## How to use

First download the executables from the [release](https://github.com/Saturn7569/ProjC/releases) page.
Then run the executable.

You can use `projc new <name>` to create a new project.
This will create a new directory based on the name you gave it.
In that directory there is an `src` directory that holds all the code.
There is also a file called `projc.toml` that you can use to configure
the build system.

Here is the config it creates with the project:

```toml
[workspace]
source_dirs = ["src"] # These are all the files included when building
exe_name = "yourexecutable" # The name of the executable (Is the project name automatically)
```

To build a project run:

```sh
projc build
```

This will build a release version of the project and put it in the `target` directory.

And to clean the built files run:

```sh
projc clean
```
