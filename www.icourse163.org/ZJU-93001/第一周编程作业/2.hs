{-# LANGUAGE RecordWildCards #-}

import           Text.Printf

data ResultRepr = ResultRepr { currSum :: !Int
                             , currMax :: !Int
                             , maxL    :: !Int
                             , maxR    :: !Int
                             , currL   :: Int
                             , currR   :: Int
                             , allNeg  :: !Bool
                             }

split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

maxSubseqSum :: [Int] -> (Int, Int, Int)
maxSubseqSum a = _maxSubseqSum a $ ResultRepr 0 0 0 0 (head a) 0 True
  where
    _maxSubseqSum :: [Int] -> ResultRepr -> (Int, Int, Int)
    _maxSubseqSum [] resultRepr@ResultRepr{..} =
      if allNeg then (0, head a, last a) else (currMax, maxL, maxR)
    _maxSubseqSum (x:xs) resultRepr@ResultRepr{..}
      | currSum + x < 0 = let
          currL' = head xs
          in
          _maxSubseqSum xs resultRepr{ currSum = 0
                                     , currL = currL'
                                     , currR = currL'
                                     }
      | otherwise = let
          currSum' = currSum + x
          currR' = x
          in
          _maxSubseqSum xs $ if currSum' > currMax
                             then
                               resultRepr{ currSum = currSum'
                                         , currMax = currSum'
                                         , maxL = currL
                                         , maxR = currR'
                                         , currR = currR'
                                         , allNeg = False
                                         }
                             else
                               resultRepr{ currSum = currSum'
                                         , currR = currR'
                                         , allNeg = False
                                         }


main :: IO ()
main = do
  input <- getLine >> getLine
  let nums = map (read :: String -> Int) (split input)
      (currMax, currL, currR) = maxSubseqSum nums
  putStrLn $ printf "%d %d %d" currMax currL currR
