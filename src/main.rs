#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::area(&rect1)
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::area(&rect2)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect2 is {:?}", rect2);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[test]
fn test_area() {
    let rect = Rectangle {
        width: 3,
        height: 4,
    };
    assert_eq!(rect.area(), 12);
}
