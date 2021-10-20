package main

type Node struct {
	value  string
	rank   int
	parent *Node
}

type Edge struct {
	u      *Node
	v      *Node
	weight int
}

type Graph struct {
	nodes []*Node
	edges []*Edge
}

func (u *Node) MakeSet() {
	u.parent = u
	u.rank = 0
}

func Union(u *Node, v *Node) {
	link(u.FindSet(), v.FindSet())
}

func link(u *Node, v *Node) {
	if u.rank > v.rank {
		v.parent = u
		return
	}

	u.parent = v
	if u.rank == v.rank {
		v.rank += 1
	}
}

func (u *Node) FindSet() *Node {
	if u != u.parent {
		u.parent = u.parent.FindSet()
	}

	return u.parent
}
