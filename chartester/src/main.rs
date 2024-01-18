use rand::Rng;

fn dice_roll(num: u8, size: u8) {
    let ran_val : u8 = rand::thread_rng().gen_range(1..=size);

    if num == 2 && size == 10 && ran_val == 20 {
        
    }
}

fn main() {
    dice_roll(2, 10);
}