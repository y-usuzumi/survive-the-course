split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

maxSubseqSum :: [Int] -> Int
maxSubseqSum a = _maxSubseqSum a 0 0
  where
    _maxSubseqSum [] currSum currMax = currMax
    _maxSubseqSum (a:as) currSum currMax
      | currSum + a < 0 = _maxSubseqSum as 0 currMax
      | otherwise = let
          currSum' = currSum + a
          currMax' = max currMax currSum'
          in
          _maxSubseqSum as currSum' currMax'


main :: IO ()
main = do
  input <- getLine >> getLine
  let nums = map (read :: String -> Int) (split input)
  putStrLn $ show $ maxSubseqSum nums
