//! includes model describing the server responses

use rocket::response::Responder;
use rocket_contrib::json::Json;

use crate::models::Reservation;

type S = String;

/// describes possible responses
#[derive(Responder)]
pub enum Response {
	/// Ok - 200
	#[response(status = 200, content_type = "json")]
	Ok(Json<Vec<(u64, Reservation)>>),

	/// Bad Request - 400
	#[response(status = 400, content_type = "json")]
	BadRequest(Option<S>),

	/// Forbidden - 403
	#[response(status = 403, content_type = "json")]
	Forbidden(Option<S>),

	/// Not Found - 404
	#[response(status = 404, content_type = "json")]
	NotFound(Option<S>),

	/// Internal Server Error - 500
	#[response(status = 500, content_type = "json")]
	InternalServerError(Option<S>),
}

impl Response {
	fn from_option(x: Option<Json<Vec<(u64, Reservation)>>>) -> Self {
		match x {
			Some(s) => Self::Ok(s),
			None => Self::NotFound(None)
		}
	}

	fn from_result(x: Result<Json<Vec<(u64, Reservation)>>, (u16, Option<S>)>) -> Self {
		match x {
			Ok(s) => Self::Ok(s),
			Err((id, content)) => match id {
				400 => {Self::BadRequest(content)},
				403 => {Self::Forbidden(content)},
				404 => {Self::NotFound(content)},
				500 => {Self::InternalServerError(content)},
				_ => panic!("Error parsing http status code : {}", id)
			}
		}
	}
}
