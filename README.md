# Unisulma-ti

<h1 align="center"> Unisulma Setor TI </h1>
<p align="justify"> Site focado em seu style rabuscado, porém com usando o WebAssembly + Rust = ❤</p>
<img alt="Website" src="https://img.shields.io/website?down_message=offline&up_message=online&url=https%3A%2F%2Funisulma-ti.web.app/"> 

<h3>Objetivos</h3>

  - [x] Barra de pesquisa;
  - [x] Json to NoSql firebase;
  - [x] Admin route;
  - [x] Forms com envio de e-mail automático;
  - [x] Info-sys, tudo implementado como componentes;
  - [x] Neomorphism design;
  - [x] Editar dados;
  - [X] Gravar novos dados;
  - [x] "Deletar dados" - foi usado um recurso de visibilidade;
  - [ ] Login baseado no ComputerID como uma senha personalizada;
  
  
  <h3>ATENÇÃO!</h3>

  
    EXECUTE TODOS OS COMANDOS NO DIRETORIO RAIZ DO PROJETO
  **lembre-se de estar com as suas toolchains na versao: _nighlty_ _(so...don't be a donkey)_** 
  
 *links uteis:
    *[versionamento com rustup](https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html)
    *[sobre arquiteturas](https://raw.githubusercontent.com/wiki/hjl-tools/x86-psABI/x86-64-psABI-1.0.pdf)
  
  <h3>Build</h3>
  
  	os comandos demonstrados aqui sao uma serie de passos,nao pule nenhum.
	
  
  <h4>Trunk</h4>
 
 	
  
  Precisa ter instaldo o compilador *Rust* 
  Depois de instalado, agora instale os seguintes ferramentas do rustup.

  
  ```
  rustup target add wasm32-unknown-unknown
  ```
  
  ```
  cargo install trunk wasm-bindgen-cli
  ```
  Depois de tudo instalado nos conformes, execute o comando:
  *Se quiser pode dá só o comando ```trunk serve```, irá funcionar da mesma forma*
  
  ```
  trunk build && trunk serve
  ```

  <h3>Build para Linux</h3>

				  
  <h4>wasm-bindgen-cli</h4>
  
  Primeiro instale o gerador de JavaScript.

  ```
  cargo install wasm-bindgen-cli
  
  ```
  
  Ferramenta do Rustup com suporte a WebAssembly.

  ```
  rustup target add wasm32-unknown-unknown
  ```

  Trunk é um construtor de aplicativo web em WASM para 
Rust, é necessária a instalção.
 
  ```
  cargo install trunk wasm-bindgen-cli
  ```
  
  O comando abaixo irá gerar um arquivo .wasm.

  ```
  cargo build --target wasm32-unknown-unknown

  ```
   
  Esse comando irá gerar um conjunto de arquivos
contendo o WebAssembly compilado do seu aplicativo e um
wrapper JavaScript que carregará o binário Wasm e o
executará.


  ```
  wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/LowStream.wasm --no-typescript
  ```
  O comando abaixo vai tornar o servidor ativo, para 
que seu acesso possa ser possibilitado.

  ```
  trunk serve
  ```
  Sempre que houver alguma modificação, o seu .wasm precisa ser atualizado, então execute:
  
  ```
  cargo build --target wasm32-unknown-unknown && trunk serve

  ```

  Build information from Lowstream-Comunity
