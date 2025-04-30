fn convert_hundred_or_above(num: u64, divisor: u64, place_value: &str) -> String {
    if num % divisor == 0 {
        convert(num / divisor) + " " + place_value
    } else {
        convert(num - num % divisor) + " " + &convert(num % divisor)
    }
}

pub fn convert(num: u64) -> String {
    match num {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),

        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        15 => String::from("fifteen"),
        18 => String::from("eighteen"),
        
        14..=19 => convert(num % 10) + "teen",

        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        80 => String::from("eighty"),

        60..=90 if num % 10 == 0 => convert(num / 10) + "ty",

        21..=99 => convert(num - num % 10) + "-" + &convert(num % 10),

        100..=999 => convert_hundred_or_above(num, 100, "hundred"),
        1000..=999_999 => convert_hundred_or_above(num, 1000, "thousand"),
        1_000_000..=999_999_999 => convert_hundred_or_above(num, 1_000_000, "million"),
        1_000_000_000..=999_999_999_999 => convert_hundred_or_above(num, 1_000_000_000, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => {
            convert_hundred_or_above(num, 1_000_000_000_000, "trillion")
        }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            convert_hundred_or_above(num, 1_000_000_000_000_000, "quadrillion")
        }

        _ => convert_hundred_or_above(num, 1_000_000_000_000_000_000, "quintillion"),
    }
}