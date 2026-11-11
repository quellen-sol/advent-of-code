"""Python's own itertools.batched already splits an iterable into pieces of n
items, so it's re-exported under the name the other languages use rather than
rewritten. It yields tuples and raises ValueError when n < 1."""

from itertools import batched as chunks

__all__ = ["chunks"]
