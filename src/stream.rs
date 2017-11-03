pub trait ReadableStream<T> {
    fn read(&mut self) -> Option<T>;
}

pub trait Stream<'a, T: Clone> {
    fn close(&mut self);
    fn closed(&self) -> bool;
    fn pipe<S>(&mut self, stream: &'a S)
        where S: WriteableStream<T> + Stream<'a, T>;
}

pub trait TransformStream<T> {
    fn transform<F>(&mut self, handler: F)
        where F: Fn(T) -> T;
}

pub trait WriteableStream<T: Clone> {
    fn write(&self, data: &T);
}
