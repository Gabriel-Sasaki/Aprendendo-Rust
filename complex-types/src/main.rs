fn main() {
    array_one();
    array_two();
    matrix();
    enumerator();
    enumerator_two();
    optional_content();
    vectors_one();
    vectors_two();
    structs();
}

fn array_one() {
    // É um array com valores do tipo f32 (float de 32 bits) com 4 elementos.
    // O número seguido do f32 é um type casting
    let degrees: [f32; 4] = [10f32, 8f32, 9.5, 6.0];

    // Forma 1
    for degree in degrees {
        println!("{}", degree);
    }

    println!("---------------------------------");

    // Forma 2: Usando o método len() é possível obter o tamanho do array em usize
    for index in 0..degrees.len() {
        println!("{}", degrees[index]);
    }

    println!("---------------------------------");
}

fn array_two() {
    // Usando a sintaxe [valor; multiplicador] o Rust gera um array com o 6.5 repetido
    // 4 vezes
    let degrees: [f64; 4] = [6.5; 4];

    for degree in degrees {
        println!("{}", degree);
    }

    println!("---------------------------------");
}

fn matrix() {
    // [[type_elements; number_elements]; number_arrays]
    let matrix: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for line in matrix {
        for column in line {
            print!("[{}]  ", column);
        }
        println!();
    }

    println!("---------------------------------");
}

fn enumerator() {
    let is_weekend: bool = is_weekend(DayOfWeek::Sunday);
    println!("É final de semana? {}", is_weekend);
}

// Decorator para permitir código morto, ou seja, código não utilizado
#[allow(dead_code)]
// Os enums são tipos personalizados
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

fn is_weekend(day: DayOfWeek) -> bool {
    match day {
        DayOfWeek::Saturday | DayOfWeek::Sunday => true,
        _ => false
    }
}

// Decorator para permitir código morto, ou seja, código não utilizado
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    // É um valor mais complexo, que armazena outros tipos dentro dele, nesse caso os tipos
    // NÃO são nomeados
    RgbColor(u8, u8, u8),
    // Também é um valor mais complexo, porém possuí valores nomeados, o que facilita o uso
    // correto
    CymkColor{cyan: u8, yellow: u8, magenta: u8, black: u8}
}

fn enumerator_two() {
    let color: Color = Color::CymkColor { cyan: 1, yellow: 1, magenta: 1, black: 1 };
    println!("{}", match color {
        Color::Red => "Vermelho",
        Color::Green => "Verde",
        Color::Blue => "Azul",
        Color::RgbColor(0, 0, 0)
            | Color::CymkColor { cyan: _, yellow: _, magenta: _, black: 255 } => "Preto",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor { cyan: _, yellow: _, magenta: _, black: _ } => "Cymk desconhecido"
    });
}

fn optional_content() {
    let file_content: Option<String> = read_file(String::from(""));

    match &file_content {
        Some(content) => println!("{}", content),
        None => println!("Conteúdo vazio!")
    };

    // Usando :? o Rust mostra a construção daquela variável, ajudando a debuggar
    println!("{:?}", file_content);

    // Usando um if let, é possível testar se uma variável atende o padrão de uma
    // estrutura e, se sim, atribuir ela. Existe também o while let, que repete
    // enquanto a variável atender o padrão
    if let Some(value) = file_content {
        println!("{}", value);
    }
}

#[allow(dead_code)]
fn read_file(_path_file: String) -> Option<String> {
    Some(String::from("Conteúdo do arquivo"))
}

fn vectors_one() {
    // O tipo vector possui tamanho dinâmico, o que permite a adição de
    // novos elementos mesmo após sua declaração
    let mut grades: Vec<f32> = Vec::<f32>::new();
    grades.push(10.0);
    grades.push(8.8);
    grades.push(6.5);
    println!("{:?}", grades);
}

fn vectors_two() {
    // Outra forma de declarar vectors é utilizando a macro vec![elements]
    let mut grades: Vec<f32> = vec![10.0, 8.8, 6.5];
    grades.push(4.4);
    println!("{:?}", grades);

    // O acesso por índices funciona também com os vectors
    println!("{}", grades[0]);

    // O método get() retorna um Option com Some() caso o elemento no índice
    // especificado exista e None caso não exista
    println!("{}", match grades.get(7) {
        // O símbolo * faz com que o value seja desreferenciado. Por padrão o
        // value seria uma referência para o valor no vector, mas o * tira a
        // referência e transforma em um valor isolado
        Some(value) => *value,
        None => 0.0
    });

    // Remove o último elemento do vector e o retorna
    let ultimo_elemento: Option<f32> = grades.pop();
    println!("{}", match ultimo_elemento {
        Some(value) => value,
        None => 0.0
    });

    grades.push(2.2);
    grades.push(7.5);
    grades.push(8.7);
    grades.push(1.3);
    grades.push(5.7);
    grades.push(9.6);

    println!("Capacidade do vetor: {}", grades.capacity());

    // Remove o último valor e imprime se ele existir
    if let Some(value) = grades.pop() {
        println!("Valor removido: {}", value);
    }

    // Remove todos os elementos até acabar
    while let Some(value) = grades.pop() {
        println!("Valor removido: {}", value);
    }
}

// Uma struct encapsula outros tipos e fornece um "template" para
// a criação de instâncias individuais. Structs são armazenadas na
// stack
struct Titular {
    nome: String,
    sobrenome: String,
    cpf: String
}

// Structs podem conter outras structs
struct Conta {
    titular: Titular,
    saldo: f64
}

// Uma implementação adiciona funções (métodos) em uma struct
impl Conta {
    // Assim como em Python, os métodos recebem uma variável chamada
    // "self", que referência a instância daquela struct. No caso abaixo,
    // como a instância vai ter o saldo modificado, então ela precisa ser
    // mutável. Outro detalhe é que por ser uma referência à uma instância
    // então precisa usar o símbolo "&"
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}


fn structs() {
    let titular: Titular = Titular { nome: String::from("Gabriel"),
                                     sobrenome: String::from("Sasaki"),
                                     cpf: String::from("123.456.789-10") };

    // Quando o nome da variável do valor for igual ao atributo / parâmetro
    // basta informar apenas o nome da variável
    let mut conta: Conta = Conta{titular, saldo: 100.0};

    println!("Nome: {}, Sobrenome: {}, CPF: {}, Saldo: {}",
             conta.titular.nome, conta.titular.sobrenome, conta.titular.cpf, conta.saldo);

    conta.sacar(50.0);

    println!("Nome: {}, Sobrenome: {}, CPF: {}, Saldo: {}",
             conta.titular.nome, conta.titular.sobrenome, conta.titular.cpf, conta.saldo);
}



