use std::rc::Rc;

mod drain;
mod pipe;
mod source;
mod segment;
mod stream;

pub use drain::Drain;
pub use pipe::Pipe;
pub use source::Source;
pub use segment::{IntoSegment, Segment};
pub use stream::*;


pub struct Chum<'a, T: 'a + Clone, S: 'a>
where S: 'a + stream::Stream<'a, T> + WriteableStream<T> {
    pipeline: Vec<Segment<'a, T, S>>,
}

impl<'a, T, S> Chum<'a, T, S>
where
    T: 'a + Clone,
    S: 'a + stream::Stream<'a, T> + WriteableStream<T> {

    /// Build pipeline and make it start working.
    /// You only have to call this method once to kick off the pipeline.
    pub fn build_pipeline(&mut self) {
        if self.pipeline.len() < 2 { return; }

        let mut pipeline = self
            .pipeline
            .iter_mut()
            .peekable()
        ;

        while let Some(mut cur_seg) = pipeline.next() {
            let next_seg = pipeline.peek();

            if next_seg.is_none() { break; }

            let next_stream = *Rc::clone(next_seg.unwrap().stream());
            cur_seg.stream_mut().pipe(&next_stream);
        }

        /*
        let iter = self
            .pipeline
            .iter_mut()
            .rev()
        ;

        let next_seg = iter.next();
        while next_seg.is_some() {
            let old_seg = next_seg.unwrap();
            next_seg = iter.next();

            if let Some(segment) = next_seg {
                segment.stream_mut().pipe(old_seg.stream());
            }
        }
        // */
        /*
        let next_seg = self.pipeline.pop();
        while next_seg.is_some() {
            let old_seg = next_seg.unwrap();
            next_seg = self.pipeline.pop();

            if let Some(segment) = next_seg {
                segment.stream_mut().pipe(old_seg.stream());
            }
        }
        // */
    }

    pub fn new() -> Self {
        Self {
            pipeline: Vec::new(),
        }
    }

    /// Register a new segment.
    pub fn register<I>(&mut self, segment: I) -> Result<&mut Self, ()> where I: IntoSegment<'a, T, S> {
        let segment: Segment<'a, T, S> = segment.into();

        match *segment.stream_type() {
            StreamType::Readable => return Err(()), // todo
            StreamType::Transformable => self.pipeline.push(segment),
            StreamType::Writeable => self.pipeline.push(segment),
        }

        Ok(self)
    }
}

impl<'a, T, S> WriteableStream<T> for Chum<'a, T, S>
where
    T: 'a + Clone,
    S: 'a + stream::Stream<'a, T> + WriteableStream<T>,
    std::rc::Rc<S>: std::borrow::Borrow<stream::WriteableStream<T>>{

    fn write(&self, data: T) {
        use std::borrow::Borrow;

        if let Some(head) = self.pipeline.first() {
            let stream: &WriteableStream<T> = head.stream().borrow();

            stream.write(data);
        }
    }
}
