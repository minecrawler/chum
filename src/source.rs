use stream::*;


pub struct Source<'a, T: 'a> {
    buf: Vec<T>,
    corked: bool,
    data_callback: Option<Box<'a + Fn(&T)>>,
    is_closed: bool,
    pipe_target: Option<&'a WriteableStream<T>>,
}

impl<'a, T> Source<'a, T>
where
    T: Clone {

    #[inline]
    pub fn end(&mut self, data: T) {
        if let Some(ref mut ws) = self.pipe_target {
            ws.write(data);
        }
        else {
            self.buf.push(data);
        }

        self.close();
        self.pipe_target = None;
    }

    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            corked: false,
            data_callback: None,
            is_closed: false,
            pipe_target: None,
        }
    }

    pub fn push(&mut self, data: T) {
        if self.is_closed {
            panic!("Cannot push to closed source");
        }

        if let Some(ref cb) = self.data_callback {
            cb(&data);
        }

        if let Some(ref mut ws) = self.pipe_target {
            ws.write(data);
        }
        else {
            self.buf.push(data);
        }
    }

    fn push_to_pipe(&mut self) {
        if let Some(ws) = self.pipe_target {
            while let Some(c) = self.read() {
                ws.write(c);
            }
        }
    }
}

impl<'a, T> PausableStream for Source<'a, T>
where
    T: Clone {

    #[inline]
    fn cork(&mut self) {
        self.corked = true;
    }

    #[inline]
    fn corked(&self) -> bool {
        self.corked
    }

    fn uncork(&mut self) {
        self.corked = false;
        self.push_to_pipe();
    }
}

impl<'a, T> ReadableStream<'a, T> for Source<'a, T>
where
    T: Clone  {

    fn read(&mut self) -> Option<T> {
        if self.corked { return None; }

        // todo: is this even performant
        self.buf.reverse();
        let item = self.buf.pop();
        self.buf.reverse();

        item
    }

    #[inline]
    fn data<H>(&mut self, handler: H)
    where H: 'a + Fn(&T) {

        self.data_callback = Some(Box::new(handler));
    }
}

impl<'a, T> Stream<'a, T> for Source<'a, T>
where
    T: Clone  {

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
        self.pipe_target = Some(stream);
        if self.corked { return; }

        self.push_to_pipe();
    }
}
