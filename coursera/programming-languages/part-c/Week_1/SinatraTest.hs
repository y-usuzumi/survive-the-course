{-# LANGUAGE TemplateHaskell #-}

import Sinatra

get "/" $ do
  doSomething "Hello"
  doSomething "World"

main :: IO ()
main = putStrLn "Hello world!"
