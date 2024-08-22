use clap::{crate_version, Arg, Command};

pub fn argc_app() -> Command {
	Command::new("lacs")
		.version(crate_version!())
		.about("Simple CLI tool that displays files in a directory sorted by their last accessed time")
		.arg(
			Arg::new("nums")
				.help("Number of biggest files to display")
				.short('n')
				.num_args(1),
		)
		.arg(
			Arg::new("glob")
			.help("globs, comma separated")
			.short('g')
			.num_args(1)
		)
		.arg(
			Arg::new("no-ext")
				.action(clap::ArgAction::SetTrue)
				.long("no-ext")
				.help("group by extension"),
		)
		.arg(
			Arg::new("oldest")
			.action(clap::ArgAction::SetTrue)
			.short('o')
			.help("sort by oldest")
		)
}