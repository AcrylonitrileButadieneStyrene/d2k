use lcf::raw::lmu::event::commands::Commands;

mod codepage;

#[derive(clap::Parser)]
struct Args {
    #[arg(short, default_value = ".")]
    input: std::path::PathBuf,
    #[arg(short, long, value_enum, default_value = "shift-jis")]
    codepage: codepage::CodePage,
    #[arg(short, long)]
    out: std::path::PathBuf,
}

#[derive(serde::Deserialize)]
struct Manifest {
    width: Option<u32>,
    height: Option<u32>,
    chipset: Option<u32>,
}

fn main() {
    let mut args = <Args as clap::Parser>::parse();
    if args.input.ends_with("D2K.toml") {
        args.input.pop();
    }
    let encoding = args.codepage.to_encoding();

    let commands = gather_commands(&args.input, encoding);
    let src = std::fs::read_to_string(args.input.join("Events.ron")).unwrap();
    let events = d2k_events::build(&src, encoding, &commands).collect::<Vec<_>>();

    let manifest =
        toml::from_str::<Manifest>(&std::fs::read_to_string(args.input.join("D2K.toml")).unwrap())
            .unwrap();
    let map = lcf::lmu::LcfMapUnit {
        width: manifest.width.unwrap_or(20),
        height: manifest.height.unwrap_or(15),
        chipset: manifest.chipset,
        events,
        ..Default::default()
    };

    let mut buf = std::io::Cursor::new(Vec::new());
    map.write(&mut buf).unwrap();
    std::fs::write(args.out, buf.into_inner()).unwrap();
}

fn gather_commands(
    base: &std::path::Path,
    codepage: &'static encoding_rs::Encoding,
) -> std::collections::HashMap<std::sync::Arc<str>, Commands> {
    let mut commands = std::collections::HashMap::new();
    for entry in std::fs::read_dir(base.join("Commands"))
        .unwrap()
        .filter_map(Result::ok)
    {
        let src = std::fs::read_to_string(entry.path()).unwrap();
        commands.insert(
            std::sync::Arc::<str>::from(entry.path().file_prefix().unwrap().to_str().unwrap()),
            d2k_script::parse(&src, codepage),
        );
    }
    commands
}
