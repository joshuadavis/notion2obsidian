# Project Structure

The main project descriptor is `Cargo.toml` in the root directory of the repository.


| Path          | Description                                                                                             |
|---------------|---------------------------------------------------------------------------------------------------------|
| `Cargo.toml`  | Cargo project descriptor.                                                                               |
| `.idea/`      | CLion files.                                                                                            |
| `src/`        | The source code of the project.                                                                         |
| `src/main.rs` | The entry point.   Parses the command line and delegates to the other modules.                          |
| `test-data/`  | Data used by tests.  Note that the tests expect the current directory to be the root of the git clone.  |

To build and test the project, use [Cargo](https://doc.rust-lang.org/cargo/) in the usual way:

```bash
cargo build
```

# Windows

Some special considerations for building this project on Windows systems:

1. `git clone` might fail due to long filenames.   To resolve this, run the following command in Powershell, as an administrator:
    ```powershell
    git config --system core.longpaths true
    ```
    This will let you clone the project on a Windows system, and it will work from CLion if you chose to use that to do the clone for you.

   