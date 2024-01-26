use std::io;

fn main() {
    let numb_u: u32 = get_number();
    let numb_d: u32 = get_number();
    println!("Le résultat est {}", numb_u + numb_d);
}

fn get_number() -> u32 {
    println!("Veuillez saisir un nombre: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Erreur");
    let number: u32 = choice.trim().parse().expect("Erreur");
    number
}
