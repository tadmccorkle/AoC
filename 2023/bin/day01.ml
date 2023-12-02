let input = Aoc.read_input 1

(* part 1 *)
let zero = int_of_char '0'

let int_of_char_opt c =
  match c with
  | '0' .. '9' -> Some (int_of_char c - zero)
  | _ -> None

let cal_digs a c =
  match a with
  | Some i, Some j -> (
      match int_of_char_opt c with
      | Some k -> (Some i, Some k)
      | None -> (Some i, Some j))
  | _ ->
      let n = int_of_char_opt c in
      (n, n)

let rec sum values =
  match values with
  | [] -> 0
  | line :: rest -> (
      match String.fold_left cal_digs (None, None) line with
      | Some a, Some b -> (10 * a) + b + sum rest
      | _ -> raise (Failure "Failed to read input"))

let () = print_endline (string_of_int (sum input))

(* part 2 *)
let r = Str.regexp {|[0-9]\|one\|two\|three\|four\|five\|six\|seven\|eight\|nine|}
let nums = [ "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine" ]

let rec find_index l x =
  match l with
  | [] -> raise Not_found
  | hd :: rest -> if hd = x then 0 else 1 + find_index rest x

let int_of_num num =
  try 1 + find_index nums num with
  | Not_found -> int_of_string num

let rec sum_2 values =
  match values with
  | [] -> 0
  | line :: rest -> (
      try
        let _ = Str.search_forward r line 0 in
        let a = int_of_num (Str.matched_string line) in
        let len = String.length line in
        let _ = Str.search_backward r line (len - 1) in
        let b = int_of_num (Str.matched_string line) in
        (10 * a) + b + sum_2 rest
      with
      | Not_found -> raise (Failure "Failed to read input"))

let () = print_endline (string_of_int (sum_2 input))
