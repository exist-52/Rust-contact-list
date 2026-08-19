# Carnet de contacts en fichier texte
##go down for english.
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

# the cli tool is now fully functional, ignore the french up ahead.
this tool lets u add or fetch a contact in a file that u give as argument. the contact name is not case sensitive but the filename is. 
to use it you open your terminal and do : contact -a name number/or email file-path, to add a number. itll create the file if it doesnt exist.
                                          contact -f name file-path, to fetch a number using the name.
                                          exemple: ```contact -a bob 8382838289 Contacts.txt ```
                                          or: ```contact -f bob Contacts.txt```
to install it, u just clode this repository with git clotne and go in the repository directory, then u do 
```cargo install -path .```
you have to have rust and cargo installed first.
ill see if i can post it on crate.io or idk what im not new to this.
i dont know how this licence thing works to lol but feel free to use modify and share this code just tag me pls im looking for opportunities.
id be happy to get feedbacks on discord: exist_52 




screw AI.
