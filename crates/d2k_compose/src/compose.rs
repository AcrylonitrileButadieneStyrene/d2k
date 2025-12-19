use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn compose(path: &std::path::Path) -> d2k_common::Expr {
    let mut expr = d2k_common::Expr::Atom(d2k_common::Atom::symbol("map")).to_list();

    let manifest_str = std::fs::read_to_string(path.join("D2K.toml")).unwrap();
    let manifest = crate::manifest::Manifest::parse(&manifest_str).unwrap();
    if let Some(map) = manifest.map {
        expr = expr
            .append_opt(
                map.width
                    .map(|width| prop("width", d2k_common::Atom::U32(width).into())),
            )
            .append_opt(
                map.height
                    .map(|height| prop("height", d2k_common::Atom::U32(height).into())),
            )
            .append_opt(
                map.chipset
                    .map(|chipset| prop("chipset", d2k_common::Atom::U32(chipset).into())),
            )
            .append_opt(
                map.extends
                    .map(|extends| prop("extends", d2k_common::Atom::Text(extends.clone()).into())),
            )
    }

    let mut required = std::collections::HashSet::new();
    let events_str = std::fs::read_to_string(path.join("Events.ron")).unwrap();
    let events = crate::events::Event::parse(&events_str).unwrap();
    for event in events {
        expr = expr.append(
            d2k_common::Expr::Atom(d2k_common::Atom::symbol("event"))
                .to_list()
                .append_opt(
                    event
                        .name
                        .map(|name| prop("name", d2k_common::Atom::Text(name.clone()).into())),
                )
                .append(prop("x", pos_to_expr(&event.x)))
                .append(prop("y", pos_to_expr(&event.y)))
                .append({
                    let mut pages =
                        d2k_common::Expr::Atom(d2k_common::Atom::symbol("pages")).to_list();
                    for page in event.pages {
                        required.insert(page.file.clone());
                        pages = pages.append(
                            d2k_common::Expr::List(Vec::new())
                                .append(prop("command", d2k_common::Atom::label(page.file).into()))
                                .append(prop(
                                    "graphic",
                                    d2k_common::Expr::List(Vec::new()).append(prop(
                                        "direction",
                                        d2k_common::Atom::symbol(
                                            page.graphic.direction.to_string(),
                                        )
                                        .into(),
                                    )),
                                )),
                        );
                    }
                    pages
                }),
        )
    }

    let Some(commands) = gather_commands(path, required) else {
        std::process::exit(1);
    };
    for (label, code) in commands.into_iter() {
        expr = expr.append(
            d2k_common::Expr::Atom(d2k_common::Atom::symbol("command"))
                .to_list()
                .append(d2k_common::Atom::label(label).into())
                .append(code),
        );
    }

    expr
}

fn prop(key: &str, value: d2k_common::Expr) -> d2k_common::Expr {
    d2k_common::Expr::List(vec![d2k_common::Atom::symbol(key).into(), value])
}

fn pos_to_expr(pos: &crate::events::Position) -> d2k_common::Expr {
    match pos {
        crate::events::Position::Constant(x) => prop("const", d2k_common::Atom::U32(*x).into()),
        crate::events::Position::Range(x, y) => d2k_common::Expr::List(vec![
            d2k_common::Atom::symbol("range").into(),
            d2k_common::Atom::U32(*x).into(),
            d2k_common::Atom::U32(*y).into(),
        ]),
    }
}

fn gather_commands(
    base: &std::path::Path,
    required: HashSet<String>,
) -> Option<HashMap<String, d2k_common::Expr>> {
    let commands = base.join("Commands");
    let (exprs, errors) = required
        .iter()
        .map(|key| {
            let name = format!("{key}.r2ks");
            let src = std::fs::read_to_string(commands.join(&name)).unwrap();
            let tokens = d2k_lexer::R2KSToken::from_file(&name, &src);
            d2k_transpiler::parse(tokens)
                .map(|expr| (key.clone(), expr))
                .map_err(|err| (err, codespan_reporting::files::SimpleFile::new(name, src)))
        })
        .partition_result::<Vec<_>, Vec<_>, _, _>();

    if errors.is_empty() {
        Some(exprs.into_iter().collect())
    } else {
        for (diagnostic, file) in errors {
            d2k_common::emit(&file, &diagnostic).unwrap();
        }

        None
    }
}
