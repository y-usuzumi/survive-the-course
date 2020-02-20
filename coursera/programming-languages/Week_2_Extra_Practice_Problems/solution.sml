fun alternate(xs : int list) =
    if null xs then 0 else hd xs - alternate (tl xs)

fun min_max(xs : int list) =
    if
        null xs
    then
        (0, 0)
    else
        if null (tl xs)
        then (hd xs, hd xs)
        else
            let
                val head = hd xs
                val min_max_tl = min_max (tl xs)
            in
                (
                  if head < #1 min_max_tl then head else #1 min_max_tl,
                  if head > #2 min_max_tl then head else #2 min_max_tl
                )
            end

fun cum_sum(xs : int list) =
    if null xs
    then []
    else
        if null (tl xs)
        then xs
        else
            hd xs :: cum_sum (hd (tl xs) + (hd xs) :: tl (tl xs))

fun greeting(name : string option) =
    "Hello there, " ^ (if isSome name then valOf name else "you") ^ "!"

fun repeat(items : int list, times_list : int list) =
    if null items orelse null times_list
    then []
    else
        if hd times_list = 0
        then repeat2(tl items, tl times_list)
        else
            hd items :: repeat2(items, hd times_list - 1 :: tl times_list)

fun addOpt(left : int option, right : int option) =
    if isSome left andalso isSome right then SOME (valOf left + valOf right) else NONE

fun addAllOpt(xs : int option list) =
    if null xs then NONE
    else
        if isSome (hd xs)
        then
            let
                val addAllOpt_tl = addAllOpt (tl xs)
            in
                if isSome addAllOpt_tl then SOME (valOf (hd xs) + valOf addAllOpt_tl) else hd xs
            end
        else
            addAllOpt(tl xs)

fun any(xs : bool list) =
    not(null xs) andalso (hd xs orelse any(tl xs))

fun all(xs : bool list) =
    null xs orelse (hd xs andalso all(tl xs))

fun zip(xs : int list, ys : int list) =
    if null xs orelse null ys
    then []
    else (hd xs, hd ys) :: zip(tl xs, tl ys)

fun zipRecycle(xs : int list, ys : int list) =
    let
        fun zipWithRemaining(xs : int list, ys : int list) =
            if null xs
            then ([], [], ys)
            else
                if null ys
                then ([], xs, [])
                else
                    let
                        val zipWithRemaining_tl = zipWithRemaining(tl xs, tl ys)
                    in
                        ((hd xs, hd ys) :: #1 zipWithRemaining_tl, #2 zipWithRemaining_tl, #3 zipWithRemaining_tl)
                    end
        fun zipCycleXs(inner_xs, inner_ys : int list) =
            if null inner_ys
            then []
            else
                if null inner_xs then zipCycleXs(xs, inner_ys) else (hd inner_xs, hd inner_ys) :: zipCycleXs(tl inner_xs, tl inner_ys)
        fun zipCycleYs(inner_xs, inner_ys : int list) =
            if null inner_xs
            then []
            else
                if null inner_ys then zipCycleYs(inner_xs, ys) else (hd inner_xs, hd inner_ys) :: zipCycleYs(tl inner_xs, tl inner_ys)
        val zwr = zipWithRemaining(xs, ys)
        val zwr_list = #1 zwr
        val zwr_xs = #2 zwr
        val zwr_ys = #3 zwr
    in
        zwr_list @ (if null (zwr_xs) then zipCycleXs(xs, zwr_ys) else zipCycleYs(zwr_xs, ys))
    end
