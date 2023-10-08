use std::fmt::Display;

use super::points::Point;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}) -> ({}, {}, {})",
            self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z
        )
    }
}

impl Bounds {
    pub const ZERO: Self = Self {
        min: Point::ZERO,
        max: Point::ZERO,
    };

    pub fn center(&self) -> Point {
        Point {
            x: self.min.x + (self.max.x - self.min.x) / 2,
            y: self.min.y + (self.max.y - self.min.y) / 2,
            z: self.min.z + (self.max.z - self.min.z) / 2,
        }
    }

    pub fn from_point(point: &Point) -> Bounds {
        Self {
            min: point.clone(),
            max: point.clone(),
        }
    }

    pub fn expand(&self, amount: &Point) -> Bounds {
        Bounds {
            min: Point {
                x: self.min.x - amount.x,
                y: self.min.y - amount.y,
                z: self.min.z - amount.z,
            },
            max: Point {
                x: self.max.x + amount.x,
                y: self.max.y + amount.y,
                z: self.max.z + amount.z,
            },
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        return point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z;
    }
}
