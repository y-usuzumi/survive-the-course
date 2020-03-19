(* Dan Grossman, Coursera PL, HW2 Provided Code *)

(* if you use this function to compare two strings (returns true if the same
   string), then you avoid several of the functions in problem 1 having
   polymorphic types that may be confusing *)
fun same_string(s1 : string, s2 : string) =
    s1 = s2

(* put your solutions for problem 1 here *)
fun all_except_option(s : string, ss : string list) =
    case ss of
        [] => NONE
      | x::xs => if same_string(s, x)
                 then SOME xs
                 else case all_except_option(s, xs) of
                          NONE => NONE
                       |  SOME xs' => SOME (x::xs')

fun get_substitutions1(substitutions : string list list, s : string) =
    case substitutions of
        [] => []
      | hd::tl => case all_except_option(s, hd) of
                        SOME xs => xs @ get_substitutions1(tl, s)
                      | NONE => get_substitutions1(tl, s)

fun get_substitutions2(substitutions : string list list, s : string) =
    let
        fun aux(substitutions, acc) =
            case substitutions of
                [] => acc
              | hd::tl => case all_except_option(s, hd) of
                              SOME xs => aux(tl, acc @ xs)
                            | NONE => aux(tl, acc)
    in
        aux(substitutions, [])
    end

fun similar_names(substitutions : string list list, {first, middle, last}) =
    let
        fun fullnames_from_substitutions(substitutions) =
            case substitutions of
                [] => []
              | x::xs => {first=x, middle=middle, last=last}::fullnames_from_substitutions(xs)
    in
        {first=first, middle=middle, last=last}::fullnames_from_substitutions(get_substitutions2(substitutions, first))
    end

(* you may assume that Num is always used with values 2, 3, ..., 10
   though it will not really come up *)
datatype suit = Clubs | Diamonds | Hearts | Spades
datatype rank = Jack | Queen | King | Ace | Num of int
type card = suit * rank

datatype color = Red | Black
datatype move = Discard of card | Draw

exception IllegalMove

(* put your solutions for problem 2 here *)
fun card_color (suit, _) =
    case suit of
        Clubs => Black
      | Diamonds => Red
      | Hearts => Red
      | Spades => Black

fun card_value (_, rank) =
    case rank of
        Ace => 11
      | Num n => n
      | _ => 10

fun remove_card(cs, c : card, e) =
    case cs of
        [] =>  raise e
      | (x::xs) => if x = c then xs else x::remove_card(xs, c, e)

fun all_same_color cs =
    case cs of
        [] => true
      | _::[] => true
      | c::c'::cs' => card_color c = card_color c' andalso
                      all_same_color(c'::cs')

fun sum_cards cs =
    let
        fun aux(cs, acc) =
            case cs of
                [] => acc
              | c::cs' => aux(cs', card_value c + acc)
    in
        aux(cs, 0)
    end

fun score(cs, goal) =
    let
        val sum = sum_cards cs
        val preliminary = if sum > goal then 3 * (sum - goal) else goal - sum
    in
        if not(all_same_color cs) then preliminary else preliminary div 2
    end

fun officiate(cs, moves, goal) =
    let
        fun officiate_with_held_cards(cs, moves, held_cards) =
            case moves of
                [] => score(held_cards, goal)
              | Discard c::moves' => officiate_with_held_cards(cs, moves', remove_card(held_cards, c, IllegalMove))
              | Draw::moves' => case cs of
                            [] => score(held_cards, goal)
                          | c::cs' => let
                              val new_held_cards = c::held_cards
                          in
                              if sum_cards new_held_cards > goal
                              then score(new_held_cards, goal)
                              else officiate_with_held_cards(cs', moves', new_held_cards)
                          end
    in
        officiate_with_held_cards(cs, moves, [])
    end

fun score_challenge(cs, goal) =
    let
        fun calc_aces cs =
            let
                fun aux(cs, acc) =
                    case cs of
                        [] => acc
                      | (_, Ace)::cs' => aux(cs', acc+1)
                      | _::cs' => aux(cs', acc)
            in
                aux(cs, 0)
            end
        val sum = sum_cards cs
        val aces = calc_aces cs
        fun calc_preliminary sum =
            if sum > goal then 3 * (sum - goal) else goal - sum
        fun calc_minimum_preliminary(sum, aces) =
            case aces of
                0 => calc_preliminary sum
              | n => Int.min(calc_preliminary sum, calc_minimum_preliminary (sum - 10, aces - 1))
        val preliminary = calc_minimum_preliminary(sum, aces)
    in
        if not(all_same_color cs) then preliminary else preliminary div 2
    end

fun careful_player(cs, goal) =
    let
        fun find_discard_draw_target(draw_card, held_cards) =
            let
                fun aux(draw_card, left_slice, right_slice) =
                    case right_slice of
                        [] => NONE
                      | c::cs' => if score(left_slice @ [draw_card] @ cs', goal) = 0
                                  then SOME c
                                  else aux(draw_card, c::left_slice, cs')
            in
                 aux(draw_card, [], held_cards)
            end
        fun aux(cs, held_cards, reversed_moves) =
            let
                val curr_score = score (held_cards, goal)
            in
                if curr_score = 0
                then reversed_moves
                else
                    case cs of
                        [] => reversed_moves
                      | c::cs' =>
                        let
                            val curr_sum = sum_cards held_cards
                        in if goal > curr_sum + 10
                           then aux(cs', c::held_cards, Draw::reversed_moves)
                           else case find_discard_draw_target(c, held_cards) of
                                    NONE => reversed_moves
                                  | SOME c => Draw::Discard c::reversed_moves
                        end
            end
        val reversed_moves = aux(cs, [], [])
        fun reverse(curr, acc) =
            case curr of
                [] => acc
              | c::cs => reverse(cs, c::acc)
    in
        reverse(reversed_moves, [])
    end
