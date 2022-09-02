use rand::distributions::{Distribution, Uniform};

pub fn roll_dice(dice_number: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    // ans = Uniform::from(1..7);
    let mut ans: i32 = 0;
    let mut count = 0;
    while count <= dice_number {
        let dice_roll = die.sample(&mut rng);
        ans += dice_roll;
        count += 1;
    }

    ans
}

pub fn gen_random(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let number = Uniform::from(0..max).sample(&mut rng);
    number
}
