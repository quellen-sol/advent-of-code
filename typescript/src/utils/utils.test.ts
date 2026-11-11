import assert from "node:assert/strict";
import { test } from "node:test";

import { chunks } from "./arrays";
import { VOWELS } from "./constants";
import { gcd, lcm } from "./math";
import { Point2D, Point3D } from "./point";
import { stringChunks } from "./strings";
import { Tree, TreeNode } from "./tree";

test("array chunks", () => {
    const items = [1, 2, 3, 4, 5, 6, 7];
    assert.deepEqual([...chunks(items, 2)], [[1, 2], [3, 4], [5, 6], [7]]);
    assert.deepEqual([...chunks(items, 7)], [items]);
    assert.deepEqual([...chunks(items, 10)], [items]);
});

test("empty array has no chunks", () => {
    assert.deepEqual([...chunks([], 2)], []);
});

test("array chunks reject a size below 1", () => {
    assert.throws(() => chunks([1, 2, 3], 0), RangeError);
    assert.throws(() => chunks([1, 2, 3], -1), RangeError);
    // number can hold a fraction, which the other languages' sizes can't.
    assert.throws(() => chunks([1, 2, 3], 1.5), RangeError);
});

test("vowels", () => {
    assert.equal(VOWELS.join(""), "aeiou");
});

test("gcd", () => {
    assert.equal(gcd(12, -18), 6);
    assert.equal(gcd(0, 5), 5);
});

test("lcm", () => {
    assert.equal(lcm(4, 6), 12);
    assert.equal(lcm(-4, 6), 12);
    assert.equal(lcm(0, 5), 0);
});

test("straight distance", () => {
    assert.equal(new Point2D(0, 0).straightDistance(new Point2D(3, -4)), 5);
    assert.equal(
        new Point3D(1, 2, 3).straightDistance(new Point3D(3, 5, 9)),
        7,
    );
});

test("gridlike corner rect area", () => {
    assert.equal(
        new Point2D(1, 1).gridlikeCornerRectArea(new Point2D(3, 4)),
        12,
    );
    assert.equal(
        new Point2D(0, 5).gridlikeCornerRectArea(new Point2D(5, 0)),
        36,
    );
    assert.equal(
        new Point2D(5, 0).gridlikeCornerRectArea(new Point2D(0, 5)),
        36,
    );
});

test("points compare by value", () => {
    assert.ok(new Point2D(1, 2).equals(new Point2D(1, 2)));
    assert.ok(!new Point3D(1, 2, 3).equals(new Point3D(1, 2, 4)));
    assert.equal(
        new Set([new Point3D(1, 2, 3).key(), new Point3D(1, 2, 3).key()]).size,
        1,
    );
});

test("chunk 1", () => {
    assert.deepEqual([...stringChunks("ABCDEFG", 2)], ["AB", "CD", "EF", "G"]);
});

test("chunk 2", () => {
    assert.deepEqual([...stringChunks("ABCDEFG", 3)], ["ABC", "DEF", "G"]);
});

test("chunks count characters", () => {
    assert.deepEqual([...stringChunks("α😀γ", 2)], ["α😀", "γ"]);
});

test("empty string has no chunks", () => {
    assert.deepEqual([...stringChunks("", 2)], []);
});

test("string chunks reject a size below 1", () => {
    assert.throws(() => stringChunks("abc", 0), RangeError);
});

/** / has children a and b; a has c, and b has d. */
function makeTree(): Tree<string> {
    const root = new TreeNode("/");
    root.insertChild("a").insertChild("c");
    root.insertChild("b").insertChild("d");
    return new Tree(root);
}

test("tree dfs iter", () => {
    const tree = makeTree();
    assert.deepEqual(
        Array.from(tree.dfsIter(), (n) => n.value),
        ["/", "b", "d", "a", "c"],
    );
    assert.deepEqual(
        Array.from(tree.dfsIter().take(2), (n) => n.value),
        ["/", "b"],
    );
});

test("tree bfs iter", () => {
    const tree = makeTree();
    assert.deepEqual(
        Array.from(tree.bfsIter(), (n) => n.value),
        ["/", "a", "b", "c", "d"],
    );
    assert.deepEqual(
        Array.from(tree.bfsIter().take(2), (n) => n.value),
        ["/", "a"],
    );
});

test("children point at their parents", () => {
    const root = makeTree().root;
    const a = root.children[0]!;
    assert.equal(a.parent, root);
    assert.equal(a.children[0]!.parent, a);
});

test("tree root has no parent", () => {
    const a = new TreeNode("/").insertChild("a");
    assert.equal(new Tree(a).root.parent, undefined);
});
