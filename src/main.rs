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
    let mut counter = 0;
    // I use this to get the file. Check line ~65.
    // If this is 0, then the filename will be the name of the file
    let mut file_index = 0;
    let mut start_index = 1;
    let mut if_index = 0;
    
    // I want to be able to index arguments so I push them to a vector.
    for x in shitty_args {
        match x.into_string() {
            Ok(r)   => args.push(r),
            Err(e)  => panic!("Something went wrong. It has something to do with command-line arguments. Error: {:?}", e),
        }
    }
    
    // Efficiency?
    let args_len = args.len();
    
    if args_len < 2 {
        panic!("\n\nYou fucked up! Not enough arguments were supplied.
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
        // Getting this to work can most likely be done in a better way.
        file_index = 1;
        start_index = 2;
        if_index = 1;
    } else if args_len == 2 {
        panic!("\n\nYou fucked up! Not enough arguments were supplied.
If you want to know how to use this program, use the --help flag (./uguupload --help)\n\n");
    }
    
    for x in range(start_index, args_len) {
        // Checking stuff so we don't go out of the index.
        if x + if_index + counter < args_len {
            let filename = format!("name={}", args[x + counter]);
            let file = format!("file=@{}", args[x + file_index + counter]);
            
            let mut curl = old_io::Command::new("curl");
                curl.args(&["-F", filename.as_slice()]);
                curl.args(&["-F", file.as_slice()]);
                curl.arg(upload_url);
            
            match curl.output() {
                Ok(r)   => {
                    if String::from_utf8_lossy(r.output.as_slice()) == "" {
                        if file_index == 1 {
                            println!("Upload failed! {}:{} (filename:file) didn't upload successfully.", args[x + counter], args[x + file_index + counter]);
                        } else {
                            println!("Upload failed! {} didn't upload successfully.", args[x + file_index + counter]);
                        }
                    } else {
                        println!("Looks like your shitty file was uploaded. Here's the link to your file: {}", String::from_utf8_lossy(r.output.as_slice()));
                    }
                },
                Err(e)  => panic!("failed to upload the file: {}", e),
            }
        
        }
        
        if file_index == 1 {
            counter += 1;
        }
    }
}
