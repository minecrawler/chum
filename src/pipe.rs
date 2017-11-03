use std::collections::LinkedList;

use stream::*;

pub struct Pipe<'a, T: 'a, F: Fn(T) -> T> {
    buf: LinkedList<T>,
    is_closed: bool,
    pipe_target: Option<&'a mut WriteableStream<T>>,
    transformer: F,
}

impl<'a, T, F> Pipe<'a, T, F> where F: Fn(T) -> T {
    pub fn new(transformer: F) -> Self {
        Self {
            buf: LinkedList::new(),
            is_closed: false,
            pipe_target: None,
            transformer,
        }
    }
}

impl<'a, T, F> ReadableStream<T> for Pipe<'a, T, F> where F: Fn(T) -> T {
    fn read(&mut self) -> Option<T> {
        self.buf.pop_front()
    }
}

impl<'a, T, F> Stream<'a, T> for Pipe<'a, T, F> where F: Fn(T) -> T {
    fn close(&mut self) {
        if self.is_closed {
            panic!("Stream is already closed!");
        }

        self.is_closed = true;
    }

    #[inline]
    fn closed(&self) -> bool {
        self.is_closed
    }

    fn pipe<S>(&mut self, stream: &'a mut S)
    where S: WriteableStream<T> + Stream<'a, T> {
        while let Some(c) = self.read() {
            stream.write(c);
        }

        self.pipe_target = Some(stream);
    }
}

impl<'a, T, F> WriteableStream<T> for Pipe<'a, T, F> where F: Fn(T) -> T {
    fn write(&mut self, data: T) {
        if self.is_closed {
            panic!("Cannot push to closed stream");
        }

        let data = (self.transformer)(data);
        if let Some(ref mut ws) = self.pipe_target {
            ws.write(data);
        }
        else {
            self.buf.push_back(data);
        }
    }
}
