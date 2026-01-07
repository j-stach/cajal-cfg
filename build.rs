
use std::{ env, fs, path::Path };

fn main() {
    select_config()
}

fn select_config() {
    
    let home = env::var("HOME").expect("Find user's home directory");
    let home = Path::new(&home);

    // TODO ANIMUS CONFIG PATH LOCATION RELATIVE TO BUILD
    // If `cajal` is building to support `animusd`, then the user may want a dedicated `neuron.cfg` file.
    // In that case, `cajal-cfg` will expect to find it ... Where? 
    let local = Path::new("./local.cfg");

    // Global preferences for neuron parameters can be kept in the framework directory.
    let global = home.join(".cajal/neuron.cfg");

    // Default configuration that is packaged with `cajal`.
    let default = Path::new("neuron.cfg");

    let config_path: &Path = 
        // Use the animus-specific config file first:
        if local.exists() { local }
        // Use the user's custom global config second:
        else if global.exists() { global.as_path() } 
        // Use the crate's default configuration if all else fails:
        else { default };

    let config_path = config_path.to_path_buf();
    let config_const = format!("pub const CONFIG_PATH: &str = \"{}\";", config_path.display());

    let out_dir = env::var("OUT_DIR").unwrap();
    let temp_file = Path::new(&out_dir).join("config_path.rs");

    fs::write(temp_file, config_const)
        .expect("Write config path to temp file.");
}

