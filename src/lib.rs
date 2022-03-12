use std::fs;
use std::path::Path;

pub fn barrel_write(filepath: &str, ext: Option<String>) {
    let path = Path::new(filepath);

    let file = path.file_stem().unwrap().to_str().unwrap();
    let ext = match ext {
        Some(ref e) => e,
        None => path.extension().unwrap().to_str().unwrap(),
    };

    let content = format!("export {{ default }} from \"./{}\";\n", file);

    let dir = path.parent().unwrap().join(file);

    fs::create_dir(&dir).expect("failed to create directory");
    fs::rename(path, dir.join(path.file_name().unwrap())).expect("failed to move file");

    fs::write(dir.join(Path::new("index").with_extension(ext)), content)
        .expect("failed to write file");
}

#[cfg(test)]
mod tests {
    use crate::barrel_write;
    use std::fs;
    use std::path::Path;

    fn setup(dir: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            Path::new(dir).join("file.js"),
            "export default function file() {}\n",
        )
        .unwrap();
    }

    fn teardown(dir: &str) {
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn write_barrel() {
        setup("tmp/1");

        barrel_write("./tmp/1/file.js", None);

        assert!(
            Path::new("./tmp/1/file/file.js").exists(),
            "expected tmp/1/file/file.js to exist"
        );
        assert!(
            Path::new("./tmp/1/file/index.js").exists(),
            "expected tmp/1/file/index.js to exist"
        );

        let index = fs::read_to_string("./tmp/1/file/index.js").unwrap();

        assert_eq!(&index, "export { default } from \"./file\";\n");

        teardown("tmp/1");
    }

    #[test]
    fn write_barrel_extension() {
        setup("tmp/2");

        barrel_write("./tmp/2/file.js", Some("ts".to_string()));

        assert!(
            Path::new("./tmp/2/file/file.js").exists(),
            "expected tmp/2/file/file.js to exist"
        );
        assert!(
            Path::new("./tmp/2/file/index.ts").exists(),
            "expected tmp/2/file/index.ts to exist"
        );

        let index = fs::read_to_string("./tmp/2/file/index.ts").unwrap();

        assert_eq!(&index, "export { default } from \"./file\";\n");

        teardown("tmp/2");
    }
}
