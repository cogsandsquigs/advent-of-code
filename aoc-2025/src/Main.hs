module Main (main) where

import Options.Applicative

data CLI = CLI
    { day :: Int,
      part :: Int,
      test :: Bool,
      inputPath :: Maybe String
    }

cli :: Parser CLI
cli =
    CLI
        <$> argument
            auto
            ( help "The day to solve"
                <> metavar "DAY"
            )
        <*> argument
            auto
            ( help "The part of the day's problem to solve"
                <> metavar "PART"
            )
        <*> switch
            ( long "test"
                <> short 't'
                <> help "Whether to run the testing input"
            )
        <*> option
            str
            ( long "path"
                <> short 'p'
                <> help "Optional input path, otherwise is `input/day-<day num>/input.txt`"
                <> metavar "PATH"
            )

main :: IO ()
main = putStrLn "Hello, Haskell!"
