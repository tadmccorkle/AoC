open StdLabels

let input = Aoc.read_input 6

(* inclusive range [a,b] *)
let ( -- ) a b =
  let rec range a b acc = if b < a then acc else range a (b - 1) (b :: acc) in
  range a b []

type race = { time : int; distance : int }

let get_races () =
  match input with
  | [ l0; l1 ] ->
      let split_substring_to_ints s pos =
        String.sub s ~pos ~len:(String.length s - pos)
        |> String.split_on_char ~sep:' '
        |> List.filter ~f:(fun x -> x <> "")
        |> List.map ~f:int_of_string
      in
      List.map2
        ~f:(fun time distance -> { time; distance })
        (split_substring_to_ints l0 5) (split_substring_to_ints l1 9)
  | _ -> Aoc.raise_input_failure ()

let multiply l = l |> List.fold_left ~f:(fun prod x -> prod * x) ~init:1

(* part 1, initial implementation *)
let () =
  let races = get_races () in
  let win_ways =
    races
    |> List.map ~f:(fun r ->
           List.fold_left
             ~f:(fun acc t -> if (r.time - t) * t > r.distance then acc + 1 else acc)
             ~init:0 (0 -- r.time))
  in
  print_endline (string_of_int (multiply win_ways))

let get_poorly_kerned_race () =
  match input with
  | [ l0; l1 ] ->
      let substring_to_int s pos =
        String.sub s ~pos ~len:(String.length s - pos)
        |> String.split_on_char ~sep:' '
        |> List.filter ~f:(fun x -> x <> "")
        |> String.concat ~sep:"" |> int_of_string
      in
      { time = substring_to_int l0 5; distance = substring_to_int l1 9 }
  | _ -> Aoc.raise_input_failure ()

(* part 2, initial implementation *)
let () =
  let race = get_poorly_kerned_race () in
  let win_ways =
    0 -- race.time
    |> List.fold_left
         ~f:(fun acc t -> if (race.time - t) * t > race.distance then acc + 1 else acc)
         ~init:0
  in
  print_endline (string_of_int win_ways)

let get_unique_win_count race min_win_time = race.time - (2 * min_win_time) + 1

let get_min_win_time race =
  List.find ~f:(fun t -> (race.time - t) * t > race.distance) (0 -- (race.time / 2))

(* part 1, faster implementation *)
let () =
  let races = get_races () in
  let win_ways = races |> List.map ~f:(fun r -> get_unique_win_count r (get_min_win_time r)) in
  print_endline (string_of_int (multiply win_ways))

(* part 2, faster implementation *)
let () =
  let race = get_poorly_kerned_race () in
  let win_ways = get_unique_win_count race (get_min_win_time race) in
  print_endline (string_of_int win_ways)

let rec get_min_win_time_search race l r =
  let t = (l + r) / 2 in
  if l >= r then t
  else if (race.time - t) * t > race.distance then get_min_win_time_search race l t
  else get_min_win_time_search race (t + 1) r

(* part 1, binary search implementation *)
let () =
  let races = get_races () in
  let win_ways =
    races
    |> List.map ~f:(fun r ->
           let min_win_time = get_min_win_time_search r 0 r.time in
           get_unique_win_count r min_win_time)
  in
  print_endline (string_of_int (multiply win_ways))

(* part 2, binary search implementation *)
let () =
  let race = get_poorly_kerned_race () in
  let min_win_time = get_min_win_time_search race 0 race.time in
  let win_ways = get_unique_win_count race min_win_time in
  print_endline (string_of_int win_ways)
