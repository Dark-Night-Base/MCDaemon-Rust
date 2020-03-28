use std::process::{Command, Stdio};
use std::io::{BufReader, prelude::Read};
use std::error::Error;
use std::thread;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};
use colored::Colorize;

extern crate ctrlc;

fn main() {
    print!("{}", "
    88888888ba,   888b      88 88888888ba
    88      `\"8b  8888b     88 88      \"8b
    88        `8b 88 `8b    88 88      ,8P
    88         88 88  `8b   88 88aaaaaa8P'
    88         88 88   `8b  88 88\"\"\"\"\"\"8b,
    88         8P 88    `8b 88 88      `8b
    88      .a8P  88     `8888 88      a8P
    88888888Y\"'   88      `888 88888888P\"\n
    ".bright_black());
    println!("{}", "Welcome to MCDaemon-Rust by Dark-Night-Base!".blue());
    let svrargs = ["-Xmx1000M", "-Xms1000M", "-jar", "server.jar", "nogui"];
    let svrdir = "minecraft";
    let mut svrproc = Command::new("java")
        .args(&svrargs)
        .current_dir(svrdir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // let svrin = Mutex::new(svrproc.stdin.as_mut().unwrap());
    let mut svrout = BufReader::new(svrproc.stdout.as_mut().unwrap());
    let mut svrbuf = [0; 4096];
    let mut cmdbuf = [0; 4096];
    let mut cmdstr = String::new();

    thread::spawn(move || {
        loop {
            match std::io::stdin().read(&mut cmdbuf) {
                Err(why) => panic!("{} {}", "Input to server failed:".red(), why.description().red()),
                // Ok(n) => println!("You typed: {}", String::from_utf8_lossy( &cmdbuf[..n])),
                Ok(n) => {
                    cmdstr = format!("{}\n", String::from_utf8_lossy( &cmdbuf[..n]));
                    // writeln!(&mut svrin.lock().unwrap(), "{}", cmdstr).unwrap();
                    // svrin.write(cmdstr.as_bytes()).unwrap();
                },
            }
        }
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        match svrout.read(&mut svrbuf) {
            Err(why) => panic!("{} {}", "Read server stdout failed:".red(), why.description().red()),
            Ok(n) if n > 0 => println!("{}",String::from_utf8_lossy( &svrbuf[..n])),
            Ok(n) if n == 0 => break,
            _ => break,
        }
    }

    match svrproc.kill() {
        Err(why) => panic!("{} {}", "Kill server failed:".red(), why.description().red()),
        Ok(_) => println!("{}", "Server killed.".blue()),
    }

}
