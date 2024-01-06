# Teste localmente:

1. Clone o repositório 
`git clone https://github.com/antonionetodeveloper/banana-lang`

2. Crie um alias para o "compilador banana"
    2.1 Abra o arquivo .bashrc ou .zshrc
    `nano ~/.bashrc` ou `nano ~/.zshrc`
    2.2 Veja o trageto absoluto do projeto: dentro do projeto, abra um terminal e digite `pwd`
    2.2 Adicione a linha e cole o comando. Atente-se para colar o caminho absoluto do projeto que você obteve no passo anterior.
    `alias banana='(diretorio absoluto do projeto)/compiler/bin/main'`
    2.3 Salve o arquivo e feche-o (ctrl + x, y, enter).
    2.4 Feche o terminal e abra novamente.

3. Agora você pode rodar arquivos .banana com o comando `banana (caminho do arquivo banana)`.


# O que dá pra fazer com a linguagem?

- [x] Somar dois números inteiros.
- [x] Subtrair dois números inteiros.
- [x] Multiplicar dois números inteiros.
- [x] Dividir dois números inteiros.

## Operações:
    `+` soma
    `-` subtração
    `*` multiplicação
    `/` divisão

## Exemplos:
    `+ 1 1`
    `- 1 1`
    `* 1 1`
    `/ 1 1`