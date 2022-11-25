use std::io;

fn conver_pnum(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}


fn main(){
    let mut sum = 0;
    //cria uma variável de valor mutável e aloca espaço na memória com o 'String::new()'
    let mut valor_ent =  String::new(); 
    /* chama o input, lê a linha quando for digitar o valor, parâmetro do '.read_line()' como variável mutável
    e utilizar o .expect() caso aconteça algum erro na leitura do valor*/
    io::stdin().read_line(&mut valor_ent).expect("Erro ao ler valor_ent"); 

    /*Cria-se uma variável mutável para receber o valor inteiro, mas, passa primeiro a conver_pnum() para 
    converter o valor em inteiro e atribuir o valor na variável que criamos agora. caso se passarmos direto
    o valor_ent para a nova váriável como o seu valor, no programa irá dar erro pois o espaço alocado em 
    valor_ent pe do tipo String*/
    let mut valor_int: i32 = conver_pnum(&valor_ent); 

    while valor_int != 0 { // cria um loop com o 'while' e enquanto a operação der diferente de '0' ele roda.
        let mut _r = valor_int % 10; // cria a variável 'r' que recebe o resto da divisão.
        sum = sum + _r; //soma o valor de 'r' na variável 'sum' (soma).
        valor_int = valor_int / 10; // a variável valor_int atualiza seu valor com a divisão de seu antigo valor por 10.
    }

    // grava na tela o valor final de 'sum' após a operação do while acabar
    println!("O valor da soma dos dígitos é:{}", sum)  


}