

fn median(lt: &Vec<f32>) -> Option<f32> {
    let mut list = lt.clone();
    let list_len: usize = list.len();

    // check if len even or odd
    if list.is_empty() {
        return None
    } else {
        // sort list ascending
        list.sort_by(|x, y| x.partial_cmp(y).unwrap());

        if list_len % 2 == 1 {
            let middle_index = &list_len / 2;
            let middle_element = &list[middle_index];
            return Some(*middle_element);
        } else {
            let last_middle_index = &list_len / 2;
            let two_middle_values = &list[last_middle_index - 1] + &list[last_middle_index];
            let average_middle =  two_middle_values / 2.0;
            return Some(average_middle);
        }
    }
}

fn main() {
    let list: Vec<f32> = vec![1.0, 3.0, 5.0, 8.1];

    assert_eq!(median(&list), Some(3.0));

    println!("Yay test completed");


}
