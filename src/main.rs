use std::io;

fn main() {
    loop {
        println!("==================================");
        println!("1. 华氏度转换成摄氏度");
        println!("2.  n 阶斐波那契数列");
        println!("3. The Twelve Days of Christmas");
        println!("输入功能数，或者其他任意数字退出。");
        println!("==================================");
        let mut func_num = String::new();
        io::stdin().read_line(&mut func_num).expect("error");
        let func_num: i32 = func_num.trim().parse().expect("error");
        if func_num == 1 {
            fahrenheit_to_celsius();
        } else if func_num == 2 {
            fibon();
        } else if func_num == 3 {
            fuck_christmas();
        } else {
            println!("滚吧！");
            break;
        }
    }
}

fn fahrenheit_to_celsius() {
    println!("输入华氏度");
    loop {
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("请输入任意华氏度");
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let celsius = 5.0 / 9.0 * (fahrenheit - 32.0);
        println!("华氏度 {} 等同于摄氏度 {}", fahrenheit, celsius);
        break;
    }
}

fn fibon() {
    println!("输入n阶");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error");
    let n: u32 = n.trim().parse().expect("error");
    for i in 1..n + 1 {
        print!("{}\t", calc_fibon(i));
        if i % 5 == 0 {
            println!("");
        }
    }
    println!("");
}

fn calc_fibon(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        calc_fibon(n - 1) + calc_fibon(n - 2)
    }
}

fn fuck_christmas() {
    let twelve_days = [
        "And a Partridge in a Pear Tree",
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];
    let num_str = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas \nMy true love sent to me:",
            num_str[i]
        );
        let mut n = i + 1;
        while n > 0 {
            if n == 1 {
                if i == 0 {
                    println!("  {}", twelve_days[n]);
                } else {
                    println!("  {}", twelve_days[0]);
                }
            } else {
                println!("  {}", twelve_days[n]);
            }
            n -= 1;
        }
    }
}
