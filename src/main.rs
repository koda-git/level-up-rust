fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {

    numbers
        .iter()
        .map( |x| x.unwrap_or_default())
        .sum()

}
//     let mut sum: i32 = 0;

//     for i in 0..numbers.len() {
//         if numbers[i] == None {
//             sum += 0;
//         } else {
//             sum += numbers[i].unwrap();
//         }
//     }
//     sum
// }

fn main() {
    println!("");
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}
