(ns aoc-clojure-2020.day-04
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]
            [clojure.spec.alpha :as v]))

(def input (as-> (get-input 4) i
             (s/split i #"\n\n")
             (map #(->> (s/split % #"[\ \n]+")
                        (map (fn [x] (let [[k v] (s/split x #":")]
                                       [(keyword (str *ns*) k) v])))
                        (into {})) i)))

(v/def ::byr #(<= 1920 (Integer/parseInt %) 2002))
(v/def ::iyr #(<= 2010 (Integer/parseInt %) 2020))
(v/def ::eyr #(<= 2020 (Integer/parseInt %) 2030))
(v/def ::hgt (v/or :cm #(if-let [[_ h] (re-matches #"(\d+)cm" %)] (<= 150 (Integer/parseInt h) 193))
                   :in #(if-let [[_ h] (re-matches #"(\d+)in" %)] (<= 59 (Integer/parseInt h) 76))))
(v/def ::hcl #(re-matches #"#[0-9a-f]{6}" %))
(v/def ::ecl #{"amb" "blu" "brn" "gry" "grn" "hzl" "oth"})
(v/def ::pid #(re-matches #"\d{9}" %))

(v/def ::passport (v/keys :req [::byr ::iyr ::eyr ::hgt ::hcl ::ecl ::pid]
                          :opt [::cid]))

(def part-1 (-> (filter #(every? % [::byr ::iyr ::eyr ::hgt ::hcl ::ecl ::pid]) input)
                count))

(def part-2 (-> (filter #(v/valid? ::passport %) input)
                count))
part-2
