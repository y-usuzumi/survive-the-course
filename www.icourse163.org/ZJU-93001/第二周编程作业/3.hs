import Control.Monad
import Data.List
import Text.Printf

type Addr = String
type Data = String

fromList :: [(Addr, Data, Addr)] -> Addr -> [(Addr, Data, Addr)]
fromList [] _ = []
fromList a entry = case findIndex (\(addr, _, _) -> addr == entry) a of
  Nothing -> []
  Just idx -> let
    (l, curr@(addr, d, nextAddr):r) = splitAt idx a
    in
    curr:fromList (l++r) nextAddr

fixAddr :: [(Addr, Data, Addr)] -> [(Addr, Data, Addr)]
fixAddr [] = []
fixAddr [a] = [a]
fixAddr ((addr, d, next):(addr', d', next'):ls) = (addr, d, addr'):(fixAddr ((addr', d', next):ls))

reverseEvery :: Int -> [(Addr, Data, Addr)] -> [(Addr, Data, Addr)]
reverseEvery n lst
  | length lst >= n = let (l, r) = splitAt n lst in fixAddr (reverse l) ++ reverseEvery n r
  | otherwise = lst

split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

main :: IO ()
main = do
  input <- getLine
  let [entry,l,cnt] = split input
  lst <- replicateM (read l) $ do
    input <- getLine
    let [add,d,next] = split input
    return (add, d, next)
  forM_ (reverseEvery (read cnt) $ fromList lst entry) $ \(addr, d, next) ->
    printf "%s %s %s\n" addr d next
