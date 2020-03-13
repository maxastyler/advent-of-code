(def asteroids (as-> (slurp "input") i
                 (clojure.string/split-lines i)
                 (map-indexed (fn [y l] (map-indexed #(vector %1 y %2) l)) i)
                 (apply concat i)
                 (filter #(= (% 2) \#) i)
                 (mapv (fn [[x y _]] [x y]) i)))

(defn dist-sq
  [x1 y1 x2 y2]
  (letfn [(square [x] (* x x))]
    (+ (square (- x2 x1)) (square (- y2 y1)))))

(defn angle
  [x1 y1 x2 y2]
  (let [r (+ (/ Math/PI 2) (Math/atan2 (- y2 y1) (- x2 x1)))]
    (if (neg? r)
      (+ (* 2 Math/PI) r)
      r)))

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
                  dis (partial dist-sq x1 y1)]
              (->> asteroids
                   (filter #(not= [x1 y1] %))
                   (group-by #(apply ang %))
                   (map (fn [[a ps]]
                          [a (sort-by #(dis (% 0) (% 1)) ps)]))
                   (sort-by #(% 0))
                   (map #(lazy-cat (second %) (repeat nil)))
                   (apply interleave)
                   (remove nil?)
                   (drop 199)
                   (first))))
