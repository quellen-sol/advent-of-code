interface Graph<T> {

}

class Node<T> {
    constructor(public value: T) {

    }
}

class UndirectedEdge {
    constructor(public idxA: number, public idxB: number) {

    }
}

export class UndirectedGraph<T> implements Graph<T> {
    nodes: Node<T>[] = [];
    edges = new Map();

    /**
     * 
     * @param value 
     * @returns The index of the inserted node
     */
    addNode(value: T) {
        return this.nodes.push(new Node(value)) - 1;
    }

    getNode(index: number) {
        return this.nodes[index];
    }

    getEdgeKey(a: number, b: number) {
        const [keyBaseA, keyBaseB] = [a, b].sort();
        return `${keyBaseA}-${keyBaseB}`;
    }

    indiciesFromKey(key: string) {
        return key.split("-").map((v) => +v);
    }

    /**
     * 
     * @param value 
     * @returns If a new edge was inserted (false if one already exists)
     */
    insertEdge(idxA: number, idxB: number): boolean {
        const key = this.getEdgeKey(idxA, idxB);
        if (this.edges.has(key)) {
            return false;
        }
        const edge = new UndirectedEdge(idxA, idxB);
        this.edges.set(key, edge);
        return true;
    }

    hasEdge(a: number, b: number) {
        const key = this.getEdgeKey(a, b);
        return this.edges.has(key);
    }

    connected(a: number, b: number) {
        
    }
}