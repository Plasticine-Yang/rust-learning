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
}
