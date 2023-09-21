# sysinfobar

`sysinfobar` is a simple program that outputs Linux or OpenBSD system
information to stdout in a format that is expected to be piped to
[Xmobar](http://projects.haskell.org/xmobar/).

## Displayed information

* CPU usage in percent
* RAM usage in percent
* swap usage in percent
* network usage down/up in percent
* battery capacity as symbol and in minutes left (if available)
* date and time (currently only in German)

The information is also highlighted according to the
impact on the system.
