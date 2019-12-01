(comment
  "The game consists of a [m, n] grid where at each point there is
either nil or something.
nil represents an inaccessible point, and something else is accessible

The state is {:grid [] :units [{:health 200 :x 0 :y 0 :attack 3 :type :goblin/:elf}]}
Each elf is a map consisting of health, x and y as keys.
"
)

(defn find-closest
  "Find the closest targets from the set of targets in the given grid
  Inaccessible positions in the grid are noted by nil

  closest is a vector [x y dist]
  "
  [start targets excluded]
  (loop [queue (conj (clojure.lang.PersistentQueue/EMPTY) (conj start 0))
         seen #{}
         closest []]
    (let [point (peek queue)
          new_queue (pop queue)
          point_x_y [(point 0) (point 1)]
          cur-dist (point 2)]
      (if (and (not-empty closest)
               (< ((peek closest) 2) cur-dist))
        closest ;; if we have stuff in closest and the current point is farther
        (if (or (seen point_x_y)
                (excluded point_x_y))
          (recur new_queue seen closest)
          (recur (apply conj new_queue (map #(conj % (inc cur-dist))
                                            (neighbours point_x_y)))
                 (conj seen point_x_y)))))))

(defn neighbours "Get the neighbours of a point"
  [[x y]] [[(dec x) y] [(inc x) y] [x (dec y)] [x (inc y)]])
