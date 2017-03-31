import Control.Monad
import Data.Char
import Data.Functor
import Data.IORef
import Data.List
import Data.Maybe

data BinTree a = Nil | BinTree a (BinTree a) (BinTree a) deriving Show
data Instr a = Push a | Pop

split :: String -> [String]
split "" = []
split a = let
  (w, remaining) = span (/= ' ') (dropWhile (== ' ') a)
  in
  w:(split remaining)

readInstr :: String -> Instr Int
readInstr s = let
  sp = split s
  in
  case sp of
    ("Push":i:_) -> Push (read i)
    ("Pop":_) -> Pop

buildTree :: [Int] -> [Int] -> BinTree Int
buildTree [] _ = Nil
buildTree (a:as) b = let
  splitPoint = fromJust $ elemIndex a b
  (l, (_:r)) = splitAt splitPoint b
  (al, ar) = splitAt splitPoint as
  in
  BinTree a (buildTree al l) (buildTree ar r)

postOrder :: BinTree a -> [a]
postOrder Nil = []
postOrder (BinTree d l r) = postOrder l ++ postOrder r ++ [d]

main = do
  preOrderRef <- newIORef []
  inOrderRef <- newIORef []
  stack <- newIORef []
  cnt <- readLn
  replicateM (2*cnt) $ do
    instr <- readInstr <$> getLine
    case instr of
      Push a -> modifyIORef preOrderRef (a:) >> modifyIORef stack (a:)
      Pop -> do
        i0 <- head <$> readIORef stack
        modifyIORef stack tail
        modifyIORef inOrderRef (i0:)

  preOrder <- readIORef preOrderRef
  inOrder <- readIORef inOrderRef
  let t = buildTree (reverse preOrder) (reverse inOrder)
  putStrLn $ intercalate " " $ map show (postOrder t)
