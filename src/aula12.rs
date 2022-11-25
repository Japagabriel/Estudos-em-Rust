use std::io;
//Calculando médias de alunos.

fn conver_pnum(data_input: & String)-> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}


fn main(){
    let mut medias: String::new();
    io::stdin().read_line(& mut medias).expect("Erro ao ler 'medias'... ");
    let mut soma_rec = 0;
    let mut soma_rep = 0;
    let mut soma_apr = 0;
    let mut i = 0;

    while conver_pnum(&medias) > i {
        let mut media_aluno: String::new();
        println!("Digite a {} nota: ",i);
        io::stdin().read_line(& mut media_aluno).expect("Erro ao ler 'medias_alunos'... ");
        i+= 1;
        if conver_pnum(&media_aluno) < 3 {
            soma_rep += 1;
        } else if conver_pnum(&media_aluno) > 3 && conver_pnum(&media_aluno) < 6{
            soma_rec += 1;
        } else if conver_pnum(&media_aluno) >= 6 {
            soma_apr += 1;
        }
    }
    println("=========================================================");
    println("O Número de Alunos Aprovados foram: {}", soma_apr);
    println("=========================================================");
    println("O Número de Alunos em Recuperação foram: {}", soma_rec);
    println("=========================================================");
    println("O Número de Alunos Reprovados foram: {}", soma_rep);
    println("=========================================================");


}



