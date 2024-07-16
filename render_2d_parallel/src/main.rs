use rayon::prelude::*;

struct Point2d {
    x: i32,
    y: i32,
}

impl Point2d {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

trait Internal: Sync + Send {
    fn is_internal(&self, point2d: &Point2d) -> bool;
}

struct BoundingBox {
    leftup: Point2d,
    rightdown: Point2d,
}

impl BoundingBox {
    fn new(leftup: Point2d, rightdown: Point2d) -> Self {
        Self { leftup, rightdown }
    }
}

impl Internal for BoundingBox {
    fn is_internal(&self, point: &Point2d) -> bool {
        if self.leftup.x <= point.x
            && point.x < self.rightdown.x
            && self.leftup.y <= point.y
            && point.y < self.rightdown.y
        {
            return true;
        }
        false
    }
}

struct Circle {
    center: Point2d,
    radius: i32,
}

impl Circle {
    fn new(center: Point2d, radius: i32) -> Self {
        Self { center, radius }
    }
}

impl Internal for Circle {
    fn is_internal(&self, point: &Point2d) -> bool {
        if (point.x - self.center.x).pow(2) + (point.y - self.center.y).pow(2) <= self.radius.pow(2)
        {
            return true;
        }
        false
    }
}

struct Canvas {
    objects: Vec<Box<dyn Internal>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Internal>) {
        self.objects.push(object);
    }
}

impl Internal for Canvas {
    fn is_internal(&self, point: &Point2d) -> bool {
        for obj in &self.objects {
            if obj.is_internal(point) {
                return true;
            }
        }
        false
    }
}

fn write_color(field: Vec<bool>) {
    for val in field {
        if val {
            println!("{} {} {}", 0, 0, 0);
        } else {
            println!("{} {} {}", 255, 255, 255);
        }
    }
}

fn render(canvas: &Canvas, image_width: i32, image_height: i32) {
    let field = (0..image_height)
        .into_par_iter()
        .flat_map(|j| {
            eprintln!("line: {:3}", j);
            let mut res = vec![false; image_width as usize];
            for i in 0..image_width {
                let point = Point2d::new(i, j);
                res[i as usize] = canvas.is_internal(&point);
            }
            res
        })
        .collect();
    write_color(field);
}

fn main() {
    let image_width = 1000;
    let image_height = 800;

    let mut canvas = Canvas::new();
    canvas.add(Box::new(BoundingBox::new(
        Point2d::new(100, 100),
        Point2d::new(300, 300),
    )));
    canvas.add(Box::new(Circle::new(Point2d::new(400, 300), 150)));

    println!("P3\n{} {}\n255", image_width, image_height);
    render(&canvas, image_width, image_height);
    eprintln!("\nDone!")
}
