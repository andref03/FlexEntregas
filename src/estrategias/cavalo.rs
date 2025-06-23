
use super::meio_de_entrega::MeioDeEntrega;

pub struct Cavalo {
    velocidade_media_kmh: f64,
}

impl Cavalo {
    pub fn new(velocidade_media_kmh: f64) -> Self {
        Cavalo { velocidade_media_kmh }
    }
}

impl MeioDeEntrega for Cavalo {
    fn calcular_tempo_estimado(&self, distancia_km: f64) -> f64 {
        distancia_km / self.velocidade_media_kmh
    }

    fn calcular_custo_combustivel(&self, _distancia_km: f64) -> f64 {
        0.0 // Cavalo não tem custo de combustível
    }

    fn verificar_restricoes(&self, condicoes_climaticas: &str) -> bool {
        !condicoes_climaticas.contains("terreno acidentado")
    }

    fn nome(&self) -> &str {
        "Cavalo"
    }
}


