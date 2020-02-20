use "solution.sml";


val t_alternate_1 = alternate([1, 2, 3, 4]) (* ~2 *)

val t_min_max_1 = min_max([1, 3, 5, 8, 6, 4]) (* (1, 8) *)

val t_cum_sum_1 = cum_sum([]) (* [] *)
val t_cum_sum_2 = cum_sum([1, 2, 3, 4]) (* [1, 3, 6, 10 ] *)

val t_greeting_1 = greeting(SOME "二狗") (* "Hello there, 二狗!" *)
val t_greeting_2 = greeting(NONE) (* "Hello there, you!" *)

val t_repeat_1 = repeat([1, 2, 3], [4, 0, 3]) (* [1, 1, 1, 1, 3, 3, 3] *)

val t_addOpt_1 = addOpt(NONE, SOME 3) (* NONE *)
val t_addOpt_2 = addOpt(SOME 5, SOME 3) (* SOME 8 *)
val t_addOpt_3 = addOpt(SOME 5, NONE) (* NONE *)

val t_addAllOpt_1 = addAllOpt([]) (* NONE *)
val t_addAllOpt_2 = addAllOpt([NONE, SOME 3, NONE, SOME 4]) (* SOME 7 *)
val t_addAllOpt_3 = addAllOpt([NONE, NONE, NONE]) (* NONE *)

val t_any_1 = any([]) (* false *)
val t_any_2 = any([false, false, true, false]) (* true *)
val t_any_3 = any([false, false, false, false]) (* false *)

val t_all_1 = all([]) (* true *)
val t_all_2 = all([false, false, true, false]) (* false *)
val t_all_3 = all([true, true, true, true]) (* true *)

val t_zip_1 = zip([1, 2, 3], [4, 5, 6, 7]) (* [(1, 4), (2, 5), (3, 6)] *)

(* [(1, 1), (2, 2), (3, 3), (1, 4), (2, 5), (3, 6), (1, 7)] *)
val t_zipReycle_1 = zipRecycle([1, 2, 3], [1, 2, 3, 4, 5, 6, 7])
(* [(1, 1), (2, 2), (3, 3), (4, 1), (5, 2), (6, 3), (7, 1)] *)
val t_zipReycle_2 = zipRecycle([1, 2, 3, 4, 5, 6, 7], [1, 2, 3])

(* [(1, 1), (2, 2), (3, 3)] *)
val t_zipRecycle_3 = zipRecycle([1, 2, 3], [1, 2, 3])
