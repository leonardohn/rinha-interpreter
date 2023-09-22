# `rinha-interpreter`

Um interpretador para a AST no formato `rinha`, com o objetivo de
competir na [Rinha de Compiler][rinha].

[rinha]: https://github.com/aripiprazole/rinha-de-compiler

## Objetivos

Este interpretador foi desenvolvido com os objetivos de ser correto,
conciso e simples de entender. Se sobrar tempo pretendo desenvolver
outra submissão orientada a performance.

## Estado

Este interpretador implementa os seguintes recursos:

- [X] Inteiros
    - [X] Add
    - [X] Sub
    - [X] Mul
    - [X] Div
    - [X] Rem
    - [X] Lt
    - [X] Gt
    - [X] Lte
    - [X] Gte
    - [X] Eq
    - [X] Neq
- [X] Booleanos
    - [X] And
    - [X] Or
    - [X] Eq
    - [X] Neq
- [X] Strings
    - [X] Add
    - [X] Eq
    - [X] Neq
- [X] Tuplas
    - [X] First
    - [X] Second
- [X] Variáveis
- [X] Condicionais
- [X] Funções

## Instruções (Docker)

Para construir a imagem:
```bash
docker build -t local:rinha-interpreter .
```

Para executar o interpretador:
```bash
docker run --rm -it -v $RINHA_PATH:/var/rinha local:rinha-interpreter
```
Onde `RINHA_PATH` é um diretório contendo o arquivo `source.rinha.json`.

## Licença

Este projeto usa a licença [MIT](LICENSE).
