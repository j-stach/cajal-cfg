
use std::{ env, fs, path::Path };

fn main() {
    select_config()
}

fn select_config() {
    
    let home = env::var("HOME").expect("Find home directory");
    let home = Path::new(&home);

    // Cajal root directory should be in the user's home directory.
    let cajal = home.join(".cajal");

    // Animus-specific config file:
    // TODO (This should search the directory where the library is being built.)
    let local = Path::new("./neuron.cfg");

    // User's custom global config:
    let global = cajal.join("neuron.cfg");

    // Default config provided by crate:
    let default = Path::new("neuron.cfg");

    let config_path: &Path = 
        // Use the animus-specific config file first:
        if local.exists() { local }
        // Use the user's custom global config second:
        else if global.exists() { global.as_path() } 
        // Use the crate's default config file if all else fails:
        else { default };

    let config_path = config_path.to_path_buf();

    let config_var = format!("pub const CONFIG_PATH: &str = \"{}\";", config_path.display());

    let out_dir = env::var("OUT_DIR").unwrap();
    let temp_file = Path::new(&out_dir).join("config.rs");

    fs::write(temp_file, config_var)
        .expect("Write const params to temp file.");
}

