sumIntegers :: Int -> Int -> Int
sumIntegers x y = x + y

main :: IO ()
main = do
    
    putStrLn "Enter the first integer:"
    x <- readLn  
    putStrLn "Enter the second integer:"
    y <- readLn 
    
    let result = sumIntegers x y
    putStrLn ("The sum of " ++ show x ++ " and " ++ show y ++ " is: " ++ show result)

