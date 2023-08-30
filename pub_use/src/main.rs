use pub_use::mix;
use pub_use::mix_secondary;
use pub_use::PrimaryColor;
use pub_use::SecondaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);

    let orange = SecondaryColor::Orange;
    let purple = SecondaryColor::Purple;
    mix_secondary(orange, purple);
}
