use rocket::response::Responder;
use rocket_contrib::json::Json;

/// describes possible responses
pub enum Response {
	pub Ok(Json),
	pub BadRequest,
	pub InternalServerError,
	pub Forbidden,
	pub NoContent,
}


