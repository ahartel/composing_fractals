use std::iter::successors;

#[derive(Clone)]
struct Point(f64, f64);

// 2 learnings:
// 1. Haskell implicitly deconstructs and constructs types
// 2. Haskell's type inference gets a 2 as a float
// Both are very convenient
fn next(Point(u, v): &Point, Point(x, y): &Point) -> Point {
    Point(x * x - y * y + u, 2.0 * x * y + v)
}

// learning: Haskell evaluates lazily, Rust does not
// Therefore, mandelbrot already needs a parameter for how
// often to iterate
fn mandelbrot(p: Point) -> Box<dyn Iterator<Item = Point>> {
    Box::new(successors(Some(Point(0.0, 0.0)), move |cur| {
        Some(next(&p, cur))
    }))
}

fn fairly_close(Point(u, v): &Point) -> bool {
    (u * u + v * v) < 100.0
}

fn _approx_test(n: usize, p: Point) -> bool {
    mandelbrot(p).take(n).all(|p| fairly_close(&p))
}

fn choose_color<T: Clone>(palette: &[T], points: Box<dyn Iterator<Item = Point>>) -> T {
    let n = palette.len() - 1;
    let palette_index = points.take_while(fairly_close).take(n).count();
    palette[palette_index].clone()
}

fn frac_image<T: Clone>(
    fractal: impl Fn(Point) -> Box<dyn Iterator<Item = Point>>,
    start: Point,
    palette: &[T],
) -> T {
    choose_color(palette, fractal(start))
}

type Grid<T> = Vec<Vec<T>>;

fn _for(steps: usize, min: f64, max: f64) -> impl Iterator<Item = f64> + Clone {
    let delta = (max - min) / (steps - 1) as f64;
    successors(Some(min), move |cur| Some(cur + delta)).take(steps)
}

fn grid(
    x_steps: usize,
    y_steps: usize,
    Point(xmin, ymin): &Point,
    Point(xmax, ymax): &Point,
) -> Grid<Point> {
    _for(y_steps, *ymin, *ymax)
        .map(|y| {
            _for(x_steps, *xmin, *xmax)
                .map(|x| Point(x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn sample<T>(grid: Grid<Point>, image_gen: impl Fn(&Point) -> T) -> Grid<T> {
    grid.iter()
        .map(|row| row.iter().map(&image_gen).collect())
        .collect()
}

fn draw<T: Clone>(
    grid: Grid<Point>,
    fractal: impl Fn(Point) -> Box<dyn Iterator<Item = Point>>,
    palette: &[T],
    render: impl Fn(Grid<T>),
) {
    render(sample(grid, |p| frac_image(&fractal, p.clone(), palette)))
}

fn char_render(grid: Grid<char>) {
    grid.iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()))
}

fn main() {
    let char_palette: Vec<char> = " ,.'\"~:;o-!|?/<>X+={^O#%&@8*$".chars().collect();
    let grid = grid(79, 37, &Point(-2.25, -1.5), &Point(0.75, 1.5));
    draw(grid, mandelbrot, &char_palette, char_render);
}
