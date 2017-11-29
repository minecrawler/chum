use std::rc::Rc;

use drain::Drain;
use pipe::Pipe;
use source::Source;
use stream::*;


pub struct Chum<'a, T: 'a> {
    // todo: use traits instead of types
    drain: Rc<Drain<'a, T>>,
    pipeline: Vec<Rc<Pipe<'a, T>>>,
    source: Rc<Source<'a, T>>,
}

impl<'a, T> Chum<'a, T> {
    /// Build pipeline and make it start working.
    /// You only have to call this method once to kick off the pipeline.
    pub fn build_pipeline(&mut self) {
        if self.pipeline.len() < 2 { return; }

        let mut pipeline = self
            .pipeline
            .iter_mut()
            .peekable()
        ;

        let mut first = true;
        while let Some(mut cur_seg) = pipeline.next() {
            if first {
                self.source.pipe(Rc::clone(cur_seg));
                first = false;
            }

            let next_seg = pipeline.peek();

            if next_seg.is_none() {
                Stream::pipe(&mut *Rc::get_mut(cur_seg).unwrap(), self.drain);
                break;
            }

            let next_stream = next_seg.unwrap();
            Stream::pipe(&mut *Rc::get_mut(cur_seg).unwrap(), **next_stream);
        }
    }

    pub fn new<S, D> (source: Rc<Source<'a, T>>, drain: Rc<Drain<'a, T>>) -> Self {

        Self {
            drain,
            pipeline: Vec::new(),
            source,
        }
    }

    /// Register a new segment.
    pub fn register<P>(&mut self, segment: Rc<Pipe<'a, T>>) {
        self.pipeline.push(segment);
    }
}

impl<'a, T> Stream<'a, T> for Chum<'a, T>
where
    T: 'a {

    fn close(&mut self) {
        unimplemented!();
    }

    fn closed(&self) -> bool {
        unimplemented!()
    }

    fn pipe(&mut self, stream: Rc<'a + WriteableStream<T>>) {
    //where S: 'a + WriteableStream<T> {
        unimplemented!();
    }
}

impl<'a, T> WriteableStream<T> for Chum<'a, T>
where
    T: 'a {

    fn write(&self, data: T) {
        if let Some(head) = self.pipeline.first() {
            (*head).write(data);
        }
    }
}
