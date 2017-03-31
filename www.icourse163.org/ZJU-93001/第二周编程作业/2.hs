import Prelude hiding (exp)

import Text.Printf
import Data.List

data Polynomial = Zero | Polynomial { coeff :: Int
                                    , exp :: Int
                                    , next :: Polynomial
                                    }
instance Show Polynomial where
  show Zero = "0 0"
  show a = intercalate " " $ map show (collect a)
    where
      collect :: Polynomial -> [Int]
      collect Zero = []
      collect a = coeff a:exp a:collect (next a)

buildPolynomial :: [Int] -> Polynomial
buildPolynomial [] = Zero
buildPolynomial (c:e:s) = Polynomial c e (buildPolynomial s)

add :: Polynomial -> Polynomial -> Polynomial
add a Zero = a
add Zero b = b
add a b
  | exp a > exp b = Polynomial (coeff a) (exp a) (add (next a) b)
  | exp a < exp b = Polynomial (coeff b) (exp b) (add a (next b))
  | otherwise = let
      coeff' = coeff a + coeff b
      in
      if coeff' == 0
      then add (next a) (next b)
      else Polynomial (coeff a + coeff b) (exp a) (add (next a) (next b))

pmap :: (Polynomial -> Polynomial) -> Polynomial -> Polynomial
pmap _ Zero = Zero
pmap f p = (f p){ next = pmap f (next p)}

single :: Polynomial -> Bool
single (Polynomial _ _ Zero) = True
single _ = False

multiply :: Polynomial -> Polynomial -> Polynomial
multiply a Zero = Zero
multiply Zero b = Zero
multiply a b = pmap (\p -> Polynomial (coeff a * coeff p) (exp a + exp p) undefined) b `add` multiply (next a) b

split :: String -> [String]
split "" = []
split a = let (l, r) = span  (/= ' ') (dropWhile (== ' ') a)
          in
            l:split r

main = do
  pnom1 <- getLine >>= return . (buildPolynomial . tail . map (read :: String -> Int) . split)
  pnom2 <- getLine >>= return . (buildPolynomial . tail . map (read :: String -> Int) . split)
  putStrLn $ show $ multiply pnom1 pnom2
  putStr $ show $ add pnom1 pnom2
