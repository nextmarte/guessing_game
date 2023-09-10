use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Adivinhe o número!");
    //gera um número aleatório entre 1 e 100 e armazena na variável secret_number i32
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("O número secreto é: {secret_number}");
    //loop infinito até quebrar o loop com break quando o usuário acertar o número
    loop {
        println!("Digite o número que você chutou.");

        //Declara uma variável mutável chamada guess que é do tipo String
        let mut guess = String::new();
            
            io::stdin()//importa a função stdin da biblioteca io que está dentro da biblioteca std
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        //converte o valor digitado de string para i32 e armazena na variável guess
        //o trim remove o /n que é adicionado quando o usuário aperta enter
        //sem esse trim o parse não funciona pois o parse não consegue converter o /n para i32
        let guess:i32 = guess.trim().parse::<i32>().expect("Por favor digite um número!");

        println!("Você chutou: {guess}");

        //compara o valor digitado com o valor gerado aleatoriamente usando o match
        //o match é um switch case do rust
        //o cmp compara o valor digitado com o valor gerado aleatoriamente
        //o cmp retorna um enum chamado Ordering que pode ser Less, Greater ou Equal
        //o match verifica o valor do enum e executa o código de acordo com o valor
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("winter is comming!"),
            Ordering::Greater => println!("Erouuuuu feio, errou rude!"),
            Ordering::Equal => {
                println!("Acerto mizeravi!");
                break;
            }
        }
    }       
}



