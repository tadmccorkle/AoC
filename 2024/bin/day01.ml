open StdLabels

let input = Aoc.read_input_lines 1

let split line = String.split_on_char ~sep:' ' line |> List.filter ~f:(fun s -> s <> "")

let rec split_list list l r =
  match list with
  | [] -> (l, r)
  | hd :: rest -> (
      match split hd with
      | [ a; b ] -> split_list rest (int_of_string a :: l) (int_of_string b :: r)
      | _ -> Aoc.raise_input_failure ())

let l, r = split_list input [] []

module IntList = struct
  let sum = List.fold_left ~init:0
  let sum2 = List.fold_left2 ~init:0
  let sort = List.sort ~cmp:compare
  let count ~value = sum ~f:(fun acc i -> if i == value then acc + 1 else acc)
end

(* part 1 *)
let () =
  let sum = IntList.sum2 ~f:(fun acc a b -> acc + abs (a - b)) (IntList.sort l) (IntList.sort r) in
  print_endline @@ string_of_int sum

(* part 2 *)
let () =
  let sum = IntList.sum ~f:(fun acc id -> acc + (id * IntList.count ~value:id r)) l in
  print_endline @@ string_of_int sum
