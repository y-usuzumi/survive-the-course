import           Year2021.Day2.VM

data Submarine = Submarine (Int, Int) Int
               deriving Show

instance VM Submarine where
  runCommand cmd (Submarine (x, y) aim) = case cmd of
    Forward steps -> Submarine (x+steps, y+aim*steps) aim
    Up steps      -> Submarine (x, y) (aim-steps)
    Down steps    -> Submarine (x, y) (aim+steps)

inputFile :: FilePath
inputFile = "input.txt"

readInput :: IO [Command Int]
readInput = map read . lines <$> readFile inputFile


main :: IO ()
main = do
  commands <- readInput
  let finalSubmarine = runCommands commands (Submarine (0, 0) 0)
  print finalSubmarine
