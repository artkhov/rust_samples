const START_NUM: i32 = 1_000_000;

fn main() {
    let x = calc_lucky_num(START_NUM);

    println!("{}", x);
}

fn calc_lucky_num(mut n: i32) -> i32 {
    let mut result = 0;

    while n < 2_000_000 {
        // let (mut left, mut right) = (0, 0);

        // for (i, c) in n.to_string().chars().enumerate() {
        // match i {
        // 1 | 2 | 3 => {
        // left += c.to_digit(10).unwrap();
        // }
        // 4 | 5 | 6 => {
        // right += c.to_digit(10).unwrap();
        // }
        // _ => (),
        // }
        // }

        let left = n.to_string()[1..4]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>();
        let right = n.to_string()[4..7]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>();

        if left == right {
            result += 1;
        }

        n += 1;
    }

    result
}
