#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    pub fn straight_distance(&self, other: &Self) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }

    pub fn gridlike_corner_rect_area(&self, other: &Self) -> i64 {
        (other.x - self.x + 1).abs() * (other.y - self.y + 1).abs()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    pub fn straight_distance(&self, other: &Self) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)) as f64)
            .sqrt()
    }
}
