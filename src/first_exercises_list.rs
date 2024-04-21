// 1)Crie um programa que imprima:
pub fn first(option: FirstExerciseOptions) -> Vec<i32>
{
    match option
    {
        // a)Os números de 1 a 10 de forma crescente
        FirstExerciseOptions::A => first_a(),
        // b)Os números de 1 a 10 de forma decrescente
        FirstExerciseOptions::B => first_b(),
        // c)Os números de 1 a 10 de forma crescente, mas apenas aqueles que forem par.
        FirstExerciseOptions::C => first_c(),
    }
}

pub enum FirstExerciseOptions {
    A,
    B,
    C,
}

fn first_a() -> Vec<i32> {
    (1..=10).collect()
}

fn first_b() -> Vec<i32> {
    (1..=10).rev().collect()
}

fn first_c() -> Vec<i32> {
    (2..=10).step_by(2).collect()
}

// 2)Imprimir a soma dos números inteiros de 1 a 100.
pub fn second() -> i32 {
    return (1..=100).sum()
}

// 3)Imprimir todo os números ímpares menores de 200.
pub fn third() -> Vec<i32> {
    return (1..=200).step_by(2).collect()
}

// 4)Calcular a média de idade de uma turma qualquer.
// O algoritmo deve parar quando for digitada a idade igual a zero.
pub fn fourth(ages: Vec<i32>) -> f64 {
    let mut sum = 0;
    for &i in &ages {
        if i == 0 {
            break;
        }
        sum += i;
    }

    if ages.len() > 0 {
        let average = sum as f64 / ages.len() as f64;
        return average
    }

    return 0.0;
}

// 5)Criar um algoritmo que peça o nome e a idade de 5 mulheres.
// Após informar estes dados, o programa deve mostrar apenas porcentagem
// de mulheres que estão com idade entre 18 e 35.
pub fn fifth(women: Vec<(String, i32)>) -> f64 {
    let ages: i32 = women
        .iter()
        .filter(|x| x.1 > 17 && x.1 < 36)
        .map(|x| x.1)
        .sum();

    return ages as f64 / women.len() as f64
}



// let dobrado: Vec<i32> = vetor.iter().map(|&x| x * 2).collect();
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