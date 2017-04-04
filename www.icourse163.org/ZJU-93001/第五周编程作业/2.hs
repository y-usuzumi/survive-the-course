{-# LANGUAGE BangPatterns      #-}
{-# LANGUAGE OverloadedStrings #-}

import           Control.Monad
import           Data.Array
import           Data.Array.IO
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.Maybe
import           Text.Printf

type SetOfSets = IOUArray Int Int

newSetOfSets :: Int -> IO SetOfSets
newSetOfSets !size = newArray (1, size) (-1)

findRoot :: Int -> SetOfSets -> IO (Int, Int)
findRoot !idx !sos = do
  root <- readArray sos idx
  if root < 0 then return (idx, root) else do
    ret@(finalRoot, w) <- findRoot root sos
    when (finalRoot /= idx) $ writeArray sos idx finalRoot
    return ret

union :: Int -> Int -> SetOfSets -> IO ()
union !r1 !r2 !sos = do
  (root1, w1) <- findRoot r1 sos
  (root2, w2) <- findRoot r2 sos
  when (root1 /= root2) $ do
    if w1 < w2
      then do
      writeArray sos root2 root1
      writeArray sos root1 (w1 + w2)
      else do
      writeArray sos root2 root1
      writeArray sos root1 (w1 + w2)

check :: Int -> Int -> SetOfSets -> IO String
check !r1 !r2 !sos = do
  (root1, _) <- findRoot r1 sos
  (root2, _) <- findRoot r2 sos
  return $ if root1 == root2 then "yes" else "no"

stop :: SetOfSets -> IO String
stop !sos = do
  !cnt <- findRootCnt sos
  return $ if cnt == 1
           then "The network is connected."
           else printf "There are %d components." cnt

findRootCnt :: SetOfSets -> IO Int
findRootCnt !sos = getElems sos >>= return . length . filter (< 0)

data Command = Input !Int !Int
             | Check !Int !Int
             | Stop
             deriving Eq

resolveCommand :: BS.ByteString -> Command
resolveCommand !s = let
   sects = C8.words s
   in
     case sects of
       "I":a:b:_ -> Input (readI a) (readI b)
       "C":a:b:_ -> Check (readI a) (readI b)
       "S":_     -> Stop
   where
     readI = fst . fromJust . C8.readInt

main = do
  size <- readLn
  sos <- newSetOfSets size
  op sos
  where
    op sos = do
      line <- BS.getLine
      let cmd = resolveCommand line
      case cmd of
        Input l r -> union l r sos
        Check l r -> check l r sos >>= putStrLn
        Stop      -> stop sos >>= putStrLn
      when (cmd /= Stop) (op sos)
