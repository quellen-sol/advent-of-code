from collections.abc import Iterable, Iterator
from enum import Enum
from typing import Final


class GridDirection(Enum):
    """Each value is the (x, y) offset of one step that way. y grows southwards."""

    NORTH = (0, -1)
    SOUTH = (0, 1)
    EAST = (1, 0)
    WEST = (-1, 0)
    NORTH_EAST = (1, -1)
    NORTH_WEST = (-1, -1)
    SOUTH_EAST = (1, 1)
    SOUTH_WEST = (-1, 1)

    @property
    def offset(self) -> tuple[int, int]:
        return self.value

    @staticmethod
    def from_offset(offset: tuple[int, int]) -> GridDirection | None:
        return _BY_OFFSET.get(offset)

    def opposite(self) -> GridDirection:
        return _OPPOSITES[self]

    def rotate_right(self) -> GridDirection:
        """Turns 90° clockwise."""
        return _RIGHT_TURNS[self]

    def rotate_left(self) -> GridDirection:
        """Turns 90° anticlockwise."""
        return _LEFT_TURNS[self]


_BY_OFFSET = {direction.value: direction for direction in GridDirection}
_OPPOSITES = {d: _BY_OFFSET[(-d.value[0], -d.value[1])] for d in GridDirection}
_RIGHT_TURNS = {d: _BY_OFFSET[(-d.value[1], d.value[0])] for d in GridDirection}
_LEFT_TURNS = {d: _BY_OFFSET[(d.value[1], -d.value[0])] for d in GridDirection}


def _steps(*directions: GridDirection) -> tuple[tuple[GridDirection, int, int], ...]:
    return tuple((d, d.value[0], d.value[1]) for d in directions)


# Clockwise from north.
_NEIGHBOR_STEPS = _steps(
    GridDirection.NORTH,
    GridDirection.NORTH_EAST,
    GridDirection.EAST,
    GridDirection.SOUTH_EAST,
    GridDirection.SOUTH,
    GridDirection.SOUTH_WEST,
    GridDirection.WEST,
    GridDirection.NORTH_WEST,
)
_PLUS_STEPS = _NEIGHBOR_STEPS[0::2]
_DIAGONAL_STEPS = _NEIGHBOR_STEPS[1::2]


class _Break:
    __slots__ = ()

    def __repr__(self) -> str:
        return "GRID_BREAK"


GRID_BREAK: Final = _Break()
"""Ends the current row when it appears in the items given to Grid.from_iter."""


class GridNode[T]:
    """Two nodes are equal when they have the same position, whatever their values."""

    __slots__ = ("value", "x", "y")

    def __init__(self, value: T, x: int, y: int) -> None:
        self.value = value
        self.x = x
        self.y = y

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, GridNode):
            return NotImplemented
        return self.x == other.x and self.y == other.y

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __repr__(self) -> str:
        return f"GridNode({self.value!r}, x={self.x}, y={self.y})"

    def add_direction(self, direction: GridDirection) -> tuple[int, int]:
        dx, dy = direction.value
        return self.x + dx, self.y + dy

    def get_node_in_direction(
        self, grid: Grid[T], direction: GridDirection
    ) -> GridNode[T] | None:
        dx, dy = direction.value
        return grid.get_node(self.x + dx, self.y + dy)

    def neighbors(self, grid: Grid[T]) -> Iterator[tuple[GridDirection, GridNode[T]]]:
        """All eight surrounding nodes that exist, clockwise from north."""
        return self._walk(grid, _NEIGHBOR_STEPS)

    def plus_walk(self, grid: Grid[T]) -> Iterator[tuple[GridDirection, GridNode[T]]]:
        """The orthogonal neighbors that exist, clockwise from north."""
        return self._walk(grid, _PLUS_STEPS)

    def diagonal_walk(
        self, grid: Grid[T]
    ) -> Iterator[tuple[GridDirection, GridNode[T]]]:
        """The diagonal neighbors that exist, clockwise from north-east."""
        return self._walk(grid, _DIAGONAL_STEPS)

    def _walk(
        self, grid: Grid[T], steps: tuple[tuple[GridDirection, int, int], ...]
    ) -> Iterator[tuple[GridDirection, GridNode[T]]]:
        for direction, dx, dy in steps:
            node = grid.get_node(self.x + dx, self.y + dy)
            if node is not None:
                yield direction, node

    def direction_iter(
        self, grid: Grid[T], direction: GridDirection
    ) -> Iterator[GridNode[T]]:
        """Walks in a straight line to the edge of the grid. Starts with the next
        node in that direction, not with this one."""
        dx, dy = direction.value
        x, y = self.x + dx, self.y + dy
        while (node := grid.get_node(x, y)) is not None:
            yield node
            x += dx
            y += dy

    def try_get_direction(self, other: GridNode[T]) -> GridDirection | None:
        """The direction of other, if it is one of this node's neighbors."""
        return _BY_OFFSET.get((other.x - self.x, other.y - self.y))

    def distance(self, other: GridNode[T]) -> int:
        """The Manhattan distance."""
        return abs(other.y - self.y) + abs(other.x - self.x)

    def slope(self, other: GridNode[T]) -> tuple[int, int]:
        """Returns (rise, run)."""
        return other.y - self.y, other.x - self.x

    def iter_node_by_slope(
        self, grid: Grid[T], slope: tuple[int, int]
    ) -> Iterator[GridNode[T]]:
        """Steps by slope = (rise, run) until it leaves the grid, starting with this node."""
        return grid.iter_by_slope((self.x, self.y), slope)


class Grid[T]:
    __slots__ = ("inner_data",)

    def __init__(self, inner_data: list[list[GridNode[T]]]) -> None:
        self.inner_data = inner_data

    @classmethod
    def from_iter(cls, items: Iterable[T | _Break]) -> Grid[T]:
        """Fills rows left to right, starting a new row at every GRID_BREAK."""
        inner_data: list[list[GridNode[T]]] = []
        row: list[GridNode[T]] = []
        for item in items:
            if item is GRID_BREAK:
                inner_data.append(row)
                row = []
            else:
                row.append(GridNode(item, len(row), len(inner_data)))

        if row:
            inner_data.append(row)

        return cls(inner_data)

    def get_node(self, x: int, y: int) -> GridNode[T] | None:
        if 0 <= y < len(self.inner_data):
            row = self.inner_data[y]
            if 0 <= x < len(row):
                return row[x]
        return None

    def scan(self) -> Iterator[GridNode[T]]:
        """Every node, row by row."""
        for row in self.inner_data:
            yield from row

    def iter_rows(self) -> Iterator[GridNode[T]]:
        """Every node, row by row."""
        return self.scan()

    def iter_rows_rev(self) -> Iterator[GridNode[T]]:
        """Every node, row by row, starting from the bottom right."""
        for row in reversed(self.inner_data):
            yield from reversed(row)

    def iter_columns(self) -> Iterator[GridNode[T]]:
        """Every node, column by column."""
        for x in range(self.num_columns()):
            for row in self.inner_data:
                if x < len(row):
                    yield row[x]

    def iter_columns_rev(self) -> Iterator[GridNode[T]]:
        """Every node, column by column, starting from the bottom right."""
        for x in reversed(range(self.num_columns())):
            for row in reversed(self.inner_data):
                if x < len(row):
                    yield row[x]

    def traverse_row(self, y: int) -> Iterator[GridNode[T]]:
        """Row y, west to east."""
        if not 0 <= y < len(self.inner_data):
            raise IndexError(f"row {y} is outside the grid")
        return iter(self.inner_data[y])

    def traverse_column(self, x: int) -> Iterator[GridNode[T]]:
        """Column x, north to south."""
        start = self.get_node(x, 0)
        if start is None:
            raise IndexError(f"column {x} is outside the grid")
        return self._column_from(start)

    def _column_from(self, start: GridNode[T]) -> Iterator[GridNode[T]]:
        yield start
        yield from start.direction_iter(self, GridDirection.SOUTH)

    def iter_by_slope(
        self, start_position: tuple[int, int], slope: tuple[int, int]
    ) -> Iterator[GridNode[T]]:
        """Steps by slope = (rise, run) until it leaves the grid, starting with the
        node at start_position = (x, y)."""
        x, y = start_position
        rise, run = slope
        while (node := self.get_node(x, y)) is not None:
            yield node
            y += rise
            x += run

    def num_rows(self) -> int:
        return len(self.inner_data)

    def num_columns(self) -> int:
        return len(self.inner_data[0]) if self.inner_data else 0
