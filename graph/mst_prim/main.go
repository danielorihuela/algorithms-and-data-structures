package main

import (
	"fmt"
	"math"
)

type Node struct {
	value  string
	parent *Node

    neighbours []*Node
    weights []int
}

type Graph struct {
	nodes []*Node
}

func (u *Node) addEdges(nodes []*Node, weights []int) {
	u.neighbours = append(u.neighbours, nodes...)
	u.weights = append(u.weights, weights...)
	for i, node := range nodes {
		node.neighbours = append(node.neighbours, u)
		node.weights = append(node.weights, weights[i])
	}
}

func mstPrim(graph *Graph, origin *Node) {
	initialPriorities := make([]int, len(graph.nodes))
    for i, node := range(graph.nodes) {
		node.parent = nil
		if node.value == origin.value {
		   initialPriorities[i] = 0
		} else {
		  initialPriorities[i] = math.MaxInt
		}
    }

	fmt.Println("The Minimum Spanning Tree edges are added in the following order:")
	pq := InitMinPriorityQueue(graph.nodes, initialPriorities)
	for pq.Len() != 0 {
		node := pq.ExtractMin()
		for i, neighbour := range node.neighbours {
			nodeInPQ, priority, position := pq.Contains(neighbour)
			if nodeInPQ && node.weights[i] < priority {
			   neighbour.parent = node
			   pq.UpdatePriority(neighbour, node.weights[i], position)
			}
		}
		if node.parent == nil {
		   fmt.Println("Root =", node.value)
		} else {
		   fmt.Print("- (", node.parent.value, ",", node.value, ")\n")
		}
	}
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
	a.addEdges(
	  []*Node{b, h},
	  []int{4, 8})
	b.addEdges(
	  []*Node{c, h},
	  []int{8, 11})
	c.addEdges(
	  []*Node{d, f, i},
	  []int{7, 4, 2})
	d.addEdges(
	  []*Node{e, f},
	  []int{9, 14})
	e.addEdges(
	  []*Node{f},
	  []int{10})
	f.addEdges(
	  []*Node{g},
	  []int{2})
	g.addEdges(
	  []*Node{h, i},
	  []int{1, 6})
	h.addEdges(
	  []*Node{i},
	  []int{7})

	// Create graph
	graph := &Graph{nodes: []*Node{a, b, c, d, e, f, g, h, i}}

	root := a
	mstPrim(graph, root)
}
