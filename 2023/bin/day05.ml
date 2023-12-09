open StdLabels

let input = Aoc.read_input 5

type map = { source_start : int; dest_start : int; length : int }

let parse_seeds () =
  List.hd input |> String.split_on_char ~sep:' '
  |> List.filter_map ~f:(fun s -> if s = "seeds:" then None else Some (int_of_string s))

let parse_maps () =
  let maps =
    match input with
    | _ :: _ :: maps ->
        List.fold_left maps
          ~f:(fun maps line ->
            match line with
            | "" -> (List.hd maps |> List.rev) :: List.tl maps
            | _ -> (
                if String.ends_with ~suffix:"map:" line then [] :: maps
                else
                  let nums = String.split_on_char ~sep:' ' line |> List.map ~f:int_of_string in
                  match nums with
                  | [ dst; src; len ] ->
                      let map = { source_start = src; dest_start = dst; length = len } in
                      (map :: List.hd maps) :: List.tl maps
                  | _ -> Aoc.raise_input_failure ()))
          ~init:[]
    | _ -> Aoc.raise_input_failure ()
  in
  (List.hd maps |> List.rev) :: List.tl maps |> List.rev

let rec get_dest src map =
  match map with
  | [] -> src
  | m :: rest ->
      if src >= m.source_start && src < m.source_start + m.length then
        src - m.source_start + m.dest_start
      else get_dest src rest

let rec convert_to_loc num maps =
  match maps with
  | [] -> num
  | map :: rest ->
      let next_num = get_dest num map in
      convert_to_loc next_num rest

(* part 1 *)
let () =
  let seeds = parse_seeds () in
  let maps = parse_maps () in
  let min_loc =
    List.fold_left seeds ~f:(fun min_loc s -> min min_loc (convert_to_loc s maps)) ~init:Int.max_int
  in
  print_endline @@ string_of_int min_loc

type range = { start : int; length : int }

let parse_seed_ranges () =
  let rec build_ranges x =
    match x with
    | start :: length :: rest -> { start; length } :: build_ranges rest
    | _ -> []
  in
  List.hd input |> String.split_on_char ~sep:' '
  |> List.filter_map ~f:(fun s -> if s = "seeds:" then None else Some (int_of_string s))
  |> build_ranges

(* part 2: ugly brute-force solution *)
let () =
  let rec min_loc_for_range s e maps m =
    if s = e then m
    else
      let loc = min m (convert_to_loc s maps) in
      min_loc_for_range (s + 1) e maps loc
  in
  let seed_ranges = parse_seed_ranges () in
  let maps = parse_maps () in
  let min_loc =
    List.fold_left seed_ranges
      ~f:(fun min_loc seed_range ->
        min min_loc
          (min_loc_for_range seed_range.start
             (seed_range.start + seed_range.length - 1)
             maps Int.max_int))
      ~init:Int.max_int
  in
  print_endline @@ string_of_int min_loc
(* print_endline "Uncomment above to see the slow brute-force solution to p2" *)

(* let concat l1 l2 = List.fold_left l1 ~f:(fun l i -> i :: l) ~init:l2 *)

(* (* ranges = list of range, map = list of map *) *)
(* let break_up_ranges ranges map = *)
(*   List.fold_left ranges *)
(*     ~f:(fun broken_ranges range -> *)
(*       let rec break_up_range map = *)
(*         match map with *)
(*         | [] -> [] *)
(*         | m :: rest -> *)
(*             if *)
(*               m.source_start < range.start && m.source_start + m.length > range.start + range.length *)
(*             then range :: break_up_range rest *)
(*             else if m.ss *)
(*             else range :: break_up_range rest *)
(*       in *)
(*       concat (break_up_range map) broken_ranges) *)
(*     ~init:[] *)

(* let () = *)
(*   let seed_ranges = parse_seed_ranges () in *)
(*   let maps = parse_maps () in *)
(*   let dst_ranges = *)
(*     List.fold_left maps ~f:(fun ranges map -> break_up_ranges ranges map) ~init:seed_ranges *)
(*   in *)
(*   let min_loc = *)
(*     List.fold_left dst_ranges ~f:(fun min_loc r -> min min_loc r.start) ~init:Int.max_int *)
(*   in *)
(*   print_endline @@ string_of_int min_loc *)
