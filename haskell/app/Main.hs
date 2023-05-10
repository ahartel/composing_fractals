module Main where


type Point = (Float, Float)

next :: Point -> Point -> Point
next (u, v) (x , y) = (x * x - y * y + u, 2 * x * y + v)

mandelbrot :: Point -> [Point]
mandelbrot p = iterate (next p) (0, 0)

fairlyClose :: Point -> Bool
fairlyClose (u, v) = (u * u + v * v) < 100

inMandelbrotSet :: Point -> Bool
inMandelbrotSet p = all fairlyClose (mandelbrot p)

approxTest :: Int -> Point -> Bool
approxTest n p = all fairlyClose (take n (mandelbrot p))

-- Notice that this definition is polymorphic: the identifier color appearing in the type
-- is a type variable, so it can be instantiated to different types in different settings.
chooseColor :: [color ] -> [Point] -> color
chooseColor palette = (palette !!). length . take n . takeWhile fairlyClose
  where n = length palette - 1

type Image color = Point -> color

fracImage :: (Point -> [Point]) -> [color ] -> Image color
fracImage fractal palette = chooseColor palette . fractal

type Grid a = [[a]]

grid :: Int -> Int -> Point -> Point -> Grid Point
grid c r (xmin, ymin) (xmax , ymax )= [[(x , y) | x <- for c xmin xmax ] | y <- for r ymin ymax ]

for :: Int -> Float -> Float -> [Float]
for n min max = take n [min, min + delta ..]
  where delta = (max - min)/fromIntegral (n - 1)

sample :: Grid Point -> Image color -> Grid color
sample points image = map (map image) points

draw :: Grid Point -> (Point -> [Point]) -> [color ] -> (Grid color -> image) -> image
draw points fractal palette render = render (sample points (fracImage fractal palette))

charPalette :: [Char ]
charPalette = " ,.'\"~:;o-!|?/<>X+={^O#%&@8*$"

charRender :: Grid Char -> IO ()
charRender = putStr . unlines

figure1 = draw points mandelbrot charPalette charRender
  where points = grid 79 37 (-2.25, -1.5) (0.75, 1.5)

julia :: Point -> Point -> [Point]
julia c = iterate (next c)

figure2 = draw points (julia (0.32, 0.043)) charPalette charRender
  where points = grid 79 37 (-1.5, -1.5) (1.5, 1.5)


main :: IO ()
main = figure1
