use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth::AuthToken;
use crate::auth::roles::{Noob, Approver, Role};

use crate::db;
use crate::db::{NewReservation, UpdateReservation, Reservation};

use diesel::prelude::*;

/*
** TODO proper type for response, handle RGI responses
*/

/// vrací všechny rezervace
///
/// GET /events "application/json"
#[get("/events", format = "application/json")]
pub fn list() -> Option<Json<Vec<Reservation>>> {
	use crate::schema::booking::dsl::*;

	let conn = db::get_con();
	 
	booking
		.load::<Reservation>(&conn)
		.ok()
		.map(Json)
}

/// vrátí JSON dané rezervace
///
/// GET /events/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
#[get("/events/<event_id>")]
pub fn get(event_id: i32, _u: AuthToken<Noob>) -> Option<Json<Reservation>> {
	use crate::schema::booking::dsl::*;

	let conn = db::get_con();

	booking
		.find(event_id)
		.first::<Reservation>(&conn)
		.optional()
		.ok()
		.flatten()
		.map(Json)
}

/// vrátí JSON dané rezervace
///
/// POST /events application/json
///
/// data: [`NewReservation`]
#[post("/events", data = "<_input>")]
pub fn post(_input: Json<NewReservation>, usr: AuthToken<Noob>) -> String {
	let name = usr.user.name;
	let user_id = usr.user.id;
	let email = usr.user.email;

	rgi! {
		POST "rgi/booking/booking.py"
		arg: user_id,
		arg: name,
		arg: email
		data: (&_input.into_inner())
	}
}

/// upraví danou rezervaci
///
/// PATCH /events/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
///
/// data:[`UpdateReservation`]
#[patch("/events/<r_id>", data = "<_input>")]
pub fn patch(r_id: i32, _input: Json<UpdateReservation>, usr: AuthToken<Noob>) -> Option<String> {
	// TODO return error instead of None on invalid states
	if r_id < 0 {
		None?
	}

	if usr.user.role.to_lowercase() != Approver::name() {
		use crate::schema::booking::dsl::*;

		let con = db::get_con();
		let reservation = booking.filter(id.eq(r_id)).first::<Reservation>(&con).ok()?;

		if reservation.author.trim() != usr.user.email.trim() {
			None? // you shouldn't be able to edit others' events
		}
	}

	let id = r_id;
	Some(rgi! {
		PATCH "rgi/booking/booking.py"
		arg: id
		data: (&_input.into_inner())
	})
}

/// vymaže danou rezervaci
///
/// DEL///
/// parametry:
/// - `id`: identifikátor dané rezervace
#[delete("/events/<r_id>")]
pub fn delete(r_id: i32, usr: AuthToken<Noob>) -> Option<String> {
	// TODO return error instead of None on invalid states
	if r_id < 0 {
		None?
	}

	if usr.user.role.to_lowercase() != Approver::name() {
		use crate::schema::booking::dsl::*;

		let con = db::get_con();
		let reservation = booking.filter(id.eq(r_id)).first::<Reservation>(&con).ok()?;

		if reservation.author.trim() != usr.user.email.trim() {
			None? // you shouldn't be able to delete others' either
		}
	}

	let id = r_id;
	Some(rgi! {
		DELETE "rgi/booking/booking.py"
		arg: id
	})
}

/// filtruje podle data
///
/// GET /events/filter/<rooms>/<begin_time>/end_time>
///
/// parametry:
/// - `rooms`:  bitflagy pro místnosti, viz [`Reservation`]
/// - `begin_time`: počáteční čas
/// - `end_time`: čas konce
#[get("/events/filter/<rooms>/<begin_time>/<end_time>")]
pub fn date_filter(rooms: i32, begin_time: String, end_time: String, _u: AuthToken<Noob>) -> String {
	rgi! {
		FILTER "rgi/booking/booking.py"
		arg: rooms,
		arg: begin_time,
		arg: end_time
	}
}

/// schválí endpoint
///
/// POST /events/<id>/approve
///
/// parametry:
/// - `id`: id rezervace
#[post("/events/<id>/approve")]
pub fn approve(id: i32, _u: AuthToken<Approver>) -> String {
	rgi! {
		APPROVE "rgi/booking/booking.py"
		arg: id
	}
}

/// vrací seznam endpointů pro nabindování do Rocketu
pub fn routes() -> Vec<Route> {
	routes![date_filter, list, approve, get, post, patch, delete,]
}
