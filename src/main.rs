#![allow(unused)]
use clap::Parser;
use indicatif::ProgressBar;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::thread::sleep;
use std::time;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();

    // reads the ENTIRE file, and then works on it
    let content = std::fs::read_to_string(&args.path).expect("could not find file!");
    
    println!("this is the read_to_string result: ");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("line found: {}", line)
        }
    }

    // reads line by line from a buffer
    let f = File::open(&args.path).expect("cannot find file");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let first_byte_size = reader.read_line(&mut line).expect("can't read line");
    println!("\nthis is the buffer read method:");
    println!("First line is {} bytes long", first_byte_size);
    println!("\nline variable START-->{}<--END\n", line);
    let second_byte_size = reader.read_line(&mut line).expect("can't read line");
    println!("Second line is {} bytes long", second_byte_size);
    println!("\nline variable START-->{}<--END\n", line);
    println!("turns out read_line, and then passing the mutable variable line in");
    println!("actually ADDS the line to the buffer");


    // https://rust-cli.github.io/book/tutorial/output.html
    let pb = indicatif::ProgressBar::new(10);
    for i in 0..10 {
        let lots = time::Duration::from_millis(100);
        sleep(lots);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    println!("grrs complete!");
}

// cargo run foo ./test.txt

// run it with cargo test, auto-finds and runs them
#[test]
fn hello_equals_hello() {
    let foo = "hello";
    assert_eq!( foo, "hello")
}

