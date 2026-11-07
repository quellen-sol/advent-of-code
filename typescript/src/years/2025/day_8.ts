import { Solution } from "../../solution";
import { UndirectedGraph } from "../../utils/graph";

export class Day8Solution2025 extends Solution<number, number> {
    graph = new UndirectedGraph<Point>();

    constructor(inputPath: string) {
        super(inputPath);
        this.parseInput();
    }

    parseInput() {
        for (const line of this.lines()) {
            const point = new Point(line);
            this.graph.addNode(point);
        }
    }

    part1(): number {
        while (true) {
            const distances: Record<string, number> = {};
            for (let i = 0; i < this.graph.nodes.length; i++) {
                for (let j = 0; j < this.graph.nodes.length; j++) {
                    if (i == j) {
                        continue;
                    }

                    const key = this.graph.getEdgeKey(i, j);
                    if (distances[key] !== undefined || this.graph.hasEdge(i, j)) {
                        continue;
                    }

                    const pointA = this.graph.getNode(i)!.value;
                    const pointB = this.graph.getNode(j)!.value;
                    const distance = pointA.distance(pointB);
                    distances[key] = distance;
                }
            }

            const ents = Object.entries(distances);
            if (ents.length === 0) {
                break;
            }

            let smallestDist = Number.POSITIVE_INFINITY;
            let closestPair: [number, number] | null = null;
            for (const [k, v] of Object.entries(distances)) {
                if (v < smallestDist) {
                    smallestDist = v;
                    closestPair = this.graph.indiciesFromKey(k) as [number, number];
                }
            }
            const [a, b] = closestPair!;
            this.graph.insertEdge(a, b);
        }

        // Find largest 3 circuits

        throw new Error("Not implemented");
    }

    part2(): number {
        throw new Error("Not implemented");
    }
}

class Point {
    str: string;
    x: number;
    y: number;
    z: number;

    constructor(pointStr: string) {
        const [x, y, z] = pointStr.split(",") as [string, string, string];
        this.x = +x;
        this.y = +y;
        this.z = +z;
        this.str = pointStr;
    }

    distance(other: Point) {
        return Math.sqrt((other.x - this.x) ** 2 + (other.y - this.y) ** 2 + (other.z - this.z) ** 2);
    }
}
