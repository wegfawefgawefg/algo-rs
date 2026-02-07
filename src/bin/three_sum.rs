use algo_rs::algos::three_sum::{three_sum, three_sum_central_mover};

fn main() {
    // Demo:
    //   cargo run --bin three_sum
    let test_case = vec![3, 0, -2, -1, 1, 2];

    println!("Test case: {:?}", test_case);
    let result = three_sum(&test_case);
    println!("three_sum result: {:?}", result);

    let result2 = three_sum_central_mover(&test_case);
    println!("three_sum_central_mover result: {:?}", result2);

    let mut correct_answer = vec![[-2, -1, 3], [-1, 0, 1], [-2, 0, 2]];
    correct_answer.sort_unstable();
    println!("Correct answer: {:?}", correct_answer);
}
