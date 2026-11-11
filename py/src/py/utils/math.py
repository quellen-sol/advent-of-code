"""Python's own gcd and lcm are written in C and take any number of
arbitrarily large integers, so they're re-exported rather than rewritten."""

from math import gcd, lcm

__all__ = ["gcd", "lcm"]
