import System.Random
import qualified Data.Map.Strict as Map
import qualified Data.Sequence as Seq
import Data.List (elemIndex, delete, minimumBy)
import Data.Ord (comparing)
import Control.Monad (replicateM)
import Data.Maybe (fromJust)

-- Utility functions for distributions
harmonic :: Int -> Double
harmonic n = sum [1 / fromIntegral i | i <- [1..n]]

biharmonic :: Int -> Double
biharmonic n = sum [1 / fromIntegral (i*i) | i <- [1..n]]

uniformDist :: Int -> [Double]
uniformDist n = replicate n (1 / fromIntegral n)

harmonicDist :: Int -> [Double]
harmonicDist n = [1 / (fromIntegral i * hn) | i <- [1..n]]
  where hn = harmonic n

biharmonicDist :: Int -> [Double]
biharmonicDist n = [1 / (fromIntegral (i*i) * hhn) | i <- [1..n]]
  where hhn = biharmonic n

geometricDist :: Int -> [Double]
geometricDist n = init probs ++ [lastProb]
  where probs = [1 / (2 ^^ i) | i <- [1..n-1]]
        lastProb = 1 / (2 ^^ (n - 1))

-- Sampling a page based on probability distribution
samplePage :: [Double] -> IO Int
samplePage probs = do
  r <- randomRIO (0.0, 1.0)
  let cum = scanl1 (+) probs
  return $ length (takeWhile (< r) cum) + 1

-- Cache replacement algorithms
simulate :: Int -> Int -> [Double] -> String -> Int -> IO Double
simulate n k probs algo steps = go [] Map.empty Map.empty 0 0
  where
    go _ _ _ faults 0 = return (fromIntegral faults / fromIntegral steps)
    go cache freqMap markMap faults remaining = do
      page <- samplePage probs
      let inCache = page `elem` cache
      let newFreqMap = Map.insertWith (+) page 1 freqMap
      let (newCache, newMarkMap, pageFault) = case algo of
            "FIFO" -> if inCache
                        then (cache, markMap, 0)
                        else (take k (page : cache), markMap, 1)
            "FWF"  -> if length cache < k
                        then if inCache then (cache, markMap, 0)
                                        else (page : cache, markMap, 1)
                        else ([page], Map.empty, 1)
            "LRU"  -> if inCache
                        then (page : delete page cache, markMap, 0)
                        else let new = if length cache >= k then init cache else cache
                             in (page : new, markMap, 1)
            "LFU"  -> if inCache
                        then (cache, markMap, 0)
                        else let evict = if length cache < k then [] else [fst $ minimumBy (comparing snd) [(p, Map.findWithDefault 0 p freqMap) | p <- cache]]
                                 new = page : foldl (flip delete) cache evict
                             in (new, markMap, 1)
            "RAND" -> if inCache
                        then (cache, markMap, 0)
                        else if length cache < k
                              then (page : cache, markMap, 1)
                              else do
                                i <- randomRIO (0, k - 1)
                                let new = take i cache ++ drop (i+1) cache
                                return (page : new, markMap, 1)
            "RMA"  -> if inCache
                        then (cache, Map.insert page True markMap, 0)
                        else if length cache < k
                              then (page : cache, Map.insert page False markMap, 1)
                              else let unmarked = [p | p <- cache, not (Map.findWithDefault False p markMap)]
                                       pickList = if null unmarked then cache else unmarked
                                   in do
                                     i <- randomRIO (0, length pickList - 1)
                                     let evict = pickList !! i
                                         new = page : delete evict cache
                                     return (new, Map.insert page False (Map.delete evict markMap), 1)
            _ -> error "Unknown algorithm"
      case newCache of
        (IO c, m, pf) -> c >>= \nc -> go nc newFreqMap m (faults + pf) (remaining - 1)
        _             -> go newCache newFreqMap newMarkMap (faults + pageFault) (remaining - 1)

-- Main experiment runner
main :: IO ()
main = do
  putStrLn "Running simulations..."
  let ns = [20,30..100]
      trials = 10000
      algos = ["FIFO", "FWF", "LRU", "LFU", "RAND", "RMA"]
      distributions = [("uniform", uniformDist), ("harmonic", harmonicDist), ("biharmonic", biharmonicDist), ("geometric", geometricDist)]
  mapM_ (\n -> do
    let ks = [n `div` 10 .. n `div` 5]
    mapM_ (\(distName, distGen) -> do
      let probs = distGen n
      mapM_ (\k -> do
        mapM_ (\algo -> do
          avg <- simulate n k probs algo trials
          putStrLn $ unwords ["n =", show n, "k =", show k, "dist =", distName, "algo =", algo, "avg =", show avg]
          ) algos) ks) distributions) ns
