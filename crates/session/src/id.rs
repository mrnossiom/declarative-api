use std::{fmt::Debug, hash::Hash, marker::PhantomData};

pub trait Idx: Copy + 'static + Eq + PartialEq + Debug + Hash {
	fn new(idx: usize) -> Self;

	fn index(self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexVec<I: Idx, T> {
	inner: Vec<T>,
	marker: PhantomData<I>,
}

impl<I: Idx, T> Default for IndexVec<I, T> {
	fn default() -> Self {
		Self {
			inner: Vec::default(),
			marker: PhantomData,
		}
	}
}

#[macro_export]
macro_rules! new_index_ty {
    {
    	$(#[$attrs:meta])*
    	$vis:vis struct $name:ident;
    } => {
    	$(#[$attrs])*
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    	$vis struct $name(usize);

        impl $crate::Idx for $name {
        	fn new(idx: usize) -> Self {
        		Self(idx)
        	}

        	fn index(self) -> usize {
        		self.0
        	}
        }
    };
}
