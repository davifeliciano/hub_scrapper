# hub_scrapper

Um Web Scrapper para extrair as soluções dos exercícios do Driven HUB com seus
respectivos enunciados, convertendo-os para Markdown.

https://github.com/davifeliciano/hub_scrapper/assets/26972046/62068090-359d-4b96-b393-ddf7269144f3

## Setup

Para instalar o scrapper é necessário ter uma instalação do toolchain do Rust em
sua maquina. Para instruções de instalação acesse
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
É preciso também ter uma instalação do Chrome para que o scrapper funcione
corretamente. Além disso, é necessário que o chrome driver esteja em execução.
Para baixar o binário do chromedriver para a sua plataforma e versão do Chrome
correspondente, visite
[https://chromedriver.chromium.org/downloads](https://chromedriver.chromium.org/downloads)
e em seguida siga os passos abaixo.

1. Execute o chromedriver

   ```bash
   $ chromedriver
   # Starting ChromeDriver <CHROME_VERSION> on port 9515
   # Only local connections are allowed.
   # Please see https://chromedriver.chromium.org/security-considerations for suggestions on keeping ChromeDriver safe.
   # ChromeDriver was started successfully.
   ```

   Por padrão ele escutará na porta 9515. Caso essa porta já esteja em uso por
   outro programa, use a opção `--port`.

2. Instale o scrapper com

   ```bash
   $ cargo install --git https://github.com/davifeliciano/hub_scrapper
   ``````

3. Teste a instalação e consulte opções de uso com

   ```bash
   $ hub_scrapper --help
   ```
