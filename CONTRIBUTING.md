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

## Testing

To test with "real" data, you can make a Notion export, then copy it to the root directory of the git clone.

If you name it `Export-blah.zip`, then the gitignore rules will ignore it.

This way you can run `notion2obsidian` from the root directory of the clone and all the directories it creates will be ignored by git.

# Windows

Some special considerations for building this project on Windows systems:

1. `git clone` might fail due to long filenames.   To resolve this, run the following command in Powershell, as an administrator:
    ```powershell
    git config --system core.longpaths true
    ```
    This will let you clone the project on a Windows system, and it will work from CLion if you chose to use that to do the clone for you.

   