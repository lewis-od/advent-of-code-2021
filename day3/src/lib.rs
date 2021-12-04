use std::convert::TryInto;

pub fn calc_power_consumption(inputs: &Vec<u32>, num_digits: u8) -> u32 {
    let gamma_rate = find_most_common(inputs, num_digits);
    let epsilon_rate = flip_bits(gamma_rate, num_digits);
    (gamma_rate * epsilon_rate).try_into().unwrap()
}

pub fn calc_life_support_rating(inputs: &Vec<u32>, num_digits: u8) -> u32 {
    let oxygen_generator_rating = calc_oxygen_generator_rating(inputs, num_digits);
    let co2_scrubber_rating = calc_co2_scrubber_rating(inputs, num_digits);
    oxygen_generator_rating * co2_scrubber_rating
}

fn calc_oxygen_generator_rating(inputs: &Vec<u32>, num_digits: u8) -> u32 {
    let mut sieve = inputs.clone();

    for shift in 1..num_digits + 1 {
        let position = num_digits - shift;
        let most_common_digits = find_most_common(&sieve, num_digits);
        let most_common = get_digit(&most_common_digits, position);
        sieve = sieve
            .iter()
            .filter(|number| get_digit(*number, position) == most_common)
            .map(|v| v.to_owned())
            .collect();
        if sieve.len() == 1 {
            return sieve[0];
        }
    }
    0
}

fn calc_co2_scrubber_rating(inputs: &Vec<u32>, num_digits: u8) -> u32 {
    let mut sieve = inputs.clone();

    for shift in 1..num_digits + 1 {
        let position = num_digits - shift;
        let most_common_digits = find_most_common(&sieve, num_digits);
        let least_common_digits = flip_bits(most_common_digits, num_digits);
        let least_common = get_digit(&least_common_digits, position);
        sieve = sieve
            .iter()
            .filter(|number| get_digit(*number, position) == least_common)
            .map(|v| v.to_owned())
            .collect();
        if sieve.len() == 1 {
            return sieve[0];
        }
    }
    0
}

pub fn find_most_common(input: &Vec<u32>, num_digits: u8) -> u32 {
    let mut output = 0;

    for n in 1..num_digits + 1 {
        let position = num_digits - n;
        let num_ones = count_ones(input, position);

        if num_ones as f32 >= (input.len() as f32 / 2.0 as f32) {
            output += 1;
        }
        output = output << 1;
    }
    output >> 1
}

fn count_ones(input: &Vec<u32>, position: u8) -> u32 {
    let mut num_ones = 0;
    for number in input {
        let digit = get_digit(number, position);
        num_ones += digit;
    }
    num_ones
}

fn get_digit(value: &u32, position: u8) -> u32 {
    if value & (1 << position) != 0 {
        1
    } else {
        0
    }
}

fn flip_bits(value: u32, num_digits: u8) -> u32 {
    let shift = 32 - num_digits;
    !(value << shift) >> shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_last_digit() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = count_ones(&input, 0);

        assert_eq!(5, result);
    }

    #[test]
    fn counts_first_digit() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = count_ones(&input, 4);

        assert_eq!(7, result);
    }

    #[test]
    fn finds_most_common_bits() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = find_most_common(&input, 5);

        assert_eq!(0b10110, result);
    }

    #[test]
    fn finds_power_rate() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = calc_power_consumption(&input, 5);

        assert_eq!(198, result);
    }

    #[test]
    fn calcs_scrubber_rating() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = calc_co2_scrubber_rating(&input, 5);

        assert_eq!(10, result);
    }

    #[test]
    fn calcs_oxygen_generator_rating() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = calc_oxygen_generator_rating(&input, 5);

        assert_eq!(23, result);
    }

    #[test]
    fn calcs_life_support_rating() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let result = calc_life_support_rating(&input, 5);

        assert_eq!(230, result);
    }
}
