# Ownership
Toda referência de um valor na heap só pode ter um "dono" simultaneamente. O que significa que se uma função A passar essa referência para uma função B, a função A "perde" o acesso à variável que guarda a referência.
```rust
fn function_a() {
    let text = String::from("Text");
    function_b(text);
    // A linha abaixo dará erro porque a variável de referência text não pertence
    // mais à function_a, pois foi passada anteriormente para a function_b
    println!("{}", text);
}

fn function_b(text: String) {
    println!("{}", text);
}
```

# Escopo
Sempre que um escopo terminar, as variáveis na heap são automaticamente desalocadas.
```rust
fn main() {
    let text = String::from("Text");
}
// Depois que o escopo main() terminar o valor de text na heap é desalocado
```

# Borrowing
Quando uma variável que guarda a referência de uma valor na heap precisar ser passada para uma função B sem a função A perder o "direito de uso" da variável, o Rust permite "emprestar" ela de uma escopo para outro.
```rust
fn function_a() {
    let text = String::from("Text");
    function_b(&text);
    // Não dará erro porque a referência foi "emprestada" para a function_b
    println!("{}", text);
}

// Precisa definir que text é um valor do tipo String que foi emprestado (&String)
fn function_b(text: &String) {
    println!("{}", text);
}
```

# Mutabilidade no Borrowing
Quando for necessário que a referência passada de uma função A para um função B seja mutável, basta acrescentar a palavra-chave *mut* no recebimento e no envio da referência. Essa palavra *mut* precisa ter o *&* antes, um espaço e o nome da variável de referência. É importante destacar que a variável de referência precisa ter sido declarada como mutável para isso acontecer.
```rust
fn function_a() {
    let mut text = String::from("Text");
    function_b(&mut text);
    println!("{}", text);
}

fn function_b(text: &mut String) {
    text.push_str("...");
    println!("{}", text);
}
```

# Pattern Matching
O pattern matching funciona através do match statement. Ao invés de ser apenas um switch case comum, o match case do Rust funciona usando pattern matching, então é possível utilizar expressões mais complexas para identificar qual opção deve ser escolhida.
```rust
fn main() {
    let number: u8 = 1;

    let matching = match number {
        1 => "Muito pouco",
        2 | 3 => "Pouco",
        4..=10 => "Médio",
        _ if number % 2 == 0 => "Muito (par)",
        _ => "Muito (ímpar)"
    };
}
```
Essas expressões utilizam os operadores de range (como 1..=10), booleanos (apenas com um | para ou etc) e condicionais (com if).

# Erros
Os erros de panic são irrecuperáveis - o Rust não possui bloco try / catch ou algo similar. Para lançar seu próprio panic basta utilizar a macro *panic!(message: String)*.
```rust
fn main() {
    panic!("Erro de exemplo");
}
```
Outra forma de gerar erro é usando o Result<success_type, error_code_type>, o Ok(message) e o Err(error_code). Com isso é possível tratar o erro usando um match statement.
```rust
fn main() {
    match error_method(true) {
        Ok(message) => println!("Sucesso! {}", message),
        Err(error_code) => panic!("Erro! Código: {}", error_code)
    }
}

fn error_method(throw_error: bool) -> Result<String, u8> {
    if throw_error {
        Err(21);
    } else {
        Ok(String::from("A operação ocorreu com sucesso!"));
    }
}
```

# Cargo
O cargo é um package manager, ou seja, um gerenciador de pacotes. Ele permite gerenciar as dependências, builds etc. Por padrão o cargo vem junto com o rustc e o rustup.
As dependências ficam registradas no arquivo *Cargo.toml*.

```toml
[package]
name = "exemplo"
version = "0.0.1"
authors = ["Gabriel Sasaki <gabrielpintosasaki@outlook.com>"]
```

```bash
# Compila e executa o projeto (precisa do arquivo Cargo.toml)
cargo run

# Cria um novo pacote no diretório informado (se não existir ele cria um novo)
cargo new <path>

# Cria um novo pacote binário (de aplicação) no diretório informado
cargo new --bin <path>

# Cria um novo pacote de biblioteca no diretório informado
cargo new --lib <path>

# Cria um pacote utilizando um projeto existente
cargo init
```