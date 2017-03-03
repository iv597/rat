extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate ical;

use std::fs::File;
use std::io::BufReader;
use self::chrono::{DateTime, TimeZone};
use self::chrono_tz::{Tz, UTC};
use self::clap::SubCommand;
use self::ical::IcalParser;

#[derive(Debug)]
enum LineInfo {
	TextInfo {
		name: String,
		value: Option<String>,
	},
	DateTimeInfo {
		name: String,
		value: DateTime<UTC>,
	}
}

pub fn entry(m: &SubCommand) -> i32 {
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
		let line = line.unwrap();

		for evt in line.events {
			println!("EVENT");

			for prop in evt.properties {
				let parsedval: LineInfo = match prop.value {
					Some(mut v) => {
						let tz = if (&v).ends_with("Z") {
							v.pop();

							UTC
						} else {
							match prop.params {
								Some(params) => {
									for &(ref pk, ref pv) in &params {
										if pk == "TZID" {
											match pv[0].parse::<Tz>() {
												Ok(tz) => tz,
												Err(_) => UTC,
											}
										}
									}

									UTC
								}
								None => UTC,
							}
						};

						match tz.datetime_from_str(&v, "%Y%m%dT%H%M%S") {
							Ok(v) => LineInfo::DateTimeInfo { name: prop.name, value: v },
							Err(_) => LineInfo::TextInfo { name: prop.name, value: Some(v) },
						}
					}
					None => LineInfo::TextInfo { name: prop.name, value: None },
				};

				println!("{:?}", parsedval);
			}
		}
	}

	0
}
