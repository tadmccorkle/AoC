let name = "Advent of Code 2023, OCaml Edition"

let left_pad day = if day < 10 then "0" ^ string_of_int day else string_of_int day

let read_input day =
  let file_path = "./input/day" ^ left_pad day ^ ".txt" in
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
