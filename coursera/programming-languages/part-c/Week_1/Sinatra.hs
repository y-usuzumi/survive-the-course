{-# LANGUAGE TemplateHaskell #-}
{-# LANGUAGE QuasiQuotes #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE DeriveFunctor #-}

module Sinatra where

import Control.Monad
import Language.Haskell.TH
import Language.Haskell.TH.Syntax

class RouteDef r where

instance RouteDef String

-- TODO
newtype Response a  = Response a deriving Functor

instance Applicative Response where
  pure a = Response a
  (<*>) = ap

instance Monad Response where
  (Response a) >>= f = f a

doSomething :: a -> Response a
doSomething a = return a

get :: RouteDef r => r -> Response String -> Q [Dec]
get _ _ = error "Not implemented"
