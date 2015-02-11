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

Usage: ./uguupload [FILENAME] [FILE]

[FILENAME] is the name you want for the file.
[FILE] is the path to the file you want to upload.

Uploading several files at once is possible. See Example 2 below.

Please note that when supplying a filename for your file, filetypes can matter.
So if you upload foo.txt with the name foo, your browser will most likely prompt the person visiting the file with a download dialog window.

Example:
./uguupload foo foo.txt
This will return http://a.uguu.se/1234567890_[FILENAME]

Example 2:
./uguupload foo foo.txt bar bar.txt foobar foobar.txt");

        return;
    } else if args_len == 2 {
        panic!("\n\nYou fucked up! Not enough arguments were supplied.
If you want to know how to use this program, use the --help flag (./uguupload --help)\n\n");
    }
    
    for x in range(1, args_len - 1) {
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
                        println!("Upload failed.");
                    } else {
                        println!("Looks like your shitty file was uploaded. Here's the link to your file: \n{}", String::from_utf8_lossy(r.output.as_slice()));
                    }
                },
                Err(e)  => panic!("failed to upload the file: {}", e),
            }
        
        }
        
        counter += 1;
    }
}
