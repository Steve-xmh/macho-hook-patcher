use apple_bundles::DirectoryBundle;
use std::{error::Error, path::Path};

pub fn find_dir_bundle(mut path: &Path) -> Result<(&Path, DirectoryBundle), Box<dyn Error>> {
    if !path.exists() {
        return Err("file not exists".into());
    }
    loop {
        if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with(".app")
        {
            if let Ok(bundle) = DirectoryBundle::new_from_path(path) {
                return Ok((path, bundle));
            }
        }
        path = match path.parent() {
            Some(p) => p,
            None => return Err("No bundle found".into()),
        };
    }
}
