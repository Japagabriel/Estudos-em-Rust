use std::io;

// nome da variável 'convert_to_int' vai pegar a variável data_input e devolver em numero inteiro 'i32'

fn convert_to_int(data_input: & String) -> i32 { 

    /*criamos a variável 'x' que irá pegar o valor de 'data_input' (string) e aplicar o método '.trim()' que irá 
    cortar a string no próximo ' ', logo utilizar o método .parse::<i32>() para converter esse valor em um tipo inteiro
    'i32' e por fim utilizará o '.unwrap()' para pegar o valor e se tiver um 'none' dentro do valor, irá retornar o erro.

    
    */    
    let x = data_input.trim().parse::<i32>().unwrap();

    // retorna o valor de 'x' ao final de todo o processo.
    x

}

fn main(){  
    // criou a variável 'number1' de tipo String e reservou um espaço de memória para ela. 
    let mut number1 = String::new();
    
    /*'io::stdin()'irá fazer a entrada (input) do usuário, '.read_line' irá ler a linha,
    passando a caracteristica da váriável como mutável e o nome dela 'number1' 
    .expect()irá fazer um tratamento de excessão para caso ocorra um erro.*/
    io::stdin().read_line(&mut number1).expect("Erro ao ler Number1");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler Number2");

    // o '(&number1)' está alterando o valor está também alterando o tipo dele.
    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("{} > {}", number1, number2);
    }else if convert_to_int(&number1) == convert_to_int(&number2){
        println!("{} = {}", number1, number2);
    }else{ 
        println!("{} > {}", number2, number1);

    }
}