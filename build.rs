use std::path::Path;

fn main() {
    {
        let bindings = bindgen::Builder::default()
            .header("gensudoku/sudoku.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .clang_arg("-fvisibility=default")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/sudoku.rs")
            .expect("Couldn't write bindings!");
    }

    {
        let lib = "sudoku";
        let path = ["gensudoku/sudoku.c", "gensudoku/glibcrng.c"];

        if path.iter().all(|p| Path::new(p).exists()) {
            let mut builder = cc::Build::new();
            for p in path.iter() {
                builder.file(p);
            }
            builder.extra_warnings(true).compile(lib);
        }
    }
}
