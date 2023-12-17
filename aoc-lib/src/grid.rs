use std::{iter::FusedIterator, ops::Index};

type Point = (i64, i64);

pub trait PointExt {
	type Comp;

	fn map(&self, f: impl FnMut(Self::Comp) -> Self::Comp) -> Self;
	fn add(&self, other: &Self) -> Self;
}

impl PointExt for Point {
	type Comp = i64;

	fn map(&self, mut f: impl FnMut(Self::Comp) -> Self::Comp) -> Self {
		(f(self.0), f(self.1))
	}

	fn add(&self, other: &Self) -> Self {
		(self.0 + other.0, self.1 + other.1)
	}
}

#[derive(Clone, Copy)]
pub struct Grid<'a> {
	base: &'a [u8],
	line_width: i64,
	width: i64,
	height: i64,
}

#[derive(Clone, Copy)]
pub struct Row<'a> {
	grid: &'a Grid<'a>,
	y: i64,
}

#[derive(Clone, Copy)]
pub struct GridIter<'a> {
	grid: &'a Grid<'a>,
	curr: usize,
	end: usize,
	step: usize,
}

impl<'a> Iterator for GridIter<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.curr >= self.end {
			None
		} else {
			let res = self.grid.base[self.curr];
			self.curr += self.step;
			Some(res)
		}
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.curr += n * self.step;
		self.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = (self.end - self.curr) / self.step;
		(len, Some(len))
	}
}

impl<'a> DoubleEndedIterator for GridIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.curr >= self.end {
			None
		} else {
			self.end -= self.step;
			Some(self.grid.base[self.end])
		}
	}

	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.end -= n * self.step;
		self.next_back()
	}
}

impl<'a> ExactSizeIterator for GridIter<'a> {}

impl<'a> FusedIterator for GridIter<'a> {}

impl<'a> Row<'a> {
	pub fn iter(&self) -> GridIter<'a> {
		let start_idx = self.grid.pos_to_idx((0, self.y)).unwrap();
		let end_idx = start_idx + self.grid.width as usize;
		GridIter {
			grid: self.grid,
			curr: start_idx,
			end: end_idx,
			step: 1,
		}
	}
}

impl<'a> IntoIterator for Row<'a> {
	type Item = u8;
	type IntoIter = GridIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[derive(Clone, Copy)]
pub struct Column<'a> {
	grid: &'a Grid<'a>,
	x: i64,
}

impl<'a> Column<'a> {
	pub fn iter(&self) -> GridIter<'a> {
		let start_idx = self.grid.pos_to_idx((self.x, 0)).unwrap();
		let end_idx = start_idx + (self.grid.height * self.grid.line_width) as usize;
		GridIter {
			grid: self.grid,
			curr: start_idx,
			end: end_idx,
			step: self.grid.line_width as usize,
		}
	}
}

impl<'a> IntoIterator for Column<'a> {
	type Item = u8;
	type IntoIter = GridIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> Grid<'a> {
	pub fn for_str(s: &'a str) -> Option<Self> {
		let s = s.trim();
		let s_len: i64 = s.len().try_into().ok()?;
		// This conversion is fine because we know line length will be
		// less than total length, which we already confirmed fits in i64
		let line_width = s.lines().next().unwrap().len() as i64 + 1;
		let height = s_len / line_width + 1;
		let width = line_width - 1;

		Some(Grid {
			base: s.as_bytes(),
			line_width,
			width,
			height,
		})
	}

	pub fn width(&self) -> i64 {
		self.width
	}

	pub fn height(&self) -> i64 {
		self.height
	}

	pub fn iter_rows(&'a self) -> impl Iterator<Item = Row<'a>> {
		(0..self.height).map(|y| Row { grid: self, y })
	}

	pub fn iter_columns(&'a self) -> impl Iterator<Item = Column<'a>> {
		(0..self.width).map(|x| Column { grid: self, x })
	}

	pub fn is_valid_pos(&self, (x, y): Point) -> bool {
		x >= 0 && x < self.width && y >= 0 && y < self.height
	}

	pub fn pos_to_idx(&self, (x, y): Point) -> Option<usize> {
		self.is_valid_pos((x, y))
			.then(|| (y * self.line_width + x) as usize)
	}

	pub fn get(&self, idx: usize) -> Option<&u8> {
		self.base.get(idx)
	}

	pub fn get_pos(&self, pos: Point) -> Option<&u8> {
		self.pos_to_idx(pos).map(|idx| &self.base[idx])
	}
}

impl<'a> Index<Point> for Grid<'a> {
	type Output = u8;

	fn index(&self, index: Point) -> &Self::Output {
		self.get_pos(index).unwrap()
	}
}
