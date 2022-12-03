use rand::Rng;


fn main() {
    matching()
}

fn new_numb() -> [u32;6] {
    let mut arr:[u32;6] = [0,0,0,0,0,0];
    for i in 0..6 {
        let rand_numb = rand::thread_rng().gen_range(1..46);
        arr[i] = rand_numb;
    }
    arr
}

fn matching() {
    let mut count :u32 = 0;
    let newnumb :[u32;6] = new_numb();
    let user_ary :[u32;6] = [1,2,3,4,5,6];
    for i in newnumb.iter() {
         println!("{}", i);
         for j in user_ary.iter() {
            if i == j {
                count = count + 1
            }
         }
    }
    println!("맞춘개수 : {}", count);
}