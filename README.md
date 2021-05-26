# AddAlias
> Easily manage bash aliases.

# Installation
Download the latest release [tar.gz](https://github.com/Rawnly/aalias/releases/latest/download/aalias.tar.gz) or [bin](https://github.com/Rawnly/aalias/releases/latest/download/aalias)

If you are on MacOS you can install `aalias` via **homebrew**:
```
    brew tap rawnly/tap
    brew install aalias
```



## Usage
```sh
USAGE:
    aalias [ARGS] [SUBCOMMAND]

ARGS:
    <name>        Name of the alias
    <value>...    Value of the alias

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    setup    setup basic configuration

EXAMPLES:
  add-alias md markdown -p
  # Equivalent to
  alias md='markdown -p'
  # Plus adding it to your rc file.
```
