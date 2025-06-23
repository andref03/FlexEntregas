pub trait MeioDeEntrega {
    fn calcular_tempo_estimado(&self, distancia_km: f64) -> f64;
    fn calcular_custo_combustivel(&self, distancia_km: f64) -> f64;
    fn verificar_restricoes(&self, condicoes_climaticas: &str) -> bool;
    fn nome(&self) -> &str;
}

