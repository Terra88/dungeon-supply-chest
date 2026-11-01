kurssitehtävä, jossa harjoitellaan virheidenhallinta mekanismeja.

-Lukee items tiedostosta "chest.txt" (yksi tavara per rivi) - Toimii cargo run komennolla, jos chest.txt on juuressa. <br>
-jos tiedostoa ei ole, fallback takaisin default tavaroihin, jotka sijaitsee main.rs <br>
-Tiedoston lopussa trigger_trap(); - joka demonstroi ("panic!") virheen. - Ajetaan RUST_BACKTRACE=1 cargo run <br>
-Tiedoston lopussa let _item = &items[999]; - joka demonstroi "out of range" -indeksointipaniikin <br>

Internetin ja tekoälyn hyödyntäminen ratkaisujen teossa.
