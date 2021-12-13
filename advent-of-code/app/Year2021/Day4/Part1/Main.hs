import           Data.Functor
import           Data.List.Split
import           Data.Matrix as M
import           Paths_advent_of_code

newtype Board = Board (Matrix Int)
              deriving Show

inputFile :: IO String
inputFile = getDataFileName "sampledata/Year2021/Day4/input.txt"

parseBoard :: [String] -> Board
parseBoard ([]:lines) = parseBoard lines
parseBoard lines = Board $ M.fromLists $ map (map read . words) lines

readInput :: IO ([Int], [Board])
readInput = do
  lines <- inputFile >>= readFile <&> lines
  let nums = map read $ splitOn "," $ head lines
      boardInputs = splitOn [""] $ tail $ tail lines
  return (nums, map parseBoard boardInputs)

main :: IO ()
main = do
  (nums, boards) <- readInput
  print boards
