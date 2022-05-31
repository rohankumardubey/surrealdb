use crate::sql::common::commas;
use crate::sql::error::IResult;
use crate::sql::escape::escape_ident;
use crate::sql::id::Id;
use crate::sql::ident::{ident_raw, Ident};
use crate::sql::thing::Thing;
use nom::multi::separated_list1;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use std::str;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tables(pub Vec<Table>);

impl Deref for Tables {
	type Target = Vec<Table>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl fmt::Display for Tables {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0.iter().map(|ref v| format!("{}", v)).collect::<Vec<_>>().join(", "))
	}
}

pub fn tables(i: &str) -> IResult<&str, Tables> {
	let (i, v) = separated_list1(commas, table)(i)?;
	Ok((i, Tables(v)))
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Table(pub String);

impl From<String> for Table {
	fn from(v: String) -> Self {
		Table(v)
	}
}

impl From<&str> for Table {
	fn from(v: &str) -> Self {
		Table(String::from(v))
	}
}

impl From<Ident> for Table {
	fn from(v: Ident) -> Self {
		Table(v.0)
	}
}

impl Deref for Table {
	type Target = String;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Table {
	pub fn generate(&self) -> Thing {
		Thing {
			tb: self.0.to_owned(),
			id: Id::rand(),
		}
	}
}

impl fmt::Display for Table {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", escape_ident(&self.0))
	}
}

pub fn table(i: &str) -> IResult<&str, Table> {
	let (i, v) = ident_raw(i)?;
	Ok((i, Table(v)))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn table_normal() {
		let sql = "test";
		let res = table(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("test", format!("{}", out));
		assert_eq!(out, Table(String::from("test")));
	}

	#[test]
	fn table_quoted_backtick() {
		let sql = "`test`";
		let res = table(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("test", format!("{}", out));
		assert_eq!(out, Table(String::from("test")));
	}

	#[test]
	fn table_quoted_brackets() {
		let sql = "⟨test⟩";
		let res = table(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("test", format!("{}", out));
		assert_eq!(out, Table(String::from("test")));
	}
}
