use std::u32;

fn main() {
    println!("{:?}", is_armstrong_number(1535555550));
}

fn checked_pow(base: u32, exponent: u32) -> Option<u32> {
    let mut result: u32 = 1;
    for _ in 0..exponent {
        match result.checked_mul(base) {
            Some(val) => result = val,
            None => return None, // Overflow occurred
        }
    }
    Some(result)
}

pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    } else if num > 0 && num < 10 {
        return true;
    } else {
        let string_num = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>()
            .into_iter();

        let num_digits = string_num.len() as u32;

        let sum_of_powers =
            string_num
                .map(|digit| checked_pow(digit, num_digits))
                .fold(Some(0), |acc, val| {
                    match (acc, val) {
                        (Some(acc), Some(val)) => {
                            let acc_u32: u32 = acc.into();
                            let val_u32: u32 = val.into();
                            if let Some(result) = acc_u32.checked_add(val_u32) {
                                Some(result)
                            } else {
                                None
                            }
                        }
                        _ => None, // Propagate None if any value is None
                    }
                });

        match sum_of_powers {
            Some(sum) => sum == num,
            None => false,
        }
    }
}
