kurssitehtävä, jossa harjoitellaan virheidenhallinta mekanismeja.

-Lukee items tiedostosta "chest.txt" (yksi tavara per rivi) - Toimii cargo run komennolla, jos chest.txt on juuressa.
-jos tiedostoa ei ole, fallback takaisin default tavaroihin, jotka sijaitsee main.rs
-Tiedoston lopussa trigger_trap(); - joka demonstroi ("panic!") virheen. - Ajetaan RUST_BACKTRACE=1 cargo run
-Tiedoston lopussa let _item = &items[999]; - joka demonstroi "out of range" -indeksointipaniikin
