import Control.Applicative
import Control.Monad
import Data.List

split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

splitLU :: Int -> [Int] -> ([Int], [Int])
splitLU a l = partition (< a) l

check :: [[Int]] -> Bool
check ll
  | all null ll = True
  | lls <- map head ll
  , not $ and $ zipWith (==) lls (tail lls) = False
check ll@((a:ls):_) = check (fst lul) && check (snd lul)
  where
    lul = (,) <$> map fst <*> map snd $ map (splitLU a) (map tail ll)

main = do
    input <- getLine
    when (input /= "0") $ do
      let (_:l:_) = split input
      loriginal <- map read <$> (split <$> getLine)
      lls <- replicateM (read l) $ map read <$> (split <$> getLine)
      forM lls $ \lx -> do
        if check [loriginal,lx] then putStrLn "Yes" else putStrLn "No"
      main
