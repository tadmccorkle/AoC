open StdLabels

let input = Aoc.read_input_lines 4

let word_search =
  let rows = List.length input in
  if rows = 0 then Aoc.raise_input_failure ()
  else
    let cols = String.length (List.hd input) in
    let ws =
      try Array.make_matrix ~dimx:rows ~dimy:cols '\x00'
      with Invalid_argument _ -> Aoc.raise_input_failure ()
    in
    List.iteri
      ~f:(fun i ->
        String.iteri ~f:(fun j c ->
            try ws.(i).(j) <- c with Invalid_argument _ -> Aoc.raise_input_failure ()))
      input;
    ws

let get_opt row col = try Some word_search.(row).(col) with Invalid_argument _ -> None

let build_word length row col dr dc =
  let rec build_word_aux remaining row col word =
    if remaining = 0 then Some (word |> List.rev |> List.to_seq |> String.of_seq)
    else
      match get_opt row col with
      | None -> None
      | Some ch -> build_word_aux (remaining - 1) (row + dr) (col + dc) (ch :: word)
  in
  build_word_aux length row col []

let xmas_search_matrix =
  let nums = [ -1; 0; 1 ] in
  List.concat (List.map ~f:(fun x -> List.map ~f:(fun y -> (x, y)) nums) nums)

let count_xmas row col =
  List.fold_left
    ~f:(fun acc (dr, dc) ->
      match build_word 4 row col dr dc with
      | Some word -> if word = "XMAS" then acc + 1 else acc
      | None -> acc)
    ~init:0 xmas_search_matrix

(* part 1 *)
let () =
  let rows = Array.length word_search in
  let cols = Array.length word_search.(0) in
  let count = ref 0 in
  for i = 0 to rows - 1 do
    for j = 0 to cols - 1 do
      count := !count + count_xmas i j
    done
  done;
  print_endline @@ string_of_int !count

let is_x_mas row col =
  let is_mas = function
    | "MAS" | "SAM" -> true
    | _ -> false
  in
  match (build_word 3 (row - 1) (col - 1) 1 1, build_word 3 (row + 1) (col - 1) (-1) 1) with
  | Some w1, Some w2 -> is_mas w1 && is_mas w2
  | _ -> false

(* part 2 *)
let () =
  let rows = Array.length word_search in
  let cols = Array.length word_search.(0) in
  let count = ref 0 in
  for i = 0 to rows - 1 do
    for j = 0 to cols - 1 do
      if is_x_mas i j then incr count
    done
  done;
  print_endline @@ string_of_int !count
