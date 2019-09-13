// TODO: rewrite this in tokio
use super::super::utils::console;
use fs_extra::dir::CopyOptions;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use yansi::Paint;

// TODO: probably run one of script to turn a directory into JSON "file_name": "content"
pub fn run() -> std::io::Result<()> {
    let application_name = std::env::args().nth(2).unwrap_or_else(|| {
        println!("You forgot to include an application name! Example: mber init example-app");

        process::exit(1);
    });

    let path = env::current_dir()?;
    let user_has_the_app_in_path: bool = path
        .iter()
        .any(|folder| folder.to_str().unwrap() == application_name);

    if user_has_the_app_in_path || user_has_app_in_current_directory(path, &application_name) {
        println!("{} already exists!", application_name);

        process::exit(1);
    }

    console::log(format!("creating {} application", application_name));

    // let copy_options = CopyOptions::new(); //Initialize default values for CopyOptions
    //                                        // SEARCH HOW TO INJECT FOLDER WITH BINARY
    //                                        // SEARCH HOW TO RUN CARGO ONE OF SCRIPTS
    // fs_extra::copy_items(&[""], application_name, &copy_options)?;
    // TODO: how to copy from ember-app-boilerplate

    // TODO: copy ember-app-boilerplate, change environment.js, package.json, test.html
    // write .gitignore
    // await fs.copy(`${__dirname}/../../ember-app-boilerplate`, TARGET_DIRECTORY);
    // TODO: also print them

    console::log(Paint::green(format!(
        "{} ember application created. Next is to do:",
        application_name
    )));
    println!("$ cd {} && npm install && mber s", application_name);

    Ok(())
}
// NOTE: in future also needs one line change to main.js

fn user_has_app_in_current_directory(path: PathBuf, application_name: &str) -> bool {
    println!("path is {:?}", path);

    return fs::read_dir(path)
        .unwrap()
        .any(|element| element.unwrap().file_name() == application_name);
}
