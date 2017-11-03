use std::collections::LinkedList;

use chunk::Chunk;
use stream::*;


pub struct Source<'a, T: 'a> {
    buf: LinkedList<Chunk<T>>,
    is_closed: bool,
    pipe_target: Option<&'a mut WriteableStream<T>>,
}

impl<'a, T> Source<'a, T> {
    #[inline]
    pub fn end(&mut self, data: T) {
        if let Some(ref mut ws) = self.pipe_target {
            ws.write(data);
        }
        else {
            self.buf.push_back(Chunk { last: true, data: data });
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
            ws.write(data);
        }
        else {
            self.buf.push_back(Chunk { last: false, data: data });
        }
    }
}

impl<'a, T> ReadableStream<T> for Source<'a, T> {
    #[inline]
    fn read(&mut self) -> Option<Chunk<T>> {
        self.buf.pop_front()
    }
}

impl<'a, T> Stream<'a, T> for Source<'a, T> {
    fn close(&mut self) {
        if self.is_closed {
            panic!("Stream is already closed!");
        }

        self.is_closed = true;

        let last_chunk = self.buf.back_mut();
        if let Some(c) = last_chunk {
            c.last = true;
        }
    }

    #[inline]
    fn closed(&self) -> bool {
        self.is_closed
    }

    fn pipe<S>(&mut self, stream: &'a mut S)
    where S: WriteableStream<T> + Stream<'a, T> {
        while let Some(c) = self.read() {
            stream.write(c.data);
            if c.last { stream.close(); break; }
        }

        self.pipe_target = Some(stream);
    }
}
