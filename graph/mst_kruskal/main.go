package main

import (
	"fmt"
	"sort"
)

func mstKruskal(graph *Graph) []*Edge {
	for _, node := range graph.nodes {
		node.MakeSet()
	}
	sort.Slice(graph.edges[:], func(i int, j int) bool {
		return graph.edges[i].weight <= graph.edges[j].weight
	})

	edges := []*Edge{}
	for _, edge := range graph.edges {
		if edge.u.FindSet() != edge.v.FindSet() {
			edges = append(edges, edge)
			Union(edge.u, edge.v)
		}
	}

	return edges
}

func main() {
	// Create nodes
	a := &Node{value: "a"}
	b := &Node{value: "b"}
	c := &Node{value: "c"}
	d := &Node{value: "d"}
	e := &Node{value: "e"}
	f := &Node{value: "f"}
	g := &Node{value: "g"}
	h := &Node{value: "h"}
	i := &Node{value: "i"}

	// Create edges
	ab := &Edge{u: a, v: b, weight: 4}
	ah := &Edge{u: a, v: h, weight: 8}
	bc := &Edge{u: b, v: c, weight: 8}
	bh := &Edge{u: b, v: h, weight: 11}
	cd := &Edge{u: c, v: d, weight: 7}
	cf := &Edge{u: c, v: f, weight: 4}
	ci := &Edge{u: c, v: i, weight: 2}
	de := &Edge{u: d, v: e, weight: 9}
	df := &Edge{u: d, v: f, weight: 14}
	ef := &Edge{u: e, v: f, weight: 10}
	fg := &Edge{u: f, v: g, weight: 2}
	gi := &Edge{u: g, v: i, weight: 6}
	gh := &Edge{u: g, v: h, weight: 1}
	hi := &Edge{u: h, v: i, weight: 7}

	// Create graph
	graph := &Graph{nodes: []*Node{a, b, c, d, e, f, g, h, i},
		edges: []*Edge{ab, ah, bc, bh, cd, cf, ci, de, df, ef, fg, gi, gh, hi}}

	fmt.Println("The Minimum Spanning Tree is made of the following edges:")
	edges := mstKruskal(graph)
	for _, edge := range edges {
		fmt.Print("  - (", edge.u.value, ",", edge.v.value, ")\n")
	}
}
