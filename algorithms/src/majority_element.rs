pub fn majority_element(numbers: Vec<i32>) -> i32 {
    let (mut candidate, mut count) = (numbers[0], 1);

    for &number in &numbers[1..] {
        if count == 0 {
            candidate = number;
            count = 1;
        } else if number == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}
