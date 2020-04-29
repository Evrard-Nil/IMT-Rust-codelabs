// Exercice écrit par N. Matsakis (2016)

pub fn main() {
    let string = format!("my friend");
    greet(&string.clone());
    greet(&string);
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}

// But #1: Convertir `greet` pour utiliser l'emprunt, et non la possession,
// pour que le programme s'exécute sans clonage (copie profonde).
//
// But #2: Utiliser une sous-tranche (subslice) pour que cela imprime
//  "Hello, friend" au lieu de "Hello, my friend".
