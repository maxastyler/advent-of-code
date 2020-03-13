(def planets (->> (slurp "input")
                  (clojure.string/split-lines)
                  (map #(re-find #"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>" %))
                  (mapv #(as-> (rest %) i
                           (map read-string i)
                           (hash-map :pos (vec i) :vel [0 0 0])))))

(def planet-dims "Separate the planets into a vector over each dimension"
  (mapv #(mapv
          (fn [{:keys [pos vel]}]
            [(pos %) (vel %)]) planets) (range 3)))

(defn pairs [n]
  (for [i (range (dec n))
        j (range (inc i) n)]
    [i j]))

(def pn (pairs (count planets)))

(defn force "get the force from positions a and b"
  [a b]
  (cond (< a b) 1
        (> a b) -1
        :else 0))

(defn abs [x] (if (< x 0) (- x) x))

(defn energy
  [{:keys [pos vel]}]
  (* (apply + (map abs pos)) (apply + (map abs vel))))

(defn step [planets]
  (->> (reduce (fn [p [i j]]
                 (let [v (map force (get-in p [i :pos]) (get-in p [j :pos]))]
                   (-> (update-in p [i :vel] #(mapv + % v))
                       (update-in [j :vel] #(mapv - % v)))))
               planets
               (pairs (count planets)))
       (mapv (fn [p] (update p :pos #(mapv + % (:vel p)))))))

(defn step-1d [p]
  (->> (reduce (fn [p [i j]]
                 (let [v (force (get-in p [i 0]) (get-in p [j 0]))]
                   (-> (update-in p [i 1] #(+ % v))
                       (update-in [j 1] #(- % v)))))
               p
               pn)
       (mapv (fn [[x v]] [(+ x v) v]))))

(defn find-repeat-1d [d]
  (loop [p (step-1d d) i 1]
    (if (= p d)
      i
      (recur (step-1d p) (inc i)))))

(def part-1 (->> (iterate step planets)
                 (drop 1000)
                 (first)
                 (map energy)
                 (apply +)))
(defn gcd 
      [a b]
      (if (zero? b)
      a
      (recur b, (mod a b))))
 
(defn lcm 
      [a b]
      (/ (* a b) (gcd a b)))
;; to calculate the lcm for a variable number of arguments
(defn lcmv [& v] (reduce lcm v))

(def part-2 (apply lcmv (mapv find-repeat-1d planet-dims)))
