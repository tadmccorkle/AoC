open StdLabels

let input = Aoc.read_input 2

type game_bag = { id : int; r : int; g : int; b : int }

let build bag cubes =
  match String.split_on_char ~sep:' ' (String.trim cubes) with
  | [ c; "red" ] ->
      let i = int_of_string c in
      if i > bag.r then { bag with r = i } else bag
  | [ c; "green" ] ->
      let i = int_of_string c in
      if i > bag.g then { bag with g = i } else bag
  | [ c; "blue" ] ->
      let i = int_of_string c in
      if i > bag.b then { bag with b = i } else bag
  | _ -> Aoc.raise_input_failure ()

let get_game_bag record =
  let s = String.split_on_char ~sep:':' record in
  let id = int_of_string (List.nth (String.split_on_char ~sep:' ' (List.hd s)) 1) in
  let cubes = Str.split (Str.regexp "[,|;]") (List.nth s 1) in
  cubes |> List.fold_left ~f:build ~init:{ id; r = 0; g = 0; b = 0 }

(* part 1 *)
let () =
  let id_if_possible g = if g.r <= 12 && g.g <= 13 && g.b <= 14 then g.id else 0 in
  let possible_sum =
    input
    |> List.fold_left ~f:(fun acc record -> (get_game_bag record |> id_if_possible) + acc) ~init:0
  in
  print_endline (string_of_int possible_sum)

(* part 2 *)
let () =
  let power g = g.r * g.g * g.b in
  let power_sum =
    input |> List.fold_left ~f:(fun acc l -> (get_game_bag l |> power) + acc) ~init:0
  in
  print_endline (string_of_int power_sum)
