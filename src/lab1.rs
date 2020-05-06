// Exercice écrit par N. Matsakis (2016)

fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: &String) -> String {
    // But #1: Que faire pour que ça compile?
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // on saute les voyelles
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}

fn print_out(name: String) {
    let devowelized_name = remove_vowels(&name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // But #2: Que se passe-t-il vous décommentez le `println` suivant?
    // Pouvez-vous changer le code qui précède pour que le code suivant compile?
    //
    println!("Removing vowels from {:?} yields {:?}",
             name, devowelized_name);

    // Bonus: Pouvez-vous le faire sans copier de données?
    // (En utilisant uniquement le transfert de possession)
}
