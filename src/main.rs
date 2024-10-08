use reqwest;
use serde_json::Value;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://sanjuan.gob.ar/ol/?or=2B331CC34D344C31875DED5E05060FAA&Prefijo=701&Numero=73&Anio=2024&Tipo=EXP&Movimientos=1";
    let mut last_office: Option<String> = None;

    loop {
        let response = reqwest::get(url).await?;
        let json: Value = response.json().await?;

        if let Some(array) = json.as_array() {
            if let Some(first_item) = array.first() {
                if let Some(office) = first_item["OFICINA"].as_str() {
                    match &last_office {
                        Some(last) if last != office => {
                            println!("¡ALERTA! La oficina ha cambiado:");
                            println!("  Anterior: {}", last);
                            println!("  Actual:   {}", office);
                            last_office = Some(office.to_string());
                        }
                        None => {
                            println!("Primera lectura. Oficina actual: {}", office);
                            last_office = Some(office.to_string());
                        }
                        _ => {
                            println!("No hay cambios. Oficina actual: {}", office);
                        }
                    }
                }
            }
        } else {
            // println!("La respuesta no es un array como se esperaba.");
            // println!("Estructura de la respuesta:");
            println!("{:#?}", json);
        }

        // Espera 1 minuto antes de la próxima petición
        thread::sleep(Duration::from_secs(60));
    }
}