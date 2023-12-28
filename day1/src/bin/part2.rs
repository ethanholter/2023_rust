fn main() {
    let input: std::str::Split<'_, &str> = include_str!("../../input.txt").split("\n");

    let mut sum: u32 = 0;
    for line in input{
    
    let mut num = String::new();
        //first digit
        for i in line.chars() {
            if i.is_numeric() {
                num.push(i);
                break;
            }
        }

        //last (second) digit
        for i in line.chars().rev() {
            if i.is_numeric() {
                num.push(i);
                break;
            }
        }
        if num.len() > 0 {
            sum += num.parse::<u32>().unwrap();

        }

    }

    println!("{}", sum);
    
}
