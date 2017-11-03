use stream::*;

pub struct Pipe<'a, T: 'a + Clone, F: Fn(T) -> T> {
    is_closed: bool,
    pipe_target: Option<&'a WriteableStream<T>>,
    transformer: F,
}

impl<'a, T, F> Pipe<'a, T, F>
where
    T: Clone,
    F: Fn(T) -> T {

    pub fn new(transformer: F) -> Self {
        Self {
            is_closed: false,
            pipe_target: None,
            transformer,
        }
    }
}

impl<'a, T, F> Stream<'a, T> for Pipe<'a, T, F>
where
    T: Clone,
    F: Fn(T) -> T {

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
    fn pipe<S>(&mut self, stream: &'a S)
    where S: WriteableStream<T> + Stream<'a, T> {
        self.pipe_target = Some(stream);
    }
}

impl<'a, T, F> WriteableStream<T> for Pipe<'a, T, F>
where
    T: Clone,
    F: Fn(T) -> T {

    fn write(&self, data: &T) {
        if self.is_closed {
            panic!("Cannot push to closed stream");
        }

        let data = (self.transformer)(data.clone());
        if let Some(ref ws) = self.pipe_target {
            ws.write(&data);
        }
    }
}
