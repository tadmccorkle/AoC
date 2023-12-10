open StdLabels

let input = Aoc.read_input 9

let split_to_ints s = s |> String.split_on_char ~sep:' ' |> List.map ~f:int_of_string

let rec differences seq =
  match seq with
  | [] -> Aoc.raise_input_failure ()
  | [ hd; last ] -> [ last - hd ]
  | hd :: rest -> (List.hd rest - hd) :: differences rest

let extrapolate seq =
  let rec extrapolate seq =
    let diffs = differences seq in
    if List.for_all diffs ~f:(fun d -> d = 0) then 0
    else (List.hd @@ List.rev diffs) + extrapolate diffs
  in
  (List.hd @@ List.rev seq) + extrapolate seq

(* part 1 *)
let () =
  let seqs = input |> List.map ~f:split_to_ints in
  let predictions = List.map seqs ~f:extrapolate in
  let sum = List.fold_left predictions ~f:(fun sum p -> sum + p) ~init:0 in
  print_endline @@ string_of_int sum

let extrapolate_backwards seq =
  let rec extrapolate seq =
    let diffs = differences seq in
    if List.for_all diffs ~f:(fun d -> d = 0) then 0 else List.hd diffs - extrapolate diffs
  in
  List.hd seq - extrapolate seq

(* part 2 *)
let () =
  let seqs = input |> List.map ~f:split_to_ints in
  let history = List.map seqs ~f:extrapolate_backwards in
  let sum = List.fold_left history ~f:(fun sum p -> sum + p) ~init:0 in
  print_endline @@ string_of_int sum
