use std::collections::HashMap;

fn part1(nums: &Vec<u32>) -> usize {
    let mut stack = nums.clone();
    // let mut last = {
    //     let x = stack.last().unwrap();
    //     *x
    // };

    // let rfind = |vec: &Vec<u32>, start, val| -> Option<usize> {
    //     let mut j = start - 1;
    //     loop {
    //         if vec[j] == val {
    //             return Some(j + 1);
    //         }
    //         if j == 0 {
    //             break;
    //         }
    //         j -= 1;
    //     }
    //     // for i in (0..start).rev() {
    //     //     if vec[i] == val {
    //     //         return Some(i);
    //     //     }
    //     // }
    //     None
    // };

    eprintln!(">>> START   stack={:?}", stack);

    let mut last = *stack.last().unwrap();
    let mut last_turn_no = stack.len();
    for i in nums.len()..2020 {
        //let prev_turn_no: Option<usize> = rfind(&stack, stack.len() - 1, last);
        let prev_turn_no = {
            let mut j = stack.len() - 2;
            loop {
                if stack[j] == last {
                    break j + 1;
                }
                if j == 0 {
                    break 0;
                }
                j -= 1;
            }
        };
        // let v = match prev_turn_no {
        //     None => 0,
        //     Some(p) => last_turn_no - p,
        // };

        let v = {
            if prev_turn_no > 0 {
                last_turn_no - prev_turn_no
            } else {
                0
            }
        };

        // eprintln!(
        //     ">>> i{}) last[{}]={},   a_idx = {:?} \t→  v = {}",
        //     i, last_idx, last, a_idx, v
        // );

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
        last = v as u32;
        stack.push(v as u32);
        last_turn_no += 1;
    }
    last as usize
}

fn part2(nums: &Vec<u32>) -> usize {
    let mut stack = nums.iter().map(|x| *x as usize).collect::<Vec<usize>>();
    //let mut last = *stack.last().unwrap();
    let max = 2020 - stack.len();
    let max = 30000000 - stack.len();
    let (last_elem, elements) = stack[..].split_last().unwrap();

    let mut hm = HashMap::new(); // last_value, last_index
    for (i, v) in elements.iter().enumerate() {
        hm.insert(*v, i + 1);
    }

    let mut last = *last_elem;
    let mut last_turn_no = stack.len();

    // let e = hm.entry(addr).or_default();
    // *e = (value & !mask.zeros) | mask.ones;

    for i in 0..max {
        let v = {
            match hm.get(&last) {
                None => 0,
                Some(prev_turn_no) => last_turn_no - prev_turn_no,
            }
        };

        let e = hm.entry(last).or_default();
        *e = last_turn_no;
        last_turn_no += 1;
        last = v;
    }
    last

    // for i in 0..max {
    //     let prev_turn_no = {
    //         let mut j = stack.len() - 2;
    //         loop {
    //             if stack[j] == last {
    //                 break j + 1;
    //             }
    //             if j == 0 {
    //                 break 0;
    //             }
    //             j -= 1;
    //         }
    //     };

    //     last = {
    //         if prev_turn_no > 0 {
    //             last_turn_no - prev_turn_no
    //         } else {
    //             0
    //         }
    //     };
    //     stack.push(last);
    //     last_turn_no += 1;
    // }
    // last
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
