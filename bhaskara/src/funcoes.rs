use std::io;
pub fn press_enter() {
    println!("Pressione ENTER para finalizar!");
    let mut vazio = String::new();
    io::stdin()
        .read_line(&mut vazio)
        .expect("Error ao ler input vazio");
}

pub fn input() -> f64 {
    loop {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Error ao ler valor input");

        let valor = data.trim();
        match valor.parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => println!("Valor invalido"),
        }
    }
}

pub fn calcular_delta(a: f64, b: f64, c: f64) -> f64 {
    let delta: f64 = (b * b) - 4.0 * a * c;
    delta
}
pub fn no_zero_a() -> f64 {
    loop {
        let mut dado = String::new();
        println!("Digite o valor de A:");
        io::stdin().read_line(&mut dado).expect("Error ao ler dado");
        let verificar = dado.trim();
        match verificar.parse::<f64>() {
            Ok(n) if n != 0.0 => return n,
            Ok(_) => println!("Numero deve ser diferente de 0"),
            Err(_) => println!("Numero invalido"),
        }
    }
}

pub fn resultado(a: f64, b: f64, delta: f64) {
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
    } else {
        println!("Delta não pode ser negativo")
    }
}