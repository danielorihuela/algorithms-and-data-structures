(ns data-structures.binary-tree
  (:require
   [clojure.string :as string]))

(defrecord BinaryTreeNode [left key right])

(defn insert-node [root, key]
  (cond
    (= root nil) (->BinaryTreeNode nil key nil)
    (< key (:key root)) (assoc root :left (insert-node (:left root) key))
    (> key (:key root)) (assoc root :right (insert-node (:right root) key))))

(defn num-childs [node]
  (cond
    (and (= (:left node) nil) (= (:right node) nil)) 0
    (and (not= (:left node) nil) (not= (:right node) nil)) 2
    :else 1))

(defn min-node-key [root]
  (if (= (:left root) nil)
    (:key root)
    (recur (:left root))))

(defn max-node-key [root]
  (if (= (:right root) nil)
    (:key root)
    (recur (:right root))))

(defn tree-search [root, key]
  (cond
    (= root nil) nil
    (= (:key root) key) root
    (< key (:key root)) (tree-search (:left root) key)
    (> key (:key root)) (tree-search (:right root) key)))

(defn remove-node [root, key]
  (cond
    (= root nil) nil
    (and (= key (:key root)) (= (num-childs root) 0)) nil
    (and (= key (:key root)) (= (:left root) nil)) (:right root)
    (and (= key (:key root)) (= (:right root) nil)) (:left root)
    #_(
       If we want to remove a node with two childs.

       1. Change the key value of this node for the min key value on the right side of the tree.
       2. Find the node with the min key value and remove it.
       )
    (and (= key (:key root)) (= (num-childs root) 2))
    (assoc root
           :key (min-node-key (:right root))
           :right (remove-node (:right root) (min-node-key (:right root))))
    (< key (:key root)) (assoc root :left (remove-node (:left root) key))
    (> key (:key root)) (assoc root :right (remove-node (:right root) key))))

(defn node-successor [root, key]
  (cond
    (= root nil) nil

    #_(
       If we find ourselves in the following situation,
       searching for key = 12.

                          15 <- root
                         /  \
       searching for -> 12  nil
                       / \
                     nil 13
                         / \
                        11 24

       Were the left child of the current node has the
       key we are searching for, 12 in this case. Since
       the sub-tree on the right side contains larges
       values, we know that the successor of 12 must be
       there. In fact, it must be the minimun (11).

       In the event that the right sub-tree does not
       exist, the successor must be its parent, which
       is always bigger.
       )
    (= key (:key (:left root))) (or (min-node-key (:right (:left root))) (:key root))
    #_(
       Similarly, if we find ourselves in the following
       situation, searching for key = 13.

                          15 (A)<- root
                         /  \
                    (B) 12  nil
                       / \
                     nil 13 (C) <- searching for
                         / \
                        11 24

       Where the right sub-tree of the left sub-tree
       of the current node has the key we are searching
       for, 13 in this case. Due to the fact that, the
       right sub-tree contains bigger elements, we know
       that the successor of 13 must be inside on its right
       subtree. In fact, it must be the minimun (24).

       Otherwise, if the right sub-tree does not exist,
       we know that the successor is node A.
       To understand this, let's take a look at the
       example. Parent of node C (B) has a lower value
       (since C it to the right of B).
       Therefore, we need to go to node A (B's parent),
       which is always bigger since B ist o the left of
       A. We reach A > C, because C is located in the
       right sub-tree of A.
       )
    (= key (:key (:right (:left root)))) (or (min-node-key (:right (:right (:left root)))) (:key root))
    (< key (:key root)) (node-successor (:left root) key)
    (> key (:key root)) (node-successor (:right root) key)
    (= key (:key root)) (min-node-key (:right root))))

(defn inorder-tree-walk
  ([root] (inorder-tree-walk root (max-node-key root)))
  ([root, max-value] (cond
                      (= (:key root) max-value) (str (:key root))
                      (not= root nil) (str (inorder-tree-walk (:left root) max-value)
                                           (:key root) ", "
                                           (inorder-tree-walk (:right root) max-value)))))


;;;; I am still learning clojure
;;;; To make my life easier I will simulate tests with
;;;; some ifs statements.

(defn print-test-title [action, key]
  (printf "Test: %s node %s to \"tree\" equals \"tree-with-node-%s\"\n", (string/capitalize action), key, key))

(defn print-result [result]
  (println "Result:"result"\n"))

(defn build-tree [keys]
  (cond
    (= keys nil) nil
    (not= (type keys) (type [])) (->BinaryTreeNode nil keys nil)
    :else (->BinaryTreeNode (build-tree (keys 0)) (keys 1) (build-tree (keys 2)))))

(def tree (build-tree [[2 5 9] 12 [[13 15 17] 18 [nil 19 22]]]))

;; insert node tests
(def tree-with-node-14 (build-tree [[2 5 9] 12 [[[nil 13 14] 15 17] 18 [nil 19 22]]]))
(def tree-with-node-1 (build-tree [[[1 2 nil] 5 9] 12 [[13 15 17] 18 [nil 19 22]]]))
(def tree-with-node-11 (build-tree [[2 5 [nil 9 11]] 12 [[13 15 17] 18 [nil 19 22]]]))

(println "INSERT TESTS")
(print-test-title "insert" 14)
(print-result (= tree-with-node-14 (insert-node tree 14)))

(print-test-title "insert" 1)
(print-result (= tree-with-node-1 (insert-node tree 1)))

(print-test-title "insert" 11)
(print-result (= tree-with-node-11 (insert-node tree 11)))

;;delete node tests
(def tree-without-12 (build-tree [[2 5 9] 13 [[nil 15 17] 18 [nil 19 22]]]))
(def tree-without-2 (build-tree [[nil 5 9] 12 [[13 15 17] 18 [nil 19 22]]]))
(def tree-without-15 (build-tree [[2 5 9] 12 [[13 17 nil] 18 [nil 19 22]]]))
(def tree-without-19 (build-tree [[2 5 9] 12 [[13 15 17] 18 [nil 22 nil]]]))

(println "\nREMOVE TESTS")
(print-test-title "remove" 12)
(print-result (= tree-without-12 (remove-node tree 12)))

(print-test-title "remove" 2)
(print-result (= tree-without-2 (remove-node tree 2)))

(print-test-title "remove" 15)
(print-result (= tree-without-15 (remove-node tree 15)))

(print-test-title "remove" 19)
(print-result (= tree-without-19 (remove-node tree 19)))

(println "Inorder tree walk of \"tree\":" (inorder-tree-walk tree))
