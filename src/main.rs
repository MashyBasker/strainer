use std::{error::Error, io::{BufReader, BufRead}, process::{Command, Stdio}};
use regex::Regex;
use std::env;


fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let pattern = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: strainer <pattern> <program> [args...]");
            std::process::exit(1);
        }
    };

    let program = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: strainer <pattern> <program> args");
            std::process::exit(1);
        }
    };

    let program_args: Vec<String> = args.collect();

    let mut child = Command::new(program)
        .args(program_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let re = Regex::new(&pattern)?;

    let re1 = re.clone();
    let t1 = std::thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                if re1.is_match(&line) {
                    println!("{line}");
                }
            }
        }
    });

    let re2 = re.clone();
    let t2 = std::thread::spawn(move || {
        for line in stderr.lines().flatten() {
            if re2.is_match(&line) {
                println!("{line}");
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let status = child.wait()?;
    eprintln!("child exited with: {status}");

    Ok(())
}
