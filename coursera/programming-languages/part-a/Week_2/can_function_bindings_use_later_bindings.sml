(* pow must be occur before cube *)

fun pow(x : int, y : int) : int =
    if y = 0
    then 1
    else x * pow(x, y-1)

fun cube(x : int) : int =
    pow(x, 2)
