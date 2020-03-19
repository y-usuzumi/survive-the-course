(* Coursera Programming Languages, Homework 3, Provided Code *)

exception NoAnswer

datatype pattern = Wildcard
		             | Variable of string
		             | UnitP
		             | ConstP of int
		             | TupleP of pattern list
		             | ConstructorP of string * pattern

datatype valu = Const of int
	            | Unit
	            | Tuple of valu list
	            | Constructor of string * valu

fun g f1 f2 p =
    let
	      val r = g f1 f2
    in
	      case p of
	          Wildcard          => f1 ()
	        | Variable x        => f2 x
	        | TupleP ps         => List.foldl (fn (p,i) => (r p) + i) 0 ps
	        | ConstructorP(_,p) => r p
	        | _                 => 0
    end

(**** for the challenge problem only ****)

datatype typ = Anything
	           | UnitT
	           | IntT
	           | TupleT of typ list
	           | Datatype of string

(**** you can put all your code here ****)

val only_capitals = List.filter (Char.isUpper o (fn s => String.sub(s, 0)))

val longest_string1 : string list -> string =
    foldl (fn (elem, acc) => if String.size elem > String.size acc then elem else acc) ""

val longest_string2 : string list -> string =
    foldl (fn (elem, acc) => if String.size elem >= String.size acc then elem else acc) ""

fun longest_string_helper pred = foldl (fn (elem, acc) => if pred(String.size elem, String.size acc) then elem else acc) ""

val longest_string3 = longest_string_helper (fn (left, right) => left > right)

val longest_string4 = longest_string_helper (fn (left, right) => left >= right)

val longest_capitalized = foldl (fn (elem, acc) => if String.size elem > String.size acc then elem else acc) "" o only_capitals

val rev_string = implode o rev o explode

fun first_answer test xs =
    case xs of
        [] => raise NoAnswer
      | x::xs => case test x of
                     SOME v => v
                   | NONE => first_answer test xs

fun all_answers f xs =
    case xs of
        [] => SOME []
      | x::xs => case f x of
                     NONE => NONE
                   | SOME curr_ans => case all_answers f xs of
                                         NONE => NONE
                                       | SOME other_ans => SOME (curr_ans @ other_ans)

val count_wildcards = g (fn _ => 1) (fn _ => 0)

val count_wild_and_variable_lengths = g (fn _ => 1) String.size

fun count_some_var(s, pat) = g (fn _ => 0) (fn var => if var = s then 1 else 0) pat

val check_pat =
    let
        fun extract_vars p =
            case p of
                Variable x => [x]
              | TupleP xs => foldl (fn (p, acc) => acc @ extract_vars p) [] xs
              | ConstructorP(_, p) => extract_vars p
              | _ => []

        fun distinct pats =
            case pats of
                [] => true
              | x::xs => not (List.exists (fn elem => elem = x) xs) andalso distinct xs

    in
        distinct o extract_vars
    end

fun match(v, p) =
    case (v, p) of
        (_, Wildcard) => SOME []
      | (v, Variable(var)) => SOME [(var, v)]
      | (Unit, UnitP) => SOME []
      | (Const(v), ConstP(p)) => if v = p then SOME [] else NONE
      | (Tuple(v), TupleP(p)) => all_answers (fn (v, p) => match(v, p)) (ListPair.zip (v, p))
      | (Constructor(s1, v), ConstructorP(s2, p)) => if s1 = s2 then match(v, p) else NONE
      | _ => NONE

fun first_match v ps =
    SOME (first_answer (fn p => match(v, p)) ps)
    handle NoAnswer => NONE
