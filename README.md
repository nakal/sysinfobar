# sysinfobar

`sysinfobar` is a simple program that outputs Linux or OpenBSD system
information to stdout in a format that is expected to be piped to
[Xmobar](http://projects.haskell.org/xmobar/).

## Screenshots

### FontAwesome support

![Screenshot of sysinfobar using FontAwesome](doc/img/sysinfobar-fontawesome.png)

See [how to compile](#With FontAwesome support) and [configuration below](#Example configuration for xmobar).

### Simple view

![Screenshot of simple sysinfobar](doc/img/sysinfobar-simple.png)

## Displayed information

* CPU usage in percent
* RAM usage in percent
* swap usage in percent
* network usage down/up in percent
* battery capacity as symbol and in minutes left (if available)
* date and time (currently only in German)

The information is also highlighted according to the
impact on the system.

## How to build sysinfobar

### With FontAwesome support

```
cargo build --features fontawesome
```

### Simple build

```
cargo build
```

## Example configuration for xmobar

```
Config {
	font =         "xft:Fantasque Sans Mono:size=13:bold:antialias=true"
	, additionalFonts = [ "xft:FontAwesome" ]
	, bgColor =      "#202020"
	, fgColor =      "orange"
	, alpha = 255
	, position =     TopW R 40
	, border =       NoBorder
	, borderColor =  "black"

	, sepChar =  "%"
	, alignSep = "}{"
	, iconRoot = ""
	, template = "}{ %UnsafeStdinReader%"

	, textOffset = -1
	, iconOffset = -1

	, lowerOnStart =     True
	, hideOnStart =      False
	, allDesktops =      True
	, overrideRedirect = True
	, pickBroadest =     False
	, persistent =       True

	, commands = [ Run UnsafeStdinReader ]
}
```

## How to use with xmobar

```
sysinfobar | xmobar -d sysinfo_xmobar.rc
```
