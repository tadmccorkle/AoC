open StdLabels

(* let input = Aoc.read_sample_lines () *)
let input = Aoc.read_input_lines 1

let parse_rot rot = String.sub rot ~pos:1 ~len:(String.length rot - 1) |> int_of_string

let rotate pos rot =
  pos
  +
  match rot.[0] with
  | 'L' -> 100 - parse_rot rot
  | _ -> parse_rot rot

let rec solve rots pos =
  (if pos = 0 then 1 else 0)
  +
  match rots with
  | [] -> 0
  | rot :: rest -> solve rest (rotate pos rot mod 100)

(* part 1 *)
let () =
  let t0 = Unix.gettimeofday () in
  let ans = solve input 50 in
  let t1 = Unix.gettimeofday () in
  Printf.printf "P1: %d (%f s)\n" ans (t1 -. t0)

let pos_mod a b = ((a mod b) + b) mod b

let rec solve rots pos =
  match rots with
  | [] -> 0
  | rot :: rest -> (
      match rot.[0] with
      | 'L' ->
          let zeroes, p =
            match (pos - parse_rot rot, pos) with
            | 0, prev when prev > 0 -> (1, 0)
            | p, prev when p < 0 -> (((abs p / 100) + if prev > 0 then 1 else 0), pos_mod p 100)
            | p, _ -> (0, p)
          in
          zeroes + solve rest p
      | _ ->
          let p = pos + parse_rot rot in
          (p / 100) + solve rest (p mod 100))

(* part 2 *)
let () =
  let t0 = Unix.gettimeofday () in
  let ans = solve input 50 in
  let t1 = Unix.gettimeofday () in
  Printf.printf "P2: %d (%f s)\n" ans (t1 -. t0)
