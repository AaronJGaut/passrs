* Fix issues with clipboard
    * Need to test generate and yank in cli and repl mode
        * Is it being properly cleared?
    * Generate should clear after a "press any key" prompt
* Add a way to automatically generate a password when creating or updating an account
    * Should generating be the default?
* Fix issue where help command recreates the whole command list independently of what is actually in use
* Fix panic on yanking when x isn't available
* Use pager for long output (list command)
* Research proper salting procedure - is it better/worse to resalt on each save?
* Confirm that generated passwords are using secure rng (os entropy?)
* Fix any other unintended panics
* Fix warnings
* Backups?
* Add a write option to write to a specified path
* Mark a database as modified if it came from an old file version
