(require '[clojure.string :as str] '[clojure.core.reducers :as red])

(def finp
  (map (fn [[a & b]] [a (Integer/parseInt (apply str b))])
    (map str/trim
        (str/split (slurp "./input") #","))))

(defn rot [c r]
  (case r
    \L (mod (+ c 1) 4)
    (mod (- c 1) 4)))

(defn vmul [[a b] s] [(* a s) (* b s)])

(defn vadd [[a b] [c d]] [(+ a c) (+ b d)])

(defn vec_from_rot [r]
  (case r
    0 [1 0]
    1 [0 1]
    2 [-1 0]
    3 [0 -1]
    ))

(defn move [[c_pos c_rot] [r d]]
  (let [new_vec (vadd c_pos
                      (vmul
                       (vec_from_rot (rot c_rot r)) d))
        new_rot (rot c_rot r)]
    [new_vec new_rot]))

(def taxi_pos_list
  (map first (reductions move [[0 0] 0] finp)))

(def freq_map (frequencies taxi_pos_list))

(def p
  (filter
   (fn [[i v]] (< 1 (count-in-list v (take (+ 1 i) taxi_pos_list))))
   (map-indexed vector taxi_pos_list)))

(def first_repeated
  (first
   (filter
    (fn [x] (> (get freq_map x) 1))
    taxi_pos_list)))

(defn count-in-list [elem list]
  (->> list
       (filter #{elem})
       count))
