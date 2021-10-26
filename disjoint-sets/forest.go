package main

import "fmt"

type node struct {
	value  string
	rank   int
	parent *node
}

type edge struct {
	u *node
	v *node
}

type graph struct {
	nodes []*node
	edges []*edge
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

func (u *node) makeSet() {
	u.parent = u
	u.rank = 0
}

func union(u *node, v *node) {
	link(u.findSet(), v.findSet())
}

func link(u *node, v *node) {
	if u.rank > v.rank {
		v.parent = u
		return
	}

	u.parent = v
	if u.rank == v.rank {
		v.rank += 1
	}
}

func (u *node) findSet() *node {
	if u != u.parent {
		u.parent = u.parent.findSet()
	}

	return u.parent
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

	// Create tree edges

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

				  b---b   f---f   i   j
				  | / |   |       |
				  b   b   f       i

		and have the following ranks

				  0---1   0---1   0   0
				  | / |   |       |
				  0   0   0       1

	*/

	connectedComponents(graph)
	for _, node := range graph.nodes {
		fmt.Println("Node", node.value, "is in set", node.findSet().value, "with rank", node.rank)
	}
}
