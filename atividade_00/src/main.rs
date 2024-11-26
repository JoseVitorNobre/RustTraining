use std::io;

fn media_positivos(numbers: [i32; 10]) -> Option<i32> {
    let mut media = 0;
    let mut qtd_positivos = 0;
    for val in numbers.iter() {
        if *val >= 0 {
            media += val;
            qtd_positivos += 1;
        }
    }
    if qtd_positivos > 0 {
        Some(media / qtd_positivos)
   } else {
        None
   }
}

fn produto_pares(numbers: [i32; 10]) -> i32 {
    let mut produto = 1;
    let mut qtd_pares = 0;
    for val in numbers.iter() {
        if *val % 2 == 0 {
            produto *= val;
            qtd_pares += 1;
        }
    }
    if qtd_pares > 0 {
        produto
   } else {
       1
   }
}

fn questao_01_main() {
    let numeros = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];
    match media_positivos(numeros) {
        Some(media) => println!("Média dos números positivos: {}", media),
        None => println!("Não há números positivos."),
    }
    let produto = produto_pares(numeros);
    println!("Produto dos números pares: {}", produto);
}

fn achar_maior(tupla: (i32, i32, i32)) -> i32 {
    let mut max = tupla.0;

    if max < tupla.1 {
        max = tupla.1;
    }

    if max < tupla.2 {
        max = tupla.2
    }

    max
}

fn achar_menor(tupla: (i32, i32, i32)) -> i32 {
    let mut min = tupla.0;

    if min > tupla.1 {
        min = tupla.1;
    }

    if min > tupla.2 {
        min = tupla.2
    }

    min
}


fn analisar_tupla(tupla: (i32, i32, i32)) -> (i32, i32, i32) {
    let soma = tupla.0 + tupla.1 + tupla.2;
    let maior = achar_maior(tupla);
    let menor = achar_menor(tupla);

    (soma, maior, menor)
}

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    println!("Digite o primeiro numero");
    io::stdin().read_line(&mut input_1).expect("Falha ao ler inteiro");
    
    println!("Digite o segundo numero");
    io::stdin().read_line(&mut input_2).expect("Falha ao ler inteiro");
    
    println!("Digite o terceiro numero");
    io::stdin().read_line(&mut input_3).expect("Falha ao ler inteiro");
    
    let val_1: i32 = input_1.trim().parse().expect("Por favor insira um numero valido");
    let val_2: i32 = input_2.trim().parse().expect("Por favor insira um numero valido");
    let val_3: i32 = input_3.trim().parse().expect("Por favor insira um numero valido");

    let resultado = analisar_tupla((val_1, val_2, val_3));
    
    println!("Soma: {}, Maior: {}, Menor: {}", resultado.0, resultado.1, resultado.2);
}
