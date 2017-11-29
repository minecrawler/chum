extern crate chum;

use std::rc::Rc;

use chum::*;


fn main() {
    let mut c = Chum::new();
    let mut s = Source::new();
    let p = Pipe::new(|x| x + 1);
    let d = Drain::new(
        |x| println!("{}", x),
        || {},
        || false,
    );

    let _ = c.register(p);
    let _ = c.register(d);

    c.build_pipeline();

    s.pipe(Rc::new(c));
    s.push(42);
}
