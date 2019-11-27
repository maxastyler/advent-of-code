(def input (slurp "./input"))

(def uc clojure.string/upper-case)

(defn reacts? [r1 r2]
  (if (or (nil? r1) (nil? r2))
    false
    (and (not= r1 r2) (= (uc r1) (uc r2)))))

(defn react [r]
  (reduce
   (fn [acc n] (if (reacts? (peek acc) n)
                 (pop acc)
                 (conj acc n)))
   (vec (take 1 r)) (drop 1 r)))

(defn char-range [start end]
  (map char (range (int start) (inc (int end)))))

(def part-1 (count (react input)))
(def part-2 (apply min (map
                        (fn [c] (->> input
                                     (filter #(not= (uc %) (uc c)))
                                     (react)
                                     (count)))
                        (char-range \a \z))))
