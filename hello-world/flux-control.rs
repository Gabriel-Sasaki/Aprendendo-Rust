fn main() {
    if_statement();
    iteration_statement();
}

fn iteration_statement() {
    let num_to_multiply: u8 = 5;

    let mut count: u8 = 0;

    while count < 10 {
        count += 1;
        let result: u8 = num_to_multiply * count;
        println!("{} x {} = {}", num_to_multiply, count, result);
    }

    println!("-----------------------------------------");

    count = 0;
    // O loop repete infinitamente, por isso exige um break, para não ficar como
    // um loop infinito. Ele pode ser usado como um do while. O continue também
    // existe caso seja necessário "pular" alguma iteração
    loop {
        count += 1;

        let result: u8 = num_to_multiply * count;
        println!("{} x {} = {}", num_to_multiply, count, result);

        if count > 10 {
            break;
        }
    }

    println!("-----------------------------------------");

    // "for i in 1..=10" faz a mesma coisa. O 1 é incluído, mas o 11 não
    for i in 1..11 {
        let result: u8 = num_to_multiply * i;
        println!("{} x {} = {}", num_to_multiply, i, result);
    }

    println!("-----------------------------------------");

    let language = "C#";
    // O match statement itera sobre cada possível valor e retorna aquele que
    // corresponder. Caso nenhum corresponda ele cairá no default que é o &_ ou
    // apenas _
    let purpouse = match language {
        "PHP" => "Web",
        "Kotlin" => "Mobile",
        "Python" => "Data Science",
        &_ => "Desconhecido"
    };
    println!("O propósito de {} é {}", language, purpouse);
}

fn if_statement() {
    let age: u8 = 15;
    let parents_authorization: bool = true;


    if age >= 18 || (age >= 16 && parents_authorization) {
        println!("Pode entrar!");
    } else {
        println!("Não pode entrar!");
    }

    let age_condition = if age >= 18 { "maior" } else { "menor" };

    println!("A pessoa é {} de idade", age_condition);
}