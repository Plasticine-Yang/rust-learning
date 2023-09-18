#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Animal {
    Dog(String),
    Cat(String),
}

impl Animal {
    fn play(&self) {
        println!("I'm playing");
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));

    dbg!(ipv4, ipv6);

    let animal1 = Animal::Dog(String::from("doggy"));
    let animal2 = Animal::Cat(String::from("catty"));

    animal1.play();
    animal2.play();

    dbg!(animal1, animal2);

    let foo = Some(666);
    let bar = Some(String::from("Hello"));
    let baz: Option<u32> = None;

    dbg!(foo, bar, baz);

    let coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(&coin);
    match_catch_all(&coin);
    if_let(&coin);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

/** 只想处理 enum 的部分 variants，忽略其他 variants */
fn match_catch_all(coin: &Coin) {
    match coin {
        Coin::Quarter(state) => {
            println!("[match_catch_all] - {:?}", state);
        }
        _ => (),
    }
}

/** if let 可以看成是 match 的其中一种情况，当只想处理 match 的一种情况是适合使用，算是 match 的语法糖 */
fn if_let(coin: &Coin) {
    if let Coin::Quarter(state) = coin {
        println!("[if_let] coin is Quarter - state is {:?}", state);
    } else if let Coin::Dime = coin {
        println!("[if_let] coin is Dime");
    } else {
        println!("[if_let] coin is other");
    }
}
