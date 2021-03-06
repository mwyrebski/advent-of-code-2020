fn part1(nums: &Vec<u32>) -> usize {
    let mut stack = nums.clone();

    let rfind = |vec: &Vec<u32>, start, val| -> Option<usize> {
        for i in (0..start).rev() {
            if vec[i] == val {
                return Some(i + 1);
            }
        }
        None
    };

    let mut last = *stack.last().unwrap();
    let mut last_turn_no = stack.len() as u32;
    for _i in nums.len()..2020 {
        let prev_turn_no: Option<usize> = rfind(&stack, stack.len() - 1, last);
        last = match prev_turn_no {
            None => 0,
            Some(p) => last_turn_no - p as u32,
        };
        stack.push(last as u32);
        last_turn_no += 1;
    }
    last as usize
}

fn part2(nums: &Vec<u32>) -> usize {
    static LEN: usize = 30000000;
    let mut mem = vec![0; LEN];

    let max = LEN - nums.len();
    let (last_elem, elements) = nums[..].split_last().unwrap();

    for (i, v) in elements.iter().enumerate() {
        mem[*v as usize] = (i + 1) as usize;
    }

    let mut last = *last_elem as usize;
    let mut last_turn_no = nums.len();

    for _i in 0..max {
        let v = {
            let prev_turn_no = mem[last];
            if prev_turn_no == 0 {
                0
            } else {
                last_turn_no - prev_turn_no
            }
        };

        mem[last] = last_turn_no;

        last_turn_no += 1;
        last = v;
    }
    last
}

pub fn run() {
    let input = include_str!("input/day15.txt");
    let nums = &input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    println!("Day 15/1: {}", part1(nums));
    println!("Day 15/2: {}", part2(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1_samples() {
        assert_eq!(436, part1(&vec![0, 3, 6]));
        assert_eq!(1, part1(&vec![1, 3, 2]));
        assert_eq!(10, part1(&vec![2, 1, 3]));
        assert_eq!(27, part1(&vec![1, 2, 3]));
        assert_eq!(78, part1(&vec![2, 3, 1]));
        assert_eq!(438, part1(&vec![3, 2, 1]));
        assert_eq!(1836, part1(&vec![3, 1, 2]));
    }

    #[test]
    fn test_day15_part2_samples() {
        assert_eq!(175594, part2(&vec![0, 3, 6]));
        assert_eq!(2578, part2(&vec![1, 3, 2]));
        assert_eq!(3544142, part2(&vec![2, 1, 3]));
        assert_eq!(261214, part2(&vec![1, 2, 3]));
        assert_eq!(6895259, part2(&vec![2, 3, 1]));
        assert_eq!(18, part2(&vec![3, 2, 1]));
        assert_eq!(362, part2(&vec![3, 1, 2]));
    }
}
