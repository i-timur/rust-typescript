mod shapes;

use std::{str::FromStr, fmt::Display};

use anyhow::Result;
use shapes::collisions::{Points, Contains};
use crate::shapes::{circ::Circle, rect::Rect, collisions::Collidable};

enum Shape {
	Circle(Circle),
	Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let (shape, data) = s.split_once(" ").unwrap_or((" ", " "));

		match shape {
			"rect" => return Ok(Shape::Rect(data.parse()?)),
			"circle" => return Ok(Shape::Circle(data.parse()?)),
			_ => return Err(anyhow::anyhow!("bad shape")),
		}
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
		match self {
			Shape::Circle(c) => return c.points(),
			Shape::Rect(c) => return c.points(),
		}
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
		match self {
			Shape::Circle(c) => return c.contains_point(point),
			Shape::Rect(c) => return c.contains_point(point),
		}
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Shape::Circle(c) => return  write!(f, "{}", c),
			Shape::Rect(r) => return  write!(f, "{}", r),
		}
    }
}

fn main() -> Result<()> {

	let shapes = std::fs::read_to_string("shapes")?
		.lines()
		.filter_map(|x| x.parse::<Shape>().ok())
		.collect::<Vec<_>>();

	let collisions = shapes
		.iter()
		.skip(1)
		.zip(shapes
			.iter()
			.take(shapes.len() - 1))
		.filter(|(a, b)| a.collide(b))
		.for_each(|(a, b)| {
			println!("{} collides with {}", a, b)
		});
	
	return Ok(());
}
