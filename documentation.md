# Documentation du code Rust

## First project

### Code

```rust
1. fn main() {
2.    let numb_u: u32 = get_number();
3.    let numb_d: u32 = get_number();
4.    println!("Le résultat est {}", numb_u + numb_d);
5. }
6.
7. fn get_number() -> u32 {
8.    println!("Veuillez saisir un nombre: ");
9.    let mut choice = String::new();
10.   io::stdin().read_line(&mut choice).expect("Erreur");
11.   let number: u32 = choice.trim().parse().expect("Erreur");
12.   number
13. }
```
Ligne 9: <br>
Définition du mutateur (variable de saisie)

Ligne 10: <br>
Lecture de la ligne et inscription de la valeur dans la variable voulu (le `expect` est important !)

Ligne 11: <br>
Mise en forme de la variable

Ligne 12: <br>
Renvoie de la valeur finale