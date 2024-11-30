use std::{
	fs::{self},
	// Screw windows users, I guess?
	os::unix::process::CommandExt,
	path::{Path, PathBuf},
	process::Command,
	sync::Arc,
};

use clap::{Parser, Subcommand};
use color_eyre::eyre::{bail, Result};
use reqwest::{blocking::Client, cookie::Jar, StatusCode};

mod config;
use config::Config;
mod utils;
use utils::{last_created_day, BIN_DIR};

use crate::utils::INPUT_DIR;

const ADVENT_DAYS: usize = 25;

/// cli application to help quickly initialize the next day, and with other aoc-related things
#[derive(Parser)]
#[clap(name = "aoc")]
#[command(disable_help_subcommand(true))]
struct Cli {
	#[command(subcommand)]
	cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
	/// fetch all the so far unfetched days
	Fetch,

	/// runs solution for a given day
	Run {
		/// day to run between 0 and 25
		day: Option<usize>,
		/// whether to run test or the final version
		#[arg(long, short)]
		test: bool,
	},
}

fn write_template(day: usize) -> String {
	format!(
		r#"use aoc_lib::{{aoc, color_eyre::eyre::Result}};

static INPUT: &str = include_str!("../../inputs/day{day:02}");
			
fn part1(input: &str) -> Result<i64> {{
	todo!()
}}

#[allow(dead_code)]
static EX_INPUT: &str = "EXAMPLE 1 HERE";

aoc! {{
	INPUT:
	part1 => (EX_INPUT) 0
}}
"#
	)
}

fn fetch() -> Result<()> {
	if !Path::new("./Cargo.toml").is_file() {
		bail!("Must be called in a root cargo directory");
	}

	let config = Config::from_env()?;

	fs::create_dir_all(BIN_DIR)?;

	let last_day = last_created_day()?;

	if last_day == ADVENT_DAYS {
		println!("No more AoC days left!");
		return Ok(());
	}

	let mut src_path = PathBuf::from(BIN_DIR);
	src_path.push("day00.rs");
	let mut input_path = PathBuf::from(INPUT_DIR);
	input_path.push("day00.rs");

	let client = {
		let cookies = Arc::new(Jar::default());
		let cookie = format!("session={}", config.session);
		cookies.add_cookie_str(&cookie, &"https://adventofcode.com".parse()?);
		Client::builder().cookie_provider(cookies).build()?
	};
	for day in last_day + 1..=ADVENT_DAYS {
		let url = format!("https://adventofcode.com/{}/day/{day}/input", config.year);
		let res = client.get(url).send()?;
		if res.status() == StatusCode::NOT_FOUND {
			return Ok(());
		}
		let res = res.error_for_status()?;

		let input = res.bytes()?;

		src_path.set_file_name(format!("day{day:02}.rs"));
		input_path.set_file_name(format!("day{day:02}"));

		std::fs::write(&src_path, write_template(day))?;
		std::fs::write(&input_path, input)?;
	}

	Ok(())
}

fn run(day: Option<usize>, test: bool) -> Result<()> {
	let day_to_run = Ok(day).transpose().unwrap_or_else(last_created_day)?;

	if day_to_run == 0 {
		bail!("No days to run!");
	} else if day_to_run > ADVENT_DAYS {
		bail!("Invalid day number");
	}

	let bin_name = format!("day{day_to_run:02}");

	let (cmd, profile) = if test {
		("test", "dev")
	} else {
		("run", "release")
	};
	let e = Command::new("cargo")
		.args([cmd, "--bin", &bin_name, "--profile", profile])
		.exec();

	Err(e.into())
}

fn main() -> Result<()> {
	dotenvy::dotenv()?;
	color_eyre::install()?;

	let cli = Cli::parse();
	match cli.cmd {
		Cmd::Fetch => fetch(),
		Cmd::Run { day, test } => run(day, test),
	}
}
