mod renamer;

fn main() {
    let show_renamer = renamer::Renamer::extract();
    show_renamer.rename_files();
}
