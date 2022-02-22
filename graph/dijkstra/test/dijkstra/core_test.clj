(ns dijkstra.core-test
  (:require [clojure.test :refer :all]
            [dijkstra.core :refer :all]))

(def nodes [:s :t :x :y :z])
(def directed-edges
  {[:s :t] 10
   [:s :y] 5
   [:t :x] 1
   [:t :y] 2
   [:y :t] 3
   [:y :x] 9
   [:y :z] 2
   [:x :z] 4
   [:z :s] 7
   [:z :x] 6})
(def origin :s)

(defn directed-neighbours [node]
  (apply conj (for [edge directed-edges
                    :let [neighbour (last (key edge))]
                    :when (= (first (key edge)) node)]
                {neighbour (last edge)})))

(deftest dijkstra-works-directed-graph-test
  (testing "Dijkstra works for directed graph"
    (let [graph-result (dijkstra origin directed-neighbours)]
      (is (= (for [node nodes]
               (get graph-result node))
             [[0 nil] [8 :y] [9 :t] [5 :s] [7 :y]])))))

(def undirected-edges
  {[:s :t] 10
   [:s :y] 5
   [:t :x] 1
   [:t :y] 2
   [:y :x] 9
   [:y :z] 2
   [:x :z] 4
   [:z :s] 7})

(defn undirected-neighbours [node]
  (apply conj (for [edge undirected-edges]
                (cond
                  (= node (last (key edge)))
                  {(first (key edge)) (last edge)}
                  (= node (first (key edge)))
                  {(last (key edge)) (last edge)}
                  :else {}))))

(deftest dijkstra-works-undirected-graph-test
  (testing "Dijkstra works for undirected graph"
    (let [graph-result (dijkstra origin undirected-neighbours)]
      (is (= (for [node nodes]
               (get graph-result node))
             [[0 nil] [7 :y] [8 :t] [5 :s] [7 :s]])))))
