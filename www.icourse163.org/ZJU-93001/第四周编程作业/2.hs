import           Control.Applicative
import qualified Data.List           as L

split :: String -> [String]
split "" = [""]
split (' ':as) = "":split as
split (a:as) = let
  fst:others = split as
  in
    (a:fst):others

type BalanceFactor = Int
data AVL a = Nil | AVL { d  :: !a
                       , bf :: !BalanceFactor
                       , l  :: !(AVL a)
                       , r  :: !(AVL a)
                       } deriving Show

getBF :: AVL a -> Int
getBF Nil = 0
getBF avl = bf avl

insert :: Ord a => a -> AVL a -> AVL a
insert a Nil = AVL a 0 Nil Nil
insert a avl@(AVL d _ Nil _)
  | a < d = avl{ bf = getBF avl - 1
               , l = insert a Nil
               }
insert a avl@(AVL d _ _ Nil)
  | a > d = avl{ bf = getBF avl + 1
               , r = insert a Nil
               }
insert a avl
  | a < d avl = let l' = insert a (l avl)
                    bfl = getBF (l avl)
                    bfl' = getBF l'
                in
                  rotate avl{ bf = if bfl' == 0
                                   then getBF avl
                                   else if bfl == bfl' then getBF avl else getBF avl - 1
                            , l = l'
                            }
  | a > d avl = let r' = insert a (r avl)
                    bfr = getBF (r avl)
                    bfr' = getBF r'
                in
                  rotate avl{ bf = if bfr' == 0
                                   then getBF avl
                                   else if bfr == bfr' then getBF avl else getBF avl + 1
                            , r = r'
                            }
  where
    height Nil = 0
    height avl = 1 + max (height (l avl)) (height (r avl))
    updateBF Nil = Nil
    updateBF avl = avl { bf = height (r avl) - height (l avl)
                       }
    rotate avl = let
      bfavl = getBF avl
      bfravl = getBF (r avl)
      bflavl = getBF (l avl)
      in case () of _
                      | bfavl == 2 && bfravl == 1 -> rotateRR avl
                      | bfavl == -2 && bflavl == -1 -> rotateLL avl
                      | bfavl == 2 && bfravl == -1 -> rotateRL avl
                      | bfavl == -2 && bflavl == 1 -> rotateLR avl
                      | otherwise -> avl
    rotateLL avl = let branch = r (l avl)
                   in (l avl){ bf = 0
                             , r = avl{ l = branch
                                      , bf = 0
                                      }
                             }
    rotateRR avl = let branch = l (r avl)
                   in (r avl){ bf = 0
                             , l = avl{ r = branch
                                      , bf = 0
                                      }
                          }
    rotateLR avl = let lavl = l (avl)
                       branch = r lavl
                       lbranch = l branch
                       rbranch = r branch
                   in branch { bf = 0
                             , l = lavl { bf = case lbranch of
                                                 Nil -> getBF lavl - 1
                                                 _   -> getBF lavl
                                        , r = lbranch
                                        }
                             , r = updateBF avl { l = rbranch
                                                }
                             }
    rotateRL avl = let ravl = r (avl)
                       branch = l ravl
                       lbranch = l branch
                       rbranch = r branch
                   in branch { bf = 0
                             , l = avl { bf = case lbranch of
                                                 Nil -> case l avl of
                                                          Nil -> 0
                                                          _   -> -1
                                                 _ -> 0
                                        , r = lbranch
                                        }
                             , r = updateBF ravl { l = rbranch
                                                 }
                             }


main = do
  _ <- getLine
  nums <- map (read :: String -> Int) . split <$> getLine
  let tree = L.foldl' (flip insert) Nil nums
  print $ d tree
