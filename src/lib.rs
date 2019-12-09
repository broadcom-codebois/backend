//! ## backend of the reservation system for auditorium
//! Backend documentation
//! 
//! Uses [Rocket](https://rocket.rs) as the web framework
//! and [sled](https://sled.rs) as the database.
//!
//! Modus operandi tohoto serveru spočívá v přijímání požadavků, validaci dat
//! a volání správného RGI, viz [`rgi!`]
//!
//! Structure:
//! ```bash,no_run
//! .
//! ├── Cargo.lock  - lockfile, do not delete! (deterministic builds)
//! ├── Cargo.toml  - manifest of the package
//! ├── Dockerfile  - dockerfile
//! ├── frontend    - submodule with the frontend
//! ├── Makefile    - make
//! ├── README.md   - README
//! ├── Rocket.toml - Rocket configuration
//! ├── rustfmt.toml- for automatic code formatting
//! └── src         - source files
//!		├── admin.rs	- admin functions
//!		├── auth.rs		- authentification functions
//!		├── bin			- additional binaries
//!		│   └── superadmin_generator.rs		- cli app for generating superadmins
//!		├── booking.rs	- booking endpoints
//!		├── db.rs		- database access functions
//!		├── lib.rs		- root; collects external crates and modules
//!		├── main.rs		- inicializes Rocket
//!		├── models.rs	- models as structures
//!		├── response.rs	- responses
//!		└── static_server.rs	- static file serving
//! ```
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(associated_type_defaults)]
#![allow(clippy::match_bool, clippy::option_map_unit_fn)]
#![deny(missing_docs)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate lazy_static;

extern crate tungstenite;
extern crate serde_cbor;
extern crate dotenv;
extern crate chrono;
extern crate serde;
extern crate sled;

use std::thread;
use std::sync::RwLock;
use std::net::{TcpListener, TcpStream};

use dotenv::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use tungstenite::{
		server::accept,
		WebSocket,
		Message,
};

pub mod static_server;
pub mod booking;
pub mod admin;
pub mod auth;

pub mod db;
pub mod models;
pub mod response;

lazy_static! {
	/// clients
	pub static ref CLIENTS: RwLock<Vec<WebSocket<TcpStream>>> = RwLock::new(vec![]);
}

/// Vrací instanci Rocketu
pub fn init() -> rocket::Rocket {
	dotenv().ok();
	let allowed_origins = AllowedOrigins::all();


	thread::spawn(|| {
		let server = TcpListener::bind("0.0.0.0:6969").unwrap();
		
		let responder = thread::spawn(|| {
		 	println!("responder thread");
 			let d = db::Database::<db::table::Reservations>::open().unwrap();
 			let users = d.read();
 			
 			for _ in users.tree.watch_prefix(vec![]) {
 			 	println!("prefix update");
				let mut c = CLIENTS.write().unwrap();

 				let mut to_kill = vec![];
 
				for (i, l) in c.iter_mut().enumerate() {
					if l.write_message(Message::text("update!")).is_err() {
						to_kill.push(i);
					}
				}				
		
				to_kill.iter().for_each(|x| { c.remove(*x); });
				to_kill.clear();
 			}
		});	

	 	println!("ws start");
		for stream in server.incoming() {
		 	println!("websocket");
			let websocket = accept(stream.unwrap()).unwrap();
			CLIENTS.write().unwrap().push(websocket);
		}
		
		responder.join()
	});

	// You can also deserialize this
	let cors = rocket_cors::CorsOptions {
		allowed_origins,
		allowed_methods: vec![Method::Get, Method::Post, Method::Options, Method::Patch, Method::Delete, Method::Head]
			.into_iter()
			.map(From::from)
			.collect(),
		allowed_headers: AllowedHeaders::all(),
		allow_credentials: true,
		..Default::default()
	}
	.to_cors()
	.unwrap();

	rocket::ignite()
		.register(catchers![static_server::not_found])
		.mount("/", routes![static_server::index, static_server::frontend, static_server::favicon, auth::me])
		.mount("/api/", booking::routes())
		.mount("/admin/", admin::routes())
		.attach(cors)
}
