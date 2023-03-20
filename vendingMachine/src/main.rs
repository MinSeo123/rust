use std::io;

#[derive(Debug)]
struct Beverage {
    id : String,
    name : String,
    price : u64,
    count : u64
}
impl Beverage {
    fn buy_beverage(&mut self , id : &str , money : &str) -> u64 {
        //물품체크
        let mut money_int : u64 = money.parse().unwrap();
        println!("들어온돈{}", money_int);
        if self.id == id {
                    //돈체크      
            if money_int >= self.price {
                       //수량체크
                if self.count > 0 {
                    self.count = self.count - 1;
                    money_int = money_int - self.price;
                    println!("남은돈 : {}", money_int); 
                } else {
                    println!("수량 부족");
                };
            } else {
                println!("돈 부족함");
            };
        } else {
            println!("존재하지않는 제품");
        };
        money_int
    }
}
#[derive(Debug)]
struct Buyer {
    money : String,
}


fn main() {
    let mut coke = put_beverage("1", "콜라", 300, 5);
    let mut orange = put_beverage("2", "오렌지주스", 500, 5);
    println!("콜라구매");
    coke.buy_beverage("1", "500");
    // println!("{:?}", coke);
    // println!("{:?}", orange);

    // println!("돈을 넣어주세요");
    // let mut money = String::new();
    // io::stdin().read_line(&mut money)
    //     .expect("Failed to read line");
    // let minseo = input_money(&money);
    // println!("입력하신 돈은 {} 원 입니다.\n" , money);
    
    
    // let mut buy = String::new();
    // println!("무엇을 구매하시겠습니까 ? \n 1: 콜라 \n 2: 오렌지주스");
    // io::stdin().read_line(&mut buy)
    //     .expect("Failed to read line");
    //     println!("구매시작");
    // let  buy_int : i32 = buy.parse().unwrap();
    // println!("{}", buy);
    // if buy_int == 1 {
    //     println!("콜라구매");
    //     coke.buy_beverage(&buy, &money);
    // } else if buy_int == 2 {
    //     orange.buy_beverage(&buy, &money);
    //     println!("오렌지구매");
    // };
    


}

fn put_beverage(id : &str, name :&str , price : u64, count : u64) -> Beverage {
    Beverage {
        id : String::from(id),
        name : String::from(name),
        price: price,
        count : count,
    }
}

fn input_money(money : &str) -> Buyer {
    Buyer {
        money : String::from(money),
    }
}