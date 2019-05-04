/*
 * This project is going to help create Deno projects
 * By using a project.yml file, you will be able to rapily scaffold projects.
 * If none is selected, a default will be chosen ("hello-world.yml")
 *
 * @author Ian Fabricatore
 * @year 2019
 * */
use std::fs::{File, create_dir, self};
use std::path::Path;
use std::io::prelude::*;

#[macro_use]
extern crate clap;
use clap::App;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

extern crate git2;
use git2::Repository;

fn main() {
    // let default_scaffold = 
// "
// hello_world:
    // index.js: \"console.log('hello world');\"
    // examples:
    //     egg.txt: \"dsfsdfsdfsdfsdfsdfsdfsdfsdfsdda\"
// ";


    // Command Line shit 
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();

    // println!("{:#?}", matches);
    let file_path = matches.value_of("SCAFFOLD").unwrap();
    let project = matches.value_of("PROJECT").unwrap();
    let mut file = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File {} not found", file_path); 
                },
                other_error => panic!("An unusual error occured: {}", error),
            }
            std::process::exit(0);
        }
    };

    let docs = YamlLoader::load_from_str(&file).unwrap();
    let doc = &docs[0];

    let project_path = format!("{}/", project);

    create(doc, doc, project_path.clone());

    if matches.is_present("git-init") {
        println!("Creating repo");
        let repo = match Repository::init(&project_path) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to initalize repo: {}", e),
        };
    }
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
    if !Path::new(&dir[..]).exists() {
        create_dir(&dir); 
    } 
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
