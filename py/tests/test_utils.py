import unittest
from itertools import islice

from py.utils.arrays import chunks
from py.utils.constants import VOWELS
from py.utils.math import gcd, lcm
from py.utils.point import Point2D, Point3D
from py.utils.strings import string_chunks
from py.utils.tree import Tree, TreeNode


class ArraysTests(unittest.TestCase):
    def test_chunks(self) -> None:
        items = [1, 2, 3, 4, 5, 6, 7]
        self.assertEqual(list(chunks(items, 2)), [(1, 2), (3, 4), (5, 6), (7,)])
        self.assertEqual(list(chunks(items, 7)), [tuple(items)])
        self.assertEqual(list(chunks(items, 10)), [tuple(items)])

    def test_empty_array_has_no_chunks(self) -> None:
        self.assertEqual(list(chunks([], 2)), [])

    def test_chunks_reject_a_size_below_1(self) -> None:
        for size in (0, -1):
            with self.assertRaises(ValueError):
                chunks([1, 2, 3], size)


class ConstantsTests(unittest.TestCase):
    def test_vowels(self) -> None:
        self.assertEqual("".join(VOWELS), "aeiou")


class MathTests(unittest.TestCase):
    def test_gcd(self) -> None:
        self.assertEqual(gcd(12, -18), 6)
        self.assertEqual(gcd(0, 5), 5)

    def test_lcm(self) -> None:
        self.assertEqual(lcm(4, 6), 12)
        self.assertEqual(lcm(-4, 6), 12)
        self.assertEqual(lcm(0, 5), 0)


class PointTests(unittest.TestCase):
    def test_straight_distance(self) -> None:
        self.assertEqual(Point2D(0, 0).straight_distance(Point2D(3, -4)), 5)
        self.assertEqual(Point3D(1, 2, 3).straight_distance(Point3D(3, 5, 9)), 7)

    def test_gridlike_corner_rect_area(self) -> None:
        self.assertEqual(Point2D(1, 1).gridlike_corner_rect_area(Point2D(3, 4)), 12)
        self.assertEqual(Point2D(0, 5).gridlike_corner_rect_area(Point2D(5, 0)), 36)
        self.assertEqual(Point2D(5, 0).gridlike_corner_rect_area(Point2D(0, 5)), 36)

    def test_points_are_values(self) -> None:
        self.assertEqual(Point2D(1, 2), Point2D(1, 2))
        self.assertEqual(len({Point3D(1, 2, 3), Point3D(1, 2, 3)}), 1)


class StringsTests(unittest.TestCase):
    def test_chunk_1(self) -> None:
        self.assertEqual(list(string_chunks("ABCDEFG", 2)), ["AB", "CD", "EF", "G"])

    def test_chunk_2(self) -> None:
        self.assertEqual(list(string_chunks("ABCDEFG", 3)), ["ABC", "DEF", "G"])

    def test_chunks_count_characters(self) -> None:
        self.assertEqual(list(string_chunks("αβγ", 2)), ["αβ", "γ"])

    def test_empty_string_has_no_chunks(self) -> None:
        self.assertEqual(list(string_chunks("", 2)), [])

    def test_string_chunks_reject_a_size_below_1(self) -> None:
        with self.assertRaises(ValueError):
            string_chunks("abc", 0)


def make_tree() -> Tree[str]:
    """/ has children a and b; a has c, and b has d."""
    root = TreeNode("/")
    root.insert_child("a").insert_child("c")
    root.insert_child("b").insert_child("d")
    return Tree(root)


class TreeTests(unittest.TestCase):
    def test_dfs_iter(self) -> None:
        tree = make_tree()
        self.assertEqual([n.value for n in tree.dfs_iter()], ["/", "b", "d", "a", "c"])
        self.assertEqual([n.value for n in islice(tree.dfs_iter(), 2)], ["/", "b"])

    def test_bfs_iter(self) -> None:
        tree = make_tree()
        self.assertEqual([n.value for n in tree.bfs_iter()], ["/", "a", "b", "c", "d"])
        self.assertEqual([n.value for n in islice(tree.bfs_iter(), 2)], ["/", "a"])

    def test_children_point_at_their_parents(self) -> None:
        root = make_tree().root
        a = root.children[0]
        self.assertIs(a.parent, root)
        self.assertIs(a.children[0].parent, a)

    def test_tree_root_has_no_parent(self) -> None:
        a = TreeNode("/").insert_child("a")
        self.assertIsNone(Tree(a).root.parent)


if __name__ == "__main__":
    unittest.main()
