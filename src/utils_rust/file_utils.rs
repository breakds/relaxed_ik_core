use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use path_slash::PathBufExt;
use yaml_rust::YamlLoader;

pub fn get_path_to_src() -> String {
    let path = env::current_dir().unwrap();
    let s = path.to_slash().unwrap();
    let s1 = String::from(s);
    let path_to_src = s1 + "/";
    path_to_src
}

pub fn find_path_to_urdf(path_to_setting: &str) -> String {
    // Step 1. Read the settings file for the name of the urdf file.
    let mut file = File::open(path_to_setting).unwrap();
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents).unwrap();
    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
    let settings = &docs[0];
    let urdf_name = settings["urdf"].as_str().unwrap();

    // Step 2. Try finding the urdf file under the same directory as the
    // settings file.
    let settings_dir = Path::new(path_to_setting).parent().unwrap();
    let local_urdf_path = settings_dir.join(urdf_name);
    if local_urdf_path.exists() {
        return local_urdf_path.to_string_lossy().into_owned();
    }

    // Step 3. Otherwise, fallback to find it based on the src path.
    let path_to_src = get_path_to_src();
    path_to_src + "configs/urdfs/" + urdf_name
}
