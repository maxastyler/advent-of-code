(def initial-string "047801")
;; (def initial-string "37")

(def num-recps (Integer/parseInt initial-string))

(def initial-recipe (vec (map #(Integer/parseInt %) (map str initial-string))))

(defn digit-vec "create a vector of digits from a number"
  [^Integer x] (->> (str x)
           (map str)
           (map #(Integer/parseInt %))
           (vec)))

(def part-1 (->> (loop [e1 0
                        e2 1
                        recipes initial-recipe]
                   (if (< (+ num-recps 10) (count recipes))
                     recipes
                     (let [new-recipes (apply conj recipes (digit-vec
                                                            (+ (recipes e1)
                                                               (recipes e2))))
                           new-e1 (mod (+ e1 (inc (recipes e1)))
                                       (count new-recipes))
                           new-e2 (mod (+ e2 (inc (recipes e2)))
                                       (count new-recipes))]
                       (recur new-e1 new-e2 new-recipes))))
                 (drop num-recps)
                 (take 10)
                 (apply str)))
