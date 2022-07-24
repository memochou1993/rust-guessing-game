use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1..101); // 產生祕密數字

    println!("祕密數字為：{}", secret_number);

    loop {
        let mut guess = String::new(); // 可變變數

        io::stdin()
            .read_line(&mut guess) // 取得使用者輸入
            .expect("讀取失敗"); // 如果接收到的 Result 是 Err 的話，就讓程式當機

        let guess: u32 = match guess
            .trim() // 去除換行符號
            .parse() // 解析成數字
        {
            Ok(num) => num, // 如果接收到的 Result 是 Ok 的話，就回傳數值
            Err(_) => continue, // 如果接收到的 Result 是 Err 的話，就繼續猜測
        };

        println!("你的猜測為：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("獲勝！");
                break;
            }
        }
    }
}
