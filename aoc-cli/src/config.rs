use std::env::var;

use color_eyre::eyre::{Result, WrapErr};

pub struct Config {
	pub year: usize,
	pub session: String,
}

impl Config {
	pub fn from_env() -> Result<Self> {
		let year = var("AOC_YEAR")?
			.parse()
			.wrap_err("Invalid AOC_YEAR value")?;
		let session = var("AOC_SESSION")?;

		Ok(Config { year, session })
	}
}
