#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.is_empty() {
        Comparison::Sublist
    } else if _second_list.is_empty() {
        Comparison::Superlist
    } else if _first_list.len() <= _second_list.len() && _second_list.windows(_first_list.len()).any(|w| w == _first_list) {
        Comparison::Sublist
    } else if _first_list.len() >= _second_list.len() && _first_list.windows(_second_list.len()).any(|w| w == _second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }

}

fn main() {
    let arr: &[u32] = &[];
    let arr2 :&[u32] = &[1,2,3];
    println!("{:#?}",sublist(&arr, &arr2));
}


// let numbers = vec![1, 2, 3, 4, 5];
// let has_even = numbers.iter().any(|&x| x % 2 == 0);
// println!("Has even number: {}", has_even);
// Here we have an array

// let numbers = vec![1, 2, 3, 4, 5];
// let any_pair_addition_is_even = numbers.windows(2).any(|slice| {
//         match slice {
//             [a, b] => (a + b) % 2 == 0,
//             _ => false,  // other slice lengths not allowed
//         }
//     });
// println!("Has even number: {:#?}", any_pair_addition_is_even);
// while windows return an iterator
//https://tndl.medium.com/rusts-slice-windows-is-really-cool-70d50cdc74c5
// https://doc.rust-lang.org/std/slice/struct.Windows.html
//An iterator over overlapping subslices of length size.
// This struct is created by the windows method on slices.