import unittest

from py.utils.constants import VOWELS
from py.utils.math import gcd, lcm
from py.utils.point import Point2D, Point3D
from py.utils.strings import string_chunks
from py.utils.tree import Tree, TreeNode


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


class TreeTests(unittest.TestCase):
    def test_dfs_and_bfs_iter(self) -> None:
        root = TreeNode("/")
        a = root.insert_child("a")
        root.insert_child("b")
        c = a.insert_child("c")
        tree = Tree(root)

        self.assertEqual([n.value for n in tree.dfs_iter()], ["/", "b", "a", "c"])
        self.assertEqual([n.value for n in tree.bfs_iter()], ["/", "a", "b", "c"])
        self.assertIs(c.parent, a)
        self.assertIs(a.parent, root)

    def test_tree_root_has_no_parent(self) -> None:
        a = TreeNode("/").insert_child("a")
        self.assertIsNone(Tree(a).root.parent)


if __name__ == "__main__":
    unittest.main()
