fn part1(nums: &Vec<u32>) -> usize {
    let mut stack = nums.clone();
    // let mut last = {
    //     let x = stack.last().unwrap();
    //     *x
    // };

    let rfind = |vec: &Vec<u32>, start, val| -> Option<usize> {
        for i in (0..start).rev() {
            if vec[i] == val {
                return Some(i);
            }
        }
        None
    };

    eprintln!(">>> START   stack={:?}", stack);

    for i in nums.len()..2020 {
        let last = *stack.last().unwrap();
        let last_idx = stack.len();
        let a_idx: Option<usize> = rfind(&stack, stack.len() - 1, last);
        let v = match a_idx {
            None => 0,
            Some(a) => last_idx - (a + 1),
        };
        eprintln!(
            ">>> i{}) last[{}]={},   a_idx = {:?} \t→  v = {}",
            i, last_idx, last, a_idx, v
        );

        // let b_idx = a_idx.and_then(|a| rfind(&stack, a, last));
        // eprintln!(">>> i{}) last = {},   a_idx = {:?}, b_idx = {:?}", i, last, a_idx,b_idx);
        // let v = match a_idx {
        //     None => 0,
        //     Some(a) => match b_idx {
        //         None => 0,
        //         Some(b) => (a + 1) as u32 - (b + 1) as u32,
        //     },
        // };
        //eprintln!(" >> push = {}", v);
        stack.push(v as u32);
    }
    *stack.last().unwrap() as usize
}

pub fn run() {
    let input = include_str!("input/day15.txt");
    let nums = &input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    println!("Day 15/1: {}", part1(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1_sample1() {
        assert_eq!(436, part1(&vec![0, 3, 6]));
        assert_eq!(1, part1(&vec![1, 3, 2]));
        assert_eq!(10, part1(&vec![2, 1, 3]));
        assert_eq!(27, part1(&vec![1, 2, 3]));
        assert_eq!(78, part1(&vec![2, 3, 1]));
        assert_eq!(438, part1(&vec![3, 2, 1]));
        assert_eq!(1836, part1(&vec![3, 1, 2]));
    }
}
