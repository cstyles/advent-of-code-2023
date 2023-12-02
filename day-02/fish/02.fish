#!/usr/bin/env fish

if test (count $argv) -gt 0
  set input (cat $argv[1]) # Read from file specified by CLI arg
else
  set input (cat ../input.txt) # Default to actual input
end

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

function part2
  function game_power -a line
    set line (string trim --left --chars='Game ' $line)
    set line (string split ': ' $line)[2]

    set max_red 0
    set max_green 0
    set max_blue 0

    for round in (string split '; ' $line)
      for pull in (string split ', ' $round)
        set temp (string split ' ' $pull)
        set count $temp[1]
        set color $temp[2]

        switch $color
          case 'red'
            if test $count -gt $max_red
              set max_red $count
            end
          case 'green'
            if test $count -gt $max_green
              set max_green $count
            end
          case 'blue'
            if test $count -gt $max_blue
              set max_blue $count
            end
        end
      end
    end

    math "$max_red * $max_green * $max_blue"
  end

  set total 0
  for line in $argv
    set result (game_power $line)
    set total (math "$total + $result")
  end

  echo $total
end

part1 $input
part2 $input
