/*
 * This project is going to help create Deno projects
 * By using a project.yml file, you will be able to rapily scaffold projects.
 * If none is selected, a default will be chosen ("hello-world.yml")
 *
 * @author Ian Fabricatore
 * @year 2019
 * */
use std::fs::{File, create_dir};
use std::io::prelude::*;

#[macro_use]
extern crate clap;
use clap::App;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

fn main() {
    let default_scaffold = 
"
hello_world:
    index.js: \"console.log('hello world');\"
    examples:
        egg.txt: \"dsfsdfsdfsdfsdfsdfsdfsdfsdfsdda\"
";
    let docs = YamlLoader::load_from_str(default_scaffold).unwrap();
    let doc = &docs[0];

    create(doc, doc, String::from("./"));

    //Command Line shit 
    //let cli_config = load_yaml!("cli.yml");
    //let matches = App::from_yaml(cli_config).get_matches();
}

trait IsHash {
    fn is_hash(&self) -> bool;
}
trait IsString {
    fn is_string(&self) -> bool;
}

impl IsHash for Yaml {
    fn is_hash(&self) -> bool {
        match *self {
            Yaml::Hash(_) => true,
            _ => false,
        }
    }
}
impl IsString for Yaml {
    fn is_string(&self) -> bool {
        match *self {
            Yaml::String(_) => true,
            _ => false,
        }
    }
}

fn create(yaml: &Yaml, current_node: &Yaml, dir: String) {
    let doc = yaml.clone();
    let node = current_node.clone();

    if let Some(hash) = node.into_hash() {
        for (key, val) in hash.iter() {
            let name = key.as_str().unwrap();
            // I feel like there's a logic issue in this function somewhere...
            let path = format!("{}{}", dir, name);
            if val.is_hash() {
                println!("folder: {}", path);
                create_dir(&path); 
                create(&doc, &val, format!("{}/", &path));
            }else {
                println!("file: {}", &path);
                if let Ok(mut file) = File::create(&path) {
                    if let Some(string) = val.clone().into_string() {
                        file.write_all(string.as_bytes()); 
                    }
                };
            }
        }
    }
}
