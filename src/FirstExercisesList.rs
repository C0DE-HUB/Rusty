#![allow(non_snake_case)]
pub fn FirstA() -> [i32; 10]
{
    let mut arr: [i32; 10] = [0; 10];

    for i in 1..11 {
        arr[i - 1] = i as i32;
    }

    arr
}
pub fn FirstB() -> i32
{
    1
}

// pub struct FirstExercises {
//     teste: i64
// }

// impl FirstExercises
// {
//     pub fn new() -> Self {
//         Self
//         {
//             teste: 2
//         }
//     }
// }