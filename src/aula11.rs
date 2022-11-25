use std::io;
// Aula de Entrada de números fatoriais com estrutura While

fn conver_pnum(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main(){
    let mut entrada_fatorial = String::new();
    io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada_fatorial");
    let mut fatorial = 1;
    let mut entrada_int = conver_pnum(&entrada_fatorial);
    let mut vezes = 1;
    let _est = entrada_fatorial;

    while entrada_int > 1 {
        fatorial = fatorial * entrada_int; // variável fatorial irá atualizar seu valor após a multiplicação
        entrada_int = entrada_int -1;
        vezes += 1;

    }

    println!("O fatorial de {} é {}",_est, fatorial);
    println!("O número de vezes que rodou foi {}", vezes);

}