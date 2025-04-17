use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::process;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(); // No arguments, print newline and exit
        process::exit(0);
    }

    let mut f_flag = false;
    let mut i_flag = false;
    let mut n_flag = false;
    let mut v_flag = false;

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
                'v' => v_flag = true,
                _ => {
                    eprintln!("Invalid option: -{}", flag);
                    process::exit(1);
                }
            }
        }
        start_index += 1;
    }

    let source_path = &args[start_index];
    let dest_path = Path::new(&args[start_index + 1]);

    let source_file = match File::open(source_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening source file {}: {}", source_path, e);
            process::exit(1);
        }
    };
    
    if dest_path.exists() {
        if n_flag {
            println!("{} not overwritten (due to -n)", dest_path.display());
            return;
        } else if i_flag {
            eprint!("overwrite {}? (y/n) ", dest_path.display());
            let mut response = String::new();
            if let Err(e) = io::stdin().read_line(&mut response) {
                eprintln!("Failed to read input: {}", e);
                process::exit(1);
            }
            let trimmed = response.trim();
            if trimmed != "y" && trimmed != "Y" {
                println!("{} not overwritten", dest_path.display());
                return;
            }
        }
    }

    let mut reader = BufReader::new(source_file);
    let dest_file = match File::create(dest_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create destination file {}: {}", dest_path.display(), e);
            process::exit(1);
        }
    };
    let mut writer = BufWriter::new(dest_file);

    let mut buffer = [0; 8192];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                if writer.write_all(&buffer[..n]).is_err() {
                    eprintln!("Write error to destination file.");
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error reading from source: {}", e);
                process::exit(1);
            }
        }
    }

    if v_flag {
        println!("{} -> {}", source_path, dest_path.display());
    }
}
