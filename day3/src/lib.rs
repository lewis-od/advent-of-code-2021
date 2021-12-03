use std::convert::TryInto;

pub fn calc_power_consumption(inputs: &Vec<u32>, num_digits: u8) -> i32 {
    let gamma_rate = find_most_common(inputs, num_digits);
    let shift = 32 - num_digits;
    let epsilon_rate = !(gamma_rate << shift) >> shift;
    (gamma_rate * epsilon_rate).try_into().unwrap()
}

pub fn find_most_common(input: &Vec<u32>, num_digits: u8) -> u32 {
    let mut output = 0;

    for n in 0..num_digits {
        let position = num_digits - n;
        let num_ones = count_ones(input, position);

        if num_ones as f32 > (input.len() / 2) as f32 {
            output += 1;
        }
        output = output << 1;
    }
    output
}

fn count_ones(input: &Vec<u32>, position: u8) -> u32 {
    let mut num_ones = 0;
    for number in input {
        let digit = if number & (1 << position) != 0 { 1 } else { 0 };
        num_ones += digit;
    }
    num_ones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_last_digit() {
        let input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        let result  = count_ones(&input, 0);

        assert_eq!(5, result);
    }

    #[test]
    fn counts_first_digit() {
        let input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        let result  = count_ones(&input, 4);

        assert_eq!(7, result);
    }

    #[test]
    fn finds_most_common_bits() {
        let input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        let result = find_most_common(&input, 5);

        assert_eq!(0b10110, result);
    }

    #[test]
    fn finds_power_rate() {
        let input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        let result = calc_power_consumption(&input, 5);

        assert_eq!(198, result);
    }
}
