# Usize e Isize
Em tamanhos de arrays, o Rust utiliza os tipos usize e isize, sendo unsigned size e integer signed size. O motivo de utilizar esse tipo ao invés de um inteiro comum, é porque uma máquina de 32 bits tem o limite do valor do ponteiro de 4 bytes, enquanto uma máquina de 64 bits tem o limite de 8 bytes. Portanto, o usize e o isize variam de tamanho de acordo com a quantidade de bits. Isso torna o Rust mais adaptativo.
```rust
fn main() {
    let array: [u16; 3] = [1, 2, 3];
    let index: usize = 0;
    // Imprime 1
    println!("{}", array[index]);
}
```

# Uso de ::
Quando precisar chamar um método estático ou alguma propriedade estática (como os valores dos enums), se utiliza *::*. Isso ocorre para poder referenciar o namespace correto de onde está vindo aquele método etc. No caso da biblioteca padrão chamada de prelude, os arquivos já estão prontos para uso, então não requerem *::*.

# Generics
Em Rust, quando um generic é definido, ele na verdade é utilizado como um template para criar uma variante para cada tipo utilizado no programa. Isso acontece em tempo de compilação. Exemplo: Se definir um enum genérico e usar ele uma vez como String e outra como u8, o compilador vai usar o template genérico para criar uma versão do enum como String e uma como u8.
```rust
fn main() {
    Example::<String>::Value(String::from("This example"));
    Example::<u8>::Value(123);
}

enum Example<T> {
    Value(T),
    OtherValue
}
```
O que o compilador fará em tempo de compilação:
```rust
fn main() {
    Example::<String>::Value(String::from("This example"));
    Example::<u8>::Value(123);
}

enum Example {
    Value(String),
    OtherValue
}

enum Example {
    Value(u8),
    OtherValue
}
```

# Vectors
Os vectors, ou vetores, são arrays dinâmicos. Eles permitem a adição e remoção dos seus elementos. Porém, existe algumas considerações. Quando um vetor é criado, o Rust aloca a quantidade de memória necessária para conter os elementos definidos na declaração. Se um novo elemento for adicionado, o vetor é copiado para outro espaço de memória, dobrando a capacidade do vetor. Realocar memória dessa forma é custoso, portanto o Rust sempre dobra a capacidade para evitar realocar novamente.
```rust
fn main() {
    // Aqui o vector tem 3 elementos, então o espaço em memória reservado é para
    // conter exatamente 3 elementos
    let mut vector: Vec<u8> = vec![1, 2, 3];

    // Nesse momento o Rust realoca os elementos para um novo vector com o dobro
    // da capacidade anterior, ou seja, agora ele suportará 6 elementos até precisar
    // ser realocado novamente
    vector.push(4);

    // Imprime a capacidade atual do vector, que nesse caso é 6
    println!("{}", vector.capacity());
}
```
Se você souber a capacidade que seu vetor precisará ter, o recomendado é já alocar esse espaço em memória. Isso pode ser feito com o método estático with_capacity(capacity: usize).
```rust
fn main() {
    // O vector já é criado com capacidade para armazenar 4 elementos
    let mut vector: Vec<u8> = Vec::with_capacity(4);
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
}
```