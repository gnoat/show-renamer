use renamer::renamer::Renamer;

fn main() {
    let show_renamer = Renamer::extract();
    show_renamer.rename_files();
}
