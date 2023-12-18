open StdLabels

let input = Aoc.read_input 10

(* | is a vertical pipe connecting north and south. *)
(* - is a horizontal pipe connecting east and west. *)
(* L is a 90-degree bend connecting north and east. *)
(* J is a 90-degree bend connecting north and west. *)
(* 7 is a 90-degree bend connecting south and west. *)
(* F is a 90-degree bend connecting south and east. *)
(* . is ground; there is no pipe in this tile. *)
(* S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has. *)

type point = { x : int; y : int }

type pipe = N of char | E of char | S of char | W of char

let string_of_point p = string_of_int p.x ^ "," ^ string_of_int p.y

let find_start lines =
  let rec fs lines row =
    match lines with
    | [] -> Aoc.raise_input_failure ()
    | hd :: rest -> (
        match String.index_opt hd 'S' with
        | Some s -> { x = row; y = s }
        | None -> fs rest (row + 1))
  in
  fs lines 0

let connects = function
  | N p -> p = '|' || p = '7' || p = 'F'
  | _ -> failwith "TODO"

let traverse = ()

let () =
  let rows = List.length input in
  let cols = List.hd input |> String.length in
  let _ = Array.make rows (Array.make cols false) in
  let start = find_start input in
  print_endline @@ string_of_point start
