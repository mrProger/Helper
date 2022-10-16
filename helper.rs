use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn create_controller(name: String) {
    let file_name: String = format!("{}.php", &name);
    let path: PathBuf = ["./controllers/", &file_name].iter().collect();
    let error_message = vec!["Не удалось создать ", &file_name].concat();
    if !Path::new("./controllers/").exists() {
        fs::create_dir("./controllers/").expect("Не удалось создать директорию controllers");
    }
    let mut out = File::create(path).expect(&error_message);
    write!(out, "<?php\n").expect(&error_message);
    write!(out, "\nclass {} {{", name).expect(&error_message);
    write!(out, "\n\t").expect(&error_message);
    write!(out, "\n}}").expect(&error_message);
}

fn create_model(name: String) {
    let file_name: String = format!("{}.php", &name);
    let path: PathBuf = ["./models/", &file_name].iter().collect();
    let error_message = vec!["Не удалось создать ", &file_name].concat();
    if !Path::new("./models/").exists() {
        fs::create_dir("./models/").expect("Не удалось создать директорию models");
    }
    let mut out = File::create(path).expect(&error_message);
    write!(out, "<?php\n").expect(&error_message);
    write!(out, "\nclass {} extends Model {{", name).expect(&error_message);
    write!(out, "\n\t").expect(&error_message);
    write!(out, "\n}}").expect(&error_message);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        if args[1] == "create-controller" && args.len() >= 3 {
            create_controller(args[2].to_string());
        } else if args[1] == "create-model" && args.len() >= 3 {
            create_model(args[2].to_string());
        } else {
            println!("Неизвестная команда, либо недостаточно аргументов");
        }
    } else {
        println!("Ошибка: Недостаточно аргументов");
        println!("Использование: helper.exe <команда> <аргументы>");
        println!("Пример: helper.exe create-controller TestController");
    }
}