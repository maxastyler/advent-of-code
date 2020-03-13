(def input (->> (slurp "input")
                (clojure.string/split-lines)
                (map #(clojure.string/split % #"\)"))
                (map (fn [[a b]] [(keyword a) (keyword b)]))))

(def orbit-tree (reduce
                 (fn [orbits [orbitee orbiter]]
                   (update-in orbits [orbitee]
                              #(if (nil? %) #{orbiter} (conj % orbiter))))
                 {} input))

(def inverse-tree (reduce
                   (fn [orbits [orbitee orbiter]]
                     (assoc orbits orbiter orbitee)) {} input))

(defn parents [nodes tree] (if-let [p ((peek nodes) tree)]
                             (recur (conj nodes p) tree)
                             nodes))

(defn value-tree [node val]
  (if-let [c (node orbit-tree)]
    (+ val (apply + (map #(value-tree % (inc val)) c)))
    val))

(def part-1 (value-tree :COM 0))

(def part-2
  (let [s (set (parents [:SAN] inverse-tree))
        y (set (parents [:YOU] inverse-tree))]
    (- (count (clojure.set/union
               (clojure.set/difference s y)
               (clojure.set/difference y s)))
       2)))
