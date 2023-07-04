pub fn fact (num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => fact(num - 1) * num
    }
}

pub fn precalc () {
    for index in 1..10 {
        println!("{}", fact(index));
    }
}