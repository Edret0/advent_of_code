fn main() {
    let filepath = include_str!("../day1_input.txt");
    let output = part1(filepath);
    dbg!(output);

}

fn part1(filepath: &str) -> u32 {

    let output: u32 = filepath.lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|ch| {
                ch.to_digit(10)
            });
            let first_number = it.next().expect("It should be a number");
            let last_number = it.last();

            match last_number {
                Some(number) => format!("{first_number}{number}"),
                None => format!("{first_number}{first_number}"),
            }
            .parse::<u32>()
            .expect("should be vaild number")
        })
        
        .sum::<u32>();
    output
}

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        
        let res = part1(include_str!("../day1_input.txt"));
        let res2 = part1(input);
        assert_eq!(res2,144);
        assert_eq!(res,53080);
    }
}
