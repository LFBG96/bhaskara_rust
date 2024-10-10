use std::io;

//criar uma função para converter em inteiro
fn conv_int(input: &str) -> f64 {
    let data = input.trim().parse().expect("Error ao converter para int");
    data //retorno da função
}
//criar uma função para o programa fechar, apenas ao pressionar ENTER
fn press_enter() {
    println!("Pressione ENTER para finalizar!");
    let mut vazio = String::new();
    io::stdin()
        .read_line(&mut vazio)
        .expect("Error ao ler input vazio");
}

fn main() {
    println!("{}", "-".repeat(30));
    println!(
        "
    CALCULADORA DE BHASKARA
    Digite os valores de A,B,C
    "
    );

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Digite o valor de A:");
    io::stdin()
        .read_line(&mut a)
        .expect("Error ao ler valor de a");

    println!("Digite o valor de B:");

    io::stdin()
        .read_line(&mut b)
        .expect("Error ao ler valor de B");

    println!("Digite o valor de C:");

    io::stdin()
        .read_line(&mut c)
        .expect("Error ao ler valor de C");

    println!(
        "Os valores digitados foram {}, {} e {}",
        a.trim(),
        b.trim(),
        c.trim()
    );
    let a = conv_int(&a);
    let b = conv_int(&b);
    let c = conv_int(&c);

    let delta: f64 = (b * b) - 4.0 * a * c;
    println!("Valor de delta {delta}");
    if a != 0.0 && delta > -1.0 {
        let x1 = (-b + delta.sqrt()) / (a * 2.0);
        let x2: f64 = (-b - delta.sqrt()) / (a * 2.0);
        let xx1 = -b + delta.sqrt();
        let xx2 = -b - delta.sqrt();
        let valor_a = a * 2.0;
        println!(
            "
        X1 = {}
        X2 = {}

        ou 

        X1 = {}
            ----
             {}

        X2 = {}
            ----
             {}
        ",
            x1, x2, xx1, valor_a, xx2, valor_a
        );
    }
    else{
        println!("Delta não pode ser negativo | Valor de A não pode ser 0 ")
    }

    println!("{}", "-".repeat(30));
    press_enter();
}
