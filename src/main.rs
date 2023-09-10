use std::io;
use rand::Rng;

fn main(){
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é: {secret_number}");

    println!("Digite o número que você chutou.");

    let mut guess = String::new();

        io::stdin()//importa a função stdin da biblioteca io que está dentro da biblioteca std
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");
    
    println!("Você chutou: {guess}");



}



