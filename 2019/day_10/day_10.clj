(def asteroids (as-> (slurp "input") i
                 (clojure.string/split-lines i)
                 (map-indexed (fn [y l] (map-indexed #(vector %1 y %2) l)) i)
                 (apply concat i)
                 (filter #(= (% 2) \#) i)
                 (mapv (fn [[x y _]] [x y]) i)))

(defn dist-sq
  "Get the distance squared"
  [x1 y1 x2 y2]
  (letfn [(square [x] (* x x))]
    (+ (square (- x2 x1)) (square (- y2 y1)))))

(defn angle
  "Get the angle from the position [x1, y2] to [x2, y2]"
  [x1 y1 x2 y2]
  (let [r (+ (/ Math/PI 2) (Math/atan2 (- y2 y1) (- x2 x1)))]
    (if (neg? r)
      (+ (* 2 Math/PI) r)
      r)))

(defn interleave-all
  "Returns a lazy seq of the first item in each coll, then the second, etc.
  Unlike `clojure.core/interleave`, the returned seq contains all items in the
  supplied collections, even if the collections are different sizes."
  {:arglists '([& colls])}
  ([] ())
  ([c1] (lazy-seq c1))
  ([c1 c2]
   (lazy-seq
    (let [s1 (seq c1), s2 (seq c2)]
      (if (and s1 s2)
        (cons (first s1) (cons (first s2) (interleave-all (rest s1) (rest s2))))
        (or s1 s2)))))
  ([c1 c2 & colls]
   (lazy-seq
    (let [ss (remove nil? (map seq (conj colls c2 c1)))]
      (if (seq ss)
        (concat (map first ss) (apply interleave-all (map rest ss))))))))

(def part-1 (->> (reduce (fn [angles [x1 y1 :as p]]
                           (assoc angles p
                                  (->> asteroids
                                       (filter #(not= p %))
                                       (map
                                        (fn [[x2 y2]] (angle x1 y1 x2 y2)))
                                       (set)
                                       (count))))
                         {} asteroids)
                 (apply max-key val)))

(def part-2 (let [x1 (get-in part-1 [0 0])
                  y1 (get-in part-1 [0 1])
                  ang (partial angle x1 y1)
                  dis (partial dist-sq x1 y1)
                  sorted (->> asteroids
                              (filter #(not= [x1 y1] %))
                              (group-by #(apply ang %))
                              (mapv (fn [[a ps]]
                                      [a (sort-by #(dis (% 0) (% 1)) ps)]))
                              (into (sorted-map))
                              (mapv (fn [[_ v]] (vec v)))
                              (apply interleave-all))]
              (nth sorted 199)))
