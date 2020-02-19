fun good_max(xs: int list) =
    if null xs
    then
        0
    else
        if null (tl xs)
        then
            hd xs
        else
            let
                val xs_first = hd xs
                val xs_max = good_max (tl xs)
            in
                if
                    xs_first > xs_max
                then
                    xs_first
                else
                    xs_max
            end
