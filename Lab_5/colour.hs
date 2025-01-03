data Color = Red | Green | Blue  

describeColor :: Color -> String
describeColor Red = "This is Red"
describeColor Green = "This is Green"
describeColor Blue = "This is Blue"

main :: IO ()
main = do
    putStrLn "Enter a color (Red, Green, Blue):"
    input <- getLine  
    let color = case input of
                  "Red" -> Red
                  "Green" -> Green
                  "Blue" -> Blue
                  _ -> error "Invalid color"  
    putStrLn (describeColor color)  
