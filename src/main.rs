use winrt_gen::*;

fn main() -> std::io::Result<()> {
    let files = std::fs::read_dir("winmds")?;
    let mut pathes = Vec::new();
    for f in files {
        let f = f?;
        let path = f.path();
        pathes.push(WinmdFile::new(path));
    }
    let reader = TypeReader::new(pathes);
    let mut limits = TypeLimits::new(&reader);
    let types = NamespaceTypes {
        namespace: "windows.ui.xaml".into(),
        limit: TypeLimit::All,
    };
    limits.insert(types).unwrap();

    let stage = TypeStage::from_limits(&reader, &limits);
    let tree = stage.into_tree();
    let _: proc_macro2::TokenStream = tree.to_tokens().collect();
    Ok(())
}
