

use futures_core::stream::Stream;


pub trait FiniteStream: Stream + FusedStream {

    type MergedItem: Stream::Item;

    fn output_next() -> MergedItem;
}

pub trait InfiniteStream: Stream {
    
    type MergedItem; 

    fn stream_join() -> impl Stream<Ouput=Self::MergedItem>; 
}


impl<T1, T2> FinitStream for (T1, T2) 
where T1: Stream + FusedStream, 
        T2: Stream + FusedStream
{
    
}