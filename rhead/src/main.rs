use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut start_index = 1;
    let mut count = 10;
    if args.len() > start_index && args[start_index] == "-n" {
        if args.len() <= start_index + 1 {
            eprintln!("Missing value after -n");
            process::exit(1);
        }

        count = match args[start_index + 1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid number after -n: {}", args[start_index + 1]);
                process::exit(1);
            }
        };
        start_index += 2;
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
        let mut line_number = 0;

        for line_data in reader.lines() {
            if line_number > count { break; }
            let line = match line_data {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    continue;
                }
            };
            line_number += 1;

            if args.len() - start_index > 1 {
                println!("==> {} <==", file_name);
            }
            println!("{}", line);
        }
    }

}
