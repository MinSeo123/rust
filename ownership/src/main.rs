// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{}", s)
// }

// fn change(some_string: &mut String)  {
//     some_string.push_str(", world");
// }

fn main() {
    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[3..len];

    // println!("{}", slice);

    let a =[1,2,3,4,5];
    let slice = &a[1..3];
    // for i in slice {
    //     println!("{}", i);
    // }
    println!("{:#?}", slice);
   

    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }