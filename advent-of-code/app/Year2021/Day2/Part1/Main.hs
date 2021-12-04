import           Year2021.Day2.VM

type Submarine = (Int, Int)

instance VM Submarine where
  runCommand (Up a) (x, y)      = (x, y-a)
  runCommand (Down a) (x, y)    = (x, y+a)
  runCommand (Forward a) (x, y) = (x+a, y)

inputFile :: FilePath
inputFile = "input.txt"

readInput :: IO [Command Int]
readInput = map read . lines <$> readFile inputFile


main :: IO ()
main = do
  commands <- readInput
  let final = runCommands commands ((0, 0) :: Submarine)
  print final
