// aula11/src/main.rs

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Usando iteradores para calcular a soma dos quadrados dos números pares
    let sum_even_squares: i32 = numbers
    .iter()                            // Cria um iterador sobre os elementos
    .filter(|&&x| x % 2 == 0)          // Filtra somente os números pares
    .map(|&x| x * x)                   // Eleva cada número ao quadrado
    .sum();                            // Soma todos os resultados

println!("Soma dos quadrados dos números pares: {}", sum_even_squares);

// Usando iteradores para encontrar o primeiro número maior que 5:
let first_gt_five = numbers.iter().find(|&&x| x > 5);

match first_gt_five {
    Some(x) => println!("Primeiro número maior que 5: {}", x),
    None => println!("Nenhum número maior que 5 encontrado."),
}
}
