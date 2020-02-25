fun is_older(left : (int*int*int), right : (int*int*int)) =
    let
        fun compute(left : (int*int*int), right : (int*int*int), funs : ((int*int*int) -> int) list) =
            if null funs
            then false
            else
                let
                    val item_getter = hd funs
                    val left_item = item_getter left
                    val right_item = item_getter right
                in
                    if left_item < right_item
                    then true
                    else
                        if left_item > right_item
                        then false
                        else compute(left, right, tl funs)
                end
    in
        compute(left, right, [#1, #2, #3])
    end

(* NOTE: This is a helper function used in multiple other functions *)
fun date_in_month(xs : (int*int*int), mo : int) =
    if #2 xs = mo then true else false

fun number_in_month(xs : (int*int*int) list, mo : int) =
    if null xs
    then 0
    else
        let
            val head = hd xs
        in
            (if date_in_month(hd xs, mo) then 1 else 0) + number_in_month(tl xs, mo)
        end

fun number_in_months(xs : (int*int*int) list, mos : int list) =
    if null mos
    then 0
    else number_in_month(xs, hd mos) + number_in_months(xs, tl mos)

fun dates_in_month(xs : (int*int*int) list, mo : int) =
    if null xs
    then []
    else
        let
            val head = hd xs
        in
            if date_in_month(head, mo) then head :: dates_in_month(tl xs, mo) else dates_in_month(tl xs, mo)
        end

fun dates_in_months(xs : (int*int*int) list, mos : int list) =
    if null mos
    then []
    else dates_in_month(xs, hd mos) @ dates_in_months(xs, tl mos)

fun get_nth(xs : string list, n : int) =
    if n = 1 then hd xs else get_nth(tl xs, n-1)

fun date_to_string(date : (int*int*int)) =
    let
        val month_reprs = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December"
        ]
        val year = Int.toString (#1 date)
        val month = get_nth(month_reprs, #2 date)
        val day = Int.toString (#3 date)
    in
        month ^ " " ^ day ^ ", " ^ year
    end

fun number_before_reaching_sum(sum : int, xs : int list) =
    let
        fun number_before_reaching_sum_with_curr_sum(curr_sum : int, xs : int list) =
            if null xs
            then 0
            else
                let
                    val new_sum = curr_sum + hd xs
                in
                    if new_sum >= sum then 0 else 1 + number_before_reaching_sum_with_curr_sum(new_sum, tl xs)
                end
    in
        number_before_reaching_sum_with_curr_sum(0, xs)
    end

fun what_month(day_of_year : int) =
    let
        val month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    in
        number_before_reaching_sum(day_of_year, month_days) + 1
    end

fun month_range(day1 : int, day2 : int) =
    if day1 > day2
    then []
    else
        what_month day1 :: month_range(day1+1, day2)

fun oldest(xs : (int*int*int) list) =
    if null xs
    then NONE
    else
        let
            fun oldest_nonempty(xs : (int*int*int) list) =
                let
                    val head = hd xs
                in
                    if null (tl xs)
                    then head
                    else
                        let
                            val tail_oldest = oldest_nonempty (tl xs)
                        in
                            if is_older(head, tail_oldest) then head else tail_oldest
                        end
                end
        in
            SOME(oldest_nonempty xs)
        end

fun dedup(xs : int list) =
    let
        fun has_elem(xs : int list, elem : int) =
            not(null xs) andalso (hd xs = elem orelse has_elem(tl xs, elem))
    in
        if null xs
        then []
        else
            let
                val head = hd xs
                val tail_dedup = dedup(tl xs)
            in
                if has_elem(tail_dedup, head) then tail_dedup else head :: tail_dedup
            end
    end

fun number_in_months_challenge(xs : (int*int*int) list, mos : int list) =
    number_in_months(xs, dedup mos)

fun dates_in_months_challenge(xs : (int*int*int) list, mos : int list) =
    dates_in_months(xs, dedup mos)

fun reasonable_date(date : (int*int*int)) =
    let
        fun is_leap_year(year : int) =
            if year mod 100 = 0
            then year mod 400 = 0
            else year mod 4 = 0

        fun check_day(month: int, day: int, is_leap_year: bool) =
            if month = 1 orelse month = 3 orelse month = 5 orelse month = 7
               orelse month = 8 orelse month = 10 orelse month = 12
            then
                day > 0 andalso day < 32
            else
                if month = 4 orelse month = 6 orelse month = 9 orelse month = 11
                then
                    day > 0 andalso day < 31
                else
                    day > 0 andalso (is_leap_year andalso day < 30 orelse day < 29)
        val year = #1 date
        val month = #2 date
        val day = #3 date
    in
        year > 0 andalso month > 0 andalso month < 13 andalso check_day(#2 date, #3 date, is_leap_year year)
    end
