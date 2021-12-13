import           Data.Bits
import           Data.Functor
import           Data.List
import           Data.Vector          (Vector)
import qualified Data.Vector          as V
import           Paths_advent_of_code
import           Text.Printf

countZerosOnes :: [Char] -> (Int, Int)
countZerosOnes chars = countZerosOnes_ chars 0 0
  where
    countZerosOnes_ [] zeros ones = (zeros, ones)
    countZerosOnes_ (x:xs) zeros ones = case x of
      '0' -> countZerosOnes_ xs (zeros+1) ones
      '1' -> countZerosOnes_ xs zeros (ones+1)
      _   -> error "WTF?????"

mostCommonBit :: [Char] -> Char
mostCommonBit bits = case countZerosOnes bits of
  (zeros, ones) | zeros <= ones -> '1'
                | zeros > ones -> '0'

leastCommonBit :: [Char] -> Char
leastCommonBit bits = case countZerosOnes bits of
  (zeros, ones) | zeros <= ones -> '0'
                | zeros > ones -> '1'

deepFilter :: ([Char] -> Char) -> [Vector Char] -> [Vector Char]
deepFilter f vs = deepFilter_ f vs 0
  where
    deepFilter_ f [] idx = []
    deepFilter_ f [v] idx = [v]
    deepFilter_ f vs idx
      | idx >= V.length (head vs) = vs
      | otherwise = let currentBits = map (V.! idx) vs in deepFilter_ f (filter (\bits -> bits V.! idx == f currentBits) vs) (idx+1)

toDecimal :: Vector Char -> Int
toDecimal chars = V.foldl' ((.|.) . flip shiftL 1) 0 (V.map (read . pure) chars)

inputFile :: IO String
inputFile = getDataFileName "sampledata/Year2021/Day3/input.txt"

readInput :: IO [Vector Char]
readInput = inputFile >>= readFile <&> map V.fromList . lines

main :: IO ()
main = do
  inputData <- readInput
  let oxygenGeneratorRating = head $ deepFilter mostCommonBit inputData
  printf "Oxygen generator rating: %s (%d)\n" (show oxygenGeneratorRating) (toDecimal oxygenGeneratorRating)
  let co2ScrubberRating = head $ deepFilter leastCommonBit inputData
  printf "CO2 scrubber rating: %s (%d)\n" (show co2ScrubberRating) (toDecimal co2ScrubberRating)
