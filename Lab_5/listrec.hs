listLength :: [a] -> Int
listLength [] = 0 
listLength (_:xs) = 1 + listLength xs  

main :: IO ()
main = do
    putStrLn "Enter a list of elements separated by spaces:"
    input <- getLine  
    let inputList = words input  
    let result = listLength inputList  
    print result  

