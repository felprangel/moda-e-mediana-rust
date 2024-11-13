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

    let num1: u32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let num2: u32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let num3: u32 = match num3.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let num4: u32 = match num4.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let num5: u32 = match num5.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let media = (num1 + num2 + num3 + num4 + num5) / 5;

    println!("A média dos números é: {}", media);

    let mut occurrences = std::collections::HashMap::new();

    for &value in &[num1, num2, num3, num4, num5] {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let mode = occurrences.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val).unwrap_or(0);

    println!("A moda dos números é: {}", mode);
}