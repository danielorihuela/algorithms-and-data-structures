package main

import (
	"fmt"
	"math"
)

type node struct {
	value    string
	estimate int
	parent   *node
}

type edge struct {
	u      *node
	v      *node
	weight int
}

type graph struct {
	nodes []*node
	edges []*edge
}

func initializeSingleSource(graph *graph, origin *node) {
	for _, node := range graph.nodes {
		node.estimate = math.MaxInt
	}
	origin.estimate = 0
}

func relax(u *node, v *node, w int) {
	if v.estimate > u.estimate+w {
		v.estimate = u.estimate + w
		v.parent = u
	}
}

func bellmanFord(graph *graph, origin *node) bool {
	initializeSingleSource(graph, origin)
	for i := 0; i < len(graph.nodes); i++ {
		if graph.nodes[i].value == origin.value {
			continue
		}

		for _, edge := range graph.edges {
			relax(edge.u, edge.v, edge.weight)
		}
	}

	for _, edge := range graph.edges {
		if edge.v.estimate > edge.u.estimate+edge.weight {
			return false
		}
	}

	return true
}

func printPath(node *node) {
	if node.parent == nil {
		fmt.Print(node.value)
		return
	}

	printPath(node.parent)
	fmt.Print(" -> ", node.value)
}

func printPathEstimates(node *node) {
	if node.parent == nil {
		fmt.Print(node.estimate)
		return
	}

	printPathEstimates(node.parent)
	fmt.Print(" -> ", node.estimate)
}

func main() {
	s := &node{value: "s"}
	t := &node{value: "t"}
	x := &node{value: "x"}
	y := &node{value: "y"}
	z := &node{value: "z"}

	st := &edge{u: s, v: t, weight: 6}
	sy := &edge{u: s, v: y, weight: 7}
	tx := &edge{u: t, v: x, weight: 5}
	ty := &edge{u: t, v: y, weight: 8}
	tz := &edge{u: t, v: z, weight: -4}
	xt := &edge{u: x, v: t, weight: -2}
	yx := &edge{u: y, v: x, weight: -3}
	yz := &edge{u: y, v: z, weight: 9}
	zs := &edge{u: z, v: s, weight: 2}
	zx := &edge{u: z, v: x, weight: 7}

	graph := &graph{nodes: []*node{s, t, x, y, z},
		edges: []*edge{st, sy, tx, ty, tz, xt, yx, yz, zs, zx}}

	origin := s
	solutionExists := bellmanFord(graph, origin)

	fmt.Println("A solution exists =", solutionExists)
	printPath(z)
	fmt.Println()
	printPathEstimates(z)
}
