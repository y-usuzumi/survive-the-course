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

val t_zipOpt_1 = zipOpt([1, 2, 3], [3, 4, 5]) (* SOME [(1, 3), (2, 4), (3, 5)] *)
val t_zipOpt_2 = zipOpt([1, 2, 3], [3, 4]) (* NONE *)
val t_zipOpt_3 = zipOpt([1, 2], [3, 4, 5]) (* NONE *)

val t_lookup_1 = lookup([("二狗", 1), ("三毛", 2)], "三毛") (* SOME 2 *)
val t_lookup_2 = lookup([("二狗", 1), ("三毛", 2)], "小四") (* NONE *)

(* ([1, 3, 6], [-2, -4, -5]) *)
val t_splitup_1 = splitup([1, ~2, 3, ~4, ~5, 6])

(* ([1, 2, 3, 4], [5, 6, 7] *)
val t_splitAt_1 = splitAt([1, 2, 3, 4, 5, 6, 7], 4)

val t_isSorted_1 = isSorted([1, 2, 3, 3, 3, 4, 5]) (* true *)
val t_isSorted_2 = isSorted([1, 2, 3, 2, 3, 4, 5]) (* false *)

val t_isAnySorted_1 = isAnySorted([1, 2, 3, 3, 3, 4, 5]) (* true *)
val t_isAnySorted_2 = isAnySorted([5, 4, 3, 3, 3, 2, 1]) (* true *)
val t_isAnySorted_3 = isAnySorted([1, 2, 3, 2, 3, 4, 5]) (* false *)

(* [1, 4, 5, 7, 8, 9] *)
val t_sortedMerge_1 = sortedMerge([1, 4, 7], [5, 8, 9])

(* [1, 2, 2, 4, 5, 7, 8, 10] *)
val t_qsort_1 = qsort([1, 4, 7, 5, 8, 2, 9, 2, 10])

(* ([1, 3, 5, 7], [2, 4, 6]) *)
val t_divide_1 = divide([1, 2, 3, 4, 5, 6, 7])
(* ([1, 3, 5], [2, 4, 6]) *)
val t_divide_2 = divide([1, 2, 3, 4, 5, 6])

(* [1, 2, 2, 4, 5, 7, 8, 10] *)
val t_not_so_quick_sort = not_so_quick_sort([1, 4, 7, 5, 8, 2, 9, 2, 10])

val t_fullDivide_1 = fullDivide(2, 40) (* (3, 5) *)
val t_fullDivide_2 = fullDivide(3, 10) (* (0, 10) *)

val t_factorize_1 = factorize(20) (* [(2, 2), (5, 1)] *)
val t_factorize_2 = factorize(36) (* [(2, 2), (3, 2)] *)
val t_factorize_3 = factorize(1) (* [] *)

val t_multiply_1 = multiply([(2, 2), (5, 1)]) (* 20 *)
val t_multiply_2 = multiply([(2, 2), (3, 2)]) (* 36 *)
val t_multiply_3 = multiply([]) (* 1 *)
