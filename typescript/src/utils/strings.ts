/**
 * Splits `s` into pieces of `chunkSize` characters. The last piece is shorter
 * when `chunkSize` doesn't divide `s` evenly. Characters are code points, so a
 * surrogate pair is never split. Throws straight away, rather than on the
 * first `next()`, when `chunkSize` isn't a positive integer.
 */
export function stringChunks(s: string, chunkSize: number): Generator<string> {
    if (!Number.isInteger(chunkSize) || chunkSize < 1) {
        throw new RangeError(
            `chunkSize must be a positive integer, got ${chunkSize}`,
        );
    }

    return codePointChunks(s, chunkSize);
}

function* codePointChunks(s: string, chunkSize: number): Generator<string> {
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
