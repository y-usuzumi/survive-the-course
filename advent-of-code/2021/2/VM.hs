module VM where

import Data.List

data Command a = Up a | Forward a | Down a
               deriving Show

readCommand :: Read a => ReadS (Command a)
readCommand s = do
  (cmd, ss) <- lex s
  (steps, sss) <- reads ss
  case cmd of
    "forward" -> return (Forward steps, sss)
    "up" -> return (Up steps, sss)
    "down" -> return (Down steps, sss)
    _ -> []

instance Read a => Read (Command a) where
  readsPrec _ "" = []
  readsPrec _ s = readCommand s

class VM vm where
  runCommand :: Command Int -> vm -> vm

runCommands :: VM vm => [Command Int] -> vm -> vm
runCommands = flip $ foldl' (flip runCommand)
