/** `[rise, run]` */
export type Slope = [number, number];

/** Compare directions with `===`: each one is a single shared instance. y grows southwards. */
export class GridDirection {
    private constructor(
        readonly name: string,
        readonly dx: number,
        readonly dy: number,
    ) {}

    static readonly North = new GridDirection("North", 0, -1);
    static readonly South = new GridDirection("South", 0, 1);
    static readonly East = new GridDirection("East", 1, 0);
    static readonly West = new GridDirection("West", -1, 0);
    static readonly NorthEast = new GridDirection("NorthEast", 1, -1);
    static readonly NorthWest = new GridDirection("NorthWest", -1, -1);
    static readonly SouthEast = new GridDirection("SouthEast", 1, 1);
    static readonly SouthWest = new GridDirection("SouthWest", -1, 1);

    static readonly all: readonly GridDirection[] = [
        GridDirection.North,
        GridDirection.South,
        GridDirection.East,
        GridDirection.West,
        GridDirection.NorthEast,
        GridDirection.NorthWest,
        GridDirection.SouthEast,
        GridDirection.SouthWest,
    ];

    /** `[dx, dy]` */
    get offset(): [number, number] {
        return [this.dx, this.dy];
    }

    static fromOffset(dx: number, dy: number): GridDirection | undefined {
        if (dx < -1 || dx > 1 || dy < -1 || dy > 1) {
            return undefined;
        }
        return DIRECTIONS_BY_OFFSET[offsetIndex(dx, dy)];
    }

    opposite(): GridDirection {
        return GridDirection.fromOffset(-this.dx, -this.dy)!;
    }

    /** Turns 90° clockwise. */
    rotateRight(): GridDirection {
        return GridDirection.fromOffset(-this.dy, this.dx)!;
    }

    /** Turns 90° anticlockwise. */
    rotateLeft(): GridDirection {
        return GridDirection.fromOffset(this.dy, -this.dx)!;
    }

    toString(): string {
        return this.name;
    }
}

function offsetIndex(dx: number, dy: number): number {
    return (dy + 1) * 3 + dx + 1;
}

const DIRECTIONS_BY_OFFSET: (GridDirection | undefined)[] = [];
for (const direction of GridDirection.all) {
    DIRECTIONS_BY_OFFSET[offsetIndex(direction.dx, direction.dy)] = direction;
}

// Clockwise from north.
const NEIGHBOR_DIRECTIONS = [
    GridDirection.North,
    GridDirection.NorthEast,
    GridDirection.East,
    GridDirection.SouthEast,
    GridDirection.South,
    GridDirection.SouthWest,
    GridDirection.West,
    GridDirection.NorthWest,
];
const PLUS_DIRECTIONS = NEIGHBOR_DIRECTIONS.filter((_, i) => i % 2 === 0);
const DIAGONAL_DIRECTIONS = NEIGHBOR_DIRECTIONS.filter((_, i) => i % 2 === 1);

/** Ends the current row when it appears in the items given to `Grid.fromIter`. */
export const GRID_BREAK: unique symbol = Symbol("GRID_BREAK");

export type GridCreationItem<T> = T | typeof GRID_BREAK;

export class GridNode<T> {
    constructor(
        public value: T,
        readonly x: number,
        readonly y: number,
    ) {}

    /** Nodes are equal when they have the same position, whatever their values. */
    equals(other: GridNode<T>): boolean {
        return this.x === other.x && this.y === other.y;
    }

    addDirection(direction: GridDirection): [number, number] {
        return [this.x + direction.dx, this.y + direction.dy];
    }

    getNodeInDirection(
        grid: Grid<T>,
        direction: GridDirection,
    ): GridNode<T> | undefined {
        return grid.getNode(this.x + direction.dx, this.y + direction.dy);
    }

    /** All eight surrounding nodes that exist, clockwise from north. */
    neighbors(grid: Grid<T>): Generator<[GridDirection, GridNode<T>]> {
        return this.walk(grid, NEIGHBOR_DIRECTIONS);
    }

    /** The orthogonal neighbors that exist, clockwise from north. */
    plusWalk(grid: Grid<T>): Generator<[GridDirection, GridNode<T>]> {
        return this.walk(grid, PLUS_DIRECTIONS);
    }

    /** The diagonal neighbors that exist, clockwise from north-east. */
    diagonalWalk(grid: Grid<T>): Generator<[GridDirection, GridNode<T>]> {
        return this.walk(grid, DIAGONAL_DIRECTIONS);
    }

    private *walk(
        grid: Grid<T>,
        directions: readonly GridDirection[],
    ): Generator<[GridDirection, GridNode<T>]> {
        for (const direction of directions) {
            const node = this.getNodeInDirection(grid, direction);
            if (node !== undefined) {
                yield [direction, node];
            }
        }
    }

    /**
     * Walks in a straight line to the edge of the grid. Starts with the next
     * node in that direction, not with this one.
     */
    *directionIter(
        grid: Grid<T>,
        direction: GridDirection,
    ): Generator<GridNode<T>> {
        let node = this.getNodeInDirection(grid, direction);
        while (node !== undefined) {
            yield node;
            node = node.getNodeInDirection(grid, direction);
        }
    }

    /** The direction of `other`, if it is one of this node's neighbors. */
    tryGetDirection(other: GridNode<T>): GridDirection | undefined {
        return GridDirection.fromOffset(other.x - this.x, other.y - this.y);
    }

    /** The Manhattan distance. */
    distance(other: GridNode<T>): number {
        return Math.abs(other.y - this.y) + Math.abs(other.x - this.x);
    }

    slope(other: GridNode<T>): Slope {
        return [other.y - this.y, other.x - this.x];
    }

    /** Steps by `slope` until it leaves the grid, starting with this node. */
    iterNodeBySlope(grid: Grid<T>, slope: Slope): Generator<GridNode<T>> {
        return grid.iterBySlope([this.x, this.y], slope);
    }
}

export class Grid<T> {
    constructor(public innerData: GridNode<T>[][]) {}

    /** Fills rows left to right, starting a new row at every `GRID_BREAK`. */
    static fromIter<T>(items: Iterable<GridCreationItem<T>>): Grid<T> {
        const innerData: GridNode<T>[][] = [];
        let row: GridNode<T>[] = [];
        for (const item of items) {
            if (item === GRID_BREAK) {
                innerData.push(row);
                row = [];
            } else {
                row.push(new GridNode(item, row.length, innerData.length));
            }
        }

        if (row.length > 0) {
            innerData.push(row);
        }

        return new Grid(innerData);
    }

    getNode(x: number, y: number): GridNode<T> | undefined {
        return this.innerData[y]?.[x];
    }

    /** Every node, row by row. */
    *scan(): Generator<GridNode<T>> {
        for (const row of this.innerData) {
            yield* row;
        }
    }

    /** Every node, row by row. */
    iterRows(): Generator<GridNode<T>> {
        return this.scan();
    }

    /** Every node, row by row, starting from the bottom right. */
    *iterRowsRev(): Generator<GridNode<T>> {
        for (let y = this.innerData.length - 1; y >= 0; y--) {
            const row = this.innerData[y]!;
            for (let x = row.length - 1; x >= 0; x--) {
                yield row[x]!;
            }
        }
    }

    /** Every node, column by column. */
    *iterColumns(): Generator<GridNode<T>> {
        const numColumns = this.numColumns();
        for (let x = 0; x < numColumns; x++) {
            for (const row of this.innerData) {
                const node = row[x];
                if (node !== undefined) {
                    yield node;
                }
            }
        }
    }

    /** Every node, column by column, starting from the bottom right. */
    *iterColumnsRev(): Generator<GridNode<T>> {
        for (let x = this.numColumns() - 1; x >= 0; x--) {
            for (let y = this.innerData.length - 1; y >= 0; y--) {
                const node = this.innerData[y]![x];
                if (node !== undefined) {
                    yield node;
                }
            }
        }
    }

    /** Row `y`, west to east. */
    traverseRow(y: number): IterableIterator<GridNode<T>> {
        const row = this.innerData[y];
        if (row === undefined) {
            throw new RangeError(`row ${y} is outside the grid`);
        }
        return row.values();
    }

    /** Column `x`, north to south. */
    traverseColumn(x: number): IterableIterator<GridNode<T>> {
        if (this.getNode(x, 0) === undefined) {
            throw new RangeError(`column ${x} is outside the grid`);
        }
        return this.iterBySlope([x, 0], [1, 0]);
    }

    /** Steps by `slope` until it leaves the grid, starting with the node at `startPosition = [x, y]`. */
    *iterBySlope(
        startPosition: [number, number],
        slope: Slope,
    ): Generator<GridNode<T>> {
        let [x, y] = startPosition;
        const [rise, run] = slope;
        let node: GridNode<T> | undefined;
        while ((node = this.getNode(x, y)) !== undefined) {
            yield node;
            y += rise;
            x += run;
        }
    }

    numRows(): number {
        return this.innerData.length;
    }

    numColumns(): number {
        return this.innerData[0]?.length ?? 0;
    }
}
