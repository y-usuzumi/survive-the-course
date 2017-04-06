-- OJ环境的GHC编译器太裸了，缺好多东西。不过确实也有些好用的东西，比如transformers，是我以为没有
-- 而实际上应该是有的。当我意识到的时候已经太晚了。 底下这些垃圾代码就好像是命令式的代码用函数式的壳
-- 包装了一下。参考价值有限。。。

{-# LANGUAGE NamedFieldPuns #-}
{-# LANGUAGE RecordWildCards #-}
{-# LANGUAGE TupleSections   #-}

import           Control.Applicative
import           Control.Monad
import           Data.Array
import           Data.Array.IO
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.Char
import           Data.Function
import           Data.List             (foldl')
import           Data.Maybe

type Size = Int
type HeapArray = IOArray Int Huffman
type CharMap = IOArray Int Int

data Huffman = HLeaf { weight :: Int
                     , char   :: Char
                     }
             | HTree { weight :: Int
                     , left   :: Huffman
                     , right  :: Huffman
                     }
             | HNil
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
      return (ret , h{ size = size'})
        where
          adjust :: HeapArray -> Huffman -> Int -> Int -> IO ()
          adjust heap huff size idx
            | idx > size = return ()
            | otherwise = do
                let nextIdx = idx * 2
                (minNode, nextIdx') <- case () of _
                                                    | nextIdx > size -> return (huff, nextIdx)
                                                    | nextIdx < size - 1 -> do
                                                        node1 <- readArray heap nextIdx
                                                        node2 <- readArray heap (nextIdx+1)
                                                        return $ if weight node1 < weight node2
                                                                 then (node1, nextIdx)
                                                                 else (node2, nextIdx+1)
                                                    | otherwise -> (,nextIdx) <$> readArray heap nextIdx
                if (weight minNode >= weight huff)
                  then do
                  writeArray heap idx huff
                  else do
                  writeArray heap idx minNode
                  adjust heap huff size nextIdx'

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

wpl :: Huffman -> Int
wpl huff = _wpl [huff] 0 0
  where
    _wpl [] _ b = b
    _wpl l a b  = sum (map (_w a) l) + b
    _w depth HLeaf{..} = depth * weight
    _w depth HTree{..} = _wpl [left, right] (depth+1) 0
    _w depth HNil = 0

parseWCPairs :: BS.ByteString -> [(Int, Char)]
parseWCPairs bs = chunksOfWCPairs (C8.words bs)
  where
    chunksOfWCPairs [] = []
    chunksOfWCPairs (a:b:s) = (fst $ fromJust $ C8.readInt b, C8.head a):chunksOfWCPairs s

newCharMap :: IO CharMap
newCharMap = newArray (0, 127) 0

writeCharMap :: CharMap -> Char -> Int -> IO ()
writeCharMap cm c weight = writeArray cm (ord c) weight

lookupChar :: CharMap -> Char -> IO Int
lookupChar cm c = readArray cm (ord c)

parseSubmit :: CharMap -> [(Char, BS.ByteString)] -> IO (Maybe Huffman)
parseSubmit cm inputs = foldM (modifyHuff cm) (Just HNil) inputs
  where
    modifyHuff :: CharMap -> Maybe Huffman -> (Char, BS.ByteString) -> IO (Maybe Huffman)
    modifyHuff _ Nothing _ = return Nothing
    modifyHuff cm (Just huff) (c, path)
      | BS.null path = do
          weight <- lookupChar cm c
          return $ if weight == 0
                   then Nothing
                   else case huff of
                          HNil -> Just HLeaf{ weight
                                            , char = c
                                            }
                          HLeaf{..} -> Nothing
                          HTree{..} -> Nothing
      | Just (h, r) <- C8.uncons path
      , otherwise = do
          weight <- lookupChar cm c
          if weight == 0
            then return Nothing
            else case huff of
                   HNil -> do
                     newHuff <- modifyHuff cm (Just HNil) (c, r)
                     return $ newHuff >>= \t -> case h of
                                                  '0' -> Just HTree{ weight = 0
                                                                   , left = t
                                                                   , right = HNil
                                                                   }
                                                  '1' -> Just HTree{ weight = 0
                                                                   , left = HNil
                                                                   , right = t
                                                                   }
                   HLeaf{..} -> return Nothing
                   ht@HTree{..} -> case h of
                     '0' -> do
                       newLeft <- modifyHuff cm (Just left) (c, r)
                       return $ newLeft >>= \t -> Just ht{ left = t
                                                         }
                     '1' -> do
                       newRight <- modifyHuff cm (Just right) (c, r)
                       return $ newRight >>= \t -> Just ht{ right = t
                                                          }

main = do
  numOfChars <- fst . fromJust . C8.readInt <$> BS.getLine
  inputs <- parseWCPairs <$> BS.getLine
  cm <- newCharMap
  forM inputs $ \(w, c) -> do
    writeCharMap cm c w
  h <- newHeap 64
  h' <- foldM (flip (insert . uncurry HLeaf)) h inputs
  huff <- buildHuffman h'
  -- print huff
  let leastWPL = wpl huff
  cnt <- readLn
  replicateM cnt $ do
    submit <- map ((\(c:p:_) -> (C8.head c, p)) . C8.words) <$> replicateM numOfChars BS.getLine
    huff <- parseSubmit cm submit
    case huff of
      Nothing -> putStrLn "No"
      Just huff -> putStrLn $ if wpl huff == leastWPL then "Yes" else "No"
