use std::process::{Command, Stdio};
use std::io::{self, BufReader, Write, BufWriter, prelude::Read};
use std::error::Error;
use std::thread;
use std::sync::{Mutex, atomic::AtomicBool, atomic::Ordering, Arc};

extern crate ctrlc;

fn main() {
    println!("Welcome to MCDaemon-Rust!");
    let mut svrargs = ["-Xmx1000M", "-Xms1000M", "-jar", "server.jar"];
    let mut svrdir = "minecraft";
    let mut svrproc = Command::new("java")
        .args(&svrargs)
        .current_dir(svrdir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let svrin = Mutex::new(svrproc.stdin.as_mut().unwrap());
    let mut svrout = BufReader::new(svrproc.stdout.as_mut().unwrap());
    let mut svrbuf = [0; 4096];
    let mut cmdbuf = [0; 4096];
    let mut cmdstr = String::new();

    let handle = thread::spawn(move || {
        loop {
            match std::io::stdin().read(&mut cmdbuf) {
                Err(why) => panic!("Input to server failed: {}", why.description()),
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
            Err(why) => panic!("couldn't read server stdout: {}", why.description()),
            Ok(n) if n > 0 => println!("{}",String::from_utf8_lossy( &svrbuf[..n])),
            Ok(n) if n == 0 => break,
            _ => break,
        }
    }

    svrproc.kill();

}
