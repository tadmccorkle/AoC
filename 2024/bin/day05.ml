open StdLabels

let input = Aoc.read_input_lines 5

module RuleMap = Map.Make (Int)

let rules, page_groups =
  let rec aux lines rules =
    match lines with
    | line :: xs ->
        if line = "" then (rules, xs)
        else
          let i = String.index line '|' in
          let lhs = int_of_string @@ String.sub ~pos:0 ~len:i line in
          let rhs =
            int_of_string @@ String.sub ~pos:(i + 1) ~len:(String.length line - (i + 1)) line
          in
          aux xs (RuleMap.add_to_list lhs rhs rules)
    | _ -> Aoc.raise_input_failure ()
  in
  let rules, remaining_input = aux input RuleMap.empty in
  let rec aux lines page_groups =
    match lines with
    | line :: xs when line <> "" ->
        let page_group = List.map ~f:int_of_string (String.split_on_char ~sep:',' line) in
        aux xs (page_group :: page_groups)
    | _ -> page_groups
  in
  (rules, aux remaining_input [])

let rec is_ordered page_group =
  match page_group with
  | page :: xs ->
      let page_ordered =
        match RuleMap.find_opt page rules with
        | Some page_rules ->
            List.for_all ~f:(fun p -> List.exists ~f:(fun r -> p = r) page_rules) xs
        | None -> true
      in
      if not page_ordered then false else is_ordered xs
  | [] -> true

let middle_item lst =
  let rec find_middle slow fast =
    match fast with
    | [] | [ _ ] -> List.hd slow
    | _ :: _ :: rest -> find_middle (List.tl slow) rest
  in
  match lst with
  | [] -> Aoc.raise_input_failure ()
  | _ -> find_middle lst lst

(* part 1 *)
let () =
  let sum =
    List.fold_left
      ~f:(fun acc page_group -> if is_ordered page_group then acc + middle_item page_group else acc)
      ~init:0 page_groups
  in
  print_endline @@ string_of_int sum

(* part 2 *)
let () =
  let sum =
    List.fold_left
      ~f:(fun acc page_group ->
        if not (is_ordered page_group) then
          let sorted_page_group =
            page_group
            |> List.sort ~cmp:(fun x y ->
                   match RuleMap.find_opt y rules with
                   | Some y_rules when List.exists ~f:(fun r -> r = x) y_rules -> 1
                   | _ -> -1)
          in
          acc + middle_item sorted_page_group
        else acc)
      ~init:0 page_groups
  in
  print_endline @@ string_of_int sum
