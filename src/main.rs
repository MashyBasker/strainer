use std::{error::Error, io::{BufRead, BufReader}, process::{Command, Stdio}};
use std::io::{self, Write};
use std::fs::File;
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

    let mut program_args: Vec<String> = args.collect();

    // Check for --out= argument and extract output path if present
    let out_path = if let Some(pos) = program_args.iter().position(|arg| arg.starts_with("--out=")) {
        let path = program_args[pos].trim_start_matches("--out=").to_string();
        program_args.remove(pos);
        Some(path)
    } else {
        None
    };

    let mut child = Command::new(&program)
        .args(&program_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let re = Regex::new(&pattern)?;
    
    let out: Box<dyn Write + Send> = if let Some(path) = out_path {
        Box::new(File::create(path)?)
    } else {
        Box::new(io::stdout())
    };
    
    let writer = std::sync::Arc::new(std::sync::Mutex::new(out));

    let re1 = re.clone();
    let w1 = writer.clone();
    let t1 = std::thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                if re1.is_match(&line) {
                    let mut out = w1.lock().unwrap();
                    writeln!(out, "{line}").ok();
                }
            }
        }
    });

    let re2 = re.clone();
    let w2 = writer.clone();
    let t2 = std::thread::spawn(move || {
        for line in stderr.lines() {
            if let Ok(line) = line {
                if re2.is_match(&line) {
                    let mut out = w2.lock().unwrap();
                    writeln!(out, "{line}").ok();
                }
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let status = child.wait()?;
    eprintln!("child exited with: {status}");

    Ok(())
}