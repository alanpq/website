use std::{sync::mpsc::channel, time::Duration, collections::hash_map::DefaultHasher, hash::Hasher, path::{self, Path}, fs};

use notify::{watcher, Watcher, RecursiveMode};
use sass_rs::{compile_file, Options};
use log::{info, warn, error};

pub async fn watch_css<P: AsRef<Path>>(path: P){
  let path = path.as_ref().to_owned();

  let (tx, rx) = channel();
  let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
  watcher.watch(&path, RecursiveMode::Recursive).unwrap();

  path.read_dir().expect("No scss directory found").for_each(|file| {
    if let Ok(file) = file {
      compile_sass(&file.path());
    }
  });

  loop {
    match rx.recv() {
      Ok(event) => match event {
        notify::DebouncedEvent::Write(path) => {
          compile_sass(path.as_path());
        },
        _ => {}
    },
      Err(e) => error!("watch error: {:?}", e),
    }
  }
}

fn hash_css(css: &str) -> String {
  let mut hasher = DefaultHasher::new();
  hasher.write(css.as_bytes());
  hasher.finish().to_string()
}

// TODO: remove all these panics/unwraps
fn compile_sass(path: &Path) -> Option<String> {
  let file_name = path.file_name()?.to_str()?;
  info!("Compiling '{}'...", path.display());

  if !path.exists() {
      let fpath = std::env::current_dir().unwrap();
      info!("The current directory is {}", fpath.display());
      panic!("file not found: {}", path.display());
  }

  let css = compile_file(path, Options::default())
      .unwrap_or_else(|_| panic!("couldn't compile sass: {}", path.display()));

  // let css_sha = format!("{}_{}", filename, hash_css(&css));
  let css_file = format!("./static/styles/{}.css", file_name.strip_suffix(".scss")?);

  fs::write(&css_file, css.into_bytes())
      .unwrap_or_else(|_| panic!("couldn't write css file: {}", &css_file));

  Some(String::from(&css_file[1..]))
}
