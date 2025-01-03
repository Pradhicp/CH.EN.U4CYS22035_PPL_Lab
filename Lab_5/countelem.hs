countElements :: [a] -> Int
countElements [] = 0  
countElements (_:xs) = 1 + countElements xs  

main :: IO ()
main = do
    putStrLn "Enter a list of numbers separated by spaces:"
    input <- getLine  
    let numList = map read (words input) :: [Int] 
    let result = countElements numList
    print result  

