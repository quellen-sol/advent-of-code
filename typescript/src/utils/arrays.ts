/**
 * Splits `arr` into pieces of `chunkSize` items. The last piece is shorter when
 * `chunkSize` doesn't divide the length evenly. Throws straight away, rather
 * than on the first `next()`, when `chunkSize` isn't a positive integer.
 */
export function chunks<T>(arr: T[], chunkSize: number): Generator<T[]> {
    if (!Number.isInteger(chunkSize) || chunkSize < 1) {
        throw new RangeError(
            `chunkSize must be a positive integer, got ${chunkSize}`,
        );
    }

    return chunksOf(arr, chunkSize);
}

function* chunksOf<T>(arr: T[], chunkSize: number): Generator<T[]> {
    for (let i = 0; i < arr.length; i += chunkSize) {
        yield arr.slice(i, i + chunkSize);
    }
}

type WindowState<T> = {
    startIdx: number;
    endIdx: number;
    window: T[]
}

export function* windows<T>(arr: T[], windowSize: number): Generator<WindowState<T>> {
    if (windowSize <= 0 || windowSize > arr.length) {
        throw new Error(`bad window size! got ${windowSize} (should be >0 and <=arr.len)`);
    }
    let start = 0;
    let end = start + windowSize;
    while (end < arr.length) {
        yield {
            startIdx: start,
            endIdx: end,
            window: arr.slice(start, end),
        }

        start++;
        end++;
    }
}
