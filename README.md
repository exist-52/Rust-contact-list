# Carnet de contacts en fichier texte
---
Ajouter/lister/supprimer des contacts, sauvegardés dans un fichier .txt (une ligne = un contact, format nom,tel). Pas de JSON pour l'instant, juste du texte brut.
Ça t'apprend : lire/écrire un fichier ligne par ligne, parser une string, structurer les données en mémoire (Vec<Contact>).
comment lancer ? 
  cargo run nom, numero, nom du fichier.

## Algorithme :
  on collecte les donnees entree,
    on retourne un message d'erreur en cas de mauvais parametres.
  on ouvre le fichier donne a la fin.
  on fait une boucle pour chaque ligne du fichier.
  a chaque iteration, on ecris le nom : le numero
  a la fin, on returne un message de succes.
---
# note: 
an error occured when i did -a first. im gonna change it and put it as last argument.
will have to change ``` fn collect(args: &Vec<String>) -> Contact {
    if args.len() < 4 {
        panic!("Usage: cargo run -a name number file_path");
    }

    Contact::build(args[0].to_string(), args[1].to_string(), args[2].to_string(), args[3].to_string())
}

fn write_contact(mut file: File, name: String, number: String) {
    let person = format!("{name} : {number}");
    file.write(person.as_bytes());
} ```
