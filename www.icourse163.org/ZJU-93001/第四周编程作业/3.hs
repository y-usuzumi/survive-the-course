import Control.Applicative
import Data.List

data Tree a = Nil | Tree a (Tree a) (Tree a) deriving Show

split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

height :: [Int] -> Int
height = (+1) . floor . logBase 2 . fromIntegral . length

buildTree :: [Int] -> Tree Int
buildTree [] = Nil
buildTree [a] = Tree a Nil Nil
buildTree l = let
  len = length l
  h = height l
  s = 2 ^ h - 2 ^ (h-2)
  splitPoint = case compare len s of
                 LT -> len - 2 ^ (h-2)
                 otherwise -> 2 ^ (h-1) - 1
  (left, (root:right)) = splitAt splitPoint l
  in
  Tree root (buildTree left) (buildTree right)

levelTraverse :: [Tree a] -> [a]
levelTraverse [] = []
levelTraverse trees = let
  (roots, remainingTrees) = takeRoots trees
  in
  roots ++ levelTraverse remainingTrees
  where
    takeRoot :: Tree a -> ([a], [Tree a])
    takeRoot Nil = ([], [])
    takeRoot (Tree a l r) = ([a], [l, r])
    takeRoots :: [Tree a] -> ([a], [Tree a])
    takeRoots trees = let
      u = map takeRoot trees
      in
      (concat (map fst u), concat (map snd u))

main = do
  count <- readLn :: IO Int
  nums <- sort . map (read :: String -> Int) . split <$> getLine
  putStrLn $ intercalate " " $ map show $ levelTraverse [buildTree nums]
