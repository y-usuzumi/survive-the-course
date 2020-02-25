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
        then repeat(tl items, tl times_list)
        else
            hd items :: repeat(items, hd times_list - 1 :: tl times_list)

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

fun zipOpt(xs : int list, ys : int list) =
    if null xs
    then
        if null ys
        then SOME []
        else NONE
    else
        if null ys
        then NONE
        else
            let
                val zipOpt_tl = zipOpt(tl xs, tl ys)
            in
                if isSome zipOpt_tl
                then
                    SOME ((hd xs, hd ys) :: valOf zipOpt_tl)
                else
                    NONE
            end

fun lookup(map : (string * int) list, key : string) =
    if null map
    then NONE
    else
        let
            val pair = hd map
        in
            if #1 pair = key
            then SOME (#2 pair)
            else
                lookup(tl map, key)
        end

fun splitup(xs : int list) =
    if null xs
    then ([], [])
    else
        let
            val head = hd xs
            val splitup_tl = splitup(tl xs)
        in
            if head < 0
            then (#1 splitup_tl, head :: #2 splitup_tl)
            else (head :: #1 splitup_tl, #2 splitup_tl)
        end

fun splitAt(xs : int list, threshold : int) =
    if null xs
    then ([], [])
    else
        let
            val head = hd xs
            val splitup_tl = splitAt(tl xs, threshold)
        in
            if head < threshold
            then (#1 splitup_tl, head :: #2 splitup_tl)
            else (head :: #1 splitup_tl, #2 splitup_tl)
        end

fun isSorted(xs : int list) =
    if null xs
    then true
    else
        if null (tl xs)
        then true
        else
            hd xs <= hd (tl xs) andalso isSorted(tl xs)

fun isAnySorted(xs : int list) =
    let
        fun reverse(xs : int list) =
            if null xs then [] else reverse(tl xs) @ [hd xs]
    in
        isSorted xs orelse isSorted(reverse xs)
    end

fun sortedMerge(xs : int list, ys : int list) =
    if null xs
    then ys
    else
        if null ys
        then xs
        else
            let
                val headx = hd xs
                val heady = hd ys
            in
                if headx <= heady
                then headx :: sortedMerge (tl xs, ys)
                else heady :: sortedMerge (xs, tl ys)
            end

fun qsort(xs : int list) =
    if null xs
    then xs
    else
        let
            val head = hd xs
            val splitAt_tl = splitAt(tl xs, head)
        in
            qsort(#2 splitAt_tl) @ [head] @ qsort(#1 splitAt_tl)
        end

fun divide(xs : int list) =
    if null xs
    then ([], [])
    else
        if null (tl xs)
        then ([hd xs], [])
        else
            let
                val h1 = hd xs
                val h2 = hd (tl xs)
                val divide_tl = divide(tl (tl xs))
            in
                (h1 :: (#1 divide_tl), h2 :: (#2 divide_tl))
            end

fun not_so_quick_sort(xs : int list) =
    if null xs
    then []
    else
        if null (tl xs)
        then xs
        else
            let
                val divided_list = divide xs
                val left_sorted = not_so_quick_sort(#1 divided_list)
                val right_sorted = not_so_quick_sort(#2 divided_list)
            in
                sortedMerge(left_sorted, right_sorted)
            end

fun fullDivide(k : int, n : int) =
    if n mod k = 0
    then
        let
            val subfullDivide = fullDivide(k, n div k)
        in
            (1 + #1 subfullDivide, #2 subfullDivide)
        end
    else
        (0, n)

fun factorize(n : int) =
    if n < 2 then []
    else
        let
            fun subFullDivide(k : int, n : int) =
                if k * k > n
                then
                    []
                else
                    let
                        val fullDivideResult = fullDivide(k, n)
                    in
                        if #1 fullDivideResult > 0
                        then (k, #1 fullDivideResult) :: subFullDivide(k+1, #2 fullDivideResult)
                        else subFullDivide(k+1, #2 fullDivideResult)
                    end
        in
            subFullDivide(2, n)
        end

fun multiply(xs : (int*int) list) =
    if null xs
    then 1
    else
        let
            val head = hd xs
            fun power(n : int, k : int) =
                if k = 0 then 1 else n * power(n, k-1)
        in
            power(#1 head,  #2 head) * multiply(tl xs)
        end

fun all_products(xs : (int*int) list) =
    (* FIXME *)
