use crossterm::{
    cursor,
    execute,
    style::{Color, SetForegroundColor, ResetColor, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use serde_json::Value;
use reqwest::{self, Url};
use std::{
    error::Error,
    io::{self, stdout},
};
use tokio::time::{sleep, Duration};
use serde::Serialize;

// #[derive(Serialize)]
// struct Record {
//     oficina: String,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    clear_screen()?;

    // let mut prefijo: i32;

    // loop {
    //     display_message("Escriba el prefijo: ", Color::Yellow);
    //     prefijo = exp_number();

    //     if prefijo != -1 {
    //         break;
    //     }
    // }


    // let mut numero_exp: i32;

    // loop {
    //     display_message("Escriba el número de expediente: ", Color::Yellow);
    //     numero_exp = exp_number();

    //     if numero_exp != -1 {
    //         break;
    //     }
    // }


    // let mut año: i32;

    // loop {
    //     display_message("Escriba el año: ", Color::Yellow);
    //     año = exp_number();

    //     if año != -1 {
    //         break;
    //     }
    // }

    // let titulo = format!(
    //     "Expediente: {} - {} / {} \n",
    //     prefijo,
    //     numero_exp,
    //     año,
    // );

    let titulo: String = String::from("Expediente 701 - 73 / 2024");

    // let url = format!(
    //     "https://sanjuan.gob.ar/ol/?or=2B331CC34D344C31875DED5E05060FAA&Prefijo={}&Numero={}&Anio={}&Tipo=EXP&Movimientos=1",
    //     prefijo, numero_exp, año
    // );

    let url: String = String::from("https://sanjuan.gob.ar/ol/?or=2B331CC34D344C31875DED5E05060FAA&Prefijo=701&Numero=73&Anio=2024&Tipo=EXP&Movimientos=1");

    clear_screen()?;
    // Loop de llamadas y cuenta regresiva
    loop {
        if let Err(e) = fetch_url(&url,&titulo).await {
            display_message(&format!("Error al obtener o procesar el JSON: {}", e), Color::Red);
        }

        let mut count_back: i8 = 10;

        while count_back != -1 {


            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
                Clear(ClearType::CurrentLine),
            )?;

            display_message(
                &format!("{} ", count_back),
                Color::DarkRed,
            );

            sleep(Duration::from_secs(1)).await;

            count_back = count_back -1;
        }
        // sleep(Duration::from_secs(10)).await;

    }
}

async fn fetch_url(url: &str,exp_titulo: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    if let Some(obj) = json.as_object() {
        match obj.get("res").and_then(Value::as_array).and_then(|arr| arr.first()) {
            Some(first_item) => {
                if let Some(oficina) = first_item.get("OFICINA") {
                    
                    //clear_screen()?;

                    execute!(
                        stdout(),
                        cursor::MoveTo(0, 1),
                    )?;

                    display_message(
                        &format!("{}", exp_titulo),
                        Color::Cyan,
                    );

                    execute!(
                        stdout(),
                        cursor::MoveTo(0, 2),
                    )?;

                    display_message(
                        &format!("{}", oficina.as_str().unwrap_or("Sin oficina")),
                        Color::Cyan,
                    );


                    //println!("{}", oficina.as_str().unwrap_or("Sin oficina").with(Color::Green));

                    // Restablecer el color para futuros textos
                    execute!(stdout(), ResetColor)?;
                }
            }
            None => {
                execute!(
                    stdout(),
                    Clear(ClearType::CurrentLine),
                    cursor::MoveTo(0, 2),
                    SetForegroundColor(Color::Red),
                )?;

                println!("{}", "No se encontraron resultados.".with(Color::Red));

                execute!(stdout(), ResetColor)?;
            },
        }
    }
    Ok(())
}

// Función para mostrar mensajes con colores
fn display_message(message: &str, color: Color) {
    execute!(
        stdout(),
        //Clear(ClearType::CurrentLine),
        //cursor::MoveToNextLine(0),
        SetForegroundColor(color),
        PrintStyledContent(message.with(color))
    ).unwrap();
    execute!(stdout(), ResetColor).unwrap();
}

// fn exp_number() -> i32 {
//     let mut numero_ingresado = String::new();

//     if let Err(_) = io::stdin().read_line(&mut numero_ingresado) {
//         display_message("Fallo al leer la línea", Color::Red);
//         return -1;
//     }

//     let numero: Result<i32, _> = numero_ingresado.trim().parse();

//     match numero {
//         Ok(num) => num,
//         Err(_) => {
//             display_message("Entrada inválida. Por favor, ingrese un número.", Color::Red);
//             -1
//         },
//     }
// }

#[allow(unused_must_use)]
fn clear_screen() -> Result<(), Box<dyn Error>>{
    execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Green),
    );
    Ok(())
}