`passrs` - A simple command-line passwork manager

This is a port of my `passman` project from python to rust.
My first goal is to reach feature pairity with `passman`, then I plan to add some new capabilities:

* Fuzzy searching entries by name
* Abbreviated subcommands from CLI (that is, not TUI) mode
* Single-entry secrets, ie entries that don't include a username
* Switch to a more cross-language parsable file format (json?)
* Timeout
