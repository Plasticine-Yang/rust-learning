/**
 * scenario
 *
 * Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list
 * as a promotion. People on the mailing list can optionally add their favorite color to their profile. If the person
 * chosen for a free shirt has their favorite color set, they get that color shirt. If the person hasn’t specified a
 * favorite color, they get whatever color the company currently has the most of.
 */

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
}

#[allow(dead_code)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[allow(dead_code)]
impl Inventory {
    fn giveaway(&self, user_favorite_color: Option<ShirtColor>) -> ShirtColor {
        user_favorite_color.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_favorite_color() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
        let user_favorite_color = Some(ShirtColor::Red);

        let result = store.giveaway(user_favorite_color);

        assert_eq!(result, ShirtColor::Red);
    }

    #[test]
    fn without_favorite_color() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
        let user_favorite_color = None;

        let result = store.giveaway(user_favorite_color);

        assert_eq!(result, ShirtColor::Blue);
    }
}
