use std::io;

// Trait definerer tre metoder som typer av trait må implementere
trait  Dyr {
    fn lager_hvilken_lyd(&self) -> String;
    fn spiser(&self) -> String;
    fn er_farlig(&self) -> bool;
}

// Definerer struct
struct Løve;

// Implementerer trait for struct
impl Dyr for Løve {
    fn lager_hvilken_lyd(&self) -> String {
        "Brøl".to_string()
    }
    fn spiser(&self) -> String {
        "Kjøtt".to_string()
    }
    fn er_farlig(&self) -> bool {
        true
    }
}

struct Hund;

impl Dyr for Hund {
    fn lager_hvilken_lyd(&self) -> String {
        "Voff".to_string()
    }
    fn spiser(&self) -> String {
        "Kjøtt".to_string()
    }
    fn er_farlig(&self) -> bool {
        false
    }
}

// Funksjon som tar inn en string og printer ut informasjon om dyret
fn hent_dyr_info(input: &str) {
    match input.to_lowercase().as_str() {
        "løve" => {
            let løve = Løve;
            println!("Lyd: {}", løve.lager_hvilken_lyd());
            println!("Spiser: {}", løve.spiser());
            println!("Farlig: {}", løve.er_farlig());
        }
        "hund" => {
            let hund = Hund;
            println!("Lyd: {}", hund.lager_hvilken_lyd());
            println!("Spiser: {}", hund.spiser());
            println!("Farlig: {}", hund.er_farlig());
        }
        _ => println!("Dette dyret er ikke i systemet"),
    }
}

// Main funksjonen som kjører programmet og håndterer input
fn main() {
    println!("Skriv inn navnet på et dyr (løve eller hund):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Kunne ikke lese linje");
    let input = input.trim();

    hent_dyr_info(input);
}