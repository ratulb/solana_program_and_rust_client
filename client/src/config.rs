//Read a config entry from like json_rpc_url/keypair_path from/commitment
//.config/solana/cli/config.yml

use linked_hash_map::LinkedHashMap;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::signer::keypair::Keypair;
use std::fs;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

//Get values from ~/.config/solana/cli/config.yml
pub fn get_config(key: &str) -> Option<String> {
    match home::home_dir() {
        Some(mut path_buff) => {
            path_buff.push(".config/solana/cli/config.yml");
            let config = fs::read_to_string(path_buff).ok()?;
            let config = YamlLoader::load_from_str(&config).ok()?;
            match &config[..] {
                [Yaml::Hash(map)] => {
                    let map @ LinkedHashMap { .. } = map;
                    match map.get(&Yaml::String(String::from(key))) {
                        Some(Yaml::String(s)) => Some(s.to_string()),
                        Some(_) => None,
                        None => None,
                    }
                }
                _ => None,
            }
        }
        None => {
            eprintln!("Error accessing home directory!");
            None
        }
    }
}

//Get key pair from file path
pub fn get_keypair(file: &str) -> Option<Keypair> {
    match read_keypair_file(file) {
        ok @ Ok(_) => ok.ok(),
        Err(err) => {
            eprintln!("Unable to read keypair from {}: {}", file, err);
            None
        }
    }
}
