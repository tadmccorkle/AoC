let name = "Advent of Code 2024, OCaml Edition"

let left_pad day = if day < 10 then "0" ^ string_of_int day else string_of_int day

let input_file_path day = "./input/day" ^ left_pad day ^ ".txt"

let read_input day =
  let file_path = input_file_path day in
  let channel = open_in file_path in
  let content =
    try really_input_string channel (in_channel_length channel) with
    | End_of_file ->
        close_in channel;
        raise (Failure "Error reading input")
  in
  close_in channel;
  content

let read_input_lines day =
  let file_path = input_file_path day in
  let channel = open_in file_path in
  let rec read_lines acc =
    try
      let line = input_line channel in
      read_lines (line :: acc)
    with
    | End_of_file ->
        close_in channel;
        List.rev acc
  in
  read_lines []

let raise_input_failure () = raise (Failure "Error reading input")

let rec print_all lines =
  match lines with
  | [] -> ()
  | x :: rest ->
      print_endline x;
      print_all rest

(* inclusive range [a,b] *)
let range a b =
  let rec range a b acc = if b < a then acc else range a (b - 1) (b :: acc) in
  range a b []
