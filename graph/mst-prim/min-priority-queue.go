package main

import "container/heap"

type Item struct {
	data interface{} 
	priority int
    index int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int {
    return len(pq)
}

func (pq PriorityQueue) Less(i int, j int) bool {
   return pq[i].priority <= pq[j].priority
}

func (pq PriorityQueue) Swap(i int, j int) {
    pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*Item)
	item.index = len(*pq)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	minIndex := len(*pq) - 1
	item := (*pq)[minIndex]
	item.index = -1

	(*pq)[minIndex] = nil
	*pq = (*pq)[0 : minIndex]

	return item
}

func (pq *PriorityQueue) Update(item *Item) {
	heap.Remove(pq, item.index)
	heap.Push(pq, item)
}