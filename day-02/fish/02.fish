#!/usr/bin/env fish

set input (cat ../input.txt)

function part1
  function is_round_possible -a round
    for pull in (string split ', ' $round)
      set temp (string split ' ' $pull)
      set count $temp[1]
      set color $temp[2]

      switch $color
        case 'red'
          if test $count -gt 12
            return 1
          end
        case 'green'
          if test $count -gt 13
            return 1
          end
        case 'blue'
          if test $count -gt 14
            return 1
          end
      end
    end

    return 0
  end

  function is_game_possible -a line
    set line (string trim --left --chars='Game ' $line)
    set temp (string split ': ' $line)
    set id $temp[1]
    set line $temp[2]

    for round in (string split '; ' $line)
      if not is_round_possible $round
        echo 0; return
      end
    end

    echo $id
  end

  set total 0
  for line in $argv
    set result (is_game_possible $line)
    set total (math "$total + $result")
  end

  echo $total
end

part1 $input
