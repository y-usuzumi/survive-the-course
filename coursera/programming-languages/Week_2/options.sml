fun max2(xs : int list) =
    if
        null xs
    then
        NONE
    else
        let
            fun max2_nonempty(xs : int list) =
                if
                    null (tl xs)
                then
                    hd xs
                else
                    let
                        val max_tl = max2_nonempty(tl xs)
                        val head = hd xs
                    in
                        if
                            max_tl > head
                        then
                            max_tl
                        else
                            head
                    end
        in
            SOME (max2_nonempty xs)
        end

