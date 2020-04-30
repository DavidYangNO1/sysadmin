extern crate serde;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Application {
	pub isinit        : bool,
	pub readtimeout   : i32,
	pub writetimeout : i32,
	pub host          : String,
	pub port          : String,
	pub name          : String,
	pub logpath       : String,
	pub env           : String,
	pub envmsg        : String,
}
