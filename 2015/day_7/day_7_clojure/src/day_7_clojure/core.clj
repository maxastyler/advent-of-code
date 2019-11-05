(ns day-7-clojure.core
    (:require [instaparse.core :as insta]))

(def input (slurp "/home/max/git/advent_of_code/2015/day_7/input"))

(def circuit-parse
  (insta/parser
   "
<multiassignment> = {assignment <whitespace>* <#'\\n'>*}
assignment = statement <whitespace>* <'->'> <whitespace>* token
<statement> = and | or | lshift | rshift | not | let
and = token <whitespace>* <'AND'> <whitespace>* token
or = token <whitespace>* <'OR'> <whitespace>* token
lshift = token <whitespace>* <'LSHIFT'> <whitespace>* token
rshift = token <whitespace>* <'RSHIFT'> <whitespace>* token
not = <'NOT'> <whitespace>* token
let = token
<token> = word | number
whitespace = #'\\s+'
word = #'[a-zA-Z]+'
number = #'[0-9]+'"))

(def circuits
  (let [d (circuit-parse input)]
    (apply hash-map
           (apply concat 
                  (map (fn [[_ left [_ right]]]
                         [right left]) d)))))

(defn eval-id [id]
  (let [[eval-type [type-1 arg-1] [type-2 arg-2]] (get circuits id)]
    (let [a1 (trans-arg type-1 arg-1)
          a2 (trans-arg type-2 arg-2)]
      (case eval-type
        :and (bit-and a1 a2)
        :or (bit-or a1 a2)
        :lshift (bit-shift-left a1 a2)
        :rshift (bit-shift-right a1 a2)
        :not (bit-not a1)
        :let a1))))

(def eval-id-memo (memoize eval-id))

(defn trans-arg [type-1 arg-1] (case type-1
                                 :word (eval-id-memo arg-1)
                                 :number (Integer/parseInt arg-1)
                                 nil nil))


(def circuits-override
  (assoc circuits "b" [:let [:number (str (eval-id "a"))]]))

(defn eval-id-p2 [id]
  (let [[eval-type [type-1 arg-1] [type-2 arg-2]] (get circuits-override id)]
    (let [a1 (trans-arg-2 type-1 arg-1)
          a2 (trans-arg-2 type-2 arg-2)]
      (case eval-type
        :and (bit-and a1 a2)
        :or (bit-or a1 a2)
        :lshift (bit-shift-left a1 a2)
        :rshift (bit-shift-right a1 a2)
        :not (bit-not a1)
        :let a1))))

(def eval-id-memo-p2 (memoize eval-id-p2))

(defn trans-arg-2 [type-1 arg-1] (case type-1
                                 :word (eval-id-memo-p2 arg-1)
                                 :number (Integer/parseInt arg-1)
                                 nil nil))
