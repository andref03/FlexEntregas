# ⬆️📦 FlexEntregas

Este repositório contém um projeto em Rust que simula a entrega de pacotes utilizando diferentes meios de transporte, como moto, bicicleta, drone e cavalo. O objetivo principal é demonstrar o uso de polimorfismo através de traits em Rust para permitir que diferentes meios de entrega sejam tratados de forma uniforme, calculando tempo estimado e custo de combustível, além de verificar restrições climáticas específicas para cada um.

## 📋 Descrição do Projeto

O projeto `FlexEntregas` é uma aplicação de linha de comando desenvolvida em Rust que simula um sistema de entregas. Ele foi projetado para ilustrar como o polimorfismo pode ser implementado em Rust usando `traits`. Cada meio de entrega (moto, bicicleta, drone, cavalo) implementa o trait `MeioDeEntrega`, que define um conjunto comum de comportamentos, como calcular tempo de entrega, custo e verificar restrições.

### 🧩 Estrutura do Projeto

O projeto está organizado da seguinte forma:

- `src/main.rs`: Contém a lógica principal da aplicação, onde as simulações de entrega são realizadas para cada meio de transporte.
- `src/estrategias/`: Este diretório contém os módulos que definem os diferentes meios de entrega e o trait `MeioDeEntrega`.
  - `src/estrategias/meio_de_entrega.rs`: Define o trait `MeioDeEntrega`, que estabelece a interface para todos os meios de transporte.
  - `src/estrategias/moto.rs`: Implementa a estrutura `Moto` e o trait `MeioDeEntrega` para entregas de moto.
  - `src/estrategias/bicicleta.rs`: Implementa a estrutura `Bicicleta` e o trait `MeioDeEntrega` para entregas de bicicleta.
  - `src/estrategias/drone.rs`: Implementa a estrutura `Drone` e o trait `MeioDeEntrega` para entregas de drone.
  - `src/estrategias/cavalo.rs`: Implementa a estrutura `Cavalo` e o trait `MeioDeEntrega` para entregas a cavalo.

## 🧮 Estratégias Utilizadas

O projeto emprega as seguintes estratégias de design e programação:

1.  **Programação Orientada a Objetos (POO) com Traits**: Embora Rust não seja uma linguagem orientada a objetos no sentido tradicional (não possui herança de classes), ele suporta conceitos de POO através de `traits`. O trait `MeioDeEntrega` atua como uma interface, permitindo que diferentes tipos de meios de transporte compartilhem um comportamento comum.

2.  **Polimorfismo em Tempo de Execução (`dyn Trait`)**: O polimorfismo é alcançado usando `&dyn MeioDeEntrega` como parâmetro na função `simular_entrega`. Isso permite que a função aceite qualquer tipo que implemente o trait `MeioDeEntrega`, sem saber o tipo concreto em tempo de compilação. As chamadas de método (como `calcular_tempo_estimado` ou `verificar_restricoes`) são despachadas dinamicamente em tempo de execução, dependendo do tipo real do objeto.

3.  **Modularização**: O código é dividido em módulos (`main.rs`, `estrategias/meio_de_entrega.rs`, `estrategias/moto.rs`, etc.) para melhorar a organização, legibilidade e manutenção do código. Cada módulo é responsável por uma parte específica da funcionalidade do sistema.

## 🎲Polimorfismo em Destaque

O polimorfismo é o pilar central deste projeto. Ele é demonstrado claramente na função `simular_entrega` em `src/main.rs`:

```rust
pub fn simular_entrega(meio: &dyn MeioDeEntrega, distancia_km: f64, condicoes_climaticas: &str) {
    // ...
}
```

Nesta função, o parâmetro `meio` é um `&dyn MeioDeEntrega`. Isso significa que `simular_entrega` pode receber uma `Moto`, uma `Bicicleta`, um `Drone` ou um `Cavalo`, desde que todos implementem o trait `MeioDeEntrega`. A beleza do polimorfismo aqui é que a função `simular_entrega` não precisa saber os detalhes internos de como cada meio de entrega calcula seu tempo, custo ou verifica restrições. Ela simplesmente chama os métodos definidos no trait, e a implementação correta é selecionada em tempo de execução.

Cada implementação do trait `MeioDeEntrega` para os diferentes veículos define seu próprio comportamento:

-   **`Moto`**: Calcula o tempo com base na velocidade média e o custo com base no consumo de combustível. Restrições: só pode entregar com 


sol ou chuva.
-   **`Bicicleta`**: Calcula o tempo com base na velocidade média. Não tem custo de combustível. Restrições: só pode entregar com sol ou vento.
-   **`Drone`**: Calcula o tempo com base na velocidade média. Não tem custo de combustível (considerado insignificante para esta simulação). Restrições: só pode entregar com sol ou vento.
-   **`Cavalo`**: Calcula o tempo com base na velocidade média. Não tem custo de combustível. Restrições: só pode entregar com sol ou neve.

Isso demonstra a flexibilidade e extensibilidade do design: novos meios de entrega podem ser adicionados simplesmente implementando o trait `MeioDeEntrega`, sem a necessidade de modificar o código existente na função `simular_entrega`.

## 🧠 Como Usar o Código

Para compilar e executar este projeto, você precisará ter o Rust instalado em sua máquina. Se você ainda não o tem, pode instalá-lo seguindo as instruções em [rustup.rs](https://rustup.rs/).

1.  **Clone o repositório:**

    ```bash
    git clone https://github.com/andref03/FlexEntregas.git
    ```

2.  **Navegue até o diretório do projeto:**

    ```bash
    cd FlexEntregas
    ```

3.  **Execute o projeto:**

    ```bash
    cargo run
    ```

O comando `cargo run` compilará o projeto (se ainda não o fez) e executará o binário resultante. As simulações de entrega serão exibidas diretamente no terminal.

## 📱 Saídas Esperadas

Ao executar o comando `cargo run`, você deverá ver uma saída similar à seguinte no seu terminal. Note que os valores de tempo e custo podem variar ligeiramente dependendo da precisão do ponto flutuante e das implementações exatas, mas a estrutura e a lógica devem ser as mesmas.

```
--- Simulações de Entrega ---
Simulação para Moto: Tempo estimado = X.XXh, Custo = R$Y.YY
Simulação para Bicicleta: Tempo estimado = X.XXh, Custo = R$Y.YY
Simulação para Drone: Restrições impedem a entrega sob as condições atuais.
Simulação para Cavalo: Restrições impedem a entrega sob as condições atuais.
Simulação para Moto: Restrições impedem a entrega sob as condições atuais.
Simulação para Bicicleta: Restrições impedem a entrega sob as condições atuais.
Simulação para Drone: Restrições impedem a entrega sob as condições atuais.
Simulação para Cavalo: Tempo estimado = X.XXh, Custo = R$Y.YY
```

Os valores `X.XXh` e `R$Y.YY` serão os resultados dos cálculos de tempo e custo para cada simulação bem-sucedida. As mensagens de 


restrição aparecerão quando as condições climáticas não forem adequadas para o meio de entrega selecionado.

Este projeto serve como um excelente exemplo de como o polimorfismo pode ser aplicado em Rust para criar um código flexível, extensível e de fácil manutenção.


## 👥 Equipe

- André Felipe
- Bernardo Ruas
- Heuller Ramos
- Gustavo dos Santos
