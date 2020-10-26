//https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust

fn descending_order(x: u64) -> u64 {
    let mut v: Vec<u32> = x.to_string().chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    v.sort_by(|n, y| y.partial_cmp(n).unwrap());

    v.iter().map(|n| n.to_string()).collect::<String>().parse().unwrap()
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
