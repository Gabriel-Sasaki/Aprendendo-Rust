// Variáveis globais precisam ter seu tipo e valor definidos na declaração

// Constantes não são armazenadas em memórias, suas referências no código são
// substituídas pelo valor durante o tempo de compilação
const PI: f32 = 3.14;

// Variáveis estáticas são normalmente usadas como variáveis globais - que podem
// ser acessadas de qualquer partes do código, além disso ela é armazenada em memória
static mut GLOBAL_VARIABLE: u8 = 1;

fn main() {
    // O uso do sinal de exclamação após o println se dá porque ele é uma macro, que é
    // uma forma mais simples e elegante de se chamar uma função
    println!("PI = {}", PI);

    // O escopo unsafe define para o compilador que o código presente nele pode ser inseguro,
    // ou seja, pode realizar operações perigosas com a memória ou com as threads
    unsafe {
        println!("GLOBAL_VARIABLE = {}", GLOBAL_VARIABLE);
    }

    let variable: u8 = 128;
    println!("variable = {}, bytes = {}", variable, std::mem::size_of_val(&variable));

    let decimal_var: f32 = 2.5;
    println!("decimal_var = {}, bytes = {}", decimal_var, std::mem::size_of_val(&decimal_var));

    let boolean_var: bool = true;
    println!("boolean_var = {}, bytes = {}", boolean_var, std::mem::size_of_val(&boolean_var));

    let letter: char = 'C';
    println!("letter = {}, bytes = {}", letter, std::mem::size_of_val(&letter));

    let value_returned: u16 = return_value(1, 2);
    println!("value_returned = {}", value_returned);
}

// Em Rust as variáveis existem apenas nos seus próprios escopos e são desalocadas quando
// saem desse escopo. Porém isso pode ser alterado usando o borrowing (emprestar). Veja mais
// no código borrowing.rs
fn return_value(a: u16, b: u16) -> u16 {
    // O retorno de uma função se dá quando a linha da operação ou da variável não possui
    // ponto e vírgula
    a + b
}