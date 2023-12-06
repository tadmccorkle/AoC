open StdLabels

let input = Aoc.read_input 1

(* inclusive range [a,b] *)
let ( -- ) a b =
  let rec range a b acc = if b < a then acc else range a (b - 1) (b :: acc) in
  range a b []

let zero = int_of_char '0'

let int_of_char_opt c =
  match c with
  | '0' .. '9' -> Some (int_of_char c - zero)
  | _ -> None

(* part 1 *)
let () =
  let get_cal_digits line =
    String.fold_left
      ~f:(fun digits c ->
        match digits with
        | Some i, Some j -> (
            match int_of_char_opt c with
            | Some k -> (Some i, Some k)
            | None -> (Some i, Some j))
        | _ ->
            let n = int_of_char_opt c in
            (n, n))
      ~init:(None, None) line
  in
  let get_cal_val line =
    match get_cal_digits line with
    | Some a, Some b -> (10 * a) + b
    | _ -> Aoc.raise_input_failure ()
  in
  let sum = List.fold_left ~f:(fun acc line -> acc + get_cal_val line) ~init:0 input in
  print_endline (string_of_int sum)

let nums =
  [
    "1";
    "2";
    "3";
    "4";
    "5";
    "6";
    "7";
    "8";
    "9";
    "one";
    "two";
    "three";
    "four";
    "five";
    "six";
    "seven";
    "eight";
    "nine";
  ]

let rec starts_with_index s l =
  match l with
  | [] -> raise Not_found
  | hd :: rest -> if String.starts_with s ~prefix:hd then 0 else 1 + starts_with_index s rest

(* part 2 *)
let () =
  let get_cal_digits line =
    let line_len = String.length line in
    List.fold_left
      ~f:(fun digits idx ->
        let len = min 5 (line_len - idx) in
        let sub = String.sub line ~pos:idx ~len in
        try
          let idx = starts_with_index sub nums in
          let digit = 1 + (idx mod 9) in
          match digits with
          | Some i, _ -> (Some i, Some digit)
          | _ -> (Some digit, Some digit)
        with
        | Not_found -> digits)
      ~init:(None, None) (0 -- line_len)
  in
  let get_cal_val line =
    match get_cal_digits line with
    | Some a, Some b -> (10 * a) + b
    | _ -> Aoc.raise_input_failure ()
  in
  let sum = List.fold_left ~f:(fun acc line -> acc + get_cal_val line) ~init:0 input in
  print_endline (string_of_int sum)
