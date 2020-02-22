//! Rust Types.

use std::io::prelude::*;
use std::convert::From;
use serde::{Serialize, Deserialize};
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::Pg;
use postgis::ewkb::Point;
use crate::sql_types::*;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct GeogPoint {
	pub lon: f64,
	pub lat: f64,
	pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
	fn from(p: Point) -> Self {
		let Point { x, y, srid } = p;
		Self { lon: x, lat: y, srid }
	}
}
impl From<GeogPoint> for Point {
	fn from(p: GeogPoint) -> Self {
		let GeogPoint { lon, lat, srid } = p;
		Self { x: lon, y: lat, srid }
	}
}

impl FromSql<Geography, Pg> for GeogPoint {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		use std::io::Cursor;
		use postgis::ewkb::EwkbRead;
		let bytes = not_none!(bytes);
		let mut rdr = Cursor::new(bytes);
		Ok(Point::read_ewkb(&mut rdr)?.into())
	}
}

impl ToSql<Geography, Pg> for GeogPoint {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
		Point::from(*self).as_ewkb().write_ewkb(out)?;
		Ok(IsNull::No)
	}
}
