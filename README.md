# recurse

## A cross-platform recursive directory traversal file management tool

![Version](https://img.shields.io/github/v/release/chrissimpkins/recurse?sort=semver)
[![GNU/Linux CI](https://github.com/chrissimpkins/recurse/workflows/GNU/Linux%20CI/badge.svg)](https://github.com/chrissimpkins/recurse/actions?query=workflow%3A%22GNU%2FLinux+CI%22)
[![macOS CI](https://github.com/chrissimpkins/recurse/workflows/macOS%20CI/badge.svg)](https://github.com/chrissimpkins/recurse/actions?query=workflow%3A%22macOS+CI%22)
[![Windows CI](https://github.com/chrissimpkins/recurse/workflows/Windows%20CI/badge.svg)](https://github.com/chrissimpkins/recurse/actions?query=workflow%3A%22Windows+CI%22)
[![codecov](https://codecov.io/gh/chrissimpkins/recurse/branch/master/graph/badge.svg)](https://codecov.io/gh/chrissimpkins/recurse)

## About

The `recurse` executable is a cross-platform, command line file management tool with *default* recursive directory traversal and regular expression pattern matching support.  It is built in Rust and tested against the stable, beta, and nightly Rust toolchains on GNU/Linux, macOS, and Windows platforms.

Features are available through sub-commands of the `recurse` executable. Support currently includes:

- `recurse walk`: recursive directory traversal file listings
- `recurse contains`: identify text file paths with valid UTF-8 text strings that match regular expression patterns

The following sub-commands are in development:

- [*coming in v0.3.0*] `recurse find`: identify lines in text files that match regular expression patterns.  The initial implementation is available for testing as of commit [8bf65ee](https://github.com/chrissimpkins/recurse/commit/8bf65ee7ebe93ac6698001d860fcbd94ae1d138d)
- [*coming in v0.4.0*] `recurse replace`: replace strings in text files that match regular expression patterns

See the Usage section below for additional details.

## Installation

### With `cargo` from crates.io

Use `cargo` to install the `recurse` executable from crates.io:

```
$ cargo install recurse
```

### With `cargo` from the master branch of the repository

Clone the git repository, compile, and install the executable with the following commands:

```
$ git clone https://github.com/chrissimpkins/recurse.git
$ cd recurse
$ cargo install --path .
```

## Usage

View the help documentation for any sub-command on the command line with the syntax:

```
$ recurse [SUB-COMMAND] --help
```

The help menu displays available options and required arguments.

Recursive directory traversal is the default behavior across all sub-commands.  

### [`contains` sub-command]()

#### `contains` Syntax

```
$ recurse contains [OPTIONS] [REGEX] [START PATH]
```

The default behavior of the contains sub-command is to list all valid UTF-8 text file paths with one or more strings that match a regular expression pattern `[REGEX]`.  Hidden paths are excluded by default and are defined as a directory or file path that begins with a period (e.g., `.hidden` directory or `.hidden.txt` file).  All directories and file paths below a hidden directory are considered hidden.  Directory traversal proceeds to the max directory depth below the user-specified start path `[START PATH]`.

#### `contains` Options

The default behavior is modified with command line options.  Supported options for the `contains` sub-command include:

- `-a | --all`: Include hidden file and directory paths
- `-e | --ext [EXTENSION]`: Filter on paths that include an EXTENSION string.  Enter an EXTENSION string argument to define the extension filter.  The EXTENSION argument may be defined with or without a period character (e.g., `txt` or `.txt`)
- `--maxdepth [DEPTH]`: maximum depth to extend traversal of file system sub-directory structure.  Enter an integer value for DEPTH to limit the directory traversal.
- `--mindepth [DEPTH]`: minimum depth to begin traversal of file system sub-directory structure.  Enter an integer value for DEPTH to limit the directory traversal.
- `--symlinks`: Follow symbolic links

### [`walk` sub-command]()

#### `walk` Syntax

```
$ recurse walk [OPTIONS] [START PATH]
```

The default behavior of the walk sub-command is to list all file paths that are not hidden in the standard output stream.  Hidden paths are defined as a directory or file that begins with a period (e.g., `.hidden` directory or `.hidden.txt` file).  All directories and files below a hidden directory path are considered hidden.  Directory traversal proceeds to the max directory depth below the user-specified start path `[START PATH]`.

#### `walk` Options

The default behavior is modified with command line options.  Supported options for the `walk` sub-command include:

- `-a | --all`: Include hidden file and directory paths
- `-d | --dir`: Filter on directory paths only, do not list file paths
- `-e | --ext [EXTENSION]`: Filter on paths that include an EXTENSION string.  Enter an EXTENSION string argument to define the extension filter.  The EXTENSION argument may be defined with or without a period character (e.g., `txt` or `.txt`)
- `--maxdepth [DEPTH]`: maximum depth to extend traversal of file system sub-directory structure.  Enter an integer value for DEPTH to limit the directory traversal.
- `--mindepth [DEPTH]`: minimum depth to begin traversal of file system sub-directory structure.  Enter an integer value for DEPTH to limit the directory traversal.
- `--symlinks`: Follow symbolic links

## Contributing

Please submit new issues on [the GitHub issue tracker](https://github.com/chrissimpkins/recurse/issues).

Contributions under the Apache License, v2.0 are welcomed.  Please open a pull request with your proposal for changes.  

## License

This project is licensed under the Apache License, v2.0.  See [LICENSE.md](LICENSE.md) for the full text of the license.
