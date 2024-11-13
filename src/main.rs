use std::io;

fn main() {
    println!("Digite o primeiro número: ");

    let mut num1 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Falha ao ler a linha");

    println!("Digite o segundo número: ");

    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num2)
        .expect("Falha ao ler a linha");

    println!("Digite o terceiro número: ");

    let mut num3 = String::new();

    io::stdin()
        .read_line(&mut num3)
        .expect("Falha ao ler a linha");

    println!("Digite o quarto número: ");

    let mut num4 = String::new();

    io::stdin()
        .read_line(&mut num4)
        .expect("Falha ao ler a linha");

    println!("Digite o quinto número: ");

    let mut num5 = String::new();

    io::stdin()
        .read_line(&mut num5)
        .expect("Falha ao ler a linha");
}