# Rusty
Learn Rust

### Tipos básicos

string
```

```
let mut input: String = String::new();
    let _ = io::stdin().read_line(&mut input);
    
### Vetores

// Criando um vetor vazio de i32
`let vetor_vazio: Vec<i32> = Vec::new();`

// Criando um vetor inicializado com elementos
`let vetor_inicializado = vec![1, 2, 3, 4, 5];`

### Operações basicas

#### Converter tipos

Converter para nº simples mas inseguro
`let numero: i32 = input.parse().unwrap();`

Converter para nº mais verboso e seguro
```
match input.parse::<i32>() {
    Ok(numero) => {
        println!("A string convertida para i32 é: {}", numero);
    }
    Err(_) => {
        println!("Não foi possível converter a string para i32.");
    }
}
```