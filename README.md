# AddAlias
> Easily manage bash aliases.

## Usage

```sh
USAGE:
    add-alias [ARGS] [SUBCOMMAND]

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
