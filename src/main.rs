mod estrategias;

use estrategias::meio_de_entrega::MeioDeEntrega;
use estrategias::moto::Moto;
use estrategias::bicicleta::Bicicleta;
use estrategias::drone::Drone;
use estrategias::cavalo::Cavalo;

pub fn simular_entrega(meio: &dyn MeioDeEntrega, distancia_km: f64, condicoes_climaticas: &str) {
    if meio.verificar_restricoes(condicoes_climaticas) {
        let tempo = meio.calcular_tempo_estimado(distancia_km);
        let custo = meio.calcular_custo_combustivel(distancia_km);
        println!("Simulação para {}: Tempo estimado = {:.2}h, Custo = R${:.2}", meio.nome(), tempo, custo);
    } else {
        println!("Simulação para {}: Restrições impedem a entrega sob as condições atuais.", meio.nome());
    }
}

fn main() {
    let moto = Moto::new(20.0, 60.0);
    let bicicleta = Bicicleta::new(15.0);
    let drone = Drone::new(80.0);
    let cavalo = Cavalo::new(10.0);

    let distancia = 100.0;

    println!("\n--- Simulações de Entrega (Distância: {}km) ---", distancia);

    simular_entrega(&moto, distancia, "sol");
    simular_entrega(&bicicleta, distancia, "sol");
    simular_entrega(&drone, distancia, "sol");
    simular_entrega(&cavalo, distancia, "sol");

    println!("\n--- Simulações com Restrições ---");

    simular_entrega(&bicicleta, distancia, "chuva");
    simular_entrega(&drone, distancia, "vento forte");
    simular_entrega(&cavalo, distancia, "terreno acidentado");
    simular_entrega(&moto, distancia, "alagamento");

    print!("\n");
}


