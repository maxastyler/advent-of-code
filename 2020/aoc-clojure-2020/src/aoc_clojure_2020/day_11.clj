(ns aoc-clojure-2020.day-11
  (:require [aoc-clojure-2020.helpers :refer [get-input]]))

(def example-input "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL")

(defn display-state [{:keys [cells width]}]
  (clojure.string/join "\n" (map
                             #(apply str (map (fn [c] (case c :full \# :empty \L \.)) %))
                             (partition width cells))))

(def input (let [inp (->> (get-input 11) (clojure.string/split-lines))
                 width (-> inp first count)
                 height (count inp)]
             {:cells (->> (mapcat (fn [xs] (map #(case % \L :empty :no-seat) xs)) inp)
                          (into []))
              :width width
              :height height
              :cart->flat (fn [[r c]] (if (and (<= 0 r (dec height))
                                               (<= 0 c (dec width)))
                                        (+ (* r width) c)
                                        nil))
              :flat->cart (fn [x] [(quot x width) (mod x width)])}))

(defn neighbour-count-part1 [{:keys [flat->cart cart->flat width height cells]}]
  (for [[r c] (map flat->cart (range (count cells)))]
    (reduce + 0 (for [r' (range (dec r) (+ r 2))
                      c' (range (dec c) (+ c 2))
                      :when (and (or (not= r' r)
                                     (not= c' c))
                                 (<= 0 r' (dec height))
                                 (<= 0 c' (dec width)))]
                  (if (= (get cells (cart->flat [r' c'])) :full) 1 0)))))

(defn neighbour-count-part2 [{:keys [flat->cart cart->flat width height cells]}]
  (for [[r c] (map flat->cart (range (count cells)))]
    (reduce #(+ %1 (if (= %2 :full) 1 0)) 0
            (for [[dr dc] [[-1 -1] [-1 0] [-1 1] [0 -1] [0 1] [1 -1] [1 0] [1 1]]]
              (reduce #(let [val (get cells (cart->flat %2))]
                         (case val
                           :full (reduced :full)
                           :empty (reduced :empty)
                           nil (reduced %1)
                           val))
                      nil
                      (drop 1 (iterate (fn [[r c]] [(+ r dr) (+ c dc)]) [r c])))))))

(defn timestep [neighbour-count-func death-seats {:keys [cells cart->flat flat->cart width height] :as state}]
  (assoc state :cells (mapv #(cond (= :no-seat %2) :no-seat
                                   (>= %1 death-seats) :empty
                                   (= 0 %1) :full true %2) (neighbour-count-func state) cells)))

(def part-1 (time (->> (reduce #(if (= (:cells %1) (:cells %2)) (reduced %1) %2)
                               (iterate (partial timestep neighbour-count-part1 4) input))
                       :cells
                       (reduce #(+ %1 (if (= %2 :full) 1 0)) 0))))

(def part-2 (time (->> (reduce #(if (= (:cells %1) (:cells %2)) (reduced %1) %2)
                               (iterate (partial timestep neighbour-count-part2 5) input))
                       :cells
                       (reduce #(+ %1 (if (= %2 :full) 1 0)) 0))))
