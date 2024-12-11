fn get_split_digits(mut num: usize) -> Option<(usize, usize)> {
    // should be more than enough
    const MAX_DIGITS: usize = 20;

    // since we are only pushing singular digits in this array
    // no real elements will be of the value 10
    let mut digits: [u8; MAX_DIGITS] = [10; MAX_DIGITS];

    let mut index = 0;
    while num > 0 {
        let digit = (num % 10) as u8;
        num /= 10;

        assert!(digit <= 9);
        digits[index] = digit;

        index += 1;
    }

    if index % 2 == 0 {
        let half = index / 2;

        let mut second = 0;
        for i in (0..half).rev() {
            let degree = usize::pow(10, i as u32);
            second += degree * digits[i] as usize;
        }

        let mut first = 0;
        for i in (half..index).rev() {
            let degree = usize::pow(10, (i - half) as u32);
            first += degree * digits[i] as usize;
        }

        return Some((first, second));
    }

    None
}
