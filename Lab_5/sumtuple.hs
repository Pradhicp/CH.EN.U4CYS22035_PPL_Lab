sumTuple :: (Int, Int) -> Int
sumTuple (x, y) = x + y  

main :: IO ()
main = do
    putStrLn "Enter two integers separated by a space:"
    input <- getLine  -- Capture the input as a string
    let [x, y] = map read (words input) :: [Int]  -- Convert input to a list of integers
    let result = sumTuple (x, y)  -- Call sumTuple with the tuple (x, y)
    print result  -- Print the result

