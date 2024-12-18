open StdLabels

let input = Aoc.read_input 9

let digit_to_int d = int_of_char d - int_of_char '0'

(* part 1 *)
let () =
  let space_id = -1 in

  let rec add_block ~id ~size blocks =
    if size <= 0 then blocks else add_block ~id ~size:(size - 1) (id :: blocks)
  in

  let blocks =
    let rec aux map id blocks =
      match map () with
      | Seq.Cons (size, xs) ->
          let space, xs =
            match xs () with
            | Seq.Cons (space, xs) -> (space, xs)
            | _ -> (0, Seq.empty)
          in
          aux xs (id + 1) (add_block ~id:space_id ~size:space (add_block ~id ~size blocks))
      | _ -> blocks
    in
    Array.of_list (List.rev (aux (Seq.map digit_to_int (String.to_seq input)) 0 []))
  in

  let rec swap_next r_idx =
    match try Some blocks.(r_idx) with Invalid_argument _ -> None with
    | None -> None
    | Some x ->
        if x <> space_id then (
          blocks.(r_idx) <- space_id;
          Some (x, r_idx))
        else swap_next (r_idx - 1)
  in

  let compact_blocks =
    let length = Array.length blocks in
    let r_idx = ref (length - 1) in
    (try
       for i = 0 to length - 1 do
         if blocks.(i) = space_id then
           if !r_idx <= i then raise Exit
           else
             match swap_next !r_idx with
             | Some (x, ri') ->
                 blocks.(i) <- x;
                 r_idx := ri'
             | None -> raise Exit
       done
     with Exit -> ());

    Seq.take !r_idx (Array.to_seq blocks)
  in

  let checksum, _ =
    Seq.fold_left (fun (acc, idx) id -> (acc + (idx * id), idx + 1)) (0, 0) compact_blocks
  in

  print_endline @@ string_of_int checksum

(* part 2 *)
let () =
  let compact_disk_map =
    let length = String.((length input / 2) + if length input mod 2 <> 0 then 1 else 0) in
    let map =
      Array.init length ~f:(fun id ->
          let idx = id * 2 in
          let size = String.get input idx |> digit_to_int in
          let space =
            match try String.get input (idx + 1) with Invalid_argument _ -> '\x00' with
            | '0' .. '9' as ch -> digit_to_int ch
            | _ -> 0
          in
          (id, size, space))
    in

    let rec find_space desired_size idx len =
      if idx >= len then None
      else
        let _, _, space = map.(idx) in
        if desired_size <= space then Some idx else find_space desired_size (idx + 1) len
    in

    let rec shift_right pos len =
      if len > 0 then (
        map.(pos + len) <- map.(pos + len - 1);
        shift_right pos (len - 1))
    in

    let move_left src dst =
      let src_id, src_size, src_space = map.(src) in
      let dst_id, dst_size, dst_space = map.(dst) in
      shift_right (dst + 1) (src - dst - 1);
      map.(dst) <- (dst_id, dst_size, 0);
      if src <> dst + 1 then (
        let lst_id, lst_size, lst_space = map.(src) in
        map.(dst + 1) <- (src_id, src_size, dst_space - src_size);
        map.(src) <- (lst_id, lst_size, lst_space + src_size + src_space))
      else map.(src) <- (src_id, src_size, dst_space + src_space)
    in

    let rec compact r_idx =
      if r_idx > 0 then
        let _, size, _ = map.(r_idx) in
        match find_space size 0 r_idx with
        | None -> compact (r_idx - 1)
        | Some idx ->
            move_left r_idx idx;
            compact r_idx
    in

    compact (length - 1);
    map
  in

  let checksum, _ =
    Array.fold_left
      ~f:(fun (sum, idx) (id, size, space) ->
        let idx_sum = List.fold_left ~f:( + ) ~init:0 (Aoc.range idx (idx + size - 1)) in
        (sum + (id * idx_sum), idx + size + space))
      ~init:(0, 0) compact_disk_map
  in

  print_endline @@ string_of_int checksum
