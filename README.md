# Uguupload
Uguupload is a program that allows you to upload files to http://uguu.se using the CLI.

It is possible to upload several files at once. See usage section.

Bug reports are very welcome. Same with suggestions.

If you don't want to compile the program, you can download binaries here: https://github.com/dongmaster/uguupload/releases
(Windows 64-bit and Linux 64-bit only available at the moment)

uguu~

![uguu]
(http://i.imgur.com/LRhtsze.jpg)

## Usage
```./uguupload --help```

```
Usage: ./uguupload [option] [FILE]

[FILE] is the path to the file you want to upload.

OPTIONS
    -h, --help
        Prints out this help message.

    -f, --filename
        Allows you to specify custom filenames for files.
        See Example 2.
    
    -r, --random
        Gives files random filenames.
        See Example 3.
        
    -d, --dir
        Uploads everything in the specified directory.
        This is not recursive.
        See Example 4.
        
    -dr, --dir-rec
        Uploads everything in the specified directory.
        This is recursive. Files in sub-directories will be uploaded.
        See Example 4. This works exactly like -d.

EXAMPLES
    Example 1:
        ./uguupload [FILE]
        ./uguupload foo.txt
        
        You can also upload several files.
        ./uguupload foo.txt bar.txt foobar.txt

    Example 2:
        ./uguupload -f [FILE] [FILENAME]
        ./uguupload -f foo.txt foo.txt bar.txt foo.txt foobar.txt foo.txt
        
        The first argument, foo.txt is the filename and the second argument, foo.txt, is the file and so on.
        bar.txt is the filename and foo.txt is the file. Same for foobar.txt and foo.txt
        
    Example 3:
        ./uguupload -r [FILE]
        ./uguupload -r foo.txt
        
        This will return http://a.uguu.se/asdfghjkl.ext where .ext is the file extension/type
        
    Example 4:
        ./uguupload -d [DIRECTORY]
        This will upload everything in the specified directory. You can add several directories
        
CONFIGURATION
    links_only:
        Boolean. This can be either true or false. Nothing else.
        
        If true, only links will be outputted, no filename.
        If false, the filename and the uguu.se link will outputted.
        
        Default is false.
```

## Configuration
You can now configure the program to change the output.
The config file is located in your home folder/.uguupload/config

In the config file you will be able to change one (at the moment) option.

The help section in the program explains what each option does.

## Dependencies
You need to have [curl](http://curl.haxx.se) installed to use uguupload.

Your package manager (99% sure) has curl. I recommend installing it that way :>.

## Compiling
You need to have [Rust](http://rust-lang.org) installed to compile this. I assume you have cargo installed as well (If you don't, you should just be able to do ```rustc main.rs``` and it should compile).

You need the latest nightly of rust

```
git clone https://github.com/dongmaster/uguupload && cd uguupload && cargo build --release
```

## Supported platforms
If you can install curl (and rust, if you are compiling uguupload) on your system, you can use this.
This has been tested on:
* Windows 7 64-bit
* Linux 64-bit

## Todo and what can be improved
Maybe show a progress bar for uploads? Not sure how to do that though.

Stop sending unnecessary requests to uguu (if you try to upload a file that doesn't exist, a request will still be sent).


