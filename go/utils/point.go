package utils

import "math"

// Point2D is comparable, so it can be a map key.
type Point2D struct {
	X, Y int64
}

func (p Point2D) StraightDistance(other Point2D) float64 {
	dx, dy := other.X-p.X, other.Y-p.Y
	return math.Sqrt(float64(dx*dx + dy*dy))
}

// GridlikeCornerRectArea is the number of cells in the rectangle that has these
// two points as opposite corners, counting the corners' own rows and columns.
func (p Point2D) GridlikeCornerRectArea(other Point2D) int64 {
	return (abs(other.X-p.X) + 1) * (abs(other.Y-p.Y) + 1)
}

// Point3D is comparable, so it can be a map key.
type Point3D struct {
	X, Y, Z int64
}

func (p Point3D) StraightDistance(other Point3D) float64 {
	dx, dy, dz := other.X-p.X, other.Y-p.Y, other.Z-p.Z
	return math.Sqrt(float64(dx*dx + dy*dy + dz*dz))
}
