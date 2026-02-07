use std::collections::HashSet;

pub fn three_sum(nums: &[i32]) -> Vec<[i32; 3]> {
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();

    let mut seen = HashSet::<[i32; 3]>::new();
    let mut solutions: Vec<[i32; 3]> = vec![];

    if sorted_nums.len() < 3 {
        return solutions;
    }

    for l in 0..sorted_nums.len() - 2 {
        let mut m = l + 1;
        let mut r = sorted_nums.len() - 1;
        while m < r {
            let a = sorted_nums[l];
            let b = sorted_nums[m];
            let c = sorted_nums[r];

            let s = a + b + c;
            if s == 0 {
                let sol = [a, b, c];
                if seen.insert(sol) {
                    solutions.push(sol);
                }
                m += 1;
            } else if s > 0 {
                r -= 1;
            } else {
                m += 1;
            }
        }
    }

    solutions
}

pub fn three_sum_central_mover(nums: &[i32]) -> Vec<[i32; 3]> {
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();

    let mut seen = HashSet::<[i32; 3]>::new();
    let mut solutions: Vec<[i32; 3]> = vec![];

    if sorted_nums.len() < 3 {
        return solutions;
    }

    for i in 1..sorted_nums.len() - 1 {
        let mut l = i - 1;
        let mut r = i + 1;
        loop {
            let a = sorted_nums[l];
            let b = sorted_nums[i];
            let c = sorted_nums[r];

            let s = a + b + c;
            if s == 0 {
                let sol = [a, b, c];
                if seen.insert(sol) {
                    solutions.push(sol);
                }

                if l == 0 || r == sorted_nums.len() - 1 {
                    break;
                }

                l -= 1;
                r = i + 1;
            } else if s > 0 {
                if l == 0 {
                    break;
                }
                l -= 1;
            } else {
                if r == sorted_nums.len() - 1 {
                    break;
                }
                r += 1;
            }
        }
    }

    solutions
}

pub fn run(_args: &[String]) -> Result<(), String> {
    // Example:
    //   cargo run -- three_sum
    let test_case = vec![3, 0, -2, -1, 1, 2];

    println!("Test case: {:?}", test_case);
    let result = three_sum(&test_case);
    println!("three_sum result: {:?}", result);

    let result2 = three_sum_central_mover(&test_case);
    println!("three_sum_central_mover result: {:?}", result2);

    let mut correct_answer = vec![[-2, -1, 3], [-1, 0, 1], [-2, 0, 2]];
    correct_answer.sort_unstable();
    println!("Correct answer: {:?}", correct_answer);

    Ok(())
}
