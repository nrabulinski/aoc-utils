use std::fs::read_dir;

use color_eyre::eyre::{eyre, Result, WrapErr};

pub static BIN_DIR: &str = "src/bin";
pub static INPUT_DIR: &str = "inputs";

/// Converts file name to day number.
/// By convention, all the files in BIN_DIR should be called dayXX.rs
/// We assume that that's the case and just throw an error if it isn't
pub fn name_to_day(day: &str) -> Result<usize> {
	let len = day.len();
	let num_str = day
		.get(len - 5..len - 3)
		.ok_or_else(|| eyre!("Invalid file name: {}", day))?;

	num_str
		.parse()
		.wrap_err_with(|| format!("Invalid file name: {}", day))
}

/// Gets the last created day.
/// Only returns an error if there's a problem reading the bin directory,
/// otherwise returns 0 by default.
pub fn last_created_day() -> Result<usize> {
	let mut days: Vec<_> = read_dir(BIN_DIR)?
		.map(|entry| {
			Ok(entry?
				.path()
				.file_name()
				.expect("directory entry should have a name")
				.to_string_lossy()
				.into_owned())
		})
		.collect::<Result<_>>()?;
	days.sort_unstable();

	days.last().map(|d| name_to_day(d)).unwrap_or(Ok(0))
}
