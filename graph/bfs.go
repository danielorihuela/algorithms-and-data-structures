package main

import (
	"fmt"
	"math"
	"sort"
)

var WHITE = 0
var GRAY = 1
var BLACK = 2

type node struct {
	value      string
	color      int
	distance   int
	parent     *node
	neighbours []*node
}

func (u *node) addEdges(nodes []*node) {
	for _, v := range nodes {
		u.neighbours = append(u.neighbours, v)
	}
}

func bfs(graph []*node, origin *node) {
	for _, node := range graph {
		node.color = WHITE
		node.distance = math.MaxInt
		node.parent = nil
	}
	origin.color = GRAY
	origin.distance = 0

	queue := []*node{}
	queue = append(queue, origin)
	for len(queue) != 0 {
		u := queue[0]
		queue = queue[1:]
		for _, v := range u.neighbours {
			if v.color != WHITE {
				continue
			}

			v.color = GRAY
			v.distance = u.distance + 1
			v.parent = u
			queue = append(queue, v)
		}
		u.color = BLACK
	}
}

func main() {
	// Create nodes
	r := &node{value: "r"}
	s := &node{value: "s"}
	t := &node{value: "t"}
	u := &node{value: "u"}
	v := &node{value: "v"}
	w := &node{value: "w"}
	x := &node{value: "x"}
	y := &node{value: "y"}

	// Create edges
	r.addEdges([]*node{s, v})
	s.addEdges([]*node{r, w})
	t.addEdges([]*node{u, w, x})
	u.addEdges([]*node{t, x, y})
	v.addEdges([]*node{r})
	w.addEdges([]*node{s, t, x})
	x.addEdges([]*node{u, t, w, y})
	y.addEdges([]*node{u, x})

	// Create graph (list of nodes)
	graph := []*node{r, s, t, u, v, w, x, y}

	/*
		We have the following graph:

		   r---s   t---u
		   |   | / | / |
		   v   w---x---y

		Given 's' as the origin, we expect the distances to be:

		   1---0   2---3
		   |   | / | / |
		   2   1---2---3
	*/

	origin := s
	bfs(graph, origin)
	sort.SliceStable(graph, func(i, j int) bool {
		return graph[i].distance < graph[j].distance
	})
	for _, node := range graph {
		fmt.Println("Node", node.value,
			"distance from the origin", origin.value,
			"is", node.distance)
	}
}
