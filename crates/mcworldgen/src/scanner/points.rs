#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl From<i64> for Point {
    fn from(val: i64) -> Self {
        Point {
            x: val,
            y: val,
            z: val,
        }
    }
}

impl Point {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn distance_to(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        let dist_squared = dx * dx + dy * dy + dz * dz;

        (dist_squared as f64).sqrt().trunc() as i64
    }

    pub fn is_near(&self, point: &Point, threshold: &Point) -> bool {
        point.x - self.x <= threshold.x
            && point.y - self.y <= threshold.y
            && point.z - self.z <= threshold.z
    }

    pub fn move_toward(&self, face: Face) -> Self {
        let delta = match face {
            Face::Top => (0, 1, 0),
            Face::Bottom => (0, -1, 0),
            Face::Left => (-1, 0, 0),
            Face::Right => (1, 0, 0),
            Face::Front => (0, 0, 1),
            Face::Back => (0, 0, -1),
        };

        Self {
            x: self.x + delta.0,
            y: self.y + delta.1,
            z: self.z + delta.2,
        }
    }

    // The neighboring positions that would directly touch one of this blocks faces
    pub fn neighbors(&self) -> impl Iterator<Item = Point> {
        vec![
            self.move_toward(Face::Top),
            // self.move_toward(Face::Top).move_toward(Face::Left),
            // self.move_toward(Face::Top).move_toward(Face::Left).move_toward(Face::Front),
            // self.move_toward(Face::Top).move_toward(Face::Left).move_toward(Face::Back),
            // self.move_toward(Face::Top).move_toward(Face::Right),
            // self.move_toward(Face::Top).move_toward(Face::Right).move_toward(Face::Front),
            // self.move_toward(Face::Top).move_toward(Face::Right).move_toward(Face::Back),
            // self.move_toward(Face::Top).move_toward(Face::Front),
            // self.move_toward(Face::Top).move_toward(Face::Back),
            self.move_toward(Face::Bottom),
            // self.move_toward(Face::Bottom).move_toward(Face::Left),
            // self.move_toward(Face::Bottom).move_toward(Face::Left).move_toward(Face::Front),
            // self.move_toward(Face::Bottom).move_toward(Face::Left).move_toward(Face::Back),
            // self.move_toward(Face::Bottom).move_toward(Face::Right),
            // self.move_toward(Face::Bottom).move_toward(Face::Right).move_toward(Face::Front),
            // self.move_toward(Face::Bottom).move_toward(Face::Right).move_toward(Face::Back),
            // self.move_toward(Face::Bottom).move_toward(Face::Front),
            // self.move_toward(Face::Bottom).move_toward(Face::Back),
            self.move_toward(Face::Left),
            // self.move_toward(Face::Left).move_toward(Face::Front),
            // self.move_toward(Face::Left).move_toward(Face::Back),
            self.move_toward(Face::Right),
            // self.move_toward(Face::Right).move_toward(Face::Front),
            // self.move_toward(Face::Right).move_toward(Face::Back),
            self.move_toward(Face::Front),
            self.move_toward(Face::Back),
        ]
        .into_iter()
    }
}
