//! static file serving
//!
//! posílá následující soubory skrze tyto routy:
//! serves the following files
//! - [`index`] -> __frontend/index.html__
//! - [`frontend`] -> files from __frontend/build__
//! - [`not_found`] -> 404.html
//!
//! adding new static route
//! ```no_run
//! #[get("/url/<path..>")]
//! pub fn moje_route(path: PathBuf) -> NamedFile {
//!     NamedFile::open(Path::new("path/to/file").join(path))
//!         .expect("unable to open file")
//! }
//! ```
//! it is needed to activate the route in main.rs
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

/// serves the index
#[get("/")]
pub fn index() -> NamedFile {
	NamedFile::open("frontend/index.html").expect("index.html not found")
}

/// returns static files of frontend
#[get("/static/<name..>")]
pub fn frontend(name: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("frontend/build/static/").join(name)).ok()
}

/// return favicon
#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
	NamedFile::open("frontend/favicon.ico").ok()
}

/// 404 catcher
#[catch(404)]
pub fn not_found() -> NamedFile {
	NamedFile::open("frontend/404.html").expect("404.html not found")
}
