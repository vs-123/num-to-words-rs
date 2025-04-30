macro_rules! test {
    ($num:expr, $num_word:expr) => {
        paste::item! {
            #[test]
            fn [<test_$num>]() {
                assert_eq!(convert($num), String::from($num_word));
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use num_to_words::convert;

    test!(999, "nine hundred ninety-nine");
    test!(999_999, "nine hundred ninety-nine thousand nine hundred ninety-nine");
    test!(999_999_999, "nine hundred ninety-nine million nine hundred ninety-nine thousand nine hundred ninety-nine");
    test!(999_999_999_999, "nine hundred ninety-nine billion nine hundred ninety-nine million nine hundred ninety-nine thousand nine hundred ninety-nine");
    test!(999_999_999_999_999, "nine hundred ninety-nine trillion nine hundred ninety-nine billion nine hundred ninety-nine million nine hundred ninety-nine thousand nine hundred ninety-nine");
    test!(999_999_999_999_999_999, "nine hundred ninety-nine quadrillion nine hundred ninety-nine trillion nine hundred ninety-nine billion nine hundred ninety-nine million nine hundred ninety-nine thousand nine hundred ninety-nine");
}