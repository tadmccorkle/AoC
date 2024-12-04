open StdLabels

let input = Aoc.read_input 3

type instruction =
  | NOP
  | MUL of char
  | LHS of (int * int)
  | RHS of (int * int * int)
  | PROD of int
  | DO of char
  | DONT of char
  | ON
  | OFF

let digit_to_int d = int_of_char d - int_of_char '0'
let add_digit value d = (value * 10) + digit_to_int d

let parse state input =
  if input = 'm' then MUL input
  else if input = 'd' then DO input
  else
    match state with
    | MUL pc -> (
        match (pc, input) with
        | 'm', 'u'
        | 'u', 'l' ->
            MUL input
        | 'l', '(' -> LHS (0, 0)
        | _ -> NOP)
    | LHS (ct, lhs) -> (
        match input with
        | '0' .. '9' -> if ct < 3 then LHS (ct + 1, add_digit lhs input) else NOP
        | ',' -> if ct > 0 then RHS (lhs, 0, 0) else NOP
        | _ -> NOP)
    | RHS (lhs, ct, rhs) -> (
        match input with
        | '0' .. '9' -> if ct < 3 then RHS (lhs, ct + 1, add_digit rhs input) else NOP
        | ')' -> if ct > 0 then PROD (lhs * rhs) else NOP
        | _ -> NOP)
    | DO pc -> (
        match (pc, input) with
        | 'd', 'o'
        | 'o', '(' ->
            DO input
        | '(', ')' -> ON
        | 'o', 'n' -> DONT input
        | _ -> NOP)
    | DONT pc -> (
        match (pc, input) with
        | 'n', '\''
        | '\'', 't'
        | 't', '(' ->
            DONT input
        | '(', ')' -> OFF
        | _ -> NOP)
    | _ -> NOP

(* part 1 *)
let () =
  let _, sum =
    String.fold_left
      ~f:(fun (last_instr, sum) next_ch ->
        match parse last_instr next_ch with
        | PROD p -> (NOP, sum + p)
        | res -> (res, sum))
      ~init:(NOP, 0) input
  in
  print_endline @@ string_of_int sum

(* part 2 *)
let () =
  let _, _, sum =
    String.fold_left
      ~f:(fun (last_instr, enabled, sum) next_ch ->
        let instr = parse last_instr next_ch in
        match instr with
        | PROD p -> (NOP, enabled, if enabled then sum + p else sum)
        | ON
        | OFF ->
            (NOP, instr = ON, sum)
        | res -> (res, enabled, sum))
      ~init:(NOP, true, 0) input
  in
  print_endline @@ string_of_int sum
