open StdLabels

let input = Aoc.read_input 7

type hand_type = High | One | Two | Three | Full | Four | Five

type hand = { hand : string; t : hand_type; bid : int }

module CharMap = Map.Make (Char)

let get_card_map hand =
  String.fold_left hand
    ~f:(fun cards c ->
      CharMap.update c
        (fun v ->
          match v with
          | Some x -> Some (x + 1)
          | None -> Some 1)
        cards)
    ~init:CharMap.empty

let parse_hands hand_type_f =
  List.fold_left input
    ~f:(fun hands line ->
      match line |> String.split_on_char ~sep:' ' with
      | [ hand; b ] ->
          let hand_type = hand_type_f hand in
          { hand; t = hand_type; bid = int_of_string b } :: hands
      | _ -> Aoc.raise_input_failure ())
    ~init:[]

let compare_hands labels a b =
  if a.t > b.t then 1
  else if a.t < b.t then -1
  else
    let rec cmp_strength i =
      try
        let a_idx = String.index labels a.hand.[i] in
        let b_idx = String.index labels b.hand.[i] in
        if a_idx < b_idx then 1 else if a_idx > b_idx then -1 else cmp_strength (i + 1)
      with
      | Not_found -> 0
    in
    cmp_strength 0

let get_hand_type hand =
  match CharMap.bindings (get_card_map hand) with
  | [ _ ] -> Five
  | [ (_, n1); (_, n2) ] -> if n1 = 4 || n2 = 4 then Four else Full
  | [ (_, n1); (_, n2); (_, n3) ] -> if n1 = 3 || n2 = 3 || n3 = 3 then Three else Two
  | [ _; _; _; _ ] -> One
  | _ -> High

let cards_labels = "AKQJT98765432"

(* part 1 *)
let () =
  let hands = parse_hands get_hand_type in
  let sorted = List.sort hands ~cmp:(compare_hands cards_labels) in
  let scored = List.mapi sorted ~f:(fun i h -> (i + 1) * h.bid) in
  let total = List.fold_left scored ~f:(fun sum s -> sum + s) ~init:0 in
  print_endline @@ string_of_int total

let get_joker_hand_type hand =
  let cards = get_card_map hand in
  let joker_count =
    try CharMap.find 'J' cards with
    | Not_found -> 0
  in
  match CharMap.bindings (get_card_map hand) with
  | [ _ ] -> Five
  | [ (_, n1); (_, n2) ] ->
      if joker_count > 0 then Five else if n1 = 4 || n2 = 4 then Four else Full
  | [ (_, n1); (_, n2); (_, n3) ] ->
      if n1 = 3 || n2 = 3 || n3 = 3 then if joker_count > 0 then Four else Three
      else if joker_count = 2 then Four
      else if joker_count = 1 then Full
      else Two
  | [ _; _; _; _ ] -> if joker_count > 0 then Three else One
  | _ -> if joker_count > 0 then One else High

let joker_cards_labels = "AKQT98765432J"

(* part 2 *)
let () =
  let hands = parse_hands get_joker_hand_type in
  let sorted = List.sort hands ~cmp:(compare_hands joker_cards_labels) in
  let scored = List.mapi sorted ~f:(fun i h -> (i + 1) * h.bid) in
  let total = List.fold_left scored ~f:(fun sum s -> sum + s) ~init:0 in
  print_endline @@ string_of_int total
