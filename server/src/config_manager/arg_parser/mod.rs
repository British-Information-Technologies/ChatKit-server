use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
	#[clap(short, long, value_parser = clap::value_parser!(u16).range(1..))]
	pub port: Option<u16>,

	#[clap(short, long, value_parser)]
	pub name: Option<String>,

	#[clap(short, long, value_parser)]
	pub owner: Option<String>,
}
