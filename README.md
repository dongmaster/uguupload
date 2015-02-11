# Uguupload
Uguupload is a program that allows you to upload files to http://uguu.se using the CLI.

It is possible to upload several files at once. See usage section.

Bug reports are very welcome. Same with suggestions.

If you don't want to compile the program, you can download binaries here: https://github.com/dongmaster/uguupload/releases
(Windows 64-bit and Linux 64-bit only available at the moment)

uguu~

## Usage
```
./uguupload --help
```

## Dependencies
You need to have [curl](curl.haxx.se) installed to use uguupload.

Your package manager (99% sure) has curl. I recommend installing it that way :>.

## Compiling
You need to have [Rust](http://rust-lang.org) installed to compile this. I assume you have cargo installed as well (If you don't, you should just be able to do ```rustc main.rs``` and it should compile).

```
git clone https://github.com/dongmaster/uguupload && cd uguupload && cargo build
```

## Supported platforms
If you can install curl and rust on your system, you can use this.
This has been tested on:
* Windows 7 64-bit
* Linux 64-bit

## Todo and what can be improved
Can the --help message be improved?

Maybe show a progress bar for uploads? Not sure how to do that though.


