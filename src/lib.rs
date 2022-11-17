use rand::Rng;
pub mod mechanics;
use mechanics::gen_random;

pub type CountryId = i32;
pub struct Country {
    pub id: CountryId,
    pub owner: i32,
    pub dice: i32,
    pub borders: Vec<CountryId>,
}

// pub type GameBoard = Vec<Country>;

impl Country {
    pub fn new(dice: i32, borders: Vec<CountryId>, owner:i32 ) -> Country {
        let mut rng = rand::thread_rng();
        let gen_id: i32 = rng.gen_range(0..=100);

        Country {
            id: gen_id,
            dice,
            borders,
            owner,
        }
    }
}

pub struct GameBoard {
    pub countries: Vec<Country>,
    pub players: Vec<i32>
}

impl GameBoard {
    pub fn new(players: Vec<i32>, countries: Vec<Country>) -> GameBoard {
        GameBoard {countries, players}
    }
}

fn gen_random_index(index:i32, max:i32) -> i32 {
    let mut return_number = gen_random(max);
    if return_number == index {
        if index < 1 {
            return_number = max;
        } else {
            return_number = gen_random(max);
        }
    }
    return_number
}

fn calculate_nearest_square(n: i32) -> i32 {
    (n as f64).sqrt().floor() as i32
}

pub fn generate_country_map(mut countries: Vec<Country>) -> i32 {
    // calculate row length 
    // then determine row numbers ? not sure if I need this
    // next iterate using row length to (find) the next layer of conected boundaries
    let row_length =  calculate_nearest_square(countries.len().try_into().unwrap());
    // let row_length_usize: usize = row_length as usize;
    let mut updated_countries: Vec<Country> = Vec::new();
    let mut count_in_row = 0;
    for i in 0..countries.len() {
        // let boarder_id = &mut countries[i + 1].id;
        // countries[i].boarders.push(*boarder_id);
        let borrow_countries = &mut countries;

        if count_in_row == 0 {
            // grab only ahead (next, below and diagnal)
            borrow_countries[i].borders.push(borrow_countries[i + 1].id);
            // current_country.borders.push(countries[row_length_usize].id);
            // current_country.borders.push(countries[row_length_usize + 1].id);
        } else if count_in_row == row_length - 1 {
            // grab only behind (next, below and diagnal)
            // current_country.borders.push(countries[i - 1].id);
            // current_country.borders.push(countries[row_length_usize].id);
            // current_country.borders.push(countries[row_length_usize - 1].id);
            count_in_row = 0;
        } else {
            // we grab ahead and behind and below
        }

        updated_countries.push(countries[i]);

    }
    row_length
    // println!("{}", row_length);
    // countries
}

pub fn game_setup(players: i32) -> GameBoard {
    let players_vec = vec![];
    let countries: Vec<Country> = vec![];
    let mut game_board:GameBoard = GameBoard::new(players_vec, countries);

    for n in 0 .. players {
        game_board.players.push(n)
    }

    let mut count = 0;
    //repeat process 3 times. 
    while count <= 2 {
        // iterate over player vec and create new countries 
        for player in &game_board.players {
            let boarders_vec: Vec<i32> = vec![];
            let country:Country = Country::new(1, boarders_vec, *player );
            game_board.countries.push(country)
        }
        count += 1;
    }

    let max:i32 = game_board.countries.len().try_into().unwrap();
    for n in 0..game_board.countries.len() {
        //iterate over countries and assign a boarder country based on shape (diamond to start)
        let n_int = n.try_into().unwrap();
        let boarder_id = gen_random_index(n_int, max);

        game_board.countries[n].borders.push(boarder_id);
    }

    game_board
}


#[cfg(test)]
mod tests {
    use crate::{GameBoard, game_setup, generate_country_map};

    #[test]
    fn test_game_setup() {
        let result:GameBoard = game_setup(4);
        assert_eq!(result.players.len(), 4);
        assert_eq!(result.countries.len(), 12);
    }

    #[test]
    fn test_generate_country_map() {
        let game_board:GameBoard = game_setup(3);

        let result = generate_country_map(game_board.countries);

        assert_eq!(result, 3)
    }
}
