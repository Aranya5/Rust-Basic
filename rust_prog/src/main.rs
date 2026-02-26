
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    fn display(&self)-> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let x: i8 = 5;
    let y: i8 = 10;
    let _a: i32;
    let mut z: i8;

    for i in 0..10 {
        z = x + y + i;
        print!("{} ", z);
    }

    let sentence = String::from("This is a sentence.");
    println!("\n{}", sentence);

    println!(
        "The area of a rectangle with length {} and width {} is: {}",x,y,give_area(x as i32, y as i32)
    );

    fn give_area(length: i32, width: i32) -> i32 {
        length * width
    }

    println!("Hello, world! {}", x + y);


    //Borrowing and references
    let mut str= String::from("Hello, Rust!");
    println!("{}", str);
    update(&mut str);
    println!("{}", str);

    //Structs and methods
    let point1 = Point { x: 3, y: 4 };
    println!("Distance of point {} from origin: {}",point1.display(), point1.distance_from_origin());

}

fn update(s: &mut String) {
    s.push_str(" This is update function.");
}
