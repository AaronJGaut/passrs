`passrs` - A simple command-line passwork manager

This is a port of my `passman` project from python to rust.
My first goal is to reach feature pairity with `passman`, then I plan to add some new capabilities:

* Fuzzy searching entries by name
* Abbreviated subcommands from CLI (that is, not TUI) mode
* Single-entry secrets, ie entries that don't include a username
* Switch to a more cross-language parsable file format (json?)
* Timeout

# Usage

There are two ways to use `passrs`:

* Call `passrs` with a subcommand to perform that operation.
* Call `passrs` without a subcommand to enter an interactive session. This saves you from having to repeatedly enter the master password for each operation.

Use the `--help` flag to get more information.

There are several environment variables that are looked at:

* `PASSRS_DB` - sets the default location of the database file
* `EDITOR` - preferred program for editing notes
* `PAGER` - preferred program for scrolling through long text
* `PASSMAN_DB` - deprecated: equivalent to `PASSRS_DB`
