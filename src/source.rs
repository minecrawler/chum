use std::collections::LinkedList;

use stream::*;


pub struct Source<'a, T: 'a> {
    buf: LinkedList<T>,
    is_closed: bool,
    pipe_target: Option<&'a WriteableStream<T>>,
}

impl<'a, T> Source<'a, T>
where
    T: Clone {

    #[inline]
    pub fn end(&mut self, data: T) {
        if let Some(ref mut ws) = self.pipe_target {
            ws.write(&data);
        }
        else {
            self.buf.push_back(data);
        }

        self.close();
        self.pipe_target = None;
    }

    pub fn new() -> Self {
        Self {
            buf: LinkedList::new(),
            is_closed: false,
            pipe_target: None,
        }
    }

    pub fn push(&mut self, data: T) {
        if self.is_closed {
            panic!("Cannot push to closed source");
        }

        if let Some(ref mut ws) = self.pipe_target {
            ws.write(&data);
        }
        else {
            self.buf.push_back(data);
        }
    }
}

impl<'a, T> ReadableStream<T> for Source<'a, T> {
    #[inline]
    fn read(&mut self) -> Option<T> {
        self.buf.pop_front()
    }
}

impl<'a, T> Stream<'a, T> for Source<'a, T>
where
    T: Clone {

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

    fn pipe<S>(&mut self, stream: &'a S)
    where S: WriteableStream<T> + Stream<'a, T> {
        while let Some(c) = self.read() {
            stream.write(&c);
        }

        self.pipe_target = Some(stream);
    }
}
