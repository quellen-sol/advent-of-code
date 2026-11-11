export class Point2D {
    constructor(
        public x: number,
        public y: number,
    ) {}

    straightDistance(other: Point2D): number {
        const dx = other.x - this.x;
        const dy = other.y - this.y;
        return Math.sqrt(dx * dx + dy * dy);
    }

    /**
     * The number of cells in the rectangle that has these two points as
     * opposite corners, counting the corners' own rows and columns.
     */
    gridlikeCornerRectArea(other: Point2D): number {
        return (
            (Math.abs(other.x - this.x) + 1) * (Math.abs(other.y - this.y) + 1)
        );
    }

    equals(other: Point2D): boolean {
        return this.x === other.x && this.y === other.y;
    }

    /** Equal points have equal keys, for use in a `Set` or `Map`. */
    key(): string {
        return `${this.x},${this.y}`;
    }
}

export class Point3D {
    constructor(
        public x: number,
        public y: number,
        public z: number,
    ) {}

    straightDistance(other: Point3D): number {
        const dx = other.x - this.x;
        const dy = other.y - this.y;
        const dz = other.z - this.z;
        return Math.sqrt(dx * dx + dy * dy + dz * dz);
    }

    equals(other: Point3D): boolean {
        return this.x === other.x && this.y === other.y && this.z === other.z;
    }

    /** Equal points have equal keys, for use in a `Set` or `Map`. */
    key(): string {
        return `${this.x},${this.y},${this.z}`;
    }
}
