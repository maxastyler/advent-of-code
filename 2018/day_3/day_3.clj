(require '[clojure.string :as str] '[clojure.core.reducers :as red])

(def size 1000)

(def parsed (->> (str/split (slurp "./input") #"\n")
                 (map
                  #(re-matches
                    #"#(?<id>\d+) @ (?<x>\d+),(?<y>\d+): (?<w>\d+)x(?<h>\d+)"
                    %))
                 (map (fn [[_ id x y w h]] {:id (Integer/parseInt id),
                                            :x (Integer/parseInt x),
                                            :y (Integer/parseInt y),
                                            :w (Integer/parseInt w),
                                            :h (Integer/parseInt h)}))))

(defn index-2d-to-1d [[m n]] (+ (* size m) n))

(def grid (vec (repeat (* size size) #{})))

(defn insert-id [grid id pos]
  (update grid (index-2d-to-1d pos) (fn [s] (conj s id))))

(defn positions "return a list of positions inside the square"
  [x y w h]
  (for [dx (range w) dy (range h)] [(+ x dx) (+ y dy)]))

(def final-grid
  (let [update-mult (fn [g positions id]
                      (reduce #(insert-id %1 id %2) g positions))]
    (reduce (fn [g {:keys [id x y w h]}]
              (update-mult g (positions x y w h) id))
            grid
            parsed)))

(def part-1 (count (filter (fn [s] (> (count s) 1)) final-grid)))
(def part-2 (let [id-set (set (map :id parsed))]
              (reduce
               (fn [allowed-ids square-ids]
                 (if (> (count square-ids) 1)
                   (apply disj allowed-ids square-ids)
                   allowed-ids))
               id-set
               final-grid)))
