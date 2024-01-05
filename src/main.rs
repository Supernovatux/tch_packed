use std::io;
use std::io::{Read, Write};
// use tch::{Device, Kind, Tensor};
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
fn main() {
    // let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    // let t = t * 2;
    // let t2 = Tensor::randn([4, 4], (Kind::Double, Device::Cpu));
    // t.print();
    // t2.print();
    pause();
}
