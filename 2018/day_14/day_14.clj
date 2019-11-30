(def str-input "047801")

(def num-input 47801)

(def vec-input [0 4 7 8 0 1])

(defn digitise [^Integer x] (->> (str x)
                        (map str)
                        (mapv #(Integer/parseInt ^String %))))

(defn recipes [n]
  "Return at least n recipes"
  (loop [e1 0, e2 1, recipes [3 7]]
    (let [new-recipes (apply conj recipes (digitise (+ (recipes e1)
                                                       (recipes e2))))
          new-e1 (mod (+ e1 (recipes e1) 1) (count new-recipes))
          new-e2 (mod (+ e2 (recipes e2) 1) (count new-recipes))]
      (if (> (count new-recipes) n)
        new-recipes
        (recur new-e1 new-e2 new-recipes)))))

(def part-1 (->> num-input
                 (+ 10)
                 (recipes)
                 (drop num-input)
                 (take 10)
                 (apply str)))

(def part-2 (find-first vec-input (recipes 30000001)))

(defn find-first [s xs] (let [ls (count s)]
                          (reduce (fn [_ i]
                                    (if (= (subvec xs i (+ i ls))
                                           s)
                                      (reduced i)
                                      i))
                                  0
                                  (range (- (count xs) ls)))))
