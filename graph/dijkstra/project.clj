(defproject dijkstra "0.1.0-SNAPSHOT"
  :description "Dijkstra algorithm from CLRS"
  :dependencies [[org.clojure/clojure "1.10.1"]
                 [org.clojure/data.priority-map "1.1.0"]]
  :main ^:skip-aot dijkstra.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
