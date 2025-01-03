describePair :: (Int, Int) -> String
describePair (0, 0) = "Origin"  
describePair (0, _) = "X-Axis"  
describePair (_, 0) = "Y-Axis"  
describePair (_, _) = "Other"  

main :: IO ()
main = do
    putStrLn "Enter a pair of integers (separated by space):"
    input <- getLine  
    let [x, y] = map read (words input) :: [Int] 
    let result = describePair (x, y)  
    putStrLn result  

