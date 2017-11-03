use std::marker::PhantomData;

use stream::*;

pub struct Drain<'a, T: 'a + Clone, F: Fn(&T), H: Fn(), C: Fn() -> bool> {
    close_handler: H,
    closed_checker: C,
    pipe_target: Option<&'a WriteableStream<T>>,
    writer: F,
    t: PhantomData<T>,
}

impl<'a, T, F, H, C> Drain<'a, T, F, H, C>
where
    T: Clone,
    F: Fn(&T),
    H: Fn(),
    C: Fn() -> bool {

    pub fn new(writer: F, close_handler: H, closed_checker: C) -> Self {
        Self {
            close_handler,
            closed_checker,
            pipe_target: None,
            writer,
            t: PhantomData,
        }
    }

    pub fn writer(&mut self, writer: F) {
        self.writer = writer;
    }
}

impl<'a, T, F, H, C> Stream<'a, T> for Drain<'a, T, F, H, C>
where
    T: Clone,
    F: Fn(&T),
    H: Fn(),
    C: Fn() -> bool {

    fn close(&mut self) {
        (self.close_handler)();
    }

    fn closed(&self) -> bool {
        (self.closed_checker)()
    }

    fn pipe<S>(&mut self, stream: &'a S)
    where S: WriteableStream<T> + Stream<'a, T> {
        self.pipe_target = Some(stream);
    }
}

impl<'a, T, F, H, C> WriteableStream<T> for Drain<'a, T, F, H, C>
where
    T: Clone,
    F: Fn(&T),
    H: Fn(),
    C: Fn() -> bool {

    fn write(&self, data: &T) {
        (self.writer)(data);
        if let Some(ref ws) = self.pipe_target {
            ws.write(data);
        }
    }
}
