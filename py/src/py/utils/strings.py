from collections.abc import Iterator


def string_chunks(s: str, chunk_size: int) -> Iterator[str]:
    """Splits s into pieces of chunk_size characters. The last piece is shorter
    when chunk_size doesn't divide s evenly. Raises straight away, rather than
    on the first next(), when chunk_size < 1."""
    if chunk_size < 1:
        raise ValueError(f"chunk_size must be positive, got {chunk_size}")

    return (s[start : start + chunk_size] for start in range(0, len(s), chunk_size))
