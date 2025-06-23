use super::meio_de_entrega::MeioDeEntrega;

pub struct Moto {
    consumo_km_litro: f64,
    velocidade_media_kmh: f64,
}

impl Moto {
    pub fn new(consumo_km_litro: f64, velocidade_media_kmh: f64) -> Self {
        Moto { consumo_km_litro, velocidade_media_kmh }
    }
}

impl MeioDeEntrega for Moto {
    fn calcular_tempo_estimado(&self, distancia_km: f64) -> f64 {
        distancia_km / self.velocidade_media_kmh
    }

    fn calcular_custo_combustivel(&self, distancia_km: f64) -> f64 {
        (distancia_km / self.consumo_km_litro) * 5.00 // Preço do combustível: R$ 5.00/litro
    }

    fn verificar_restricoes(&self, condicoes_climaticas: &str) -> bool {
        !condicoes_climaticas.contains("alagamento")
    }

    fn nome(&self) -> &str {
        "Moto"
    }
}


