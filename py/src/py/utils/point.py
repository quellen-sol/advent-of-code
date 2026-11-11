from dataclasses import dataclass
from math import sqrt


@dataclass(frozen=True, slots=True)
class Point2D:
    x: int
    y: int

    def straight_distance(self, other: Point2D) -> float:
        dx = other.x - self.x
        dy = other.y - self.y
        return sqrt(dx * dx + dy * dy)

    def gridlike_corner_rect_area(self, other: Point2D) -> int:
        """The number of cells in the rectangle that has these two points as
        opposite corners, counting the corners' own rows and columns."""
        return (abs(other.x - self.x) + 1) * (abs(other.y - self.y) + 1)


@dataclass(frozen=True, slots=True)
class Point3D:
    x: int
    y: int
    z: int

    def straight_distance(self, other: Point3D) -> float:
        dx = other.x - self.x
        dy = other.y - self.y
        dz = other.z - self.z
        return sqrt(dx * dx + dy * dy + dz * dz)
