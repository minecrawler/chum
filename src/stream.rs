use chunk::Chunk;


pub trait ReadableStream<T> {
    fn read(&mut self) -> Option<Chunk<T>>;
}

pub trait Stream<'a, T> {
    fn close(&mut self);
    fn closed(&self) -> bool;
    fn pipe<S>(&mut self, stream: &'a mut S)
        where S: WriteableStream<T> + Stream<'a, T>;
}

pub trait TransformStream<T> {
    fn transform<F>(&mut self, handler: F)
        where F: Fn(Chunk<T>) -> Chunk<T>;
}

pub trait WriteableStream<T> {
    fn write(&mut self, data: T);
}
