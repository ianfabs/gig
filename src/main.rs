/*
 * This project is going to help create Deno projects
 * By using a project.yml file, you will be able to rapily scaffold projects.
 * If none is selected, a default will be chosen ("hello-world.yml")
 *
 * @author Ian Fabricatore
 * @year 2019
 * */
use std::fs;

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
    
    //let project = doc["project"].as_str().unwrap();
    //if let Err(message) = fs::create_dir(format!("./{}", project)) {
        // This is cool and all, but implementing this well is kinda a hassle. For now, just panic
        // when something kinda sketchy happens.

        /* match message.kind() {
            std::io::ErrorKind::AlreadyExists => {
                println!("Unable to create project dir: A folder by that name already exists"); 
            },
            other_error => panic!("An unusual error occured: {}", message),
        } */
        //panic!("Oopsie Woopsie! The Fowowing Ewer Okwered uWu: {}", message);
    //};

    /*
    if let Some(hash) = &doc["root"].clone().into_hash() {
        for (key, value) in hash.iter() {
            // For value.as_str(), you should handle the exception where it's another array of
            // files, perhaps by moving all of this into a recursive function.
            // Perhaps use the Yaml.is_array() method? that should do the trick.
            // let (file_name, content) = (key.as_str().unwrap(), value.as_str().unwrap());
            let file_name = key.as_str().unwrap();
            let content = if value.is_array() {
                
            }
            println!("File: {}", file_name);
            println!("Contents: {}", content);
            // If file already exists, then ask if they want to overwrite it!
            if let Err(error) = fs::File::create(file_name) {
               panic!("An error occured creating that file: {}", error) 
            };
            // For now, we'll just fail if another file or folder already exists.
        }
    };*/

    createDir(doc);

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

fn createDir(yaml: &Yaml) {
    let doc = yaml.clone();
    // I'll have to handle this eventually, but right now people can just not
    // though if i did, the array would let you create multiple empty files.

    // if yaml.is_array() {
    //match &doc.into_hash() {
    //    Some(hash) => {
    //        // Loop
    //        for (key, val) in hash.iter() {
    //            let name = key.as_str().unwrap();
    //            //TODO: Here, if the type is Hash, then create dir and call function again
    //            //Else: Create file
    //            if val.is_hash() {
    //                println!("Creating dir {} with contents {:?}", name, val);
    //                createDir(val);
    //            } else {
    //                println!("Creating file {} with contents {:?}", name, val);
    //                createDir(val);
    //            }
    //        }
    //    },
    //    None => {
    //        //Fuckall happens here
    //        /*
    //        match yaml.into_string() {
    //            Some(string) => {
    //                // Add contents to file here
    //            }
    //        }
    //        */
    //    }
    //}
    if doc.is_hash() {
        // Make dir here
        println!("dir");
        if let Some(hash) = doc.into_hash() {
            for (key, val) in hash.iter() {
                let name = key.as_str().unwrap();
                createDir(val);
            }
        }
    }else {
        // Make file here
        println!("file");
    }
}
