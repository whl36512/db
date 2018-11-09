extern crate simplelog;

use self::simplelog::*;
use std::fs::File;

// url::form_urlencoded::parse()
//
pub fn logger_init (log_file: &str) {
	CombinedLogger::init(
		vec![
		TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
		WriteLogger::new(LevelFilter::Info, Config::default(),
		File::create(log_file).unwrap()),
		]
	).unwrap();
}

