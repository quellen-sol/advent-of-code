import assert from "node:assert/strict";
import { test } from "node:test";

import { GRID_BREAK, Grid, GridDirection, GridNode } from "./grid";

const GRID = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

function makeGrid(): Grid<string> {
    return Grid.fromIter(
        Array.from(GRID, (c) => (c === "\n" ? GRID_BREAK : c)),
    );
}

function values(nodes: Iterable<GridNode<string>>): string {
    return Array.from(nodes, (node) => node.value).join("");
}

function walked(
    steps: Iterable<[GridDirection, GridNode<string>]>,
): [GridDirection, string][] {
    return Array.from(steps, ([direction, node]) => [direction, node.value]);
}

function node(grid: Grid<string>, x: number, y: number): GridNode<string> {
    const found = grid.getNode(x, y);
    assert.ok(found);
    return found;
}

test("grid from iter", () => {
    const grid = makeGrid();
    assert.equal(node(grid, 0, 9).value, "M");
    assert.deepEqual([grid.numRows(), grid.numColumns()], [10, 10]);
});

test("grid get node", () => {
    const grid = makeGrid();
    assert.equal(node(grid, 0, 0).value, "M");
    assert.equal(node(grid, 1, 1).value, "S");
});

test("grid get node outside the grid", () => {
    const grid = makeGrid();
    for (const [x, y] of [
        [-1, 0],
        [0, -1],
        [10, 0],
        [0, 10],
    ] as const) {
        assert.equal(grid.getNode(x, y), undefined);
    }
});

test("grid neighbors", () => {
    const grid = makeGrid();
    assert.deepEqual(walked(node(grid, 0, 0).neighbors(grid)), [
        [GridDirection.East, "M"],
        [GridDirection.SouthEast, "S"],
        [GridDirection.South, "M"],
    ]);
});

test("directional iter", () => {
    const grid = makeGrid();
    const nodes = node(grid, 0, 0).directionIter(grid, GridDirection.South);
    assert.equal(values(nodes).slice(0, 4), "MAMX");
});

test("grid iter columns", () => {
    assert.equal(values(makeGrid().iterColumns()).slice(0, 4), "MMAM");
});

test("grid iter columns rev", () => {
    assert.equal(values(makeGrid().iterColumnsRev()).slice(0, 4), "XMAS");
});

test("grid iter rows", () => {
    assert.equal(values(makeGrid().iterRows()), GRID.replaceAll("\n", ""));
});

test("grid iter rows rev", () => {
    assert.equal(values(makeGrid().iterRowsRev()).slice(0, 4), "XSAM");
});

test("grid plus walk", () => {
    const grid = makeGrid();
    assert.equal(
        values(Array.from(node(grid, 1, 1).plusWalk(grid), ([, n]) => n)),
        "MAMM",
    );
});

test("grid plus walk 2", () => {
    const grid = makeGrid();
    assert.equal(
        values(Array.from(node(grid, 0, 0).plusWalk(grid), ([, n]) => n)),
        "MM",
    );
});

test("diagonal walk on an edge", () => {
    const grid = makeGrid();
    assert.deepEqual(walked(node(grid, 9, 1).diagonalWalk(grid)), [
        [GridDirection.SouthWest, "M"],
        [GridDirection.NorthWest, "S"],
    ]);
});

test("traverse row and column include the first node", () => {
    const grid = makeGrid();
    assert.equal(values(grid.traverseRow(0)), "MMMSXXMASM");
    assert.equal(values(grid.traverseColumn(0)), "MMAMXXSSMM");
    assert.throws(() => grid.traverseRow(10), RangeError);
});

test("iter by slope", () => {
    const grid = makeGrid();
    assert.equal(values(grid.iterBySlope([0, 0], [1, 1])), "MSXMAXSAMX");
    assert.equal(
        values(node(grid, 0, 9).iterNodeBySlope(grid, [-1, 2])),
        "MMMSM",
    );
});

test("node relations", () => {
    const grid = makeGrid();
    const origin = node(grid, 0, 0);
    assert.equal(
        origin.tryGetDirection(node(grid, 1, 1)),
        GridDirection.SouthEast,
    );
    assert.equal(origin.tryGetDirection(node(grid, 2, 2)), undefined);
    assert.equal(origin.tryGetDirection(origin), undefined);
    assert.equal(origin.distance(node(grid, 3, 4)), 7);
    assert.deepEqual(origin.slope(node(grid, 3, 4)), [4, 3]);
    assert.deepEqual(origin.addDirection(GridDirection.NorthWest), [-1, -1]);
});

test("nodes are equal by position", () => {
    assert.ok(new GridNode("a", 1, 2).equals(new GridNode("b", 1, 2)));
    assert.ok(!new GridNode("a", 1, 2).equals(new GridNode("a", 2, 1)));
});

test("direction rotations", () => {
    assert.equal(GridDirection.North.rotateRight(), GridDirection.East);
    assert.equal(
        GridDirection.NorthWest.rotateRight(),
        GridDirection.NorthEast,
    );
    assert.equal(GridDirection.North.rotateLeft(), GridDirection.West);
    assert.equal(GridDirection.SouthEast.rotateLeft(), GridDirection.NorthEast);
    for (const direction of GridDirection.all) {
        assert.equal(direction.rotateRight().rotateLeft(), direction);
        assert.equal(
            direction.rotateRight().rotateRight(),
            direction.opposite(),
        );
    }
});

test("direction offsets", () => {
    assert.deepEqual(GridDirection.SouthWest.offset, [-1, 1]);
    assert.equal(GridDirection.fromOffset(1, -1), GridDirection.NorthEast);
    assert.equal(GridDirection.fromOffset(0, 0), undefined);
    assert.equal(GridDirection.fromOffset(2, 0), undefined);
});
