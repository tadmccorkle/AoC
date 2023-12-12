open StdLabels

let input = Aoc.read_input 8

type lr = L | R

type lr_instruction = { value : lr; mutable next : lr_instruction }

let to_instructions s =
  let len = String.length s in
  let rec tmp_next = { value = L; next = tmp_next } in
  let rec to_instructions prev i =
    if i = len then []
    else
      let curr =
        match s.[i] with
        | 'L' -> { value = L; next = tmp_next }
        | 'R' -> { value = R; next = tmp_next }
        | _ -> Aoc.raise_input_failure ()
      in
      prev.next <- curr;
      curr :: to_instructions curr (i + 1)
  in
  let instr = to_instructions tmp_next 0 in
  (List.hd @@ List.rev instr).next <- List.hd instr;
  List.hd instr

module NetworkMap = Map.Make (String)

let parse_map () =
  match input with
  | _ :: _ :: map_lines ->
      List.fold_left map_lines
        ~f:(fun map line ->
          let split_line = String.split_on_char line ~sep:'=' |> List.map ~f:String.trim in
          let key = List.hd split_line in
          let lr = List.nth split_line 1 in
          let l = String.sub lr ~pos:1 ~len:3 in
          let r = String.sub lr ~pos:6 ~len:3 in
          NetworkMap.add key (l, r) map)
        ~init:NetworkMap.empty
  | _ -> Aoc.raise_input_failure ()

let rec navigate is_end_f map element instructions step =
  if is_end_f element then step
  else
    let l, r = NetworkMap.find element map in
    let next =
      match instructions.value with
      | L -> l
      | R -> r
    in
    navigate is_end_f map next instructions.next (step + 1)

let instructions = List.hd input |> to_instructions
let network_map = parse_map ()

(* part 1 *)
let () =
  let steps_to_zzz = navigate (fun e -> e = "ZZZ") network_map "AAA" instructions 0 in
  print_endline @@ string_of_int steps_to_zzz

let gcd a b =
  let rec gcd a b = if b = 0 then a else gcd b (a mod b) in
  gcd (max a b) (min a b)

let lcm a b = a * b / gcd a b

(* part 2 *)
let () =
  let starting_elements =
    NetworkMap.bindings network_map
    |> List.filter ~f:(fun (e, _) -> String.ends_with e ~suffix:"A")
    |> List.map ~f:(fun (e, _) -> e)
  in
  let min_steps =
    List.map starting_elements ~f:(fun e ->
        navigate (String.ends_with ~suffix:"Z") network_map e instructions 0)
  in
  let steps = List.fold_left min_steps ~f:lcm ~init:1 in
  print_endline @@ string_of_int steps
