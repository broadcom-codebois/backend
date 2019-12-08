//! a module containing  booking functionality
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth::AuthToken;
use crate::auth::roles::{Noob, Approver};

use crate::db::{Database, table::Reservations};

use crate::models::{NewReservation, UpdateReservation, Reservation};

/*
** TODO proper type for response, handle RGI responses
*/

/// returns all reservations
///
/// GET /events "application/json"
#[get("/events", format = "application/json")]
pub fn list(db: Database<Reservations>) -> Json<Vec<(u64, Reservation)>> {
	Json(db.read().iter().collect::<Vec<(u64, Reservation)>>())
}

/// returns JSON of particular reservation
///
/// GET /events/<id> application/json
///
/// parameters:
/// - `id`: identification number of the reservation
#[get("/events/<id>")]
pub fn get(id: u64, db: Database<Reservations>, _u: AuthToken<Noob>) -> Option<Json<Reservation>> {
	db.read()
		.get(id) // can't fail
		.map(Json)
}

/// returns JSON of particular reservation
///
/// POST /events application/json
///
/// data: [`NewReservation`]
#[post("/events", data = "<input>")]
pub fn post(input: Json<NewReservation>, mut db: Database<Reservations>, usr: AuthToken<Noob>) -> Option<()> {
	let has_conflict = db.read().iter().any(|(_, x)| {
		x.approved
			&& x.begin_time <= input.end_time
			&& x.end_time >= input.begin_time
			&& (x.rooms == 3 || x.rooms == input.rooms)
	});

	if has_conflict {
		return None; // todo proper errors
	}

	let mut new_res: Reservation = input.into_inner().into();

	new_res.author = usr.user.email;

	db.write().insert(Database::<Reservations>::get_key().unwrap(), new_res).ok()?.map(|_| ())
}

/// edits a particular reservation
///
/// PATCH /events/<id> application/json
///
/// parameters:
/// - `id`: identification number of the reservation
///
/// data:[`UpdateReservation`]
#[patch("/events/<id>", data = "<input>")]
pub fn patch(
	id: u64,
	input: Json<UpdateReservation>,
	mut db: Database<Reservations>,
	usr: AuthToken<Noob>,
) -> Option<()> {
	let event = db.read().get(id)?;

	// TODO  roles are uggly
	if event.author != usr.user.email || usr.user.role != "approver" {
		return None;
	}

	let update_result = db
		.write()
		.update::<_, Reservation, _>(id, |val| {
			if let Some(mut val) = val {
				match input.clone() {
					UpdateReservation { name, description, rooms, begin_time, end_time, layout, people } => {
						name.map(|x| { val.name = x });
						description.map(|x| { val.description = x });
						rooms.map(|x| { val.rooms = x });
						begin_time.map(|x| { val.begin_time = x });
						end_time.map(|x| { val.end_time = x });
						layout.map(|x| { val.layout = x });
						people.map(|x| { val.people = x });
					}
				}

				val.approved = false;

				Some(val)
			} else {
				None
			}
		});

	if update_result.is_err() {
		return None;
	}

	Some(())
}

/// deletes the reservation
///
/// DELETE /events/<id>/
/// parameters:
/// - `id`: identification number of the reservation
#[delete("/events/<id>")]
pub fn delete(id: u64, mut db: Database<Reservations>, usr: AuthToken<Noob>) -> Option<()> {
	use crate::auth::roles::Role;

	let event = db.read().get(id)?;

	// TODO  roles are uggly
	if event.author != usr.user.email || usr.user.role != Approver::name() {
		None?
	}

	db.write().delete(id).ok()?;

	Some(())
}

/// filters by data
///
/// GET /events/filter/<rooms>/<begin_time>/end_time>
///
/// parameters:
/// - `rooms`: room bitflags
/// - `begin_time`: begin time :D
/// - `end_time`: end time :D
#[get("/events/filter/<rooms>/<begin_time>/<end_time>")]
pub fn date_filter(
	rooms: u8,
	begin_time: String,
	end_time: String,
	db: Database<Reservations>,
	_u: AuthToken<Noob>,
) -> Option<Json<Vec<(u64, Reservation)>>> {
	use chrono::{DateTime, offset::Utc};
	let begin_time = DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&begin_time).ok()?);
	let end_time = DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&end_time).ok()?);

	Some(Json(
		db.read()
			.iter()
			.filter(|(_, v)| v.begin_time >= begin_time && v.begin_time <= end_time)
			.filter(|(_, v)| v.rooms == rooms)
			.collect::<Vec<(u64, Reservation)>>(),
	))
}

/// approves the endpoint
///
/// POST /events/<id>/approve
///
/// parameters:
/// - `id`: identification number of the reservation
#[post("/events/<id>/approve")]
pub fn approve(id: u64, mut db: Database<Reservations>, _u: AuthToken<Approver>) -> Option<()> {
	let event = db.read().get(id)?;

	// TODO maybe also delete conflicting events
	let has_conflict = db.read().iter().any(|(_, x)| {
		x.approved
			&& x.begin_time <= event.end_time
			&& x.end_time >= event.begin_time
			&& (x.rooms == 3 || x.rooms == event.rooms)
	});

	if !has_conflict {
		db.write()
			.update::<_, Reservation, _>(id, |x| if let Some(mut x) = x {
				x.approved = true;
				Some(x)
			} else {None}).ok()?;
	}

	Some(())
}

/// returns a list of endpoints for rocket binding
pub fn routes() -> Vec<Route> {
	routes![date_filter, list, approve, get, post, patch, delete,]
}
