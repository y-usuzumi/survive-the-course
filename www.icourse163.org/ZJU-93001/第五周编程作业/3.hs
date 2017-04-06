{-# LANGUAGE RecordWildCards #-}

import           Control.Applicative
import           Control.Monad
import           Data.Array
import           Data.Array.IO
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.Maybe

type Size = Int
type HeapArray = IOArray Int Huffman

data Huffman = HLeaf { weight :: Int
                     , char   :: Char
                     }
             | HTree { weight :: Int
                     , left   :: Huffman
                     , right  :: Huffman
                     }
             deriving Show

data Heap = Heap { size :: Int
                 , heap :: HeapArray
                 }

newHeap :: Size -> IO Heap
newHeap s = Heap 0 <$> newArray (1, s) (HLeaf 0 ' ')

insert :: Huffman -> Heap -> IO Heap
insert huff h@Heap{..} = do
  let idx = size + 1
  writeArray heap idx huff
  adjust heap huff idx
  return h{ size = size + 1
          }
  where
    adjust :: HeapArray -> Huffman -> Int -> IO ()
    adjust heap huff 1 = writeArray heap 1 huff
    adjust heap huff idx
      | idx < 1 = error "You idiot"
      | otherwise = do
          let nextIdx = idx `quot` 2
          node <- readArray heap nextIdx
          if (weight node < weight huff)
            then do
            writeArray heap idx huff
            else do
            writeArray heap idx node
            adjust heap huff nextIdx

pop :: Heap -> IO (Huffman, Heap)
pop h@Heap{..}
  | size <= 0 = error "You idiot @ pop"
  | otherwise = do
      ret <- readArray heap 1
      huff <- readArray heap size
      let size' = size - 1
      adjust heap huff size' 1
      putStrLn "NEW heap"
      take size' <$> getElems heap >>= print
      return (ret , h{ size = size'})
        where
          adjust :: HeapArray -> Huffman -> Int -> Int -> IO ()
          adjust heap huff size idx
            | idx > size = return ()
            | otherwise = do
                let nextIdx = idx * 2
                node1 <- readArray heap nextIdx
                node2 <- readArray heap (nextIdx+1)
                if (weight node > weight huff)
                  then do
                  writeArray heap idx huff
                  else do
                  writeArray heap idx node
                  adjust heap huff size nextIdx

buildHuffman :: Heap -> IO Huffman
buildHuffman h@Heap{..} = do
  h' <- foldM build h [1..size - 1]
  readArray heap 1
  where
    build h _ = do
      (huff1, h') <- pop h
      (huff2, h'') <- pop h'
      let t = HTree (weight huff1 + weight huff2) huff1 huff2
      insert t h''

parseWCPairs :: BS.ByteString -> [(Int, Char)]
parseWCPairs bs = chunksOfWCPairs (C8.words bs)
  where
    chunksOfWCPairs [] = []
    chunksOfWCPairs (a:b:s) = (fst $ fromJust $ C8.readInt b, C8.head a):chunksOfWCPairs s
main = do
  _ <- BS.getLine
  inputs <- parseWCPairs <$> BS.getLine
  h <- newHeap 64
  h' <- foldM (flip (insert . uncurry HLeaf)) h inputs
  huff <- buildHuffman h'
  print huff
