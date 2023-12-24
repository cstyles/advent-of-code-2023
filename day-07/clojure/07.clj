#!/usr/bin/env clojure -M

(require 'clojure.string)

; (def input (slurp "../test_input.txt"))
(def input (slurp "../input.txt"))

(defrecord Hand [cards bid])

(defn explode [string] (clojure.string/split string #""))

(defn parse_card [card]
  (case card
    "2" 2
    "3" 3
    "4" 4
    "5" 5
    "6" 6
    "7" 7
    "8" 8
    "9" 9
    "T" 10
    "J" 11
    "Q" 12
    "K" 13
    "A" 14))

(defn parse_hand [string]
  (let [[cards bid] (clojure.string/split string #" " 2)
        cards (map parse_card (explode cards))
        bid (Integer/parseInt bid)]
    (->Hand cards bid)))

(def hands
  (->>
   input
   (clojure.string/split-lines)
   (map parse_hand)))

(def FiveOfAKind 10)
(def FourOfAKind 9)
(def FullHouse 8)
(def ThreeOfAKind 7)
(def TwoPair 6)
(def OnePair 5)
(def HighCard 4)

(defn hand_type [hand]
  (let [counts (frequencies (:cards hand))]
    (cond
      (some (fn [c] (= 5 c)) (vals counts)) FiveOfAKind ; five of a kind
      (some (fn [c] (= 4 c)) (vals counts)) FourOfAKind
      (some (fn [c] (= 3 c)) (vals counts))
      (if (some (fn [c] (= 2 c)) (vals counts))
        FullHouse
        ThreeOfAKind)
      :else (case (count (filter (fn [c] (= 2 c)) (vals counts)))
              2 TwoPair
              1 OnePair ; one pair
              0 HighCard
              (throw (Throwable. "not a valid hand"))))))

; Returns the first element in a sequence that matches a predicate
; (or nil if no such element exists).
(defn find-seq [f s] (first (filter f s)))

(defn zip [a b] (map vector a b))

(defn break_tie [a b]
  (let [a (:cards a)
        b (:cards b)
        zipped (zip a b)
        compared (map (fn [[a b]] (compare a b)) zipped)
        maybe_diff (find-seq (fn [x] (not= 0 x)) compared)]
    (or
     maybe_diff
     0)))

(defn compare_hands1 [a b]
  (let [compared_hands (compare (hand_type a) (hand_type b))]
    (if (= 0 compared_hands)
      (break_tie a b)
      compared_hands)))

(defn answer [hands]
  (->>
   hands
   (map-indexed (fn [rank hand] (* (+ 1 rank) (:bid hand))))
   (reduce +)))

(let [sorted_hands (sort compare_hands1 hands)]
  (println "part1 =" (answer sorted_hands)))
