extern crate chum;

use chum::*;

fn main() {
    let mut d = Drain::new(|data| println!("{}", data));
    let mut p = Pipe::new(|data| data + 1);
    let mut s = Source::new();

    s.push(42);
    s.push(17);
    p.pipe(&mut d);
    s.pipe(&mut p);
    s.end(7);
}
