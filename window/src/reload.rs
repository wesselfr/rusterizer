extern crate libloading;
use std::{fs, time::SystemTime};

use libloading::Library;
use shared::State;

const LIB_PATH: &str = "../rusterizer/target/release/rusterizer.dll";
const LIB_PATH_ACTIVE: &str = "active/rusterizer.dll";

pub struct Application(pub Library);
impl Application {
    pub fn get_message(&self) -> &'static str {
        unsafe {
            let f = self
                .0
                .get::<fn() -> &'static str>(b"get_message\0")
                .unwrap();
            f()
        }
    }
    pub fn setup(&self, test: &State) {
        unsafe {
            let f = self.0.get::<fn(&State)>(b"setup\0").unwrap();
            f(test)
        }
    }
    pub fn update(&self, test: &State) {
        unsafe {
            let f = self.0.get::<fn(&State)>(b"update\0").unwrap();
            f(test)
        }
    }
}

pub fn load_lib() -> Application {
    fs::copy(LIB_PATH, LIB_PATH_ACTIVE).unwrap();
    let app: Application;
    unsafe {
        app =
            Application(Library::new(LIB_PATH_ACTIVE).unwrap_or_else(|error| panic!("{}", error)));
    }
    app
}

pub fn reload(mut app: Application) -> Application {
    drop(app);
    app = load_lib();
    app
}

pub fn should_reload(last_modified: SystemTime) -> bool {
    let metadata = std::fs::metadata(LIB_PATH);

    if let Ok(metadata) = metadata {
        let modified = metadata.modified().unwrap();

        if modified > last_modified {
            return true;
        }
        return false;
    }
    false
}
