# recurse

## A cross-platform recursive directory traversal file management tool

![Version](https://img.shields.io/github/v/release/chrissimpkins/recurse?sort=semver)
[![GNU/Linux CI](https://github.com/chrissimpkins/recurse/workflows/GNU/Linux%20CI/badge.svg)](https://github.com/chrissimpkins/recurse/actions?query=workflow%3A%22GNU%2FLinux+CI%22)
[![macOS CI](https://github.com/chrissimpkins/recurse/workflows/macOS%20CI/badge.svg)](https://github.com/chrissimpkins/recurse/actions?query=workflow%3A%22macOS+CI%22)
[![Windows CI](https://github.com/chrissimpkins/recurse/workflows/Windows%20CI/badge.svg)](https://github.com/chrissimpkins/recurse/actions?query=workflow%3A%22Windows+CI%22)
[![codecov](https://codecov.io/gh/chrissimpkins/recurse/branch/master/graph/badge.svg)](https://codecov.io/gh/chrissimpkins/recurse)

## About

The `recurse` executable is a cross-platform command line tool for file management with *default* recursive directory traversal and regular expression pattern matching support.  It is built in Rust and tested against the stable, beta, and nightly Rust toolchains on GNU/Linux, macOS, and Windows platforms.

Features are available through sub-commands of the `recurse` executable. Support currently includes:

- `recurse contains`: identify valid UTF-8 encoded text file paths with contents that match regular expression patterns
- `recurse find`: identify regular expression pattern match line and byte offsets in valid UTF-8 encoded text files
- `recurse walk`: recursive directory traversal file listings

The following features are in development:

- [*coming in v0.4.0*] `recurse replace`: replace strings in text files that match regular expression patterns (issue #6)
- [*coming in v0.5.0*] add optional canonical Unicode normalization support for text input to sub-commands that support text matching (issue #8)

See the [Usage section](#usage) below for additional details.

See the [FAQ.md](FAQ.md) for answers to frequently asked questions.

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

The contains sub-command's default behavior is to list all text file paths with one or more valid UTF-8 encoded Unicode scalar values that match a regular expression pattern `[REGEX]`.  Hidden paths are excluded by default and are defined as a directory or file path that begins with a period (e.g., `.hidden` directory or `.hidden.txt` file).  All directory and file paths below a hidden directory are considered hidden.  Directory traversal proceeds to the max depth below the user-specified start path `[START PATH]`.

#### `contains` Options

Command line options modify the default behavior. Supported options for the `contains` sub-command are:

- `-a | --all`: Include hidden file and directory paths
- `-e | --ext [EXTENSION]`: Filter on paths that include an EXTENSION string.  Enter an EXTENSION string argument to define the extension filter.  The EXTENSION argument may be defined with or without a period character (e.g., `txt` or `.txt`)
- `--maxdepth [DEPTH]`: maximum depth to extend traversal of file system sub-directory structure.  Enter an integer value for DEPTH to limit the directory traversal.
- `--mindepth [DEPTH]`: minimum depth to begin traversal of file system sub-directory structure.  Enter an integer value for DEPTH to limit the directory traversal.
- `--symlinks`: Follow symbolic links

### [`find` sub-command]()

#### `find` Syntax

```
$ recurse find [OPTIONS] [REGEX] [START PATH]
```

The `find` sub-command's default behavior is to list all lines in text files with valid UTF-8 encoded Unicode scalar values that match a regular expression pattern `[REGEX]`.  The report includes the following data for each line in a file with a match:

```
[FILEPATH] [LINE NUMBER]:[START BYTE OFFSET INDEX]-[END BYTE OFFSET INDEX] [ MATCHED STRING ]
```

Here is an example of a match result on the regular expression pattern `[Rr]ecurse` in this repository:

```
./src/command/find.rs 12:11-18 [ Recurse ]
```

Note that the byte offsets will not map 1:1 to "character offsets" when multi-byte encoded characters are in or before the matched string in a given line of text.

Hidden paths are excluded by default and are defined as a directory or file path that begins with a period (e.g., `.hidden` directory or `.hiddent.txt` file).  All directory and file paths below a hidden directory are considered hidden.  Directory traversal proceeds to the max depth below the user-specified start path `[START PATH]`.

#### `find` Options

Command line options modify the default behavior. Supported options for the `find` sub-command are:

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

The walk sub-command's default behavior is to list all file paths that are not hidden in the standard output stream.  Hidden paths are defined as a directory or file that begins with a period (e.g., `.hidden` directory or `.hidden.txt` file).  All directory and file paths below a hidden directory path are considered hidden.  Directory traversal proceeds to the max depth below the user-specified start path `[START PATH]`.

#### `walk` Options

Command line options modify the default behavior. Supported options for the `walk` sub-command are:

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

Apache License, v2.0.  See [LICENSE.md](LICENSE.md) for the full text of the license.
