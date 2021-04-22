(ns data-structures.red-black-tree
  (:require
   [clojure.string :as string]))

(def Red "red")
(def Black "black")
(def BlackBlack "black-black")
(defrecord RedBlackTreeNode [color left key right])

(def black-leaf (->RedBlackTreeNode Black nil nil nil))
(def black-black-leaf (->RedBlackTreeNode BlackBlack nil nil nil))

(defn red? [node]
  (= (:color node) Red))

(defn black? [node]
  (= (:color node) Black))

(defn black-black? [node]
  (= (:color node) BlackBlack))

(defn child [sides, node]
  "Get a successor of the node, given the path"
  (->> sides
       seq
       (map #(cond
              (= % \l) :left
              (= % \r) :right))
       (reduce #(%2 %1) node)))

;; Functional implementation following the blueprint in
;; http://matt.might.net/papers/germane2014deletion.pdf
;; The match function of the clojure pattern matching
;; library could be used to make the code easier.
;; https://github.com/clojure/core.match
(defn balance [root]
  (cond
    ;; Figure 1 and 2
    (and (black? root) (red? (:left root)) (red? (child "ll" root)))
    (->RedBlackTreeNode
     Red
     (assoc (child "ll" root) :color Black)
     (:key (:left root))
     (assoc root :color Black :left (child "lr" root)))
    (and (black? root) (red? (:left root)) (red? (child "lr" root)))
    (->RedBlackTreeNode
     Red
     (assoc (:left root) :color Black :right (child "lrlr" root))
     (:key (child "lr" root))
     (assoc root :color Black :left (child "rrl" root)))
    (and (black? root) (red? (:right root)) (red? (child "rr" root)))
    (->RedBlackTreeNode
     Red
     (assoc root :color Black :right (child "rl" root))
     (:key (:right root))
     (assoc (:right (:right root)) :color Black))
    (and (black? root) (red? (:right root)) (red? (child "rl" root)))
    (->RedBlackTreeNode
     Red
     (assoc root :color Black :right (child "rll" root))
     (:key (child "rl" root))
     (assoc (:right root) :color Black :left (child "rlr" root)))
    ;; Figure 8
    (and (black-black? root) (red? (:left root)) (red? (child "lr" root)))
    (->RedBlackTreeNode
     Black
     (assoc (:left root) :color Black :right (child "lrl" root))
     (:key (child "lr" root))
     (assoc root :color Black :left (child "lrr" root)))
    (and (black-black? root) (red? (:right root)) (red? (child "rl" root)))
    (->RedBlackTreeNode
     Black
     (assoc root :color Black :right (child "rll" root))
     (:key (child "rl" root))
     (assoc (:right root) :color Black :left (child "rlr" root)))
    ;; Default case
    :else root))

(defn rotate [root]
  (cond
    ;; Figure 6
    (and (red? root) (black-black? (:left root)) (black? (:right root)))
    (balance
     (->RedBlackTreeNode
      Black
      (assoc root :left (assoc (:left root) :color Black) :right (child "rl" root))
      (:key (:right root))
      (child "rr" root)))
    (and (red? root) (black? (:left root)) (black-black? (:right root)))
    (balance
     (->RedBlackTreeNode
      Black
      (child "ll" root)
      (:key (:left root))
      (assoc root :left (child "lr" root) :right (assoc (:right root) :color Black))))
    ;; Figure 7
    (and (black? root) (black-black? (:left root)) (black? (:right root)))
    (balance
     (->RedBlackTreeNode
      BlackBlack
      (assoc root :color Red :left (assoc (:left root) :color Black) :right (child "rl" root))
      (:key (:right root))
      (child "rr" root)))
    (and (black? root) (black? (:left root)) (black-black? (:right root)))
    (balance
     (->RedBlackTreeNode
      BlackBlack
      (child "ll" root)
      (:key (:left root))
      (assoc root :color Red :left (child "lr" root) :right (assoc (:right root) :color Black))))
    ;; Figure 9
    (and (black? root) (black-black? (:left root)) (red? (:right root)) (black? (child "rl" root)))
    (assoc
     (balance
      (->RedBlackTreeNode
       Black
       (->RedBlackTreeNode
        Black
        (->RedBlackTreeNode
         Red
         (assoc (:left root) :color Black)
         (:key root)
         (child "rll" root))
        (:key (child "rl" root))
        (child "rlr" root))
       (:key (:right root))
       (child "rr" root)))
     :color Black)
    (and (black? root) (red? (:left root)) (black? (child "lr" root)) (black-black? (:right root)))
    (assoc
     (balance
      (->RedBlackTreeNode
       Black
       (child "ll" root)
       (:key (:left root))
       (->RedBlackTreeNode
        Black
        (child "lrl" root)
        (:key (child "lr" root))
        (->RedBlackTreeNode
         Red
         (child "lrr" root)
         (:key root)
         (assoc (:right root) :color Black)))))
     :color Black)
    ;; Default case
    :else root))

(defn min-delete [node]
  (cond
    (= black-leaf node)
    nil
    (and (red? node) (= black-leaf (:left node)) (= black-leaf (:right node)))
    [(:key node) black-leaf]
    (and (black? node) (= black-leaf (:left node)) (= black-leaf (:right node)))
    [(:key node) black-leaf]
    (and (black? node) (= black-leaf (:left node)) (red? (:right node)))
    [(:key node) (assoc (:right node) :color Black)]
    :else
    (let [result (min-delete (:left node))]
      [(get result 0)
       (rotate (assoc node :left (get result 1)))])))

(defn insert-aux [root, key]
  (cond
    (= black-leaf root)
    (->RedBlackTreeNode Red black-leaf key black-leaf)
    (< key (:key root))
    (balance (assoc root :left (insert-aux (:left root) key)))
    (> key (:key root))
    (balance (assoc root :right (insert-aux (:right root) key)))
    :else
    root))

(defn insert [root, key]
  (assoc (insert-aux root key) :color Black))

(defn del [root, value]
  (cond
    (= black-leaf root)
    black-leaf
    (and (= value (:key root)) (red? root) (= black-leaf (:left root) (:right root)))
    black-leaf
    (and (= value (:key root)) (black? root) (red? (:left root)) (= black-leaf (:right root)))
    (assoc (:left root) :color Black)
    (and (= value (:key root)) (black? root) (= black-leaf (:left root) (:right root)))
    black-black-leaf
    :else
    (cond
      (< value (:key root))
      (rotate (assoc root :left (del (:left root) value)))
      (= value (:key root))
      (let [result (min-delete (:right root))]
        (rotate (assoc root :key (get result 0) :right (get result 1))))
      (> value (:key root))
      (rotate (assoc root :right (del (:right root) value))))))

(defn redden [root]
  (if (and (black? root) (= black-leaf (:left root) (:right root)))
    (assoc root :color Red)
    root))

(defn delete [root, value]
  (del (redden root) value))
