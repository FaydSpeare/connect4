
pub fn print_i(i: u64){
    println!("0b{:b}", i);
}

pub fn is_set(i: u64, n: i32) -> bool {
    i & (1 << n) == (1 << n)
}

