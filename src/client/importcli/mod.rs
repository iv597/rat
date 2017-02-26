extern crate clap;
extern crate ical;

use self::clap::SubCommand;
use self::ical::IcalParser;
use std::io::BufReader;
use std::fs::File;

pub fn entry(m: &SubCommand) -> i32 {
	println!("{:?}", m);
	let fname = m.matches.value_of("filename").unwrap();

	let file = match File::open(fname) {
		Ok(f) => f,
		Err(f) => {
			println!("{}", f);
			return 1;
		}
	};

	let input = IcalParser::new(BufReader::new(file));

	for line in input {
		println!("{:?}", line);
	}

	0
}
