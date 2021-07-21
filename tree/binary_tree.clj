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

(defn inorder-tree-walk [root]
  (flatten (cond
             (= root nil) []
             :else (concat[(inorder-tree-walk (:left root))]
                          [(:key root)]
                          [(inorder-tree-walk (:right root))]))))

(defn node-successor [root, key]
  (let [values (apply vector (inorder-tree-walk root))]
    (->> key
         (.indexOf values)
         (#(get values (+ % 1))))))

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

(def tree (build-tree [[3 5 9] 12 [[13 15 17] 18 [nil 19 22]]]))

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



(println "\nNODE SUCCESSOT TESTS")
(def tree2 (build-tree [[5 10 [12 13 [14 15 16]]] 23 [24 25 26]]))
(println "Inorder tree walk of \"tree\":" (inorder-tree-walk tree2))
(println "node-successor of 25 should return 26")
(print-result (= 26 (node-successor tree2 25)))
(println "node-successor of 26 should return nil")
(print-result (= nil (node-successor tree2 26)))
(println "node-successor of 16 should return 23")
(print-result (= 23 (node-successor tree2 16)))

