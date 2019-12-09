//! a module containing admin functions
use rocket::Route;
use rocket_contrib::json::Json;

use std::env;

use crate::auth::AuthToken;
use crate::auth::roles::{Role, Superadmin};

use crate::db::{
	Database,
	table::Reservations,
	table::Users,
};

use crate::models::{User, Reservation};

/// generates a superadmin
#[post("/generate_sa/<email>/<password>")]
pub fn generate_superadmin(email: String, password: String, mut db: Database<Users>) -> Option<()> {
	if password != env::var("SA_SECRET").unwrap() { return None }

	db.write()
		.insert(&email, User { email: email.clone(), name: "Superadmin".to_string(), role: Superadmin::name().to_string() })
		.ok()?
		.map(|_| ())
}

/// get all users
#[get("/users", format = "application/json")]
pub fn users(db: Database<Users>, _u: AuthToken<Superadmin>) -> Json<Vec<(String, User)>> {
	Json(db.read().iter().collect::<Vec<(String, User)>>())
}

/// change user role
#[patch("/users/<email>/<new_role>", format = "application/json")]
pub fn change_role(email: String, new_role: String, mut db: Database<Users>, _u: AuthToken<Superadmin>) -> Option<()> {
	if !["Noob", "Superadmin", "Approver", "FacilityManager"].contains(&new_role.as_str()) {
		return None;
	}

	db.write()
		.update::<_, User, _>(email, |x| if let Some(mut x) = x {
			x.role = new_role.clone();
			Some(x)
		} else {None}).ok()?;

	Some(())
}


/// returns a list of endpoints for rocket binding
pub fn routes() -> Vec<Route> {
	routes![users, generate_superadmin, change_role]
}
