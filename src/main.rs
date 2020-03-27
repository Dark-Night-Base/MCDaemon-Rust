use std::process::{Command, Stdio};
use std::io::{BufReader, Write, BufWriter};
use std::io::prelude::*;
use std::error::Error;
use std::thread;

fn main() {
    println!("Welcome to MCDaemon-Rust!");
    let mut svrargs = ["-Xmx1000M", "-Xms1000M", "-jar", "server.jar"];
    let mut svrproc = Command::new("java")
        .args(&svrargs)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();


    let svrin = svrproc.stdin.as_mut().unwrap();
    let mut svrout = BufReader::new(svrproc.stdout.as_mut().unwrap());
    let mut bufin = [0; 4096];

    loop {
        // svrin.write("ls\n".as_bytes()).unwrap();
        // match svrproc.stdout.unwrap().read_to_string(&mut line) {
        // match svrout.read_line(&mut line) {
        match svrout.read(&mut bufin) {
            Err(why) => panic!("couldn't read bash stdout: {}", why.description()),
            Ok(n) => println!("{}", String::from_utf8_lossy( &bufin[..n])),
        }
    }
}
