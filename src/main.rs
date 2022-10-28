mod renamer;

fn main() {
    let renamer = renamer::Renamer::extract();
    renamer.rename_files();
}
