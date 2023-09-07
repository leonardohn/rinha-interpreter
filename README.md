# `rinha-interpreter`

Um interpretador para a AST no formato `rinha`, com o objetivo de
competir na [Rinha de Compiler][rinha].

[rinha]: https://github.com/aripiprazole/rinha-de-compiler

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
