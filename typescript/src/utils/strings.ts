/**
 * Splits `s` into pieces of `chunkSize` characters. The last piece is shorter
 * when `chunkSize` doesn't divide `s` evenly. Characters are code points, so a
 * surrogate pair is never split.
 */
export function* stringChunks(s: string, chunkSize: number): Generator<string> {
    if (!(chunkSize >= 1)) {
        throw new RangeError(`chunkSize must be positive, got ${chunkSize}`);
    }

    let start = 0;
    let end = 0;
    let count = 0;
    for (const char of s) {
        end += char.length;
        if (++count === chunkSize) {
            yield s.slice(start, end);
            start = end;
            count = 0;
        }
    }

    if (start < s.length) {
        yield s.slice(start);
    }
}
