(ns user (:require [clojure.string :as str]))

(defn parse-instruction [i]
  {:dir (keyword (str (first i)))
   :dist (read-string (apply str (rest i)))})

(def input (->> (slurp "./input")
                (str/split-lines)
                (map #(str/split % #","))
                (mapv #(mapv parse-instruction %))))

(defn gen-positions [[init-x init-y] {:keys [dir dist]}]
  (map (fn [[x y]] [(+ x init-x) (+ y init-y)])
       (case dir
         :U (map #(vector 0 %) (range 1 (inc dist)))
         :D (map #(vector 0 %) (range -1 (dec (- dist)) -1))
         :L (map #(vector % 0) (range -1 (dec (- dist)) -1))
         :R (map #(vector % 0) (range 1 (inc dist))))))

(defn wire "given a set of instructions build the wire map"
  [instructions] ((let [positions (reduce (fn [positions instruction]
                                            (apply conj positions
                                                   (gen-positions
                                                    (peek positions)
                                                    instruction)))
                                          [[0 0]] instructions)]
                    (reduce (fn [[d acc] p]
                              [(inc d)
                               (update acc p #(if (nil? %) d %))])
                            [0 {}] positions)) 1))
(def w1 (wire (input 0)))
(def w2 (wire (input 1)))

(def part-1 (->> (clojure.set/intersection (into #{} (keys w1))
                                           (into #{} (keys w2)))
                 (map (fn [[x y]] (+ (Math/abs x) (Math/abs y))))
                 (filter pos?)
                 (apply min)))

(def part-2 (->> (let [intersections (clojure.set/intersection
                                      (into #{} (keys w1))
                                      (into #{} (keys w2)))]
                       (map #(+ (Math/abs (w1 %))
                                (Math/abs (w2 %))) intersections))
                 (filter pos?)
                 (apply min)))
