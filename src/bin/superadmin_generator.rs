extern crate backend;

#[macro_use]
extern crate ferris_print;
#[macro_use]
extern crate scanln;

extern crate rocket;
extern crate yansi;

use yansi::Paint;
use backend::init;

use rocket::http::Status;
use rocket::local::Client;

use std::env;

fn main() {
	let rocket = init();
	let client = Client::new(rocket).expect("rocket instance is not valid");

	println!();
	ferrisprint!("Superadmin generator v1");

	let admin_email = scanln!("[{}] {}", Paint::magenta("rust booking"), Paint::yellow("please insert superadmin login email:"));

	println!("[{}] {}", Paint::magenta("rust booking"), Paint::yellow("reading super secret password"));
	
	let password = env::var("SA_SECRET").unwrap();

	println!("[{}] {}", Paint::magenta("rust booking"), Paint::yellow("generating superadmin"));
	let req = client.post(format!("/admin/generate_sa/{}/{}", admin_email, password)).dispatch();

	if req.status() == Status::NotFound {
		 println!("[{}] {}", Paint::magenta("rust booking"), Paint::yellow("failed to generate superadmin account"))
	} else {
		 println!("[{}] {}", Paint::magenta("rust booking"), Paint::yellow("successfully generated superadmin account"))
	}
}
