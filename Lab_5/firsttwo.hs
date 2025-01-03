firstTwoElements :: [a] -> [a]
firstTwoElements [] = [] 
firstTwoElements [x] = [x]  
firstTwoElements (x:y:_) = [x, y]  

main :: IO ()
main = do
    putStrLn "Enter a list of elements separated by spaces:"
    input <- getLine  
    let inputList = words input 
    let result = firstTwoElements inputList 
    print result  

