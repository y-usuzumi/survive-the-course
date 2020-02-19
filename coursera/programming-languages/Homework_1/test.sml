use "solution.sml";

val t_is_older_1 = is_older((2020, 2, 19), (2020, 2, 20)); (* true *)
val t_is_older_2 = is_older((2020, 2, 19), (2019, 2, 18)); (* false *)
val t_is_older_3 = is_older((2020, 2, 19), (2020, 2, 19)); (* false *)

(* 3 *)
val t_number_in_month_1 = number_in_month(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
        ],
        2
    );

(* 4 *)
val t_number_in_months_1 = number_in_months(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
        ],
        [2, 3]
    );

(* 2020/2/10, 2020/2/14, 1000/2/20 *)
val t_dates_in_month_1 = dates_in_month(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
        ],
        2
    );

(* 2020/2/10, 2020/2/14, 1000/2/20, 1000/3/31 *)
val t_dates_in_months_1 = dates_in_months(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
        ],
        [2, 3]
    );

val t_get_nth_1 = get_nth(["Hello", "World", "Foo", "Bar"], 3) (* "Foo" *)

val t_date_to_string_1 = date_to_string((2013, 1, 20)) (* January 20, 2013 *)
val t_date_to_string_2 = date_to_string((2020, 2, 19)) (* February 19, 2020 *)

val t_number_before_reaching_sum_1 = number_before_reaching_sum(12, [1, 2, 3, 4, 5]) (* 4 *)
val t_number_before_reaching_sum_2 = number_before_reaching_sum(12, [1, 2, 3]) (* 3 *)
val t_number_before_reaching_sum_3 = number_before_reaching_sum(10, [1, 2, 3, 4, 5]) (* 3 *)

val t_what_month_1 = what_month(31) (* 1 *)
val t_what_month_2 = what_month(32) (* 2 *)
val t_what_month_3 = what_month(365) (* 12 *)

val t_month_range_1 = month_range(31, 32)  (* [1, 2] *)
val t_month_range_2 = month_range(32, 33) (* [2] *)
val t_month_range_3 = month_range(15, 70) (* [1, 2, 3] *)

(* 2020/2/14 *)
val t_oldest_1 = oldest(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
    ])

(* FIXME: for duplicate items, the one that was preserved was the last-occurred one *)
(* [1, 7, 4, 8, 3] *)
val t_dedup_1 = dedup([1, 3, 1, 4, 4, 7, 4, 8, 3])

(* 4 *)
val t_number_in_months_challenge_1 = number_in_months_challenge(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
        ],
        [2, 3, 2, 2, 2, 3]
    );

(* 2020/2/10, 2020/2/14, 1000/2/20, 1000/3/31 *)
val t_dates_in_months_challenge_1 = dates_in_months_challenge(
        [
          (2020, 2, 10),
          (2020, 2, 14),
          (2020, 1, 10),
          (1000, 3, 31),
          (1000, 2, 20)
        ],
        [2, 3, 2, 2, 2, 3]
    );

val t_reasonable_date_1 = reasonable_date(1988, 2, 29) (* true *)
val t_reasonable_date_2 = reasonable_date(1988, 2, 30) (* false *)
val t_reasonable_date_3 = reasonable_date(2000, 2, 29) (* true *)
val t_reasonable_date_4 = reasonable_date(2100, 2, 29) (* false *)
val t_reasonable_date_5 = reasonable_date(~1, ~1, ~1) (* false *)
