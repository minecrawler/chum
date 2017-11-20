use std::marker::PhantomData;
use std::rc::Rc;

use drain::Drain;
use pipe::Pipe;
use stream::*;


pub trait IntoSegment<'a, T, S>
where
    T: Clone,
    S: Stream<'a, T> + WriteableStream<T> {

    fn into(self) -> Segment<'a, T, S>;
}


pub struct Segment<'a, T, S>
where
    T: Clone,
    S: Stream<'a, T> + WriteableStream<T> {

    stream_type: StreamType,
    stream: Rc<S>,
    #[allow(dead_code)]
    l: &'a PhantomData<()>,
    t: PhantomData<T>,
}


impl<'a, T, S> Segment<'a, T, S>
where
    T: Clone,
    S: Stream<'a, T> + WriteableStream<T> {

    pub fn stream(&self) -> &Rc<S> {
        &self.stream
    }

    pub fn stream_mut(&mut self) -> &mut Rc<S> {
        &mut self.stream
    }

    pub fn stream_type(&self) -> &StreamType {
        &self.stream_type
    }
}

impl<'a, T> IntoSegment<'a, T, Pipe<'a, T>> for Pipe<'a, T>
where
    T: Clone {

    fn into(self) -> Segment<'a, T, Pipe<'a, T>> {
        Segment {
            stream: Rc::new(self),
            stream_type: StreamType::Transformable,
            l: &PhantomData,
            t: PhantomData,
        }
    }
}

impl<'a, T, F, H, C> IntoSegment<'a, T, Drain<'a, T, F, H, C>> for Drain<'a, T, F, H, C>
where
    T: Clone,
    F: Fn(T),
    H: Fn(),
    C: Fn() -> bool {

    fn into(self) -> Segment<'a, T, Drain<'a, T, F, H, C>> {
        Segment {
            stream: Rc::new(self),
            stream_type: StreamType::Writeable,
            l: &PhantomData,
            t: PhantomData,
        }
    }
}
