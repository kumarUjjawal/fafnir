use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;
fn main() {
    let stdin = io::stdin();
    let bytes = stdin.bytes();
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in bytes {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?}\r", b);
        } else {
            println!("{:?} ({}) \r", b, c);
        }

        if c == 'q' {
            break;
        }
    }
}
