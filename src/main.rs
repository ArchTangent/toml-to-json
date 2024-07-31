//! TOML to JSON conversion binary using Rust.

use tomltojson::cmd;

fn main() {
    println!("--- TOML to JSON ---");

    // let matches = cli();
    // for m in matches.ids() {
    //     println!("{m}");
    // }

    // let matches = cmd().get_matches_from(["tomltojson" , "foo.toml", "foo.json"]);
    // for m in matches.ids() {
    //     println!("{m}");
    // }
    
    // let vals = matches.get_raw("source");
    // for v in vals.iter() {
    //         println!("{v:?}");
    // }

    let matches = cmd().get_matches();
    for m in matches.ids() {
        println!("{m}");
    }
}
