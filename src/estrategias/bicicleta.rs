use super::meio_de_entrega::MeioDeEntrega;

pub struct Bicicleta {
    velocidade_media_kmh: f64,
}

impl Bicicleta {
    pub fn new(velocidade_media_kmh: f64) -> Self {
        Bicicleta { velocidade_media_kmh }
    }
}

impl MeioDeEntrega for Bicicleta {
    fn calcular_tempo_estimado(&self, distancia_km: f64) -> f64 {
        distancia_km / self.velocidade_media_kmh
    }

    fn calcular_custo_combustivel(&self, _distancia_km: f64) -> f64 {
        0.0 // Bicicleta não tem custo de combustível
    }

    fn verificar_restricoes(&self, condicoes_climaticas: &str) -> bool {
        !condicoes_climaticas.contains("chuva") && !condicoes_climaticas.contains("neve")
    }

    fn nome(&self) -> &str {
        "Bicicleta"
    }
}


