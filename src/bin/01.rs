advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let double_list: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect(); //>= input.split_whitespace(

    let mut list_one: Vec<i32> = double_list.iter().step_by(2).copied().collect();
    let mut list_two: Vec<i32> = double_list.iter().skip(1).step_by(2).copied().collect();

    list_one.sort();
    list_two.sort();

    let mut sum: i32 = 0;

    for i in 0..list_one.len() {
        sum = sum + (list_one[i] - list_two[i]).abs();
    }

    return Some(sum as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let double_list: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect(); //>= input.split_whitespace(

    let list_one: Vec<i32> = double_list.iter().step_by(2).copied().collect();
    let list_two: Vec<i32> = double_list.iter().skip(1).step_by(2).copied().collect();


    let mut out: i32 = 0;
    let mut occurences: i32;

    for i in 0..list_one.len() {

        occurences = list_two.iter().filter(|x| **x == list_one[i]).count() as i32;
        out = out + (list_one[i] * occurences);
    }

    println!("{}", out);

    return Some(out as u32);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
