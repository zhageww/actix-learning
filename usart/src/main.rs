use std::io::Read;
use std::io::Write;
use serial_core::SerialDevice;
//use serial_core::SerialPort;
use std::path::Path;
use std::{time};
use serial_unix::TTYPort;
use std::vec::Vec;

const MAX_BUFF_SIZE:usize = 256;
fn main() {
    let tout = time::Duration::new(100, 0);
    let mut f = TTYPort::open(Path::new("/dev/ttyS4")).unwrap();
    f.set_timeout(tout);
    let mut buffer:Vec<u8> = vec![0; MAX_BUFF_SIZE];
    let mut s = String::from("");
    loop {
        buffer = [0;MAX_BUFF_SIZE].to_vec();
        f.write("Hello  ".as_bytes()).unwrap();
        f.write("world\r\n".as_bytes()).unwrap();
        f.read(&mut buffer).unwrap();
        buffer.retain(|&x| x != 0);
        s= String::from_utf8(buffer.clone()).unwrap();

        println!("res: {:?}  len: {:?}", s, s.len());
    }
    
}