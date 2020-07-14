use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数游戏");
    let rand_number = rand::thread_rng().gen_range(1, 101);
    println!("产生的数字是: {}", rand_number);
    loop {
        println!("输入你的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("读取失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };    
        println!("你猜的数字: {}", guess);
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }
    }
}