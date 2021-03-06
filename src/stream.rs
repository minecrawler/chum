use std::rc::Rc;


pub enum StreamType {
    Readable,
    Transformable,
    Writeable,
}

pub trait PausableStream {
    fn cork(&mut self);
    fn corked(&self) -> bool;
    fn uncork(&mut self);
}

pub trait ReadableStream<'a, T> {
    fn data<F>(&mut self, handler: F)
        where F: 'a + Fn(&T);
    fn read(&mut self) -> Option<T>;
}

pub trait Stream<'a, T> {
    fn close(&mut self);
    fn closed(&self) -> bool;
    fn pipe(&mut self, stream: Rc<'a + WriteableStream<T>>);
    //    where S: 'a + WriteableStream<T>;
}

pub trait TransformableStream<'a, T> {
    fn transform (&self, data: T) -> T;

    fn transformer<F>(&mut self, handler: F)
        where F: 'a + Fn(T) -> T;
}

pub trait WriteableStream<T> {
    fn write(&self, data: T);
}
