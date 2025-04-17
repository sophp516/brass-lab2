use std::env;
use std::process;
use std::fs::{File, remove_file};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f_flag = false;
    let mut i_flag = false;
    let mut n_flag = false;
    let mut start_index = 1;
    while start_index < args.len() && args[start_index].starts_with('-') {
        for flag in args[start_index].chars().skip(1) {
            match flag {
                'f' => {
                    f_flag = true;
                    i_flag = false;
                    n_flag = false;
                }
                'i' => {
                    i_flag = true;
                    f_flag = false;
                    n_flag = false;
                }
                'n' => {
                    n_flag = true;
                    f_flag = false;
                    i_flag = false;
                }
            }
        }
        start_index += 1;
    }

    if args.len() != start_index + 3 {
        eprint!("Incorrect number of arguments");
        process::exit(1);
    }
    
    let source_path = args[start_index];
    let target_path = Path::new(&args[start_index + 1]);

    let source_file = match File::open(source_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening source file {}: {}", source_path, e);
            process::exit(1);
        }
    };

    if target_path.exists() {
        if n_flag {
            println!("{} not overwritten (due to -n)", target_path.display());
            return;
        } else if i_flag {
            eprint!("overwrite {}? (y/n) ", target_path.display());
            let mut response = String::new();
            if let Err(e) = io::stdin().read_line(&mut response) {
                eprintln!("Failed to read input: {}", e);
                process::exit(1);
            }
            let trimmed = response.trim();
            if trimmed != "y" && trimmed != "Y" {
                println!("{} not overwritten", target_path.display());
                return;
            }
        }
    }

    let mut reader = BufReader::new(source_file);
    remove_file(source_path);

    let target_file = match File::create(target_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create destination file {}: {}", target_path.display(), e);
            process::exit(1);
        }
    };
    
    let mut writer = BufWriter::new(target_file);
    let mut buffer = [0; 8192];
    
}
