use std::{thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = String::from("localhost:21234"))]
    host: String,

    #[clap(short, long)]
    file: String,

    #[clap(short, long)]
    sleep: u64,

    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    println!("args => {:?}", args);
    match TcpStream::connect(args.host.as_str()) {
        Ok(mut stream) => {
            let start = time::Instant::now();
            println!("Successfully connected {}", args.host.as_str());
            let file = File::open(args.file.as_str()).unwrap();
            let reader = BufReader::new(file);
            let mut total_lines = 0;
            let mut ok = 0;
            for line in reader.lines() {
                total_lines += 1;
                match stream.write(format!("{}\r\n", line.unwrap()).as_bytes()) {
                    Ok(v) if args.debug => {
                        println!("{} written {} bytes",  args.file.as_str(), v);
                        println!("{} send msg to {} ok", args.file.as_str(), args.host.as_str());
                        ok += 1;
                    }
                    Ok(_v) => {
                        ok += 1;
                    }
                    Err(e) => {
                        println!("writ error: {}", e);
                        break;
                    }
                }
                match stream.flush() {
                    Ok(()) => {}
                    Err(fe) => {
                        eprintln!("flush write error {:?}", fe);
                    }
                }
                if args.sleep > 0 {
                    let sleep_ms: time::Duration = time::Duration::from_millis(args.sleep);
                    thread::sleep(sleep_ms);
                }
            }
            let cost = start.elapsed().as_micros();
            println!("cost {} ms", cost/1000);
            println!("total lines {}, ok {},  qps {}/s", total_lines, ok, total_lines * 1000 * 1000/cost);
        }
        Err(e) => {
            eprintln!("Failed to connect args {:?} , result => {:?}", args, e);
        }
    }
    println!("Terminated.");
}
