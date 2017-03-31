{-# LANGUAGE NamedFieldPuns #-}

import Control.Monad
import Data.Char
import Data.List
import Data.Maybe
import Data.STRef
import Control.Monad.ST
import Debug.Trace

data RNode = RNode { rd :: Char
                   , rl :: Int
                   , rr :: Int
                   , rIsRoot :: Bool
                   } deriving Show

data Node = Nil | Node { d :: Char, l :: Node, r :: Node
                       } deriving Show

input :: IO (Char, Int, Int)
input = do
  (d:' ':l:' ':r:[]) <- getLine
  return $ (d, normalize l, normalize r)
  where
    normalize :: Char -> Int
    normalize c = if c == '-' then -1 else ord c - 48

rnodeListToNode :: [RNode] -> Int -> Node
rnodeListToNode nodes rootIdx = let
  root = nodes !! rootIdx
  in
  Node { d = rd root
       , l = if rl root == -1 then Nil else rnodeListToNode nodes (rl root)
       , r = if rr root == -1 then Nil else rnodeListToNode nodes (rr root)
       }

initTree :: IO Node
initTree = do
  cnt <- readLn
  inputs <- replicateM cnt input
  return $ runST $ do
    refs <- replicateM 10 ( newSTRef RNode { rd = ' '
                                           , rl = -1
                                           , rr = -1
                                           , rIsRoot = True
                                           }
                          )
    forM (zip [0..] inputs) $ \(idx, (rd, rl, rr)) -> do
      modifySTRef (refs !! idx) $ \v -> v { rd, rl, rr }
      when (rl /= -1) $ do
        modifySTRef (refs !! rl) $ \v -> v { rIsRoot = False }
      when (rr /= -1) $ do
        modifySTRef (refs !! rr) $ \v -> v { rIsRoot = False }
    rnodes <- mapM readSTRef refs
    let rootIdx = fromJust $ flip findIndex rnodes rIsRoot
    return $ rnodeListToNode rnodes rootIdx

isIsomorphic :: Node -> Node -> Bool
isIsomorphic Nil Nil = True
isIsomorphic Nil right = False
isIsomorphic left Nil = False
isIsomorphic left right = d left == d right &&
  let
    ll = l left
    lr = r left
    rl = l right
    rr = r right
  in
    isIsomorphic ll rl && isIsomorphic lr rr || isIsomorphic ll rr && isIsomorphic rl lr

main = do
  t1 <- initTree
  t2 <- initTree
  putStrLn $ if (isIsomorphic t1 t2) then "Yes" else "No"
