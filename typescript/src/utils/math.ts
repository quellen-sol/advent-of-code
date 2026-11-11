// Exact while every value involved stays within Number.MAX_SAFE_INTEGER.

export function gcd(a: number, b: number): number {
    a = Math.abs(a);
    b = Math.abs(b);
    while (b !== 0) {
        const remainder = a % b;
        a = b;
        b = remainder;
    }

    return a;
}

export function lcm(a: number, b: number): number {
    if (a === 0 || b === 0) {
        return 0;
    }

    // Dividing first keeps the intermediate value as small as the result.
    return Math.abs((a / gcd(a, b)) * b);
}
