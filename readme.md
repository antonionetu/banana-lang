# Teste localmente:

1. Clone o repositório 
`git clone https://github.com/antonionetodeveloper/banana-lang`

2. Crie um alias para o "compilador banana"</br>
    2.1 No seu computador, abra o arquivo .bashrc ou .zshrc com
    `nano ~/.bashrc` ou `nano ~/.zshrc`</br>
    2.2 Veja a rota absoluta do projeto. Para isso, entre no projeto e dentro dentro dele, abra um terminal e digite: `pwd`</br>
    2.2 No ".bashrc" ou no ".zshrc", adicione uma linha e cole o comando abaixo. Atente-se para colar o caminho absoluto do projeto que você obteve no passo anterior.</br>
    `alias banana='(diretorio absoluto do projeto)/compiler/bin/main'`</br>
    2.3 Salve o arquivo e feche-o (ctrl + x, y, enter).</br>
    2.4 Feche o terminal e abra novamente.</br>

3. Agora você pode rodar arquivos .banana com o comando `banana (caminho do arquivo .banana)`.


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