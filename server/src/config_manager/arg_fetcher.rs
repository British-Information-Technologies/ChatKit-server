use clap::{App, Arg, ArgMatches, value_parser};

pub(crate) fn get_args() -> ArgMatches {
	App::new("Rust Chat Server")
		.author("Michael Bailey & Mitchel Hardie")
		.version("0.1.0")
		.about("A chat server written in rust, with a custom json protocol, based on serde and actix")
		.arg(
			Arg::new("port")
				.short('p')
				.long("port")
				.takes_value(true)
				.value_parser(value_parser!(u16))
				.default_value("5600")
				.help("overrides the default port")
		)
		.arg(
			Arg::new("server name")
				.short('n')
				.long("name")
				.takes_value(true)
				.help("overrides the default port of the server")
		)
		.arg(
			Arg::new("server owner")
				.short('o')
				.long("owner")
				.takes_value(true)
				.help("overrides the owner of the server")
		)
		.arg(
			Arg::new("config file")
				.short('c')
				.long("config_file")
				.takes_value(true)
				.help("overrides the default config file location")
		)
		.after_help("This is a chat server made to test out writing a full application in rust \
											It has evolved over time to use different frameworks\
											It is currently using actix")
		.get_matches()
}