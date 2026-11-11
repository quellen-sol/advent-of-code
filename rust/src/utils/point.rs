#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    pub fn straight_distance(&self, other: &Self) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }

    /// The number of cells in the rectangle that has these two points as
    /// opposite corners, counting the corners' own rows and columns.
    pub fn gridlike_corner_rect_area(&self, other: &Self) -> i64 {
        ((other.x - self.x).abs() + 1) * ((other.y - self.y).abs() + 1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gridlike_corner_rect_area_in_every_orientation() {
        let area = |(ax, ay), (bx, by)| {
            Point2D { x: ax, y: ay }.gridlike_corner_rect_area(&Point2D { x: bx, y: by })
        };
        assert_eq!(area((1, 1), (3, 4)), 12);
        assert_eq!(area((0, 5), (5, 0)), 36);
        assert_eq!(area((5, 0), (0, 5)), 36);
    }
}
