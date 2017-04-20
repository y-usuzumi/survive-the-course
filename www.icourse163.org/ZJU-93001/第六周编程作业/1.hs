import           Control.Applicative
import           Control.Monad
import           Data.Array.IO
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.List
import           Data.Maybe
import           Data.Tuple
import           Text.Printf

type Graph = IOArray Int [Int]
type VisitState = IOArray Int Bool

type TraversingGraph = (Graph, VisitState)

newGraph :: Int -> IO Graph
newGraph cnt = do
  edgeArray <- newArray (0, cnt-1) []
  return edgeArray

newVisitState :: Int -> IO VisitState
newVisitState cnt = newArray (0, cnt-1) False

connect :: Graph -> Int -> Int -> IO ()
connect g a b = do
  peers <- readArray g a
  writeArray g a (b:peers)

readIntPair :: IO (Int, Int)
readIntPair = do
  a:b:_ <- map (fst . fromJust . C8.readInt) . C8.words <$> BS.getLine
  return (a, b)

traverseDFS :: Graph -> IO [[Int]]
traverseDFS g = do
  (minBound, maxBound) <- getBounds g
  visitArray <- newVisitState (maxBound+1)
  res <- forM [0..maxBound] $ \idx -> do
    _traverseDFS g visitArray idx
  return $ filter ((>0) . length) res
  where
    _traverseDFS :: Graph -> VisitState -> Int -> IO [Int]
    _traverseDFS g va entry = do
      visited <- readArray va entry
      if visited then return [] else do
        writeArray va entry True
        (entry:) <$> (readArray g entry >>= fmap join . mapM (_traverseDFS g va) . sort)

traverseBFS :: Graph -> IO [[Int]]
traverseBFS g = do
  (minBound, maxBound) <- getBounds g
  visitArray <- newVisitState (maxBound+1)
  res <- forM [0..maxBound] $ \idx -> do
    _traverseBFS g visitArray [idx]
  return $ filter ((>0) . length) res
  where
    _traverseBFS :: Graph -> VisitState -> [Int] -> IO [Int]
    _traverseBFS g va [] = pure []
    _traverseBFS g va entries = do
      unvisitedEntries <- filterM (readArray va >=> return . not) entries
      forM unvisitedEntries $ (flip (writeArray va) True)
      peers <- nub . join <$> forM unvisitedEntries (readArray g >=> return . sort)
      (unvisitedEntries ++) <$> _traverseBFS g va peers

printResult :: [[Int]] -> IO ()
printResult = mapM_ (printf "{ %s }\n" . intercalate " " . map show)

main = do
  (vertexCount, edgeCount) <- readIntPair
  g <- newGraph vertexCount
  edges <- replicateM edgeCount readIntPair
  forM edges $ \edge -> do
    uncurry (connect g) edge
    uncurry (connect g) (swap edge)
  traverseDFS g >>= printResult
  traverseBFS g >>= printResult
