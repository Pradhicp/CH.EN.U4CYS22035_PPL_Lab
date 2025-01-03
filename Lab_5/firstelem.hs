firstElement :: Show a => [a] -> String 
firstElement [] = "Empty list"  
firstElement (x:_) = "First element is " ++ show x 

main :: IO ()
main = do
    putStrLn "Enter a list of elements separated by spaces:"
    input <- getLine
    let inputList = words input 
    let result = firstElement inputList  
    putStrLn result 

