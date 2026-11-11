import assert from "node:assert/strict";
import { test } from "node:test";

import { VOWELS } from "./constants";
import { gcd, lcm } from "./math";
import { Point2D, Point3D } from "./point";
import { stringChunks } from "./strings";
import { Tree, TreeNode } from "./tree";

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

test("tree dfs and bfs iter", () => {
    const root = new TreeNode("/");
    const a = root.insertChild("a");
    root.insertChild("b");
    const c = a.insertChild("c");
    const tree = new Tree(root);

    assert.deepEqual(
        Array.from(tree.dfsIter(), (n) => n.value),
        ["/", "b", "a", "c"],
    );
    assert.deepEqual(
        Array.from(tree.bfsIter(), (n) => n.value),
        ["/", "a", "b", "c"],
    );
    assert.equal(c.parent, a);
    assert.equal(a.parent, root);
});

test("tree root has no parent", () => {
    const a = new TreeNode("/").insertChild("a");
    assert.equal(new Tree(a).root.parent, undefined);
});
