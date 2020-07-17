use std::net::{TcpStream, IpAddr};
use std::io;

pub fn main(){
    let mut ip_addr = String::new();

    println!("Enter your switch's IP Address: ");
    io::stdin().read_line(&mut ip_addr)
        .ok()
        .expect("Couldn't read line");

    let ip: IpAddr = ip_addr.trim()
        .replace(" ", "")
        .parse()
        .unwrap();

    println!("---------------------------------------------------------------");

    let stdout = std::io::stdout();

    loop {
        if let Ok(mut logger) = TcpStream::connect((ip, 6969)) {
            let _ = std::io::copy(&mut logger, &mut stdout.lock());
        }
    }
}