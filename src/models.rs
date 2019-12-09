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
	/// event name
	pub name: String,
	/// event description
	pub description: String,
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
	/// begin time
	pub begin_time: DateTime<Utc>,
	/// end time
	pub end_time: DateTime<Utc>,
	/// furniture layout
	pub layout: u8,
	/// amount of people
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
	/// event name
	pub name: Option<String>,
	/// event description
	pub description: Option<String>,
	/// rooms that the author wills to reserve
	///
	/// bitflag based:
	/// ```
	/// 0b00 -> no room - nothing happens
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> the whole auditorium
	/// ```
	pub rooms: Option<u8>,
	/// begin time
	pub begin_time: Option<DateTime<Utc>>,
	/// end time
	pub end_time: Option<DateTime<Utc>>,
	/// furniture layout
	pub layout: Option<u8>,
	/// amount of people
	pub people: Option<u16>,
}

/// user model
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
	/// user name
	pub name: String,
	/// user email
	pub email: String,
	/// user role
	pub role: String,
}
