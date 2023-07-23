

use futures_core::stream::stream;



pub trait FiniteStreamOps { 
    pub fn all(self, predicate: impl Fn() -> bool) -> All;
    pub fn any(self, predicate: impl Fn() -> bool) -> Any;
    pub fn join(self) -> StreamJoin;
    pub fn max(self) -> MaxElem;
    pub fn min(self) -> MinElem;
    pub fn insert_on(self, index: usize) -> IndexInsert;
    pub fn merge(self) -> MergedStream;
    pub fn k_merge(self) -> KMergeStream;
}


pub trait InfiniteStreamOps {
    pub fn max_nth(self) -> NthMax;
    pub fn min_nth(self) -> MthMin;
    pub fn insert_on(self, index: usize) -> InsertOn;
    pub fn stream_merge(self) -> StreamMerge;
}


impl<T> FiniteStreamOps for T 
    where T: FiniteStream 
{
    fn all(self, predicate: impl Fn() -> bool) -> All {
        self.output_next()
    }

    fn any(self, predicate: impl Fn() -> bool ) -> Any {

    }
}


#[derive(Debug, )]
pub struct All {
    Item: T
}

impl Future for All {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Self::Output {

    }
}