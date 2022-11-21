#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_perferenct: Option<ShirtColor>) -> ShirtColor {
        user_perferenct.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut number_red = 0;
        let mut number_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => number_red += 1,
                ShirtColor::Blue => number_blue += 1,
            }
        }

        if number_red > number_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[test]
fn Inventory_Test(){
    let mi=Inventory{
        shirts: vec![ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue,]
    };
    
    let user_perferenct  = Some(ShirtColor::Blue);
    let giveway = mi.giveaway(user_perferenct);
    println!(
        "The user with preference {:?} gets {:?}",
        user_perferenct,giveway,
    );

    let user_pref2 = None;
    let giveaway2 = mi.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
