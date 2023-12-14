
fn main() {
    let filepath = include_str!("../day1_input.txt");
    let output = part2(filepath);
    dbg!(output);

}

fn part2(filepath: &str) -> u32 {
    

    let output: u32 = filepath.lines()
        .map(process_line)
        .sum::<u32>();
    output
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len())
        .filter_map(|index| {
            let reduced_line = &line[index..];
            let result = if reduced_line.starts_with("one") {
                '1'
            } else if reduced_line.starts_with("two") {
                '2'
            } else if reduced_line.starts_with("three") {
                '3'
            } else if reduced_line.starts_with("four") {
                '4'
            } else if reduced_line.starts_with("five") {
                '5'
            } else if reduced_line.starts_with("six") {
                '6'
            } else if reduced_line.starts_with("seven") {
                '7'
            } else if reduced_line.starts_with("eight") {
                '8'
            } else if reduced_line.starts_with("nine") {
                '9'
            } else {
                reduced_line.chars().next().unwrap()
            };

            result.to_digit(10)
        });

    let first = it.next().expect("It should be a number");
    let last = it.last();

    match last {
        Some(number) => format!("{first}{number}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a valid number")
}

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        
        let res = part2(include_str!("../day1_input.txt"));
        let res2 = part2(input);
        assert_eq!(res2,281);
    }

    use rstest::rstest;

    #[rstest]
    #[case("two1nine",29)]
    #[case("eightwothree",83)]
    #[case("abcone2threexyz",13)]
    #[case("xtwone3four",24)]
    #[case("4nineeightseven2",42)]
    #[case("zoneight234",14)]
    #[case("7pqrstsixteen",76)]
    fn line_test (
        #[case] line: &str,
        #[case] expected: u32,
        ) {
        assert_eq!(expected,process_line(line));
    }

}
