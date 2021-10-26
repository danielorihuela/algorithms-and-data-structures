package main

import (
"container/heap"
)

func InitMinPriorityQueue(nodes []*Node, priorities []int) *PriorityQueue {
    pq := make(PriorityQueue, len(nodes))
	for i, node := range nodes {
		pq[i] = &Item {
			  data: node,
			  priority: priorities[i],
		}
	}
   heap.Init(&pq)

   return &pq
}

func (pq *PriorityQueue) UpdatePriority(node *Node, priority int, index int) {
	 item := &Item {
	 	  data: node,
		  priority: priority,
		  index: index,
	 }
	 pq.Update(item)
}

func (pq *PriorityQueue) ExtractMin() *Node {
	 return heap.Pop(pq).(*Item).data.(*Node)
}

func (pq *PriorityQueue) Contains(node *Node) (bool, int, int) {
	 for i, n := range *pq {
	 	 if n.data.(*Node).value == node.value {
		 	return true, n.priority, i
		 }
	 }

	 return false, -1, -1
}