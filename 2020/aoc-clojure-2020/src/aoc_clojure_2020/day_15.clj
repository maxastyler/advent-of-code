(ns aoc-clojure-2020.day-15)

(defn take-turn [[[val t] turns]]
  [[(if-let [t' (turns val)] (- t t') 0) (inc t)] (assoc turns val t)])
(defn game-start [nums] [[(last nums) (dec (count nums))] (into {} (map vector nums (range)))])
(-> (iterate take-turn (game-start [18 11 9 0 5 1])) (nth (- 2020 6)) first first)
(-> (iterate take-turn (game-start [18 11 9 0 5 1])) (nth (- 30000000 6)) first first)
