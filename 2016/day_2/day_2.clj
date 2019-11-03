(require '[clojure.string :as str] '[clojure.core.reducers :as red])

(def input
    (map str/trim
        (str/split (slurp "./input") #"\n")))

(defn vadd [[a1 a2] [b1 b2]] [(+ a1 b1) (+ a2 b2)])

(defn inside [a b]
  (< 0 (count (filter #(= a %) b))))

(defn instruction-to-vector [i]
  (case i
    \U [0 1]
    \D [0 -1]
    \L [-1 0]
    \R [1 0]))

(def grid1 {
            [0 0] 7
            [0 1] 4
            [0 2] 1
            [1 0] 8
            [1 1] 5
            [1 2] 2
            [2 0] 9
            [2 1] 6
            [2 2] 3})

(def grid2 {[0 0] 5
            [1 1] 2
            [1 0] 6
            [1 -1] 'A
            [2 2] 1
            [2 1] 3
            [2 0] 7
            [2 -1] 'B
            [2 -2] 'D
            [3 1] 4
            [3 0] 8
            [3 -1] 'C
            [4 0] 9})

(defn move-grid [p1 v g]
  (if (inside (vadd p1 v) (keys g)) (vadd p1 v) p1))

(defn instructions-to-vectors [i g initial]
  (reduce #(move-grid %1 %2 g) initial
   (map instruction-to-vector i)))

(def vector-instructions-g1
  (map grid1
       (map #(instructions-to-vectors % grid1 [1 1]) input)))

(def vector-instructions-g2
  (map grid2
       (map #(instructions-to-vectors % grid2 [0 0]) input)))
