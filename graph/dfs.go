package main

import "fmt"

var WHITE = 0
var GRAY = 1
var BLACK = 2

type node struct {
	value         string
	color         int
	discoveryTime int
	finishingTime int
	parent        *node
	neighbours    []*node
}

func (u *node) addEdges(nodes []*node) {
	for _, v := range nodes {
		u.neighbours = append(u.neighbours, v)
	}
}

func dfs(graph []*node) {
	for _, node := range graph {
		node.color = WHITE
		node.parent = nil
	}

	time := 1
	for _, node := range graph {
		if node.color == WHITE {
			time = dfsVisit(graph, node, time) + 1
		}
	}
}

func dfsVisit(graph []*node, u *node, time int) int {
	u.discoveryTime = time
	u.color = GRAY
	for _, v := range u.neighbours {
		if v.color == WHITE {
			v.parent = u
			time = dfsVisit(graph, v, time+1)
		}
	}
	u.color = BLACK
	time += 1
	u.finishingTime = time

	return time
}

func main() {
	// Create nodes
	u := &node{value: "u"}
	v := &node{value: "v"}
	w := &node{value: "w"}
	x := &node{value: "x"}
	y := &node{value: "y"}
	z := &node{value: "z"}

	// Create edges
	u.addEdges([]*node{v, x})
	v.addEdges([]*node{y})
	w.addEdges([]*node{y, z})
	x.addEdges([]*node{v})
	y.addEdges([]*node{x})
	z.addEdges([]*node{z})

	// Create graph (list of nodes)
	graph := []*node{u, v, w, x, y, z}

	/*
		We have the following graph:

		   u 🡪 v   w
		   🡫 🡭 🡫 🡯 🡫
		   x 🡨 y   z ⮌

		Given 'u' as the origin, we expect the discovery times to be:

		   1 🡪 2   9
		   🡫 🡭 🡫 🡯 🡫
		   4 🡨 3   10 ⮌

		and the finish times to be:

		   8 🡪 7   12
		   🡫 🡭 🡫 🡯 🡫
		   5 🡨 6   11 ⮌
	*/

	dfs(graph)
	for _, node := range graph {
		fmt.Println("Node", node.value,
			"was visited at", node.discoveryTime,
			"and finished at ", node.finishingTime)
	}
}
