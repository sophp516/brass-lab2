use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(); // No arguments, print newline and exit
        process::exit(0);
    }

    // Flags for -b, -n, -s
    let mut b = false;
    let mut n = false;
    let mut s = false;

    // Start reading file names from this index
    let mut start_index = 1;

    // Parse flags if present
    if args[1].starts_with('-') {
        for flag in args[1].chars().skip(1) {
            match flag {
                'b' => b = true,
                'n' => n = true,
                's' => s = true,
                _ => {
                    eprintln!("Invalid option: -{}", flag);
                    process::exit(1);
                }
            }
        }
        start_index = 2;
    }

    for i in start_index..args.len() {
        let file_name = &args[i];
        let file = match File::open(file_name) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening file {}: {}", file_name, e);
                continue;
            }
        };

        let reader = BufReader::new(file);
        let mut line_number = 1;
        let mut prev_blank = false;

        for line_data in reader.lines() {
            let line = match line_data {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    continue;
                }
            };

            let is_blank = line.trim().is_empty();

            // Squeeze multiple adjacent blank lines
            if s && is_blank && prev_blank {
                continue;
            }

            // Number non-blank lines
            // -b overrides -n
            if b {
                if !is_blank {
                    println!("{:>6}\t{}", line_number, line);
                    line_number += 1;
                } else {
                    println!();
                }
            }
            // Number all lines
            else if n {
                println!("{:>6}\t{}", line_number, line);
                line_number += 1;
            }
           
            else {
                println!("{}", line);
            }

            prev_blank = is_blank;
        }
    }
}
