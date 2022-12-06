//일반 계산식
// fn main() {
//     let length1 = 50;
//     let width1 = 30;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(length1, width1)
//     );
// }

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

//튜플 이용해 구하기
// fn main() {
//     let rect1 = (50, 30);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
//구조체 이용
// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width : u32,
// }

// fn main () {
//     let rect1 = Rectangle { length: 50, width: 30};
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
//     println!("rect1 is {:?}", rect1);
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.width
// }

//메소드 만들기

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}