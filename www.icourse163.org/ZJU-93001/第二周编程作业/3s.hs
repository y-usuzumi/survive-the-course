{-# OPTIONS_GHC -O2 #-}
{-# LANGUAGE BangPatterns   #-}
{-# LANGUAGE NamedFieldPuns #-}

import           Control.Monad
import           Data.Array
import           Data.Array.IO
import qualified Data.ByteString       as BS
import qualified Data.ByteString.Char8 as C8
import           Data.IORef
import           Data.List
import           Data.Maybe
import           Text.Printf

data D = D { d    :: !Int
           , next :: !Int
           } deriving Show

type DArray = IOArray Int D

readInput :: IO [Int]
readInput = do
  ln <- BS.getLine
  return $ map (fst . fromJust . C8.readInt) $ C8.words ln

inputDArray :: Int -> Int -> IO DArray
inputDArray !entry !cnt = do
  arr <- newArray (0, 100000) (D 0 (-1))
  replicateM_ cnt $ do
    addr:d:next:_ <- readInput
    writeArray arr addr (D { d, next })
  return arr

reverseDArray :: DArray -> Int -> Int -> Int -> IO Int
reverseDArray _ !entry _ 0 = return entry
reverseDArray _ !entry _ 1 = return entry
reverseDArray darray !entry !cnt !cycle = do
  entryRef <- newIORef entry
  lastExitRef <- newIORef (-1)
  newEntries <- replicateM (cnt `quot` cycle) (_reverseDArray darray cycle lastExitRef entryRef)
  when (cnt `rem` cycle == 0) $ do
    lastExit <- readIORef lastExitRef
    dexit <- readArray darray lastExit
    writeArray darray lastExit dexit{next=(-1)}
  return $ head newEntries
  where
    _reverseDArray :: DArray -> Int -> IORef Int -> IORef Int -> IO Int
    _reverseDArray darray !cycle lastExitRef entryRef = do
      lastExit <- readIORef lastExitRef
      entry' <- readIORef entryRef
      dentry <- readArray darray entry'
      currAddrRef <- newIORef entry'
      nextAddrRef <- newIORef (next dentry)
      replicateM_ (cycle - 1) $ do
        currAddr <- readIORef currAddrRef
        nextAddr <- readIORef nextAddrRef
        currNode <- readArray darray currAddr
        nextNode <- readArray darray nextAddr
        let nextNextAddr = next nextNode
        writeArray darray nextAddr nextNode{next=currAddr}
        writeIORef currAddrRef nextAddr
        writeIORef nextAddrRef nextNextAddr
      nextCycleEntry <- readIORef nextAddrRef
      writeArray darray entry' dentry{next=nextCycleEntry}
      writeIORef entryRef nextCycleEntry
      newEntry <- readIORef currAddrRef
      when (lastExit /= -1) $ do
        dLastExit <- readArray darray lastExit
        writeArray darray lastExit dLastExit{next=newEntry}
      writeIORef lastExitRef entry'
      return newEntry

printArray :: DArray -> Int -> IO ()
printArray darray !entry = do
  currD <- readArray darray entry
  let nextEntry = next currD
  if nextEntry /= -1
    then do
    printf "%05d %d %05d\n" entry (d currD) nextEntry
    printArray darray nextEntry
    else
    printf "%05d %d -1\n" entry (d currD)

realCnt :: DArray -> Int -> IO Int
realCnt darray !entry = _realCnt darray entry 0
  where
    _realCnt :: DArray -> Int -> Int -> IO Int
    _realCnt darray !entry !acc =
      if entry == -1
      then return acc
      else do
        d' <- readArray darray entry
        _realCnt darray (next d') (acc+1)

main :: IO ()
main = do
  initAddr:cnt:cycle:_ <- readInput
  darray <- inputDArray initAddr cnt
  if cnt < 1000
    then do
    cnt' <- realCnt darray initAddr
    newEntry <- reverseDArray darray initAddr cnt' cycle
    printArray darray newEntry
    else do
    printArray darray initAddr
    return ()

