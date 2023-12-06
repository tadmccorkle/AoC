let input = Aoc.read_input 3

let sum_list l = List.fold_left (fun sum x -> sum + x) 0 l

(* inclusive range [a,b] *)
let ( -- ) a b =
  let rec range a b acc = if b < a then acc else range a (b - 1) (b :: acc) in
  range a b []

(* search a line for the start of a number *)
let rec search_num_start line idx =
  if idx < String.length line then
    match line.[idx] with
    | '0' .. '9' -> Some idx
    | _ -> search_num_start line (idx + 1)
  else None

(* find the inclusive start index of a number *)
let rec find_num_start line idx =
  if idx > 0 then
    match line.[idx] with
    | '0' .. '9' -> find_num_start line (idx - 1)
    | _ -> idx + 1
  else idx

(* find the exclusive start index of a number *)
let rec find_num_end line idx =
  if idx < String.length line then
    match line.[idx] with
    | '0' .. '9' -> find_num_end line (idx + 1)
    | _ -> idx
  else idx

let is_symbol c =
  match c with
  | '0' .. '9'
  | '.' ->
      false
  | _ -> true

let is_part_number row col len =
  let start_row = max 0 (row - 1) in
  let start_col = max 0 (col - 1) in
  let end_row = min (List.length input - 1) (row + 1) in
  let end_col = min (String.length (List.hd input) - 1) (col + len) in
  let rec symbol_exists range =
    match range with
    | [] -> false
    | r :: rest ->
        let sub = String.sub (List.nth input r) start_col (end_col - start_col + 1) in
        String.exists is_symbol sub || symbol_exists rest
  in
  symbol_exists (start_row -- end_row)

let rec collect_part_numbers row ?(col = 0) line =
  match search_num_start line col with
  | Some n_start ->
      let n_end = find_num_end line (n_start + 1) in
      let len = n_end - n_start in
      if is_part_number row n_start len then
        let pn = String.sub line n_start len in
        int_of_string pn :: collect_part_numbers row line ~col:n_end
      else collect_part_numbers row line ~col:n_end
  | None -> []

(* part 1 *)
let () =
  let part_numbers = List.mapi (fun i line -> collect_part_numbers i line) input in
  let sum = List.fold_left (fun sum part_numbers -> sum + sum_list part_numbers) 0 part_numbers in
  print_endline (string_of_int sum)

module SchematicNumber = struct
  type t = int * int * string
  let compare (r0, c0, n0) (r1, c1, n1) =
    match Stdlib.compare r0 r1 with
    | 0 -> (
        match Stdlib.compare c0 c1 with
        | 0 -> Stdlib.compare n0 n1
        | c -> c)
    | c -> c
end

module SNSet = Set.Make (SchematicNumber)

let rec collect_gear_ratios row ?(col = 0) line =
  if col < String.length line then
    if line.[col] = '*' then
      let r0 = max 0 (row - 1) in
      let c0 = max 0 (col - 1) in
      let r1 = min (List.length input - 1) (row + 1) in
      let c1 = min (String.length line - 1) (col + 1) in
      let schematic_numbers =
        List.fold_left
          (fun nums r ->
            let line = List.nth input r in
            List.fold_left
              (fun nums c ->
                match line.[c] with
                | '0' .. '9' ->
                    let num_start = find_num_start line c in
                    let num_end = find_num_end line (c + 1) in
                    let num = String.sub line num_start (num_end - num_start) in
                    (num_start, num_end, num) :: nums
                | _ -> nums)
              nums (c0 -- c1))
          [] (r0 -- r1)
      in
      match SNSet.elements (SNSet.of_list schematic_numbers) with
      | [ (_, _, n1); (_, _, n2) ] ->
          let ratio = int_of_string n1 * int_of_string n2 in
          ratio :: collect_gear_ratios row line ~col:(col + 1)
      | _ -> collect_gear_ratios row line ~col:(col + 1)
    else collect_gear_ratios row line ~col:(col + 1)
  else []

(* part 2 *)
let () =
  let gear_ratios = List.mapi (fun i line -> collect_gear_ratios i line) input in
  let sum = List.fold_left (fun sum line_ratios -> sum + sum_list line_ratios) 0 gear_ratios in
  print_endline (string_of_int sum)
