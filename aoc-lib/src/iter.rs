use std::{
	iter::{Fuse, FusedIterator, Peekable},
	mem::MaybeUninit,
};

pub trait IterExt: Iterator {
	fn peekable2(self) -> Peek2<Self>
	where
		Self: Sized,
	{
		Peek2::from_iter(self)
	}

	/// Optimized equivalent of `self.count() == n`
	fn has_n(self, n: usize) -> bool
	where
		Self: Sized,
	{
		let mut curr = 0;
		for _ in self {
			curr += 1;
			if curr > n {
				return false;
			}
		}
		curr == n
	}

	/// Iterator::array_chunks, but stable
	fn arr_chunks<const N: usize>(self) -> Chunks<Fuse<Self>, N>
	where
		Self: Sized,
	{
		assert!(N != 0, "chunk size must be non-zero");
		Chunks { iter: self.fuse() }
	}

	/// Chains two iterators, but always returns the smaller element of the two.
	///
	/// This means that if the iterators that are being chained are sorted,
	/// the resulting iterator will be sorted as well.
	///
	/// # Examples
	/// ```
	/// use aoc_lib::iter::IterExt;
	///
	/// let a = [1, 3, 5];
	/// let b = [2, 4, 6];
	/// let sorted_iter: Vec<_> = a.into_iter().chain_sorted(b).collect();
	///
	/// assert_eq!(sorted_iter, [1, 2, 3, 4, 5, 6]);
	/// ```
	fn chain_sorted<U>(self, other: U) -> ChainSorted<Self, U::IntoIter>
	where
		Self: Sized,
		// TODO: Could I get away with PartialOrd?
		Self::Item: Ord,
		U: IntoIterator<Item = Self::Item>,
	{
		ChainSorted {
			a: self.peekable(),
			b: other.into_iter().peekable(),
		}
	}
}

impl<I: Iterator> IterExt for I {}

pub struct Peek2<I: Iterator> {
	iter: I,
	peek1: Option<Option<I::Item>>,
	peek2: Option<Option<I::Item>>,
}

impl<I: Iterator> Peek2<I> {
	fn from_iter(iter: I) -> Self {
		Peek2 {
			iter,
			peek1: None,
			peek2: None,
		}
	}

	pub fn peek(&mut self) -> Option<&I::Item> {
		if self.peek1.is_none() {
			self.peek1 = Some(self.iter.next());
		}
		self.peek1.as_ref().unwrap().as_ref()
	}

	pub fn peek2(&mut self) -> Option<&I::Item> {
		let _ = self.peek();
		if self.peek2.is_none() {
			self.peek2 = Some(self.iter.next());
		}
		self.peek2.as_ref().unwrap().as_ref()
	}

	pub fn peek_pair(&mut self) -> (Option<&I::Item>, Option<&I::Item>) {
		let _ = self.peek();
		let _ = self.peek2();
		(
			self.peek1.as_ref().unwrap().as_ref(),
			self.peek2.as_ref().unwrap().as_ref(),
		)
	}
}

impl<I: Iterator> Iterator for Peek2<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match self.peek1.take() {
			Some(v) => {
				self.peek1 = std::mem::take(&mut self.peek2);
				v
			}
			None => self.iter.next(),
		}
	}
}

pub struct Chunks<I: FusedIterator, const N: usize> {
	iter: I,
}

impl<I: FusedIterator, const N: usize> Iterator for Chunks<I, N> {
	type Item = [I::Item; N];

	fn next(&mut self) -> Option<Self::Item> {
		let mut res = MaybeUninit::<Self::Item>::uninit();

		for i in 0..N {
			let elem = self.iter.next()?;
			// SAFETY: We're writing to the array, not reading from it,
			//         and the index is guaranteed to be in bounds.
			unsafe {
				let r = &mut *res.as_mut_ptr();
				*r.get_unchecked_mut(i) = elem;
			}
		}

		// SAFETY: We know we have filled in all the array elements
		Some(unsafe { res.assume_init() })
	}
}

impl<I: FusedIterator, const N: usize> FusedIterator for Chunks<I, N> {}

pub struct ChainSorted<I1, I2>
where
	I1: Iterator,
	I1::Item: Ord,
	I2: Iterator<Item = I1::Item>,
{
	a: Peekable<I1>,
	b: Peekable<I2>,
}

impl<I1, I2> Iterator for ChainSorted<I1, I2>
where
	I1: Iterator,
	I1::Item: Ord,
	I2: Iterator<Item = I1::Item>,
{
	type Item = I1::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match (self.a.peek(), self.b.peek()) {
			(Some(a), Some(b)) => {
				if a < b {
					self.a.next()
				} else {
					self.b.next()
				}
			}
			(Some(_), None) => self.a.next(),
			(None, Some(_)) => self.b.next(),
			_ => None,
		}
	}
}
