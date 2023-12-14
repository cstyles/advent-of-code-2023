// For more information see https://aka.ms/fsharp-console-apps

open System.IO

type Status =
    | Operational
    | Damaged
    | Unknown

let parse_status =
    function
    | '.' -> Operational
    | '#' -> Damaged
    | '?' -> Unknown
    | c -> failwith $"bad input: {c}"

type Row =
    { springs: Status list
      groups: int64 list }

let parse_row (line: string) =
    let split = line.Split " " in
    let springs = split[0].ToCharArray() |> Array.map parse_status |> Array.toList in
    let groups = split[1].Split "," |> Array.map int64 |> Array.toList in
    { springs = springs; groups = groups }

let replace_head new_head =
    function
    | [] -> []
    | head :: tail -> new_head :: tail

// let input = "../test_input.txt"
let input = "../input.txt"

let lines = File.ReadLines input |> Seq.map (fun x -> x.TrimEnd())
let rows = Seq.map parse_row lines

let rec trim_and_continue row cache =
    match row.springs with
    | head :: tail -> possible_arrangements { row with springs = tail } cache
    | _ -> failwith "empty row"

and process_stuff row cache =
    match row.springs, row.groups with
    | [], [ 0L ] -> (1L, cache)
    | [], 0L :: _ -> (0L, cache)
    | [], _ -> (0L, cache)
    | Damaged :: _rest, 0L :: _ -> (0L, cache)
    | Damaged :: rest, group_size :: rest_of_groups ->
        process_stuff
            { springs = rest
              groups = (group_size - 1L) :: rest_of_groups }
            cache
    | Operational :: rest_of_springs, 0L :: rest_of_groups ->
        possible_arrangements
            { springs = rest_of_springs
              groups = rest_of_groups }
            cache
    | Operational :: _, _ -> (0L, cache) // invalid
    | Unknown :: rest_of_springs, 0L :: rest_of_groups ->
        possible_arrangements
            { springs = rest_of_springs
              groups = rest_of_groups }
            cache
    | Unknown :: rest, group_size :: rest_of_groups ->
        process_stuff
            { springs = rest
              groups = (group_size - 1L) :: rest_of_groups }
            cache
    | _, [] -> failwith "groups can't be empty"

and possible_arrangements row cache =
    match Map.tryFind (row.springs, row.groups) cache with
    | Some cached_value -> (cached_value, cache)
    | None ->
        match (List.tryHead row.springs, List.tryHead row.groups) with
        | None, None -> (1L, cache) // done
        | None, Some _ -> (0L, cache) // invalid
        | Some Damaged, None -> (0L, cache) // invalid
        | Some Unknown, None -> trim_and_continue row cache
        | Some Operational, _ -> trim_and_continue row cache
        | Some Damaged, Some _group_size -> process_stuff row cache
        | Some Unknown, Some _ ->
            let springs' = replace_head Operational row.springs in
            let new_row = { row with springs = springs' } in
            let (try_operational, cache) = possible_arrangements new_row cache in
            let cache = Map.add (springs', row.groups) try_operational cache in

            let springs' = replace_head Damaged row.springs in
            let new_row = { row with springs = springs' } in
            let (try_damaged, cache) = possible_arrangements new_row cache in
            let cache = Map.add (springs', row.groups) try_damaged cache in

            (try_operational + try_damaged, cache)

let unfold (row: Row) =
    let springs =
        row.springs
        @ [ Unknown ]
        @ row.springs
        @ [ Unknown ]
        @ row.springs
        @ [ Unknown ]
        @ row.springs
        @ [ Unknown ]
        @ row.springs in

    let groups = List.replicate 5 row.groups |> List.collect id in
    { springs = springs; groups = groups }

let cache: Map<(Status list * int64 list), int64> = Map.empty

let folder (sum : int64, cache) row =
    let (arr, cache) = possible_arrangements row cache in (sum + arr, cache)

let (part1, cache') = Seq.fold folder (0L, cache) rows
printfn "part1 = %d" part1

let (part2, _cache) = Seq.fold folder (0L, cache') (Seq.map unfold rows)
printfn "part2 = %d" part2
