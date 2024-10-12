
mod funcoes;

fn main() {
    println!("{}", "-".repeat(30));
    println!(
        "
    CALCULADORA DE BHASKARA
    Digite os valores de A,B,C
    "
    );

    let a = funcoes::no_zero_a();
    println!("Digite o valor de B:");
    let b = funcoes::input();
    println!("Digite o valor de C:");
    let c = funcoes::input();

    let delta = funcoes::calcular_delta(a, b, c);

    println!("Valor de delta {delta}");
    funcoes::resultado(a, b, delta);
    println!("{}", "-".repeat(30));
    funcoes::press_enter();
}
