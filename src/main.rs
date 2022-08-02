use clap::Parser;
use ellipse::Ellipse as _;
use std::io::{self, BufRead as _};

#[derive(Parser)]
#[clap(name = "ellipse")]
#[clap(about, version)]
struct Args {
	/// The number of grapheme clusters after which to truncate.
	#[clap(short = 'n')]
	len: usize,

	/// The string used for ellipse.
	#[clap(short, default_value = "…")]
	ellipsis: String
}

fn main() -> io::Result<()> {
	let args = Args::parse();
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		println!(
			"{}",
			line?
				.as_str()
				.truncate_ellipse_with(args.len, &args.ellipsis)
		);
	}
	Ok(())
}
