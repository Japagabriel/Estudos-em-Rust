/*Na Linguagem RUST é de suma importância todas as variáveis e identações ser feitas em letra minuscula, caso contrário, 
irá gerar erro de sintaxe "convert the identifier to snake case:".

*/ 

fn main() {
    // para definir uma variável é necessário ': &tipo_da_variável =' caso contrário retornará erro!
    let _nome: &str = "Gabriel"; // let define a variável estática, ou seja, não mutável e impossibilitando uma nova atribuição ou troca de valores
    let mut _comida: &str = "eu sou"; // let + mut torna a variável mutável, ou seja, agora sim pondemos alterar e atribuir novos valores. 
    _comida = "seu almoço";
    println!("Hello, {}!", _nome); // as {} definem o espaço onde irá declarar o lugar da variável, separando por fora com ',' e chamando a variável 'nome' para preencher.
    println!("Ei... {}",_comida); 
    let _i: i16 = 90; //o termo 'i' define como valor tamanho de bit'16' em BCD com maior capacidade de representatividade '-32768' até '32767'.
    let _x: u8 = 70; //o termo 'u' define a varável como tamanho de bit'8' de forma polinômica com menor representatividade '0' á '255'.
    let _y: bool = true; //o termo 'bool' define a váriável com valor Booleano, ou seja, 'true' ou 'false'.
    let _z: f64 = 56.304553434; // o termo 'f' define a váriável com valor quebrado, ou seja, 'float'.


    let numb1 =43; // variáveis de controle utilizadas no 'if' e 'else' a seguir...
    let numb2 =43; 

    if numb1 > numb2 {
        println!("{} > {}", numb1,numb2); // declaração de fluxo de controles com 'if' e 'else', espressando condições
    
    }else if numb1 == numb2 {
        println!("{} = {}", numb1,numb2);
    }else{
        println!("{} > {}", numb2,numb1);
    }

}
