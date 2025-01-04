use std::{fmt::Debug, hash::Hash, marker::PhantomData};

pub trait Idx: Copy + 'static + Eq + PartialEq + Debug + Hash {
	fn new(idx: usize) -> Self;

	fn index(self) -> usize;

	#[must_use]
	fn inc(self) -> Self;
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

impl<I: Idx, T> IndexVec<I, T> {
	#[must_use]
	pub const fn items(&self) -> &Vec<T> {
		&self.inner
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

        	fn inc(self) -> Self {
        		Self(self.0.checked_add(1).expect(concat!("Ran out of ", stringify!($name))))
        	}
        }
    };
}
