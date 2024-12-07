use chrono::{NaiveDate, Local, Duration};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: score [punti] [data/opzione]");
        return;
    }

    let today = Local::now().date_naive();
    let yesterday = today.pred_opt().expect("Errore nel calcolo di ieri");

    // Separiamo i punteggi dai possibili argomenti di data
    let mut points_sum = 0;
    let mut date_args: Vec<String> = Vec::new(); // Cambiato da Vec<&String> a Vec<String>
    
    for arg in &args[1..] {
        if is_valid_points(arg) {
            points_sum += parse_points(arg);
        } else {
            date_args.push(arg.to_string()); // Usa to_string per aggiungere una Stringa
        }
    }

    // Se non ci sono date, impostiamo la data di oggi come predefinita
    if date_args.is_empty() {
        date_args.push("t".to_string()); // "t" è la sigla per oggi
    }

    // Ciclo attraverso le date e applico la somma dei punti se è presente
    if points_sum != 0 {
        for date in date_args {
            match date.as_str() {
                "t" | "today" => {
                    println!(
                        "{} {} punti alla data di oggi ({})",
                        if points_sum > 0 { "Aggiungo" } else { "Rimuovo" },
                        points_sum.abs(),
                        today
                    );
                }
                "y" | "yesterday" => {
                    println!(
                        "{} {} punti alla data di ieri ({})",
                        if points_sum > 0 { "Aggiungo" } else { "Rimuovo" },
                        points_sum.abs(),
                        yesterday
                    );
                }
                "w" | "week" => {
                    let week_dates = get_week_dates(&today);
                    for date in week_dates {
                        println!(
                            "{} {} punti alla data ({})",
                            if points_sum > 0 { "Aggiungo" } else { "Rimuovo" },
                            points_sum.abs(),
                            date
                        );
                    }
                }
                _ => {
                    if let Ok(parsed_date) = NaiveDate::parse_from_str(&date, "%d-%m-%Y") {
                        println!(
                            "{} {} punti alla data specificata ({})",
                            if points_sum > 0 { "Aggiungo" } else { "Rimuovo" },
                            points_sum.abs(),
                            parsed_date
                        );
                    } else {
                        eprintln!("Data non valida: {}", date);
                    }
                }
            }
        }
    } else {
        // Se non ci sono punteggi, mostriamo solo i punteggi per le date
        for date in date_args {
            match date.as_str() {
                "t" | "today" => {
                    println!("Mostro il punteggio di oggi ({})", today);
                }
                "y" | "yesterday" => {
                    println!("Mostro il punteggio di ieri ({})", yesterday);
                }
                "w" | "week" => {
                    let week_dates = get_week_dates(&today);
                    for date in week_dates {
                        println!("Mostro il punteggio della data ({})", date);
                    }
                }
                _ => {
                    if let Ok(parsed_date) = NaiveDate::parse_from_str(&date, "%d-%m-%Y") {
                        println!("Mostro il punteggio della data specificata ({})", parsed_date);
                    } else {
                        eprintln!("Data non valida: {}", date);
                    }
                }
            }
        }
    }
}

// Funzione per determinare se un argomento è un punteggio valido
fn is_valid_points(points: &str) -> bool {
    points.starts_with('+') || points.starts_with('-') || points.parse::<i32>().is_ok()
}

// Funzione per parsare un punteggio da una stringa
fn parse_points(points: &str) -> i32 {
    points.parse::<i32>().unwrap_or(0)
}

// Funzione per ottenere le date della settimana (oggi e 6 giorni precedenti)
fn get_week_dates(today: &NaiveDate) -> Vec<NaiveDate> {
    let mut week_dates = Vec::new();
    for i in (0..=6).rev() {
        week_dates.push(*today - Duration::days(i));
    }
    week_dates
}
