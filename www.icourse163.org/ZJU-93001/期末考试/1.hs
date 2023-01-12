import           Control.Monad
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.Char
import           Data.Functor
import           Data.IORef
import           Data.List
import           Data.Maybe

data BinTree a = Nil | BinTree a (BinTree a) (BinTree a) deriving Show

readInput :: IO [Int]
readInput = map (fst . fromJust . C8.readInt) . C8.words <$> BS.getLine

buildPreFromPostAndIn :: [Int] -> [Int] -> IO ()
buildPreFromPostAndIn postOrder inOrder = do


main = do
  undefined
