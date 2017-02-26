#[macro_use]
extern crate clap;

extern crate ical;
extern crate rat;

use clap::{Arg, AppSettings, SubCommand};
use rat::client::{importcli};

fn main() {
	let m: clap::ArgMatches = app_from_crate!()
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(
			SubCommand::with_name("agenda")
				.about("Display agenda for specified timeframe")
				.visible_aliases(&[
					"today",
					"tomorrow",
					"week",
					"workweek"
				])
				.arg(Arg::with_name("start").takes_value(true).long("start").short("s"))
				.arg(Arg::with_name("end").takes_value(true).long("end").short("e"))
				.arg(Arg::with_name("verbose").takes_value(true).long("verbose").short("v"))
		)
		.subcommand(
			SubCommand::with_name("accounts")
				.about("Display list of accounts")
				.visible_aliases(&["accs"])
		)
		.subcommand(
			SubCommand::with_name("calendars")
				.about("Display list of calendars (across all accounts, or a specific account")
				.visible_aliases(&["cals"])
				.arg(Arg::with_name("account"))
		)
		.subcommand(
			SubCommand::with_name("invitations")
				.about("Display list of unhandled invitations")
				.visible_aliases(&["invites"])
		)
		.subcommand(
			SubCommand::with_name("prune")
				.arg(Arg::with_name("account"))
				.arg(Arg::with_name("force").long("force").short("f"))
		)
		.subcommand(
			SubCommand::with_name("import")
				.about("Import ICS file to calendar")
				.arg(Arg::with_name("filename").required(true))
				.arg(Arg::with_name("account").long("account").short("a"))
				.arg(Arg::with_name("calendar").long("calendar").short("c"))
		)
		.subcommand(
			SubCommand::with_name("export")
				.about("Export event to ICS file")
				.arg(Arg::with_name("id"))
				.arg(Arg::with_name("account").long("account").short("a"))
		)
		.subcommand(SubCommand::with_name("accept").arg(Arg::with_name("id")))
		.subcommand(SubCommand::with_name("decline").arg(Arg::with_name("id")))
		.get_matches();

	let exit_code = match m.subcommand_name() {
		Some("import") => importcli::entry(&(*m.subcommand.unwrap())),
		_ => {
			println!("Not yet implemented :(");
			255
		},
	};

	std::process::exit(exit_code);
}
