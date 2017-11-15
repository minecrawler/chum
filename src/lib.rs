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


pub struct Chum<'a, T: 'a + WriteableStream<T> + Clone, S: 'a>
where S: stream::Stream<'a, T> + WriteableStream<T> {
    pipeline: Vec<Segment<'a, T, S>>,
}

impl<'a, T, S> Chum<'a, T, S>
where
    T: 'a + WriteableStream<T> + Clone,
    S: 'a + stream::Stream<'a, T> + WriteableStream<T> {

    /// Build pipeline and make it start working.
    /// You only have to call this method once to kick off the pipeline.
    pub fn build_pipeline(&mut self) {
        if self.pipeline.len() < 2 { return; }

        // /*
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
    T: WriteableStream<T> + Clone,
    S: stream::Stream<'a, T> + WriteableStream<T> {

    fn write(&self, data: T) {
        if let Some(head) = self.pipeline.first() {
            head.stream().write(data);
        }
    }
}
