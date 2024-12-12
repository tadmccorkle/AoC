open StdLabels

let input = Aoc.read_input_lines 6

let grid =
  let rows = List.length input in
  if rows = 0 then Aoc.raise_input_failure ()
  else
    let cols = String.length (List.hd input) in
    let arr =
      try Array.make_matrix ~dimx:rows ~dimy:cols '\x00'
      with Invalid_argument _ -> Aoc.raise_input_failure ()
    in
    List.iteri
      ~f:(fun i ->
        String.iteri ~f:(fun j c ->
            try arr.(i).(j) <- c with Invalid_argument _ -> Aoc.raise_input_failure ()))
      input;
    arr

module Location = struct
  type t = int * int

  let compare (x0, y0) (x1, y1) =
    match Stdlib.compare x0 x1 with
    | 0 -> Stdlib.compare y0 y1
    | c -> c
end

module LocSet = Set.Make (Location)

let start_row, start_col =
  let rec aux rows row =
    match rows with
    | [] -> Aoc.raise_input_failure ()
    | x :: xs -> (
        match String.index_opt x '^' with
        | Some col -> (row, col)
        | None -> aux xs (row + 1))
  in
  aux input 0

let get_opt grid row col = try Some grid.(row).(col) with Invalid_argument _ -> None

let traverse =
  let rec aux row col dr dc visits =
    match get_opt grid (row + dr) (col + dc) with
    | None -> visits
    | Some ch ->
        let next_row, next_col, next_dr, next_dc =
          if ch <> '#' then (row + dr, col + dc, dr, dc) else (row, col, dc, dr * -1)
        in
        aux next_row next_col next_dr next_dc (LocSet.add (next_row, next_col) visits)
  in
  aux start_row start_col (-1) 0 (LocSet.singleton (start_row, start_col))

(* part 1 *)
let () =
  let distinct_positions = LocSet.elements traverse |> List.length in
  print_endline @@ string_of_int distinct_positions

module LocDir = struct
  type t = int * int * int * int

  let compare (a0, b0, c0, d0) (a1, b1, c1, d1) =
    match Stdlib.compare a0 a1 with
    | 0 -> (
        match Stdlib.compare b0 b1 with
        | 0 -> (
            match Stdlib.compare c0 c1 with
            | 0 -> Stdlib.compare d0 d1
            | c -> c)
        | c -> c)
    | c -> c
end

module LocDirSet = Set.Make (LocDir)

let is_loop obstr_row obstr_col =
  let rec aux row col dr dc visits =
    if LocDirSet.find_opt (row, col, dr, dc) visits <> None then true
    else
      match get_opt grid (row + dr) (col + dc) with
      | None -> false
      | Some ch ->
          let next_row, next_col, next_dr, next_dc =
            if ch <> '#' then (row + dr, col + dc, dr, dc) else (row, col, dc, dr * -1)
          in
          aux next_row next_col next_dr next_dc (LocDirSet.add (row, col, dr, dc) visits)
  in
  grid.(obstr_row).(obstr_col) <- '#';
  let res = aux start_row start_col (-1) 0 LocDirSet.empty in
  grid.(obstr_row).(obstr_col) <- '.';
  res

(* part 2 *)
let () =
  let obstructions = LocSet.remove (start_row, start_col) traverse in
  let loop_count =
    List.fold_left
      ~f:(fun acc (r, c) -> if is_loop r c then acc + 1 else acc)
      ~init:0 (LocSet.elements obstructions)
  in
  print_endline @@ string_of_int loop_count
