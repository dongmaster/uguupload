#![feature(env)]
#![feature(core)]
#![feature(io)]
#![feature(os)]

use std::env;
use std::old_io;

fn main() {
    let upload_url = "http://uguu.se/api.php?d=upload";
    let shitty_args = env::args();
    let mut args = Vec::new();
    let counter: usize = 0;
    
    // I want to be able to index arguments so I push them to a vector.
    for x in shitty_args {
        match x.into_string() {
            Ok(r)   => args.push(r),
            Err(e)  => panic!("Something went wrong. It has something to do with command-line arguments. Error: {:?}", e),
        }
    }
    
    // Efficiency?
    let args_len: usize = args.len();
    
    if args_len < 2 {
        panic!("\n\nYou fucked up. Not enough arguments were supplied.
If you want to know how to use this program, use the --help flag (./uguupload --help)\n\n");
    }
    
    if args[1] == "--help" {
        println!("Uguupload - A program for uploading files to http://uguu.se

Usage: ./uguupload [options] [FILE]

[FILENAME] is the name you want for the file.
[FILE] is the path to the file you want to upload.

Uploading several files at once is possible. See Example 2 below.

OPTIONS
    -f, --filenames
        Allows you to specify filenames for files.
        See Example 3.

EXAMPLES
    Example 1:
        ./uguupload foo.txt
        This will return http://a.uguu.se/sdfyse_[FILE]

    Example 2:
        ./uguupload foo.txt bar.txt foobar.txt
        This will return three (3) files

    Example 3:
        ./uguupload -f foo.txt foo.txt bar.txt foo.txt foobar.txt foo.txt
        
        The first argument, foo.txt is the filename and the second argument, foo.txt, is the file and so on.
        bar.txt is the filename and foo.txt is the file. Same for foobar.txt and foo.txt
        
        Syntax in other words:
        ./uguupload -f [FILENAME] [FILE]");

        return;
    } else if args[1] == "-f" || args[1] == "--filenames" {
        upload_files_filenames(upload_url, args, args_len, counter);
    } else {
        upload_files(upload_url, args, args_len, counter);
    }
}

// Upload files where the name of the file is the filename.
fn upload_files(upload_url: &str, args: Vec<String>, args_len: usize, counter: usize) {
    for x in range(1, args_len) {
        // Checking stuff so we don't go out of the index.
        if x + counter < args_len {
            let filename = format!("name={}", args[x + counter]);
            let file = format!("file=@{}", args[x + counter]);
            
            let mut curl = old_io::Command::new("curl");
                curl.args(&["-F", filename.as_slice()]);
                curl.args(&["-F", file.as_slice()]);
                curl.arg(upload_url);
            
            match curl.output() {
                Ok(r)   => {
                    if String::from_utf8_lossy(r.output.as_slice()) == "" {
                        println!("Failed to upload: {}", args[x + counter]);
                    } else {
                        println!("\nSuccessfully uploaded {}\n{}", args[x + counter], String::from_utf8_lossy(r.output.as_slice()));
                    }
                },
                Err(e)  => panic!("Failed to upload: {}", e),
            }
        }
    }
}

//Upload files with user-specified filenames
fn upload_files_filenames(upload_url: &str, args: Vec<String>, args_len: usize, mut counter: usize) {
    // The index starts at two (2) because of the flag
    for x in range(2, args_len) {
        // Checking stuff so we don't go out of the index.
        if x + 1 + counter < args_len {
            let filename = format!("name={}", args[x + counter]);
            let file = format!("file=@{}", args[x + 1 + counter]);
            
            let mut curl = old_io::Command::new("curl");
                curl.args(&["-F", filename.as_slice()]);
                curl.args(&["-F", file.as_slice()]);
                curl.arg(upload_url);
            
            match curl.output() {
                Ok(r)   => {
                    if String::from_utf8_lossy(r.output.as_slice()) == "" {
                        println!("Failed to upload {}:{} (filename:file)", args[x + counter], args[x + 1 + counter]);
                    } else {
                        println!("\nSuccessfully uploaded {}:{} (filename:file)\n{}", args[x + counter], args[x + 1 + counter], String::from_utf8_lossy(r.output.as_slice()));
                    }
                },
                Err(e)  => panic!("Failed to upload: {}", e),
            }
        }
        
        counter += 1;
    }
}
