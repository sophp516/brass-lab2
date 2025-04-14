use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!();
        process::exit(0);
    }

    if args.len() > 1 && args[1] == "-n" {
        for i in 2..args.len() {
            print!("{}", args[i]);
            if i < args.len() - 1 {
                print!(" "); 
            }
        }
    } else {
        for i in 1..args.len() {
            print!("{}", args[i]);
            if i < args.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }

    process::exit(0);

}
