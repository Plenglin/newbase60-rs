use rstest::rstest;
use super::*;

// Tests stolen from https://github.com/indieweb/newBase60py/blob/master/newbase60test.py

#[rstest(input, expected)]
#[case(0, "0")]
#[case(1, "1")]
#[case(60, "10")]
fn test_num_to_sxg(input: u128, expected: &str) {
    assert_eq!(num_to_sxg(input), expected)
}

#[rstest(input, expected)]
#[case("N", Some(22))]
#[case("H", Some(17))]
#[case("10", Some(60))]
#[case("NH", Some(1337))]
#[case("0", Some(0))]
#[case("asc", Some(129157))]
#[case("a̶̹͓̿̇̈́̔͗͝s̴̒̓̈͌̍̈́̀̐͂͛̑̊̿̑̈͜ͅc̷̢͙͔͈̠͎̱̭̭̏ͅ", Some(129157))]
#[case("1", Some(1))]
#[case("l", Some(1))]
#[case("l", Some(1))]
#[case("N🏳️‍⚧H", Some(1337))]
#[case("N̷̛͎̝͕͓͙̟̺͎̳̯̙͙̦̋͗͒̀̐̏̅͗̎̕̚͘ͅͅH̴̭̳͉͚͂̀̀̔", Some(1337))]
#[case("I", Some(1))]
#[case("O", Some(0))]
#[case("|", Some(0))]
#[case(",", Some(0))]
#[case("🥺", Some(0))]
#[case("sadfui9fasjf", Some(1908097676891172549880))]
#[case(
"this is a very long string that will overflow the multiplication buffer",
None
)]
fn test_sxg_to_num(input: &str, expected: Option<u128>) {
    assert_eq!(sxg_to_num(input), expected)
}

#[test]
fn test_round_trip_n_s_n() {
    for n in 0..100000 as u128 {
        let s = num_to_sxg(n);
        assert_eq!(sxg_to_num(s.as_str()), Some(n))
    }
}
