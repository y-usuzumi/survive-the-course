(* So I was thinking that if the structure implements the abstract type as a
 * type synonym, then it might be possible that we can use the abstract type
 * in its underlying form. Turns out no, the ML type-checker is more
 * robust than I thought. *)

signature SIG = sig
    type rational
    val make_rational : int * int -> rational
end


structure Sig :> SIG = struct
type rational = (int * int)

fun make_rational (v : Sig.rational) =
    Int.toString (#1 v) ^ ", " ^ Int.toString (#2 v) (* does not type-check *)
end
