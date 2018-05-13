import           Control.Applicative
import           Control.Monad
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.List
import           Data.Maybe

consumable :: [Int]
consumable = scanl (+) 1 [6, 10..]

mostConsumable :: Int -> (Int, Int)
mostConsumable 0 = (0, -1)
mostConsumable num = let
  (l, r) = span (<= num) consumable
  in
  (last l, length l - 1)

printHourGlass :: Int -> Char -> IO ()
printHourGlass iterCnt c = do
  forM_ [iterCnt,iterCnt-1..1] $ \i -> printHGLine i iterCnt c
  forM_ [0..iterCnt] $ \i -> printHGLine i iterCnt c
  where
    printHGLine n maxN c = do
      putStr $ replicate (maxN-n) ' '
      putStrLn $ replicate ((n*2)+1) c

main = do
  numS:cS:_ <- C8.words <$> BS.getLine
  let num = fst $ fromJust $ C8.readInt numS
      c = C8.head cS
      (mc, iterCnt) = mostConsumable num
  printHourGlass iterCnt c
  print $ num - mc
