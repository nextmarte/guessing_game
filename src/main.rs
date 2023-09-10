use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("O número secreto é: {secret_number}");
    loop {
        println!("Digite o número que você chutou.");

        let mut guess = String::new();

            io::stdin()//importa a função stdin da biblioteca io que está dentro da biblioteca std
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");
        
        let guess:i32 = guess.trim().parse::<i32>().expect("Por favor digite um número!");

        println!("Você chutou: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ta frio!"),
            Ordering::Greater => println!("Ta quente demais!"),
            Ordering::Equal => {
                println!("Acerto mizeravi!");
                break;
            }
        }
    }       
}



