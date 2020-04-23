pub fn reverse_int(mut n: u32) -> u32 {
    let mut num = 0;
    while n != 0 {
        let md = n % 10;
        num = 10 * num + md;
        n = n / 10;
    }

    num
}
