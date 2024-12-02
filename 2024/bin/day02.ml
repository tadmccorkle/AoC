open StdLabels

let input = Aoc.read_input 2

module ListEx = struct
  let count ~pred = List.fold_left ~f:(fun acc i -> if pred i then acc + 1 else acc) ~init:0

  (* gets all versions of a list with one of its items removed *)
  let without_item =
    let rec without acc list =
      match list with
      | [] -> []
      | hd :: rest -> List.rev_append acc rest :: without (hd :: acc) rest
    in
    without []
end

let levels report = String.split_on_char ~sep:' ' report |> List.map ~f:(fun l -> int_of_string l)

let diffs report =
  let rec acc_diffs curr other diffs =
    match other with
    | [] -> diffs
    | hd :: rest -> (hd - curr) :: acc_diffs hd rest diffs
  in
  acc_diffs (List.hd report) (List.tl report) []

let are_diffs_safe report_level_diffs =
  let sign i = if i = 0 then 0 else if i < 0 then -1 else 1 in
  let s = sign (List.hd report_level_diffs) in
  let within_range diff = diff <> 0 && diff >= -3 && diff <= 3 in
  List.for_all ~f:(fun diff -> sign diff = s && within_range diff) report_level_diffs

(* part 1 *)
let () =
  let is_safe report = report |> levels |> diffs |> are_diffs_safe in
  let safe_count = ListEx.count ~pred:is_safe input in
  print_endline @@ string_of_int safe_count

(* part 2 *)
let () =
  let either pred1 pred2 value = pred1 value || pred2 value in
  let is_undampened_safe levels = levels |> diffs |> are_diffs_safe in
  let is_dampened_safe levels = List.exists ~f:is_undampened_safe (ListEx.without_item levels) in
  let is_safe report = report |> levels |> either is_undampened_safe is_dampened_safe in
  let safe_count = ListEx.count ~pred:is_safe input in
  print_endline @@ string_of_int safe_count
