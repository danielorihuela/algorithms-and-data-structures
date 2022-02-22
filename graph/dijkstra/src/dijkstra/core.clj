(ns dijkstra.core
  (:require
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.set :refer [difference]])
  (:gen-class))

(defn- distance [graph node]
  (first (node graph [Integer/MAX_VALUE nil])))

(defn- neighbours-to-update [graph u neighbours]
  (for [v (map (partial first) neighbours)
        :let [new-distance (+ (distance graph u) (v neighbours))]
        :when (< new-distance (distance graph v))]
    [v [new-distance u]]))

(defn- new-neighbours [graph node neighbours]
  (->> node neighbours (neighbours-to-update graph node) not-empty))

(defn- node-to-distance [graph]
  (map #(vec [(% 0) ((% 1) 0)]) graph))

(defn dijkstra [origin neighbours]
  (loop [visited #{} graph {origin [0 nil]} pq (priority-map origin 0)]
    (if-let [partial-graph (new-neighbours graph (ffirst pq) neighbours)]
      (recur (conj visited (ffirst pq))
             (into graph partial-graph)
             (into (pop pq) (node-to-distance partial-graph)))
      graph)))
