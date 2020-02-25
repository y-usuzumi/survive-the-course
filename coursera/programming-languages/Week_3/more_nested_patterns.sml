fun nondecreasing xs = (* int list -> bool *)
    case xs of
        [] => true
      | _::[] => true
      | x::y::xs => x <= y andalso nondecreasing (y::xs)

datatype sgn = P |  N |  Z

fun multsign (x1, x2) =
    let fun sign x = if x < 0 then N else if x > 0 then P else Z
    in
        case (sign x1, sign x2) of
            (P, P) => P
         |  (P, N) => N
         |  (N, P) => N
         |  (N, N) => P
         |  _ => Z (* this covers 5 other cases which is arguably the shortest code, and does not sacrifice readability *)
    end

fun my_len xs =
    case xs of
        [] => 0
     | _::xs' => 1 + len xs'
