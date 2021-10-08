package main

import "fmt"

type disjointSet struct {
	head        *node
	tail        *node
	numElements int
}

type graph struct {
	nodes []*node
	edges []*edge
}

type node struct {
	value string
	next  *node
	set   *disjointSet
}

type edge struct {
	u *node
	v *node
}

func connectedComponents(graph *graph) {
	for _, node := range graph.nodes {
		node.makeSet()
	}
	for _, edge := range graph.edges {
		if edge.u.findSet() != edge.v.findSet() {
			union(edge.u, edge.v)
		}
	}
}

func union(u *node, v *node) {
	biggerSet := u.set
	smallerSet := v.set
	if u.set.numElements < v.set.numElements {
		biggerSet = v.set
		smallerSet = u.set
	}

	biggerSet.tail.next = smallerSet.head
	biggerSet.tail = smallerSet.tail
	biggerSet.numElements += smallerSet.numElements

	currentNode := smallerSet.head
	for currentNode != nil {
		currentNode.set = biggerSet
		currentNode = currentNode.next
	}
}

func (vertex *node) makeSet() *disjointSet {
	ds := &disjointSet{head: vertex, tail: vertex, numElements: 1}
	vertex.set = ds
	return ds
}

func (vertex *node) findSet() string {
	return vertex.set.head.value
}

func main() {
	// Create nodes
	a := &node{value: "a"}
	b := &node{value: "b"}
	c := &node{value: "c"}
	d := &node{value: "d"}
	e := &node{value: "e"}
	f := &node{value: "f"}
	g := &node{value: "g"}
	h := &node{value: "h"}
	i := &node{value: "i"}
	j := &node{value: "j"}

	// Create edges
	ab := &edge{u: a, v: b}
	ac := &edge{u: a, v: c}
	bc := &edge{u: b, v: c}
	bd := &edge{u: b, v: d}
	ef := &edge{u: e, v: f}
	eg := &edge{u: e, v: g}
	hi := &edge{u: h, v: i}

	// Create graph
	graph := &graph{nodes: []*node{a, b, c, d, e, f, g, h, i, j},
		edges: []*edge{ab, ac, bc, bd, ef, eg, hi}}

	/*
		Given the following graph:

				  a---b   e---f   h   j
				  | / |   |       |
				  c   d   g       i


		We expect each node to return the following set representative:

				  a---a   e---e   h   j
				  | / |   |       |
				  a   a   e       h

	*/

	connectedComponents(graph)
	for _, node := range graph.nodes {
		fmt.Println("Node", node.value, "is in set", node.set.head.value)
	}
}
