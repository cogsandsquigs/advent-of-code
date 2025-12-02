import Control.Monad
import System.IO

main :: IO ()
main = do
    result <- readAndApply "input/day-1/input.txt" part2
    print result

readAndApply :: (Show a) => String -> (String -> a) -> IO a
readAndApply path f = do
    handle <- openFile path ReadMode
    contents <- hGetContents handle
    return (f contents)

part1 :: String -> Int
part1 = snd . foldl go (50, 0) . getRots
    where
        go :: (Int, Int) -> Int -> (Int, Int)
        go (s, x) a = (s', x')
            where
                s' = (s + a) `mod` 100
                x' = if s' == 0 then x + 1 else x

part2 :: String -> Int
part2 = snd . foldl go (50, 0) . getRots
    where
        go :: (Int, Int) -> Int -> (Int, Int)
        go (s, x) a = (s', x')
            where
                (q, s') = (s + a) `divMod` 100
                x' =
                    if a >= 0 then
                        x + q
                    else
                        x + ((s - 1) `div` 100) - ((s + a - 1) `div` 100)

getRots :: String -> [Int]
getRots input = map toRot (lines input)
    where
        toRot :: String -> Int
        toRot (s : x)
            | s == 'R' = read x
            | otherwise = -(read x)
