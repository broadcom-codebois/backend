//! ## backend rezervačního systému pro auditorium
//! Dokumentace backendu.
//! Jako webový framework projekt používá [Rocket](https://rocket.rs), na
//! komunikaci s databází [Diesel](https://diesel.rs).
//!
//! Modus operandi tohoto serveru spočívá v přijímání požadavků, validaci dat
//! a volání správného RGI, viz [`rgi!`]
//!
//! Struktura:
//! ```bash,no_run
//! .
//! ├── build.rs    - build skript
//! ├── Cargo.lock  - lockfile, nemazat! (deterministické buildy)
//! ├── Cargo.toml  - manifest balíčku
//! ├── diesel.toml - konfigurace Diesel.rs
//! ├── Dockerfile  - dockerfile
//! ├── frontend    - submodul s frontendem
//! ├── Makefile    - make
//! ├── migrations  - migrace
//! │   ├── 00000000000000_diesel_initial_setup
//! │   │   ├── down.sql
//! │   │   └── up.sql
//! │   └── 2019-11-25-143159_reservations - migrace relace reservations
//! │       ├── down.sql
//! │       └── up.sql
//! ├── README.md   - README
//! ├── rgi         - obsahuje RGI, viz modul rgi
//! │   ├── booking - booking rgi
//! │   │   ├── booking.py
//! │   │   ├── curltest
//! │   │   └── mod.rs
//! │   └── mod.rs
//! ├── Rocket.toml  - konfigurační soubor Rocketu
//! ├── rustfmt.toml - pravidla pro automatické formátování kódu
//! └── src          - zdrojové soubory
//!     ├── db.rs            - databázové modely, utility
//!     ├── main.rs          - entrypoint programu
//!     ├── rgi -> ../rgi/   - symlink
//!     ├── schema.rs        - schéma databáze (vygenerováno Dieselem, neupravovat!)
//!     └── static_server.rs - statický server
//! ```
#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_docs)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use std::str::FromStr;

pub mod auth;
pub mod rgi;
pub mod db;
pub mod static_server;

/// schéma databáze (vygenerováno Dieselem)
#[allow(missing_docs)]
pub mod schema;

use db::DbConn;

/// Vrací instanci Rocketu
pub fn init() -> rocket::Rocket {
	dotenv().ok();
	let allowed_origins = AllowedOrigins::all();

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
		.mount("/rgi/", rgi::routes())
		.attach(cors)
		.attach(DbConn::fairing())
}
