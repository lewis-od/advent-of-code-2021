use std::convert::TryInto;

pub fn calc_power_consumption(inputs: &Vec<u32>, num_digits: u8) -> u32 {
    let gamma_rate = find_most_common(inputs, num_digits);
    let epsilon_rate = flip_bits(gamma_rate, num_digits);
    (gamma_rate * epsilon_rate).try_into().unwrap()
}

fn flip_bits(value: u32, num_digits: u8) -> u32 {
    let shift = 32 - num_digits;
    !(value << shift) >> shift
}

//fn max_index(list: &Vec<u32>) -> usize {
//    let mut max_idx = 0;
//    let mut max_value = 0;
//    for (idx, &value) in list.iter().enumerate() {
//        if value > max_value {
//            max_idx = idx;
//            max_value = value;
//        }
//    }
//    max_idx.try_into().unwrap()
//}

pub fn find_most_common(input: &Vec<u32>, num_digits: u8) -> u32 {
    let mut output = 0;

    for n in 0..num_digits {
        let position = num_digits - n;
        let num_ones = count_ones(input, position);

        if num_ones as f32 >= (input.len() / 2) as f32 {
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

    #[test]
    fn finds_max_index() {
        let inputs = vec![0, 1, 5, 4, 2];

        let index = max_index(&inputs);
        assert_eq!(2, index);
    }

//    #[test]
//    fn calcs_scrubber_rating() {
//        let input = vec![
//            0b00100,
//            0b11110,
//            0b10110,
//            0b10111,
//            0b10101,
//            0b01111,
//            0b00111,
//            0b11100,
//            0b10000,
//            0b11001,
//            0b00010,
//            0b01010,
//        ];
//
//        let result = calc_scrubber_rating(&input, 5);
//
//        assert_eq!(10, result);
//    }

//    #[test]
//    fn calcs_oxygen_rating() {
//        let input = vec![
//            0b00100,
//            0b11110,
//            0b10110,
//            0b10111,
//            0b10101,
//            0b01111,
//            0b00111,
//            0b11100,
//            0b10000,
//            0b11001,
//            0b00010,
//            0b01010,
//        ];
//
//        let result = calc_oxygen_rating(&input, 5);
//
//        assert_eq!(23, result);
//    }
}
