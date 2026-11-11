# Utils API map

The contract between the four utils libraries. Each `` ## `module` `` section is a module (a file named after it in every language), and each table row is one public symbol, named the way `scripts/inventory.py` reports it:

- Rust: `Type::method`, `Type::Variant`, `Type.field`, or a bare item name
- Go, TypeScript, Python: `Type.member`, or a bare name for top-level items

A cell holds one of these values:

| Cell | Meaning |
| --- | --- |
| `` `name` `` | Implemented under that name. Several names mean that language needs more than one symbol for the concept. |
| `—` | Deliberately absent. The Notes say why, usually because the language covers it another way. |
| `native: …` | The language or its standard library already does it, so the utils don't wrap it. |
| *(empty)* | Not ported yet. The inventory lists it under "To port". |
| `wip` | The reference is unfinished, so the port waits. The inventory lists it under "Waiting" without counting it as drift. Clear the marker once the reference is finished. |

The Notes column records behaviour that deliberately differs between languages, so that a later sync doesn't "fix" it back.

## `constants`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| `VOWELS` | `Vowels` | `VOWELS` | `VOWELS` | Rust slice, Go slice, TS readonly array, Python tuple |

## `math`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| `gcd` | `GCD` | `gcd` | `gcd` | Always non-negative. Python re-exports `math.gcd` (C, arbitrary precision). Go is generic over `types.Number`. TS is exact up to 2^53. |
| `lcm` | `LCM` | `lcm` | `lcm` | 0 when either input is 0. Divides before multiplying, so it only overflows when the result does. Python re-exports `math.lcm`. |

## `point`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| `Point2D` | `Point2D` | `Point2D` | `Point2D` | Value type. Rust derives `Eq`/`Hash`, Go structs are comparable, Python is a frozen dataclass, and TS has `equals`/`key` |
| `Point2D.x` | `Point2D.X` | `Point2D.x` | `Point2D.x` | Integers: Rust and Go `i64`/`int64` |
| `Point2D.y` | `Point2D.Y` | `Point2D.y` | `Point2D.y` | |
| — | — | `Point2D.constructor` | — | Rust and Go use struct literals. The dataclass generates Python's `__init__` |
| `Point2D::straight_distance` | `Point2D.StraightDistance` | `Point2D.straightDistance` | `Point2D.straight_distance` | Euclidean. Sums the squares as integers, then takes one square root, so every language gets the same float |
| `Point2D::gridlike_corner_rect_area` | `Point2D.GridlikeCornerRectArea` | `Point2D.gridlikeCornerRectArea` | `Point2D.gridlike_corner_rect_area` | `(\|dx\| + 1) * (\|dy\| + 1)`: cells in the rectangle with the two points as opposite corners, in any orientation |
| native: `==`, `Hash` | native: `==`, map keys | `Point2D.equals`, `Point2D.key` | native: `==`, `hash` | TS objects compare by identity, so `key()` is the string to put in a `Set` or `Map` |
| `Point3D` | `Point3D` | `Point3D` | `Point3D` | Same value semantics as `Point2D` |
| `Point3D.x` | `Point3D.X` | `Point3D.x` | `Point3D.x` | |
| `Point3D.y` | `Point3D.Y` | `Point3D.y` | `Point3D.y` | |
| `Point3D.z` | `Point3D.Z` | `Point3D.z` | `Point3D.z` | |
| — | — | `Point3D.constructor` | — | As for `Point2D` |
| `Point3D::straight_distance` | `Point3D.StraightDistance` | `Point3D.straightDistance` | `Point3D.straight_distance` | |
| native: `==`, `Hash` | native: `==`, map keys | `Point3D.equals`, `Point3D.key` | native: `==`, `hash` | |

## `strings`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| `StringChunksIter`, `StringChunksIter::new` | `StringChunks` | `stringChunks` | `string_chunks` | Pieces of n characters, the last one shorter. A character is a Rust `char`, a Go rune, a TS code point and a Python str item. The ports reject n < 1, while Rust yields nothing for 0 |

## `tree`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| `Tree` | `Tree` | `Tree` | `Tree` | |
| `Tree.root` | `Tree.Root` | `Tree.root` | `Tree.root` | |
| `Tree::new_from_root`, `Tree::new_from_ref` | `NewTree` | `Tree.constructor` | `Tree.__init__` | Rust needs one for an owned node and one for a `NodeRef`. With shared references, one constructor covers both. All of them clear the root's parent |
| `Tree::dfs_iter` | `Tree.DFSIter` | `Tree.dfsIter` | `Tree.dfs_iter` | Pre-order from a stack, so siblings come out last to first |
| `Tree::bfs_iter` | `Tree.BFSIter` | `Tree.bfsIter` | `Tree.bfs_iter` | Level by level, siblings first to last |
| `TreeDFSIter`, `TreeBFSIter` | — | — | — | Rust iterator types. The others return a generator or `iter.Seq` |
| `TreeNode` | `TreeNode` | `TreeNode` | `TreeNode` | |
| `TreeNode.value` | `TreeNode.Value` | `TreeNode.value` | `TreeNode.value` | |
| `TreeNode.children` | `TreeNode.Children` | `TreeNode.children` | `TreeNode.children` | |
| `TreeNode.parent` | `TreeNode.Parent` | `TreeNode.parent` | `TreeNode.parent` | `None`, `nil` or `undefined` at the root |
| `TreeNode::new` | `NewTreeNode` | `TreeNode.constructor` | `TreeNode.__init__` | |
| `NodeRef` | — | — | — | `Rc<RefCell<TreeNode>>`. The other languages share plain references |
| `NodeRefable` | — | — | — | Rust trait that puts `insert_child` on `NodeRef` |
| `NodeRefable::insert_child` | `TreeNode.InsertChild` | `TreeNode.insertChild` | `TreeNode.insert_child` | Appends a child, sets its parent and returns it |

## `grid`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| `GridDirection` | `GridDirection` | `GridDirection` | `GridDirection` | y grows southwards. Go uses `int` constants with lookup tables. TS uses a class of shared instances, compared with `===`, because TS enums can't have methods. Python uses an `Enum` whose values are the offsets |
| `GridDirection::North` | `North` | `GridDirection.North` | `GridDirection.NORTH` | |
| `GridDirection::South` | `South` | `GridDirection.South` | `GridDirection.SOUTH` | |
| `GridDirection::East` | `East` | `GridDirection.East` | `GridDirection.EAST` | |
| `GridDirection::West` | `West` | `GridDirection.West` | `GridDirection.WEST` | |
| `GridDirection::NorthEast` | `NorthEast` | `GridDirection.NorthEast` | `GridDirection.NORTH_EAST` | |
| `GridDirection::NorthWest` | `NorthWest` | `GridDirection.NorthWest` | `GridDirection.NORTH_WEST` | |
| `GridDirection::SouthEast` | `SouthEast` | `GridDirection.SouthEast` | `GridDirection.SOUTH_EAST` | |
| `GridDirection::SouthWest` | `SouthWest` | `GridDirection.SouthWest` | `GridDirection.SOUTH_WEST` | |
| — | — | `GridDirection.all` | native: `iter(GridDirection)` | TS can't iterate a class's statics |
| — | — | `GridDirection.dx`, `GridDirection.dy` | — | TS keeps the offset on each instance |
| native: `Debug` | `GridDirection.String` | `GridDirection.name`, `GridDirection.toString` | native: `.name` | |
| `GridDirection::get_offset` | `GridDirection.Offset` | `GridDirection.offset` | `GridDirection.offset` | `(dx, dy)`. A Go method returning two values, a property in TS and Python |
| `GridDirection::from_offset` | `GridDirectionFromOffset` | `GridDirection.fromOffset` | `GridDirection.from_offset` | No direction for `(0, 0)` or anything further than one step. Go returns `(direction, ok)`. TS takes `(dx, dy)` and Python a tuple |
| `GridDirection::opposite` | `GridDirection.Opposite` | `GridDirection.opposite` | `GridDirection.opposite` | |
| `GridDirection::rotate_right` | `GridDirection.RotateRight` | `GridDirection.rotateRight` | `GridDirection.rotate_right` | 90° clockwise, diagonals included |
| `GridDirection::rotate_left` | `GridDirection.RotateLeft` | `GridDirection.rotateLeft` | `GridDirection.rotate_left` | |
| `GridCreationItem` | `GridCreationItem` | `GridCreationItem` | — | TS: `T \| typeof GRID_BREAK`. Python passes plain values, with `GRID_BREAK` between rows |
| `GridCreationItem::Item` | `GridItem`, `GridCreationItem.Value` | — | — | TS and Python pass the value itself |
| `GridCreationItem::Break` | `GridBreak`, `GridCreationItem.Break` | `GRID_BREAK` | `GRID_BREAK` | Ends the current row |
| `Grid` | `Grid` | `Grid` | `Grid` | |
| `Grid.inner_data` | `Grid.InnerData` | `Grid.innerData` | `Grid.inner_data` | Rows of nodes, indexed `[y][x]` |
| `Grid::new` | `NewGrid` | `Grid.constructor` | `Grid.__init__` | |
| `Grid::from_iter` | `GridFromIter` | `Grid.fromIter` | `Grid.from_iter` | Go takes an `iter.Seq[GridCreationItem[T]]` |
| `Grid::get_node` | `Grid.GetNode` | `Grid.getNode` | `Grid.get_node` | Missing outside the grid, including at negative coordinates (Python must not wrap around) |
| `Grid::scan` | `Grid.Scan` | `Grid.scan` | `Grid.scan` | Row by row |
| `Grid::iter_rows` | `Grid.IterRows` | `Grid.iterRows` | `Grid.iter_rows` | Same as `scan` |
| native: `Grid::iter_rows().rev()` | `Grid.IterRowsRev` | `Grid.iterRowsRev` | `Grid.iter_rows_rev` | Rust's iterators are double-ended. The others get a method that starts from the bottom right |
| `Grid::iter_columns` | `Grid.IterColumns` | `Grid.iterColumns` | `Grid.iter_columns` | Column by column |
| native: `Grid::iter_columns().rev()` | `Grid.IterColumnsRev` | `Grid.iterColumnsRev` | `Grid.iter_columns_rev` | |
| `Grid::traverse_row` | `Grid.TraverseRow` | `Grid.traverseRow` | `Grid.traverse_row` | The whole row, including x = 0. Panics or throws if the row doesn't exist |
| `Grid::traverse_column` | `Grid.TraverseColumn` | `Grid.traverseColumn` | `Grid.traverse_column` | The whole column, including y = 0 |
| `Grid::iter_by_slope` | `Grid.IterBySlope` | `Grid.iterBySlope` | `Grid.iter_by_slope` | Starts with the start node. Slope is `(rise, run)`. Go takes `(startX, startY, rise, run)` |
| `Grid::num_rows` | `Grid.NumRows` | `Grid.numRows` | `Grid.num_rows` | |
| `Grid::num_columns` | `Grid.NumColumns` | `Grid.numColumns` | `Grid.num_columns` | Length of the first row |
| `GridRowIterator`, `GridColumnIterator`, `GridNodeNeighborsIterator`, `GridPlusIterator`, `GridNodeDirectionIterator`, `GridSlopeIterator` | — | — | — | Rust iterator types. The others return generators or `iter.Seq`/`iter.Seq2` |
| `GridNodeNeighborsIterator::direction_from_idx`, `GridPlusIterator::direction_from_idx` | — | — | — | Rust iterator internals |
| `GridNode` | `GridNode` | `GridNode` | `GridNode` | Equal when the positions match, whatever the values. Hashes by position too |
| `GridNode.value` | `GridNode.Value` | `GridNode.value` | `GridNode.value` | |
| `GridNode.x` | `GridNode.X` | `GridNode.x` | `GridNode.x` | |
| `GridNode.y` | `GridNode.Y` | `GridNode.y` | `GridNode.y` | |
| — | — | `GridNode.constructor` | `GridNode.__init__` | Rust and Go use struct literals |
| native: `==` | `GridNode.Equals` | `GridNode.equals` | native: `==` | By position. A grid shares its node instances, so identity works too |
| `GridNode::add_direction` | `GridNode.AddDirection` | `GridNode.addDirection` | `GridNode.add_direction` | Coordinates one step away, whether or not they're in the grid |
| `GridNode::get_node_in_direction` | `GridNode.GetNodeInDirection` | `GridNode.getNodeInDirection` | `GridNode.get_node_in_direction` | |
| `GridNode::neighbors` | `GridNode.Neighbors` | `GridNode.neighbors` | `GridNode.neighbors` | `(direction, node)` for each of the 8 neighbours that exists, clockwise from north |
| `GridNode::plus_walk` | `GridNode.PlusWalk` | `GridNode.plusWalk` | `GridNode.plus_walk` | N, E, S, W, the ones that exist |
| `GridNode::diagonal_walk` | `GridNode.DiagonalWalk` | `GridNode.diagonalWalk` | `GridNode.diagonal_walk` | NE, SE, SW, NW, the ones that exist |
| `GridNode::direction_iter` | `GridNode.DirectionIter` | `GridNode.directionIter` | `GridNode.direction_iter` | Excludes the node itself and runs to the edge |
| `GridNode::try_get_direction` | `GridNode.TryGetDirection` | `GridNode.tryGetDirection` | `GridNode.try_get_direction` | Only for an adjacent node. Go returns `(direction, ok)` |
| `GridNode::distance` | `GridNode.Distance` | `GridNode.distance` | `GridNode.distance` | Manhattan |
| `GridNode::slope` | `GridNode.Slope` | `GridNode.slope` | `GridNode.slope` | `(rise, run)` = `(dy, dx)` |
| — | — | `Slope` | — | TS tuple type `[rise, run]` |
| `GridNode::iter_node_by_slope` | `GridNode.IterNodeBySlope` | `GridNode.iterNodeBySlope` | `GridNode.iter_node_by_slope` | Includes the node itself |

## `arrays`

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| native: `<[T]>::chunks` | native: `slices.Chunk` | `chunks` | `chunks` | Pieces of n items, the last one shorter. Every language rejects n < 1 when called: Rust and Go panic, TS throws a `RangeError` (for fractions too) and Python raises `ValueError`. Rust and Go yield subslices that share the input's memory, while TS yields copied arrays. Python re-exports `itertools.batched` as `chunks`, which takes any iterable and yields tuples |

## `graph`

Work in progress in TypeScript: `connected` has no body yet. Port the graph once it's finished.

| Rust | Go | TypeScript | Python | Notes |
| --- | --- | --- | --- | --- |
| wip | wip | `UndirectedGraph` | wip | |
| wip | wip | `UndirectedGraph.nodes` | wip | |
| wip | wip | `UndirectedGraph.edges` | wip | |
| wip | wip | `UndirectedGraph.addNode` | wip | Returns the new node's index |
| wip | wip | `UndirectedGraph.getNode` | wip | |
| wip | wip | `UndirectedGraph.getEdgeKey` | wip | Sorts the two indices as strings, so `(9, 10)` gives `"10-9"`. Keys still match in either order |
| wip | wip | `UndirectedGraph.indiciesFromKey` | wip | |
| wip | wip | `UndirectedGraph.insertEdge` | wip | False if the edge already existed |
| wip | wip | `UndirectedGraph.hasEdge` | wip | |
| wip | wip | `UndirectedGraph.connected` | wip | No body yet |
