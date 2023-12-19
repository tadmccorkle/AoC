open StdLabels

type point = { x : int; y : int }
type direction = N | E | S | W
type pipe = U of char | R of char | D of char | L of char

let find_start lines =
  let rec fs lines row =
    match lines with
    | [] -> Aoc.raise_input_failure ()
    | hd :: rest -> (
        match String.index_opt hd 'S' with
        | Some s -> { x = s; y = row }
        | None -> fs rest (row + 1))
  in
  fs lines 0

let input = Aoc.read_input 10
let maze = Array.of_list input
let start = find_start input

let get_char pt = String.get (Array.get maze pt.y) pt.x

let start_connects pt dir =
  match get_char pt with
  | '|' -> dir = N || dir = S
  | '-' -> dir = E || dir = W
  | 'L' -> dir = S || dir = W
  | 'J' -> dir = E || dir = S
  | '7' -> dir = N || dir = E
  | 'F' -> dir = N || dir = W
  | _ -> Aoc.raise_input_failure ()

let rec traverse start curr_pipe curr_pt step =
  if start = curr_pt then step
  else
    let next_pt, dir =
      match curr_pipe with
      | U '|'
      | R 'J'
      | L 'L' ->
          ({ curr_pt with y = curr_pt.y - 1 }, N)
      | U 'F'
      | R '-'
      | D 'L' ->
          ({ curr_pt with x = curr_pt.x + 1 }, E)
      | R '7'
      | D '|'
      | L 'F' ->
          ({ curr_pt with y = curr_pt.y + 1 }, S)
      | U '7'
      | D 'J'
      | L '-' ->
          ({ curr_pt with x = curr_pt.x - 1 }, W)
      | _ -> Aoc.raise_input_failure ()
    in
    let next_char = get_char next_pt in
    let next_pipe =
      match dir with
      | N -> U next_char
      | E -> R next_char
      | S -> D next_char
      | W -> L next_char
    in
    traverse start next_pipe next_pt (step + 1)

let () =
  let rows = Array.length maze in
  let cols = String.length (List.hd input) in
  let north = { start with y = start.y - 1 } in
  let east = { start with x = start.x + 1 } in
  let south = { start with y = start.y + 1 } in
  let west = { start with x = start.x - 1 } in
  let next_pipe, next_pt =
    if north.y >= 0 && start_connects north N then (U (get_char north), north)
    else if east.x < cols && start_connects east E then (R (get_char east), east)
    else if south.y < rows && start_connects south S then (D (get_char south), south)
    else if west.x >= 0 && start_connects west W then (L (get_char west), west)
    else Aoc.raise_input_failure ()
  in
  let steps = traverse start next_pipe next_pt 1 in
  print_endline @@ string_of_int (steps / 2)
