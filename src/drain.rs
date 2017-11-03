use std::marker::PhantomData;

use stream::*;


pub struct Drain<T, F: Fn(T)> {
    writer: F,
    t: PhantomData<T>,
}

impl<T, F> Drain<T, F> where F: Fn(T) {
    pub fn new(writer: F) -> Self {
        Self {
            writer,
            t: PhantomData,
        }
    }
}

impl<T, F> WriteableStream<T> for Drain<T, F> where F: Fn(T) {
    fn write(&mut self, data: T) {
        (self.writer)(data);
    }
}

impl<'a, T, F> Stream<'a, T> for Drain<T, F> where F: Fn(T) {
    fn close(&mut self) {

    }

    fn closed(&self) -> bool {
        false
    }

    fn pipe<S>(&mut self, stream: &'a mut S)
    where S: WriteableStream<T> + Stream<'a, T> {

    }
}
