use super::meio_de_entrega::MeioDeEntrega;

pub struct Drone {
    velocidade_media_kmh: f64,
}

impl Drone {
    pub fn new(velocidade_media_kmh: f64) -> Self {
        Drone { velocidade_media_kmh }
    }
}

impl MeioDeEntrega for Drone {
    fn calcular_tempo_estimado(&self, distancia_km: f64) -> f64 {
        distancia_km / self.velocidade_media_kmh
    }

    fn calcular_custo_combustivel(&self, _distancia_km: f64) -> f64 {
        0.0 // Drone usa eletricidade, custo considerado insignificante para esta simulação
    }

    fn verificar_restricoes(&self, condicoes_climaticas: &str) -> bool {
        !condicoes_climaticas.contains("chuva") && !condicoes_climaticas.contains("vento forte")
    }

    fn nome(&self) -> &str {
        "Drone"
    }
}


