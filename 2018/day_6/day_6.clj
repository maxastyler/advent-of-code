(require '[clojure.string :as str])

(def dangerous-coords
  (->> "./input"
       (slurp)
       (str/split-lines)
       (map #(str/split % #", "))
       (map #(vector (Integer/parseInt (nth %2 0))
                     (Integer/parseInt (nth %2 1))
                     %1) (range))))

(def xlims (let [xcoords (map first dangerous-coords)]
             [(- (apply min xcoords) 1) (+ (apply max xcoords) 1)]
             ))

(def ylims (let [ycoords (map second dangerous-coords)]
             [(- (apply min ycoords) 1) (+ (apply max ycoords) 1)]
             ))

(def coords (for [x (range (first xlims) (+ (second xlims) 1))
                  y (range (first ylims) (+ (second ylims) 1))
                  :let [value (if (or (<= x (first xlims))
                                      (>= x (second xlims))
                                      (<= y (first ylims))
                                      (>= y (second ylims)))
                                ##Inf
                                1)]]
              [x y value]))

(defn manhattan-distance [p1 p2] (apply + (map #(Math/abs ^long (- %1 %2)) p1 p2)))

(defn closest-dangerous-coord [[x y value]]
  (let [new-values (map (fn [[p1x p1y k]]
                          [(manhattan-distance [x y] [p1x p1y]) k value])
                        dangerous-coords)
        dists (map first new-values)
        min-dist (apply min dists)
        [[_ id value] :as filtered-values] (filter
                                            #(= min-dist (first %))
                                            new-values)
        min-count (count filtered-values)]
    (if (> min-count 1)
      nil
      {:id id :value value})
    ))

(def symbol-areas (->> coords
                          (map closest-dangerous-coord)
                          (filter #(not (nil? %)))
                          (group-by :id)
                          (map (fn [[k v]] [k (reduce #(+ %1 (:value %2))
                                                      0 v)]))))

(defn total-distance [[x y _]]
  (apply +
         (map (fn [[p1x p1y k]]
                (manhattan-distance [x y] [p1x p1y]))
              dangerous-coords)))

(def part-1 (->> symbol-areas
                 (filter (fn [[_ a]] (< a ##Inf)))
                 (apply max-key second)
                 (second)))

(def part-2 (->> coords
                 (map total-distance)
                 (filter #(< % 10000))
                 (count)))
