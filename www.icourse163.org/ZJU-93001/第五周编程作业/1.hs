{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE RecordWildCards  #-}

import           Control.Applicative
import           Control.Monad
import           Control.Monad.ST
import qualified Data.Array          as FA
import           Data.Array.IO
import           Data.Array.MArray
import           Data.List

split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

readNums :: IO [Int]
readNums = map read . split <$> getLine

data Heap a = Heap { array :: IOArray Int a
                   , size  :: Int
                   }

newHeap :: Int -> IO (Heap Int)
newHeap size = do
  array <- newArray (0, size) 0
  writeArray array 0 (-10001)
  return $ Heap{ array = array
               , size = 0
               }

insertHeap :: Int -> Heap Int -> IO (Heap Int)
insertHeap num heap@Heap{..} = do
  let idx = size + 1
  _insert idx num array
  return heap{ size = size + 1 }
  where
    _insert idx num array = do
      let newIdx = idx `quot` 2
      cItem <- readArray array newIdx
      if cItem > num
        then do
        writeArray array idx cItem
        _insert newIdx num array
        else do
        writeArray array idx num

getPath :: IOArray Int Int -> Int -> IO [Int]
getPath _ 0 = return []
getPath arr idx = do
  num <- readArray arr idx
  remaining <- getPath arr (idx `quot` 2)
  return $ num:remaining
main = do
  n:m:_ <- readNums
  heap <- newHeap n
  nums <- readNums
  newHeap <- foldM (flip insertHeap) heap nums
  indices <- readNums
  forM indices $ \idx -> do
    path <- getPath (array newHeap) idx
    putStrLn $ intercalate " " $ map show path
