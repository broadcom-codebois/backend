//! contains models

use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;

use serde::{Serialize, Deserialize};
use sled::{Db, Tree};
use serde_cbor;

use std::env;
use std::ops::Drop;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::iter::Iterator;
use std::marker::PhantomData;

lazy_static! {
	/// a global handle to the Sled database
	pub static ref DB: RwLock<Db> = RwLock::new({
		Db::open(&env::var("DATABASE_URL").expect("failed to read DATABASE_URL environment variable"))
			.expect("failed to open database")
	});
}

/// manages a tree and ensures it's type safety
/// also allows automatic type conversions
pub struct TreeMan<K, V>
where
	for<'a> K: Serialize + Deserialize<'a>,
	for<'b> V: Serialize + Deserialize<'b>,
{
	tree: Tree,
	_k:   PhantomData<K>,
	_v:   PhantomData<V>,
}

impl<K, V> TreeMan<K, V>
where
	for<'a> K: Serialize + Deserialize<'a>,
	for<'b> V: Serialize + Deserialize<'b>,
{
	/// create a new tree manager from a tree
	pub fn from_tree(tree: Tree) -> Self {
		Self { tree, _k: PhantomData, _v: PhantomData }
	}

	/// creates an iterator over (K, V)
	pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
		self.tree.iter().filter_map(|res| {
			if let Ok((k, v)) = res {
				Some((serde_cbor::from_slice::<K>(&k).ok()?, serde_cbor::from_slice::<V>(&v).ok()?))
			} else {
				None
			}
		})
	}

	/// try to get a value from the database
	pub fn get<Key: Borrow<K>>(&self, k: Key) -> Option<V> {
		self.tree
			.get(serde_cbor::to_vec(k.borrow()).unwrap()) // can't fail
			.ok()
			.flatten()
			.map(|v| serde_cbor::from_slice::<V>(&*v).ok())
			.flatten()
	}

	/// try to insert into database
	pub fn insert<Key: Borrow<K>, Value: Borrow<V>>(&mut self, k: Key, v: Value) -> sled::Result<Option<sled::IVec>> {
		let tmp = self.tree.insert(serde_cbor::to_vec(k.borrow()).unwrap(), serde_cbor::to_vec(v.borrow()).unwrap());
		println!("{:?}", tmp);
		tmp
		// can't fail
	}

	/// update a key
	pub fn update<Key, Value, F>(&mut self, k: Key, fun: F) -> sled::Result<Option<sled::IVec>>
	where
		Key: Borrow<K>,
		Value: Borrow<V>,
		F: Fn(Option<V>) -> Option<V>,
	{
		self.tree.update_and_fetch(serde_cbor::to_vec(k.borrow()).unwrap(), |value| {
			let value = value.and_then(|val| serde_cbor::from_slice(val.borrow()).ok());
			let res = fun(value);
			res.and_then(|v| serde_cbor::to_vec(v.borrow()).ok())
		})
	}

	/// remove a value
	pub fn delete<Key: Borrow<K>>(&mut self, k: Key) -> sled::Result<Option<sled::IVec>> {
		self.tree.remove(serde_cbor::to_vec(k.borrow()).unwrap())
	}
}

/// wraps the database
///
/// the reasons are two:
/// 1. to allow  FromRequest implementation
/// 2. declarative table access
pub struct Database<T: Table>(TreeMan<T::Key, T::Value>, PhantomData<T>);

impl<T: Table> Database<T> {
	/// read-only access to tree
	pub fn read(&self) -> &TreeMan<T::Key, T::Value> {
		&self.0
	}

	/// read and write access to tree
	pub fn write(&mut self) -> &mut TreeMan<T::Key, T::Value> {
		&mut self.0
	}

	/// procures a new random u64 key
	pub fn get_key() -> sled::Result<u64> {
		let lock = DB.read().expect("the rwlock has been poisoned");
		lock.generate_id()
	}

	/// opens the databasse
	pub fn open() -> Option<Self> {
		if T::has_get_tree() {
			match T::get_tree() {
				Ok(t) => Some(Database(TreeMan::from_tree(t), PhantomData)),
				Err(_) => None,
			}
		} else {
			match T::get_tree_naive() {
				Ok(t) => Some(Database(TreeMan::from_tree(t), PhantomData)),
				Err(_) => None,
			}
		}
	}
}

/// trait for the Table marker types
pub trait Table {
	/// opening a table might not always work,
	/// this type should explain what's the issue
	type TableError = sled::Error;
	/// type of the key/ID
	type Key: Serialize + for<'a> Deserialize<'a>;
	/// type of the value
	type Value: Serialize + for<'b> Deserialize<'b>;

	/// name (actually prefix) of the table
	fn name() -> &'static str;
	/// gets the actual tree, should do it using the global DB handle
	fn get_tree_naive() -> Result<Tree, sled::Error> {
		let lock = DB.read().expect("the database rwlock has been poisoned");
		// ^ this usually implies a deeper underlying problem,
		// so it's probably okay to hard crash

		lock.open_tree(&Self::name())
	}

	/// should return true if a custom get tree function is available
	fn has_get_tree() -> bool {
		false
	}

	/// optional custom function for fetching a tree,
	/// can call [`Table::get_tree_naive`]
	fn get_tree() -> Result<Tree, Self::TableError> {
		unimplemented!()
	}
}

/// module containing table markers
pub mod table {
	use super::Table;
	use crate::models::{Reservation, User};

	/// Reservation database table marker
	pub struct Reservations;

	impl Table for Reservations {
		type Key = u64;
		type Value = Reservation;

		fn name() -> &'static str {
			"reservation"
		}
	}

	/// Users database table marker
	pub struct Users;

	impl Table for Users {
		type Key = String;
		type Value = User;

		fn name() -> &'static str {
			"user"
		}
	}
}

impl<'a, 'r, T: Table> FromRequest<'a, 'r> for Database<T> {
	type Error = &'static str;

	fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		if let Some(db) = Database::<T>::open() {
			Outcome::Success(db)
		} else {
			Outcome::Failure((Status::InternalServerError, match T::has_get_tree() {
				true => "failed to run custom db-loading function",
				false => "failed to load database",
			}))
		}
	}
}

impl<T: Table> Drop for Database<T> {
	fn drop(&mut self) {
		self.0.tree.flush();
		println!("tree {}: {}", String::from_utf8_lossy(&self.0.tree.name()), self.0.tree.len())
	}
}
