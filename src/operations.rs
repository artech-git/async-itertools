
use futures_core::stream::stream;


pub trait FiniteStreamOps { 
    pub fn all<E>(self, predicate: impl Fn(E) -> bool) -> All;
    pub fn any<U>(self, predicate: impl Fn(U) -> bool) -> Any;
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
    fn all<E>(self, predicate: impl Fn(E) -> bool) -> All {
        All::new(self, predicate)
    }

    fn any(self, predicate: impl Fn() -> bool ) -> Any {

    }

    fn join(self,) -> StreamJoin {

    }
    
    fn max(self) -> MaxElem {

    }

    fn min(self) -> MinElem {

    }
    fn insert_on(self) -> IndexInsert {
        
    }
}


#[derive(Debug, )]
pub struct All<T> {
    Item: T,
    Stream: Box<dyn FiniteStream>,
    condition: Box<dyn Fn(T) -> bool> 
}

impl<T> Future for All<T> 
    where T: std::cmp::PartialEq 
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Self::Output {
        let curr_item = self.stream.output_next();

        if let true =  (self.condition)(curr_item){
            return None;
        }
        else {
            return Some(curr_item)
        }
    }
}

impl<T> All<T> {

    fn new(stream: impl FiniteStream, condition: impl Fn() -> bool) -> Self {
        Self {
            stream: Box::new(Stream), 
            condition: Box::new(condition),
        }
    }


}