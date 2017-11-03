extern crate chum;

use chum::*;

fn main() {
    // Define segments
    let d2 = Drain::new(
        |data| println!("2nd trans\t{}", data),
        || {},
        || false,
    );
    let mut p2 = Pipe::new(|data| data + 1);
    let mut d1 = Drain::new(
        |data| println!("1st trans\t{}", data),
        || {},
        || false,
    );
    let mut p1 = Pipe::new(|data| data + 1);
    let mut d0 = Drain::new(
        |data| println!("No trans\t{}", data),
        || {},
        || false,
    );
    let mut s = Source::new();

    // Push anytime
    s.push(42);
    s.push(17);
    s.end(7);

    // Build the whole pipeline before connecting it to the source
    // Alternatively, you can also just cork the source first
    // and uncork it once you are ready to process chunks
    p2.pipe(&d2);
    d1.pipe(&p2);
    p1.pipe(&d1);
    d0.pipe(&p1);

    // Connect it to the source in the very end
    s.cork();
    s.pipe(&d0);
    println!("Sample for a simple stream");
    s.uncork();
}
