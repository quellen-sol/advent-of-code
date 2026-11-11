import unittest

from py.utils.grid import GRID_BREAK, Grid, GridDirection, GridNode

GRID = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""


def make_grid() -> Grid[str]:
    return Grid.from_iter(GRID_BREAK if c == "\n" else c for c in GRID)


def values(nodes) -> str:
    return "".join(node.value for node in nodes)


def node(grid: Grid[str], x: int, y: int) -> GridNode[str]:
    found = grid.get_node(x, y)
    assert found is not None
    return found


class GridTests(unittest.TestCase):
    def test_from_iter(self) -> None:
        grid = make_grid()
        self.assertEqual(node(grid, 0, 9).value, "M")
        self.assertEqual((grid.num_rows(), grid.num_columns()), (10, 10))

    def test_get_node(self) -> None:
        grid = make_grid()
        self.assertEqual(node(grid, 0, 0).value, "M")
        self.assertEqual(node(grid, 1, 1).value, "S")

    def test_get_node_outside_the_grid(self) -> None:
        grid = make_grid()
        for x, y in [(-1, 0), (0, -1), (10, 0), (0, 10)]:
            self.assertIsNone(grid.get_node(x, y))

    def test_neighbors(self) -> None:
        grid = make_grid()
        neighbors = list(node(grid, 0, 0).neighbors(grid))
        self.assertEqual(
            [(d, n.value) for d, n in neighbors],
            [
                (GridDirection.EAST, "M"),
                (GridDirection.SOUTH_EAST, "S"),
                (GridDirection.SOUTH, "M"),
            ],
        )

    def test_direction_iter(self) -> None:
        grid = make_grid()
        nodes = node(grid, 0, 0).direction_iter(grid, GridDirection.SOUTH)
        self.assertEqual(values(nodes)[:4], "MAMX")

    def test_iter_columns(self) -> None:
        self.assertEqual(values(make_grid().iter_columns())[:4], "MMAM")

    def test_iter_columns_rev(self) -> None:
        self.assertEqual(values(make_grid().iter_columns_rev())[:4], "XMAS")

    def test_iter_rows(self) -> None:
        self.assertEqual(values(make_grid().iter_rows()), GRID.replace("\n", ""))

    def test_iter_rows_rev(self) -> None:
        self.assertEqual(values(make_grid().iter_rows_rev())[:4], "XSAM")

    def test_plus_walk(self) -> None:
        grid = make_grid()
        self.assertEqual(values(n for _, n in node(grid, 1, 1).plus_walk(grid)), "MAMM")

    def test_plus_walk_2(self) -> None:
        grid = make_grid()
        self.assertEqual(values(n for _, n in node(grid, 0, 0).plus_walk(grid)), "MM")

    def test_diagonal_walk_on_an_edge(self) -> None:
        grid = make_grid()
        walk = list(node(grid, 9, 1).diagonal_walk(grid))
        self.assertEqual(
            [(d, n.value) for d, n in walk],
            [(GridDirection.SOUTH_WEST, "M"), (GridDirection.NORTH_WEST, "S")],
        )

    def test_traverse_row_and_column_include_the_first_node(self) -> None:
        grid = make_grid()
        self.assertEqual(values(grid.traverse_row(0)), "MMMSXXMASM")
        self.assertEqual(values(grid.traverse_column(0)), "MMAMXXSSMM")

    def test_iter_by_slope(self) -> None:
        grid = make_grid()
        self.assertEqual(values(grid.iter_by_slope((0, 0), (1, 1))), "MSXMAXSAMX")
        self.assertEqual(values(node(grid, 0, 9).iter_node_by_slope(grid, (-1, 2))), "MMMSM")

    def test_node_relations(self) -> None:
        grid = make_grid()
        origin = node(grid, 0, 0)
        self.assertEqual(origin.try_get_direction(node(grid, 1, 1)), GridDirection.SOUTH_EAST)
        self.assertIsNone(origin.try_get_direction(node(grid, 2, 2)))
        self.assertIsNone(origin.try_get_direction(origin))
        self.assertEqual(origin.distance(node(grid, 3, 4)), 7)
        self.assertEqual(origin.slope(node(grid, 3, 4)), (4, 3))
        self.assertEqual(origin.add_direction(GridDirection.NORTH_WEST), (-1, -1))

    def test_nodes_are_equal_by_position(self) -> None:
        self.assertEqual(GridNode("a", 1, 2), GridNode("b", 1, 2))
        self.assertNotEqual(GridNode("a", 1, 2), GridNode("a", 2, 1))
        self.assertEqual(len({GridNode("a", 1, 2), GridNode("b", 1, 2)}), 1)


class GridDirectionTests(unittest.TestCase):
    def test_rotations(self) -> None:
        self.assertEqual(GridDirection.NORTH.rotate_right(), GridDirection.EAST)
        self.assertEqual(GridDirection.NORTH_WEST.rotate_right(), GridDirection.NORTH_EAST)
        self.assertEqual(GridDirection.NORTH.rotate_left(), GridDirection.WEST)
        self.assertEqual(GridDirection.SOUTH_EAST.rotate_left(), GridDirection.NORTH_EAST)
        for direction in GridDirection:
            self.assertEqual(direction.rotate_right().rotate_left(), direction)
            self.assertEqual(direction.rotate_right().rotate_right(), direction.opposite())

    def test_offsets(self) -> None:
        self.assertEqual(GridDirection.SOUTH_WEST.offset, (-1, 1))
        self.assertEqual(GridDirection.from_offset((1, -1)), GridDirection.NORTH_EAST)
        self.assertIsNone(GridDirection.from_offset((0, 0)))
        self.assertIsNone(GridDirection.from_offset((2, 0)))


if __name__ == "__main__":
    unittest.main()
