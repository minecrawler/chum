use std::marker::PhantomData;
use std::rc::Rc;

use stream::*;

pub struct Drain<'a, T> {
    close_handler: Option<Box<'a + Fn()>>,
    closed_checker: Option<Box<'a + Fn() -> bool>>,
    pipe_target: Option<Rc<'a + WriteableStream<T>>>,
    writer: Box<'a + Fn(&T)>,
    t: PhantomData<T>,
    #[allow(dead_code)]
    l: &'a PhantomData<()>
}

impl<'a, T> Drain<'a, T> {
    // todo: add builder
    pub fn new<F, H, C> (writer: F, close_handler: Option<H>, closed_checker: Option<C>) -> Self
    where F: 'a + Fn(&T), H: 'a + Fn(), C: 'a + Fn() -> bool {
        Self {
            close_handler: if close_handler.is_some() { Some(Box::new(close_handler.unwrap())) } else { None },
            closed_checker: if closed_checker.is_some() { Some(Box::new(closed_checker.unwrap())) } else { None },
            pipe_target: None,
            writer: Box::new(writer),
            t: PhantomData,
            l: &PhantomData,
        }
    }
}

impl<'a, T> Stream<'a, T> for Drain<'a, T> {
    fn close(&mut self) {
        if let Some(ch) = self.close_handler {
            (ch)();
        }
    }

    fn closed(&self) -> bool {
        if let Some(cc) = self.closed_checker {
            (cc)()
        }
        else { false }
    }

    fn pipe(&mut self, stream: Rc<'a + WriteableStream<T>>) {
    //where S: 'a + WriteableStream<T> {
        self.pipe_target = Some(Rc::clone(&stream));
    }
}

impl<'a, T> WriteableStream<T> for Drain<'a, T> {
    fn write(&self, data: T) {
        if let Some(ref ws) = self.pipe_target {
            (self.writer)(&data);
            (*ws).write(data);
        }
        else {
            (self.writer)(&data);
        }
    }
}
