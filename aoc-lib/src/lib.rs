pub use color_eyre;
pub use rangemap;
pub use regex;

pub mod iter;

#[macro_export]
macro_rules! aoc {
	($input:ident : $part1:ident => ($part1_ex:ident) $part1_test:expr, $part2:ident => ($part2_ex:ident) $part2_test:expr) => {
		fn main() -> $crate::color_eyre::eyre::Result<()> {
			$crate::color_eyre::install()?;

			let part1 = $part1($input)?;
			println!("Part 1: {part1}");

			let part2 = $part2($input)?;
			println!("Part 2: {part2}");

			Ok(())
		}

		#[cfg(test)]
		mod test {
			#[test]
			fn part1_test() {
				assert_eq!(
					super::$part1(super::$part1_ex).unwrap(),
					$part1_test
				);
			}

			#[test]
			fn part2_test() {
				assert_eq!(
					super::$part2(super::$part2_ex).unwrap(),
					$part2_test
				);
			}
		}
	};
	($input:ident : $part1:ident => ($part1_ex:ident) $part1_test:expr) => {
		fn __part2_todo(_input: &str) -> $crate::color_eyre::eyre::Result<i64> {
			todo!()
		}

		$crate::aoc! { $input: $part1 => ($part1_ex) $part1_test, __part2_todo => ($part1_ex) $part1_test }
	};
}

pub fn to_lines(s: &str) -> impl Iterator<Item = &str> {
	s.trim().lines().map(|line| line.trim())
}

pub fn map_with_idx<T, U, const N: usize>(arr: [T; N], mut f: impl FnMut(usize, T) -> U) -> [U; N] {
	let mut idx = 0;
	let mapper = move |elem| {
		let res = f(idx, elem);
		idx += 1;
		res
	};
	arr.map(mapper)
}
