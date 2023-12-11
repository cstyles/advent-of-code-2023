(local fennel (require :fennel))

; Turn a string into an array of characters.
(fn chars [string]
  (let [result {}]
    (each [c (string.gmatch string ".")]
      (table.insert result c))
    result))

; Turn an input file into a 2D array of characters.
(fn load_map [file]
  (let [lines (io.lines file)]
    (let [rows (icollect [line lines]
                 (icollect [_ c (ipairs (chars line))]
                   c))]
      rows)))

(fn all [func array]
  (var found true)
  (each [_ item (ipairs array) &until (not found)]
    (if (not (func item))
        (set found false)))
  found)

(fn any [func array]
  (var found false)
  (each [_ item (ipairs array) &until found]
    (if (func item)
        (set found true)))
  found)

(fn contains [needle haystack]
  (any #(= needle $1) haystack))

(fn empty_space [char]
  (= "." char))

(fn all_empty [table]
  (all empty_space table))

; TODO: Flatten?
(fn find_galaxies [map]
  (let [galaxies []]
    (each [y row (ipairs map)]
      (each [x c (ipairs row)]
        (if (= "#" c)
            (table.insert galaxies {: y : x}))))
    galaxies))

(fn find_empty_rows [map]
  (icollect [y row (ipairs map)]
    (if (all_empty row) y)))

(fn columns [map]
  (var columns [])
  (let [row_length (length (. map 1))]
    (for [x 1 row_length]
      (table.insert columns (icollect [y _ (ipairs map)] (. (. map y) x)))))
  columns)

(fn find_empty_columns [map]
  (icollect [x column (ipairs (columns map))]
    (if (all_empty column) x)))

(fn abs_diff [a b]
  (if (> a b)
      (- a b)
      (- b a)))

(fn manhattan_distance [a b]
  (+ (abs_diff (. a :y) (. b :y)) (abs_diff (. a :x) (. b :x))))

(fn range [a b]
  (if (< a b)
      [a b]
      [b a]))

(fn in_range [x [a b]]
  (and (>= x a) (< x b)))

(fn overlap [empty_row_or_columns some_range]
  (accumulate [sum 0 _ item (ipairs empty_row_or_columns)]
    (+ sum (if (in_range item some_range) 1 0))))

(fn solve [galaxies empty_rows empty_columns]
  (var part1 0)
  (var part2 0)
  (for [a 1 (length galaxies)]
    (for [b (+ 1 a) (length galaxies)]
      (let [galaxy_a (. galaxies a)
            galaxy_b (. galaxies b)
            naive_distance (manhattan_distance galaxy_a galaxy_b)
            double_rows (overlap empty_rows
                                 (range (. galaxy_a :y) (. galaxy_b :y)))
            double_columns (overlap empty_columns
                                    (range (. galaxy_a :x) (. galaxy_b :x)))]
        (set part1 (+ part1 naive_distance double_rows double_columns))
        (set part2
             (+ part2 naive_distance (* 999999 (+ double_rows double_columns)))))))
  [part1 part2])

(fn main [file]
  (let [map (load_map file)
        galaxies (find_galaxies map)
        empty_rows (find_empty_rows map)
        empty_columns (find_empty_columns map)
        [part1 part2] (solve galaxies empty_rows empty_columns)]
    (print (.. "part1 = " part1))
    (print (.. "part2 = " part2))))

; (main :../test_input.txt)
(main :../input.txt)
