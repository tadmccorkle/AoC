open StdLabels

let input = Aoc.read_input_lines 8

module Location = struct
  type t = int * int

  let compare (x0, y0) (x1, y1) =
    match Stdlib.compare x0 x1 with
    | 0 -> Stdlib.compare y0 y1
    | c -> c
end

module CharMap = Map.Make (Char)
module LocSet = Set.Make (Location)

let min_x, min_y = (0, 0)
let max_x, max_y = (String.length (List.hd input) - 1, List.length input - 1)

let _, antennas =
  List.fold_left
    ~f:(fun (y, map) line ->
      let _, map =
        String.fold_left
          ~f:(fun (x, map) ch ->
            match ch with
            | '0' .. '9' | 'a' .. 'z' | 'A' .. 'Z' -> (x + 1, CharMap.add_to_list ch (x, y) map)
            | _ -> (x + 1, map))
          ~init:(0, map) line
      in
      (y + 1, map))
    ~init:(0, CharMap.empty) input

let add_antinodes ~f points antinodes =
  let rec aux remaining antinodes =
    match remaining with
    | [] -> antinodes
    | (x, y) :: xs ->
        aux xs
          (List.fold_left
             ~f:(fun acc (other_x, other_y) ->
               if x = other_x || y = other_y then acc
               else
                 let dx, dy = (x - other_x, y - other_y) in
                 f x y dx dy acc)
             ~init:antinodes points)
  in
  aux points antinodes

let add_antinode x y dx dy antinodes =
  let ax, ay = (x + dx, y + dy) in
  let in_bounds = ax >= min_x && ax <= max_x && ay >= min_y && ay <= max_y in
  if not in_bounds then antinodes else LocSet.add (ax, ay) antinodes

(* part 1 *)
let () =
  let add_antinodes = add_antinodes ~f:add_antinode in
  let antinodes = CharMap.fold (fun _ p acc -> add_antinodes p acc) antennas LocSet.empty in
  let count = antinodes |> LocSet.elements |> List.length in
  print_endline @@ string_of_int count

let rec add_antinodes_in_line x y dx dy antinodes =
  let in_bounds = x >= min_x && x <= max_x && y >= min_y && y <= max_y in
  if not in_bounds then antinodes
  else add_antinodes_in_line (x + dx) (y + dy) dx dy (LocSet.add (x, y) antinodes)

(* part 2 *)
let () =
  let add_antinodes = add_antinodes ~f:add_antinodes_in_line in
  let antinodes = CharMap.fold (fun _ p acc -> add_antinodes p acc) antennas LocSet.empty in
  let count = antinodes |> LocSet.elements |> List.length in
  print_endline @@ string_of_int count
