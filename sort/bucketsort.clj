(ns data-structures.bucket-sort)

(defn distribute-into-buckets [values]
  (let [length (count values)]
    (reduce
     #(assoc
       %1
       (int (* length %2))
       (conj (nth %1 (int (* length %2))) %2))
     (vec (take length (repeat (vector))))
     values)))

(defn bucket-sort [values]
  (flatten (map sort (filter not-empty (distribute-into-buckets values)))))

(let [values (take 20 (repeatedly #(rand 1)))]
  (do
    (println "Sorted values")
    (println (bucket-sort values))))
