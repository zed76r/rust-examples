use std::io;

fn main() {
    loop {
        println!("==================================");
        println!("1. 华氏度/摄氏度转换");
        println!("2.  n 阶斐波那契数列");
        println!("3. The Twelve Days of Christmas");
        println!("输入功能数，或者其他任意数字退出。");
        println!("==================================");
        let mut func_num = String::new();
        io::stdin().read_line(&mut func_num).expect("error");
        let func_num: i32 = func_num.trim().parse().expect("error");
        match func_num {
            1 => f_c_converter(),
            2 => fibon(),
            3 => fuck_christmas(),
            _ => {
                println!("滚吧！");
                break;
            }
        };
    }
}

fn f_c_converter() {
    println!(
        "输入一个值，以F结尾代表华氏度，或者以C结尾为摄氏度。比如77F。"
    );
    loop {
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("输入一个值，以F结尾代表华氏度，或者以C结尾为摄氏度。比如77F。");
        input_str = input_str.trim().to_string();
        let at = input_str.len() - 1;
        let (value, unit) = input_str.split_at_mut(at);
        unit.make_ascii_uppercase();
        if "F" == unit {
            let fahrenheit: f64 = match value.parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let celsius = 5.0 / 9.0 * (fahrenheit - 32.0);
            println!("华氏度 {} 等同于摄氏度 {}", fahrenheit, celsius);
        } else if "C" == unit {
            let celsius: f64 = match value.parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let fahrenheit = 9.0 / 5.0 * celsius + 32.0;
            println!("摄氏度 {} 等同于华氏度 {}", celsius, fahrenheit);
        } else {
            println!("输入一个值，以F结尾代表华氏度，或者以C结尾为摄氏度。比如77F。");
            continue;
        }
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
