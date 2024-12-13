open StdLabels

let input = Aoc.read_input_lines 7

let total_cal_result possible_results =
  List.fold_left
    ~f:(fun acc eq ->
      let idx = String.index eq ':' in
      let ans = int_of_string (String.sub eq ~pos:0 ~len:idx) in
      let operands = String.sub eq ~pos:(idx + 2) ~len:(String.length eq - idx - 2) in
      let operands = List.map ~f:int_of_string (String.split_on_char ~sep:' ' operands) in
      let is_possible = operands |> possible_results |> List.exists ~f:(fun x -> x = ans) in
      if is_possible then acc + ans else acc)
    ~init:0 input

(* part 1 *)
let () =
  let result =
    total_cal_result (fun operands ->
        let rec aux curr remaining results =
          match remaining with
          | [] -> curr :: results
          | x :: xs -> aux (curr + x) xs (aux (curr * x) xs results)
        in
        aux (List.hd operands) (List.tl operands) [])
  in
  print_endline @@ string_of_int result

let concat n1 n2 = (n1 * int_of_float (10. ** (floor (log10 (float n2)) +. 1.))) + n2

(* part 2 *)
let () =
  let result =
    total_cal_result (fun operands ->
        let rec aux curr remaining results =
          match remaining with
          | [] -> curr :: results
          | x :: xs -> aux (curr + x) xs (aux (curr * x) xs (aux (concat curr x) xs results))
        in
        aux (List.hd operands) (List.tl operands) [])
  in
  print_endline @@ string_of_int result
