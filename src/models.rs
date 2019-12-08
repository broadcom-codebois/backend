//! contains database models and helper structs

use serde::{Serialize, Deserialize};
use chrono::{DateTime, offset::Utc};

use std::convert::From;

/// reservation model, as is saved in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
	/// event name
	pub name: String,
	/// event description
	pub description: String,
	/// reservation author
	pub author: String,
	/// rooms that the author wills to reserve
	///
	/// bitflag based:
	/// ```
	/// 0b00 -> no room - nothing happens
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> the whole auditorium
	/// ```
	pub rooms: u8,
	/// begin time of the reservation
	pub begin_time: DateTime<Utc>,
	/// end time of the reservation
	pub end_time: DateTime<Utc>,
	/// furniture layout
	pub layout: u8,
	/// is the reservation approved?
	pub approved: bool,
	/// amount of people
	pub people: u16,
}

/// reservation model for adding to the database
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewReservation {
	/// název události
	pub name: String,
	/// popis události
	pub description: String,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: u8,
	/// počáteční čas rezervace
	pub begin_time: DateTime<Utc>,
	/// čas, kdy rezervace končí
	pub end_time: DateTime<Utc>,
	/// rozložení nábytku v audioriu
	pub layout: u8,
	/// počet lidí
	pub people: u16,
}

impl From<NewReservation> for Reservation {
	fn from(src: NewReservation) -> Reservation {
		Reservation {
			name:        src.name,
			description: src.description,
			author:      String::new(),
			rooms:       src.rooms,
			begin_time:  src.begin_time,
			end_time:    src.end_time,
			layout:      src.layout,
			approved:    false,
			people:      src.people,
		}
	}
}

/// Weird quick models
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateReservation {
	/// název události
	pub name: Option<String>,
	/// popis události
	pub description: Option<String>,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: Option<u8>,
	/// počáteční čas rezervace
	pub begin_time: Option<DateTime<Utc>>,
	/// čas, kdy rezervace končí
	pub end_time: Option<DateTime<Utc>>,
	/// rozložení nábytku v audioriu
	pub layout: Option<u8>,
	/// počet lidí
	pub people: Option<u16>,
}

/// Model usera
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
	/// jméno uživatele
	pub name: String,
	/// email
	pub email: String,
	/// role
	pub role: String,
}
