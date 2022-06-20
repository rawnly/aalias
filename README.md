# AddAlias
> Easily manage bash aliases.

# Installation
Download the latest release [tar.gz](https://github.com/Rawnly/aalias/releases/latest/download/aalias.tar.gz) or [bin](https://github.com/Rawnly/aalias/releases/latest/download/aalias)

Via **homebrew**:
```
    brew tap rawnly/tap
    brew install aalias
```



## Usage

```bash
aalias md 'markdown -p'
```

```bash
USAGE:
    aalias [ARGS] [SUBCOMMAND]

ARGS:
    <name>        Name of the alias
    <value>...    Value of the alias

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    disable    Disable an alias
    enable     Enable an alias
    help       Prints this message or the help of the given subcommand(s)
    list       List alla aliases
    setup      Setup basic configuration
```
