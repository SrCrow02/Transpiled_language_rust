# Transpiler para Ganck

Este projeto é um transpiler para a linguagem "Ganck", transformando código escrito em Ganck para código Rust com suporte para Actix. O transpiler lê um arquivo de entrada com código Ganck e gera o código Rust correspondente.

## Estrutura do Projeto

- `src/main.rs`: Arquivo principal do transpiler.
- `src/transpiler.rs`: Arquivo que faz o transpiler
- `src/file_io.rs`: Leitura e escrita de arquivos


## Comandos de Transpilação

O transpiler suporta a transcrição de várias estruturas de controle e funções da linguagem Ganck para Rust. O processo de transpiração é feito através da análise de um arquivo de entrada que contém código Ganck.

## Como Usar

### Pré-requisitos

Certifique-se de ter o [Rust](https://www.rust-lang.org/tools/install) instalado no seu sistema. Você pode instalar o Rust usando o `rustup`.

### Instalação

Clone o repositório e navegue até o diretório do projeto:

```bash
git clone https://github.com/SrCrow02/Transpiled_language_Rust
cd transpiled_language_Rust
```

Edite o codigo Ganck em input.ganck caso queira:

```
function main
    for i in 0..10
        print("")
    endfor

    while x < 10
        run()
        if x < 5
            print("legal")
        endif
        else 
            print("Nao e legal")
        endelse
    endwhile
endfunction

function run
    println("oi")
endfunction
```

Rode o codigo:

```
cargo run
```


