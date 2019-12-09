(def image-width 25)
(def image-height 6)

(def image (as-> (slurp "input") i
             (clojure.string/trim-newline i)
             (map str i)
             (mapv read-string i)
             (partition (* image-height
                           image-width) i)
             (mapv vec i)
             (conj i (vec (repeat (* image-height
                                     image-width) 0)))))

(def part-1 (as-> image i
              (apply min-key #(count (filter zero? %)) i)
              (* (count (filter #(= 1 %) i))
                 (count (filter #(= 2 %) i)))))

(def part-2 (as-> image i
              (apply map (fn [& xs] (first (drop-while #(= 2 %) xs))) i)
              (partition image-width i)))
