//정수형, 부동소수점 숫자, boolean, 그리고 문자 > 스칼라
//복합 타입들 ( 배열, 객체, 튜플 ) > 컴파운드


fn main() {
    // levarry()
}

// fn another_function (x :i32, y: i32) {
//     println!("value : {}", x);
//     println!("value : {}", y);
//     let sum = x + y;
//     println!("sum : {}", sum);
// }

//구문식 표현식
// fn zz () {
//     let x = 5;
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("x value is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn ifandelse () {
//     let number = 3;

//     if number < 5 {
//         println!("codition was true");
//     } else {
//         println!("condition was false");
//     }
// } 

// fn letif () {
//     let codition = true;
//     let number = if codition {
//         5
//     } else {
//         6
//     } ; 
//     println!("The value is : {}", number)
// }

// fn looop () {
//     loop {
//         println!("again!");
//     }
// }

// fn whileloop () {
//     let mut number = 3;

//     while number !=0 {
//         println!("{}!", number);

//         number = number - 1;
//     }
//     println!("LIFTOFF!");
// }

// fn arryloop () {
//     let a = [10,20,30,40,50];
//     let mut index = 0;

//     while index < 5 {
//         println!("value : {}", a[index]);
//         index = index + 1;
//     }
// }

// fn forarray () {
//     let a = [10,20,30,40,50];
//     for element in a.iter() {
//         println!("the value is: {}", element)
//     }
// }

fn levarry() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}