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
	/// Internal Server Error - 500
	#[response(status = 500, content_type = "json")]
	InternalServerError(Option<S>),
	/// Forbidden - 403
	#[response(status = 403, content_type = "json")]
	Forbidden(Option<S>),
	/// No Content - 204
	#[response(status = 204, content_type = "json")]
	NoContent(Option<S>),
	/// Not Found - 404
	#[response(status = 404, content_type = "json")]
	NotFound(Option<S>),
}


