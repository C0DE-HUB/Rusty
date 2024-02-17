// 1)Crie um programa que imprima:
#![allow(non_snake_case)]
pub fn First(option: FirstExerciseOptions) -> Vec<i32>
{
    match option
    {
        // a)Os números de 1 a 10 de forma crescente
        FirstExerciseOptions::A => FirstA(),
        // b)Os números de 1 a 10 de forma decrescente
        FirstExerciseOptions::B => FirstB(),
        // c)Os números de 1 a 10 de forma crescente, mas apenas aqueles que forem par.
        FirstExerciseOptions::C => FirstC(),
    }
}

pub enum FirstExerciseOptions {
    A,
    B,
    C,
}

fn FirstA() -> Vec<i32>
{
    (1..=10).collect()
}

fn FirstB() -> Vec<i32>
{
    (1..=10).rev().collect()
}

fn FirstC() -> Vec<i32>
{
    (2..=10).step_by(2).collect()
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