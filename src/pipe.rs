use std::rc::Rc;

use stream::*;

pub struct Pipe<'a, T: 'a> {
    is_closed: bool,
    pipe_target: Option<Rc<WriteableStream<T>>>,
    transformer: Option<Box<'a + Fn(T) -> T>>,
}

impl<'a, T> Pipe<'a, T> {

    pub fn new<R>(transformer: R) -> Self
    where R: 'a + Fn(T) -> T {
        Self {
            is_closed: false,
            pipe_target: None,
            transformer: Some(Box::new(transformer)),
        }
    }
}

impl<'a, T> Stream<'a, T> for Pipe<'a, T> {

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

    #[inline]
    fn pipe(&mut self, stream: Rc<'a + WriteableStream<T>>) {
    //where S: 'a + WriteableStream<T> {
        self.pipe_target = Some(stream);
    }
}

impl<'a, T> TransformableStream<'a, T> for Pipe<'a, T> {

    fn transform (&self, data: T) -> T {
        if let Some(ref trans) = self.transformer {
            (trans)(data)
        }
        else {
            data
        }
    }

    fn transformer<H>(&mut self, handler: H)
    where H: 'a + Fn(T) -> T {
        self.transformer = Some(Box::new(handler));
    }
}

impl<'a, T> WriteableStream<T> for Pipe<'a, T> {

    fn write(&self, data: T) {
        if self.is_closed {
            panic!("Cannot push to closed stream");
        }

        let data = self.transform(data);
        if let Some(ref ws) = self.pipe_target {
            (*ws).write(data);
        }
    }
}
