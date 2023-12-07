open StdLabels

let input = Aoc.read_input 4

(* inclusive range [a,b] *)
let ( -- ) a b =
  let rec range a b acc = if b < a then acc else range a (b - 1) (b :: acc) in
  range a b []

module SSet = Set.Make (String)

let filter_blanks l = l |> List.filter ~f:(fun x -> x <> "")

let match_count card =
  let nums = List.nth (String.split_on_char ~sep:':' card) 1 in
  let winning, mine =
    match String.split_on_char ~sep:'|' nums with
    | [ n1; n2 ] ->
        let get_nums s = s |> String.split_on_char ~sep:' ' |> filter_blanks in
        (get_nums n1, get_nums n2)
    | _ -> Aoc.raise_input_failure ()
  in
  List.length (SSet.elements (SSet.inter (SSet.of_list winning) (SSet.of_list mine)))

(* part 1 *)
let () =
  let total_points =
    input
    |> List.fold_left
         ~f:(fun points card ->
           let ct = match_count card in
           if ct > 0 then points + int_of_float (2. ** float_of_int (ct - 1)) else points)
         ~init:0
  in
  print_endline (string_of_int total_points)

(* part 2: recursive solution (pretty slow, but how I first solved it) *)
let () =
  let card_points = input |> List.map ~f:match_count in

  let rec play idx points =
    if points > 0 then
      let copy_range = idx + 1 -- (idx + points) in
      let copy_count =
        copy_range |> List.fold_left ~f:(fun acc i -> acc + play i (List.nth card_points i)) ~init:0
      in
      copy_count + 1
    else 1
  in

  let sum l = l |> List.fold_left ~f:(fun sum x -> sum + x) ~init:0 in
  let total_cards = card_points |> List.mapi ~f:(fun i points -> play i points) |> sum in
  print_endline (string_of_int total_cards)

(* part 2: iterative solution *)
let () =
  let card_points = input |> List.map ~f:match_count in
  let card_counts = Array.make (List.length card_points) 1 in
  let get = Array.get card_counts in
  let set = Array.set card_counts in

  card_points
  |> List.iteri ~f:(fun i points ->
         if points > 0 then
           let card_count = get i in
           let copy_range = i + 1 -- (i + points) in
           copy_range |> List.iter ~f:(fun j -> set j (card_count + get j)));

  let sum a = a |> Array.fold_left ~f:(fun sum x -> sum + x) ~init:0 in
  let total_cards = sum card_counts in
  print_endline (string_of_int total_cards)
