#[derive(Debug)]
pub struct Point2D {
    x: f64,
    y: f64,
}

pub fn take_ownership(p: Point2D) {
    println!("The point is {:?}", p)
    // Rust's compiler implicitly adds `drop(p)` which physically frees the memory held by `p`
}

pub fn borrow_point_immutably(p: &Point2D) {
    println!("The point is {:?}", p)
}

pub fn borrow_point_mutably(p: &mut Point2D) {
    println!("The point is {:?}", p);
    p.x = 3.;
    println!("New point is {:?}", p);
}

#[derive(Copy, Clone, Debug)]
pub struct RGBColor(u8, u8, u8);

pub fn show_color(color: RGBColor) {
    println!("The color is {:?}", color)
}

#[derive(Debug)]
pub struct Palette<'a> {
    colors: &'a [RGBColor],
}

pub fn show_palette(palette: &Palette) {
    println!("Color palette: {:?}", palette);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Allocate `Point2D` data on current stack frame and alias it immutably as `p`
        let point = Point2D { x: 4., y: 2. };
        // Move the ownership of data held by `p` to function `take_ownership`. This
        take_ownership(point);

        // Because the memory held by `point` has been dropped inside `take_ownership`, Rust
        // prevents any further usage of `point` (use after free / double free)!

        // Allocate `Point2D` data on current stack frame and alias it immutably as `p1`
        let p1 = Point2D { x: 1., y: 2. };
        // This time we borrow a *shared reference* (basically a fat pointer monitored by Rust's
        // Borrow Checker) to function `borrow_point_immutably`. The function gets a read-only
        // access to `p1`'s memory and returns it at the end.
        borrow_point_immutably(&p1);
        borrow_point_immutably(&p1);
        borrow_point_immutably(&p1);

        // Move memory ownership from `p1` to mutable alias `p2`
        let mut p2 = p1;
        // Finally, a function can also borrow some value mutably as an *exclusive reference* and
        // make a side-effect on these. There can either be multiple shared references to a piece
        // of memory or single exclusive reference (cannot read and write at the same time).
        borrow_point_mutably(&mut p2);

        // Because the memory has been borrowed (even though mutably), this call compiles.
        borrow_point_mutably(&mut p2);

        // `assert_eq!(3., p1.x)` won't compile because the ownership of memory held by `p1`
        // has been moved to `p2`
        assert_eq!(3., p2.x);
    }

    #[test]
    fn call_by_value() {
        let color = RGBColor(128, 0, 128);

        // Function `show_color` takes ownership of the colour argument. However, `RGBColor` struct
        // derives `Copy` trait which means that this struct is easy and fast to copy and pass by
        // value (this is how primitive types such as integers and floats are handled).
        show_color(color);

        // Because the ownership has not been moved from `colour` (rather a copy has been created
        // during previous function call), following call is safe and thus compiles.
        show_color(color);
    }

    #[test]
    fn lifetimes() {
        let colors = vec![RGBColor(1, 1, 1), RGBColor(2, 2, 2)];

        // Lifetimes define the minimum span of how long a piece of memory must remain valid so
        // that it does not outlive some other memory it references.
        {
            // In this example a palette references a vector of colors. This works because we've
            // bound the palette lifetime to the lifetime of the color vector.
            let palette = Palette { colors: &colors };

            // Releasing colors (e.g. with an explicit `drop(colors);`) before this call would be
            // prevented by the Borrow Checker for lifetime validation.
            show_palette(&palette);

            // At this point palette is dropped.
        }

        // Even though the palette was freed, `colors` is still points to valid memory.
        for color in colors.into_iter() {
            show_color(color);
        }
    }
}
