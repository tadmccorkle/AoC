let input = Aoc.read_input 1
let zero = int_of_char '0'

let int_of_char_opt c =
  match c with
  | '0' .. '9' -> Some (int_of_char c - zero)
  | _ -> None
;;

let cal_digs a c =
  match a with
  | Some i, Some j ->
    (match int_of_char_opt c with
     | Some k -> Some i, Some k
     | None -> Some i, Some j)
  | _ ->
    let n = int_of_char_opt c in
    n, n
;;

let rec sum values =
  match values with
  | [] -> 0
  | line :: rest ->
    (match String.fold_left cal_digs (None, None) line with
     | Some a, Some b -> (10 * a) + b + sum rest
     | _ -> 0)
;;

(* part 1 *)
let () = print_endline (string_of_int (sum input))

