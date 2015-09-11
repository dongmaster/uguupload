/*
 *  This source code is subject to the terms of the 2-clause BSD license (FreeBSD License).
 *  If a copy of the 2-clause BSD license was not included with this file,
 *  you can refer to http://en.wikipedia.org/wiki/BSD_licenses#2-clause_license_.28.22Simplified_BSD_License.22_or_.22FreeBSD_License.22.29
 *  or http://opensource.org/licenses/BSD-2-Clause
 * */

#![feature(fs_walk)]
#![feature(path_ext)]

extern crate rustc_serialize;

use std::env;
use std::process;
use std::fs;
use std::path::Path;
use std::fs::PathExt;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::str::FromStr;

use rustc_serialize::json;

const URL : &'static str = "http://uguu.se/api.php?d=upload";

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    links_only: bool,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let config = first_run();

    if args.len() == 1 {
        panic!("\n\n    Not enough arguments were supplied!\n\n")
    }

    handle_arguments(args, config);
}

fn first_run() -> Config {
    let home = env::home_dir().unwrap();
    let home_dir = Path::new(&home);

    let config_dir = home_dir.join(".uguupload");
    let config_dir_path = Path::new(&config_dir);

    let config_file = config_dir.join("config");

    if config_dir_path.is_dir() == false {
        // Create the initial config directory and then the config file
        match fs::create_dir(config_dir_path) {
            Ok(_)   => (),
            Err(e)  => panic!("Failed to create ~/.uguupload! Error: {}", e),
        }

        let default_config = Config {
            links_only: false,
        };

        save_config(default_config);
    }

    load_config()
}

fn handle_arguments(args: Vec<String>, config: Config) {
    // Handles command-line arguments

    let arg = args[1].clone();

    match arg.as_ref() {
        "-h"  | "--help"        => help(),
        "-f"  | "--filename"    => upload_f(args, config),
        "-r"  | "--random"      => upload_r(args, config),
        "-d"  | "--dir"         => upload_d(args, config),
        "-dr" | "--dir-rec"     => upload_dr(args, config),
        "-c"  | "--config"      => change_config(&args),
        "-lc" | "--list-config" => list_config(),
        _                       => upload_n(args, config),
    }
}

fn change_config(args: &Vec<String>) {
    if args.len() != 4 {
        panic!("Too many or too little arguments were supplied!");
    }

    let ref config_parameter = args[2];
    let ref config_value = args[3];

    let mut current_config: Config = load_config();

    let links_only = "links_only".to_string();

    match config_value {
        links_only  => current_config.links_only = match FromStr::from_str(config_value.as_ref()) {
            Ok(o)   => o,
            Err(e)  => panic!("HELP: {}", e),
        },
    }

    save_config(current_config);
}

fn load_config() -> Config {
    let home = env::home_dir().unwrap();
    let home_dir = Path::new(&home);

    let config_dir = home_dir.join(".uguupload");

    let config_file = config_dir.join("config");

    let mut boop = File::open(&config_file).unwrap();
    let mut output_from_config = String::new();

    let content = match File::read_to_string(&mut boop, &mut output_from_config) {
        Ok(o)   => o,
        Err(e)  => panic!("HELP: {}", e),
    };

    json::decode::<Config>(&output_from_config).unwrap();
}

fn save_config(new_config: Config) {
    let home = env::home_dir().unwrap();
    let home_dir = Path::new(&home);

    let config_dir = home_dir.join(".uguupload");

    let config_file = config_dir.join("config");

    let conf_file = File::create(&config_file);

    conf_file.unwrap().write_all(json::encode(&new_config).unwrap().as_bytes());
}

fn list_config() {
    // This is really shitty.
    // Use the serde crate to do this in a better way.
    let config: Config = load_config();

    println!("links_only : {}", config.links_only);
}

fn upload(f: &String, file: String, filename: String, random: bool, config: &Config) {
    // Handles the uploading of files.

    let mut curl = process::Command::new("curl");
            curl.args(&["-F", filename.as_ref()]);
            curl.args(&["-F", file.as_ref()]);
            if random == true {
                curl.args(&["-F", "randomname=true"]);
            }
            curl.arg(URL);

    match curl.output() {
        Ok(r)   => {
            if String::from_utf8_lossy(r.stdout.as_ref()) == "" {
                println!("Failed to upload: {}", f);
            } else {
                if config.links_only == true {
                    println!("{}", String::from_utf8_lossy(r.stdout.as_ref()));
                } else {
                    println!("{}\n{}", f, String::from_utf8_lossy(r.stdout.as_ref()));
                }
            }
        },
        Err(e)  => panic!("Failed to upload: {}", e),
    }
}

fn upload_n(args: Vec<String>, config: Config) {
    // NORMAL
    // Handles what files should be uploaded along with what filename the file should have.

    let files = &args[1..args.len()];

    for f in files {
        let filename = format!("name={}", f);
        let file = format!("file=@{}", f);

        upload(f, file, filename, false, &config);
    }
}

fn upload_f(args: Vec<String>, config: Config) {
    // FILENAME
    // Handles what files should be uploaded along with that filename the file should have.
    // -f was used here.
    let files = &args[2..args.len()];
    let mut counter = 0;

    for x in 0..files.len() {
        if x + 1 + counter < files.len() {
            let filename = format!("name={}", files[x + 1 + counter]);
            let file = format!("file=@{}", files[x + counter]);

            upload(&files[x + counter], file, filename, false, &config);
        }

        counter += 1;
    }
}

fn upload_r(args: Vec<String>, config: Config) {
    // RANDOM
    // Handles what files should be uploaded. Filenames are random.
    // -r was used here
    let files = &args[2..args.len()];

    for f in files {
        let filename = format!("name={}", f);
        let file = format!("file=@{}", f);

        upload(f, file, filename, true, &config);
    }
}

fn upload_d(args: Vec<String>, config: Config) {
    // DIRECTORY
    // Handles what files in the directory should be uploaded.
    // -d was used here.

    let directories = &args[2..args.len()];

    for d in directories {
        let path = Path::new(d);
        let mut fls: Vec<_> = vec!();

        if path.is_dir() == true {
            match fs::read_dir(&path) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => for path in paths {
                    fls.push(path.unwrap().path());
                },
            }

            for x in fls {
                let filename_processed = x.as_path().file_name().unwrap().to_str().unwrap().to_string();
                let file_processed = x.as_path().to_str().unwrap().to_string();

                let filename = format!("name={}", filename_processed);
                let file = format!("file=@{}", file_processed);

                upload(&file_processed, file, filename, false, &config);
            }
        }
    }
}

fn upload_dr(args: Vec<String>, config: Config) {
    // DIRECTORY
    // Handles what files in the directory should be uploaded.
    // -d was used here.

    let directories = &args[2..args.len()];

    for d in directories {
        let path = Path::new(d);
        let mut fls: Vec<_> = vec!();

        if path.is_dir() == true {
            match fs::walk_dir(&path) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => for path in paths {
                    fls.push(path.unwrap().path());
                },
            }

            for x in fls {
                let filename_processed = x.as_path().file_name().unwrap().to_str().unwrap().to_string();
                let file_processed = x.as_path().to_str().unwrap().to_string();

                let filename = format!("name={}", filename_processed);
                let file = format!("file=@{}", file_processed);

                upload(&file_processed, file, filename, false, &config);
            }
        }
    }
}

fn help() {
    println!("Usage: ./uguupload [option] [FILE]

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

    -c, --config
        Changes config values
        See Example 5

    -lc, --list-config
        Lists config options

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

    Example 5:
        ./uguupload -c [PARAMETER] [VALUE]
        ./uguupload -c links_only true
        This will change the config option links_only to true.

CONFIGURATION
    links_only:
        Type: boolean. This can be either true or false. Nothing else.

        If true, only links will be outputted, no filename.
        If false, the filename and the uguu.se link will outputted.

        Default is false.");

    return;
}
