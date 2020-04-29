// Exercice écrit par N. Matsakis (2016)

pub fn main() {
    let (mut str1, str2) = two_words();
    str1 = join_words(str1, &str2);
    println!("concatenated string is {:?}", str1);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

/// Concatène `suffix` à la fin de `prefix`.
fn join_words(mut prefix: String, suffix: &String) -> String {
    prefix.push(' '); // on sépare les mots avec un espace
    for ch in suffix.chars() {
        prefix.push(ch);
    }
    prefix
}

// Challenge: Convertir `join_words` pour utiliser l'emprunt au lieu de la
// possession.
// La nouvelle fonction doit modifier `prefix` en place, et ne doit pas prendre
// possession de `suffix`.
// (Indice plus bas)

// Question: Maintenant que vous avez converti `join_words`, que se passe-t-il
// si vous appelez `join_words` en utilisant le même string pour `prefix` et
// `suffix`? Pourquoi?
// Value borrowed after move because join_words become owner of the string, we can't give it as an argument.






























// Indice: Vous voulez changer la signature de `join_words` comme suit:
//
// fn strcat(prefix: &mut String, suffix: &String) {
//     ...
// }
// 
// Maintenant, `prefix` est une référence mutable vers un String de l'appelant.
// Une référence mutable est nécessaire pour pouvoir modifier le String.
// 
// `suffix` est une référence partagée. Une référence partagée suffit parce que
// on ne fait que lire dans `suffix`.
// 
// La valeur de retour change aussi: puisque l'on modifier `prefix` en place,
// il n'y a plus besoin de retourner quoi que ce soit.