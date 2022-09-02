#[allow(dead_code)]
use dice_game::mechanics::roll_dice;
use dice_game::Country;
use dice_game::game_setup;

fn main() {
    println!("the dice rolled: {}", roll_dice(8));
    let boarders = vec![21, 3];
    let new_country = Country::new(3, boarders, 2);
    println!("{:?}", new_country.id);
    let game = game_setup(2);
    for country in game.countries {
        println!("country id is {:?}", country.id);
    }
    
}
