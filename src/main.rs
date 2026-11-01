use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

//Lukee 'chest.txt' rivin ja palauttaa listan tavaroita.
// Ohjaa IO virheet takaisin kutsujalle, käyttäen '?' operaattoria.
fn load_items() -> Result<Vec<String>, io::Error> {
    let file = File::open("chest.txt")?;
    let reader = BufReader::new(file);

    let mut items = Vec::new();
    for line in reader.lines() {
        let item = line?;
        if !item.trim().is_empty() {
            items.push(item);
        }
    }
    Ok(items)
}

// palauttaa turvallisesti tavaroiden listan, indexillä, ilman indexöintiä.
fn inspect_slot(items: &[String], index: usize) -> Option<&String> {
    items.get(index)
}

// Laukaisee anomalian, joka aiheuttaa ohjelman kaatumisen.
fn trigger_trap() {
    panic!("Critical Error: Supply chest collapsed into an impossibly glitched state!");
}

fn main() {
    // Kokeillaan ladata tavaratiedosto ja käsitellään mahdolliset virheet.
    let items = match load_items() {
        Ok(loaded) => {
            println!("Successfully loaded chest contents from file.");
            loaded
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("No chest file found; using the standard supplies.");
            vec![
                "Rope".to_string(),
                "Torch".to_string(),
                "Potion".to_string(),
            ]
        }
        Err(err) => {
            eprintln!("Failed to load chest contents: {}", err);
            return;
        }
    };

println!("\nThe chest contains:");
for item in &items {
    println!("- {}", item);
}
println!();

// Demonstroi turvallista tavaroiden tarkastelua.
let valid_index = 1;
match inspect_slot(&items, valid_index) {
    Some(item) => println!("Slot {} contains: {}", valid_index, item),
    None => println!("There is no item in slot {}.", valid_index),
}

let invalid_index = 99;
match inspect_slot(&items, invalid_index) {
    Some(item) => println!("Slot {} contains: {}", invalid_index, item),
    None => println!("There is no item in slot {}.", invalid_index),
}
//----------------------------------------------------------------
//     Panic & Backtrace Demonstration
//----------------------------------------------------------------   
    // Ota kommentit pois nähdäksesi eksplisiittinen virhe:
    // trigger_trap();
    //
    // Ota kommentit pois nähdäksesi "out of range" indexöinti paniikki:
    // let _item = &items[999];
}