import           Data.Bits
import           Data.List
import           Text.Printf

countZerosOnes :: [Char] -> (Int, Int)
countZerosOnes chars = countZerosOnes_ chars 0 0
  where
    countZerosOnes_ [] zeros ones = (zeros, ones)
    countZerosOnes_ (x:xs) zeros ones = case x of
      '0' -> countZerosOnes_ xs (zeros+1) ones
      '1' -> countZerosOnes_ xs zeros (ones+1)
      _ -> error "WTF?????"

mostCommonBit :: [Char] -> Int
mostCommonBit bits = case countZerosOnes bits of
  (zeros, ones) | zeros < ones -> 1
                | zeros > ones -> 0
                | otherwise -> error "WTF???"

leastCommonBit :: [Char] -> Int
leastCommonBit bits = case countZerosOnes bits of
  (zeros, ones) | zeros < ones -> 0
                | zeros > ones -> 1
                | otherwise -> error "WTF???"

reportRate :: ([Char] -> Int) -> [String] -> Int
reportRate reportFunc data_ = foldl' ((.|.) . flip shiftL 1) 0 $ map reportFunc (transpose data_)

inputFile :: FilePath
inputFile = "input.txt"

readInput :: IO [String]
readInput = lines <$> readFile inputFile

main :: IO ()
main = do
  inputData <- readInput
  printf "Gamma rate: %d" (reportRate mostCommonBit inputData)
  printf "Epsilon rate: %d" (reportRate leastCommonBit inputData)
