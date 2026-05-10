use std::{env, fmt::Write, fs, path::Path};

use syn::{Expr, ExprArray, ExprLit, ExprTuple, Item, Lit};

// this file autogenerates `src/gdobj/ids.rs`, which is a file with
// all of the currently implemented ids for objects and properties as consts.
// currently broken due to `kAXX` properties

fn to_const_name(s: &str) -> String {
    let mut seen_underscore = false;
    s.chars()
        .map(|c| {
            if c == ' ' {
                '_'
            } else {
                c.to_ascii_uppercase()
            }
        })
        .filter(|c| !matches!(c, '/' | '\'' | '+' | '?' | '-' | '"' | '.'))
        .filter(|c| {
            let keep = !(*c == '_' && seen_underscore);
            seen_underscore = *c == '_';
            keep
        })
        .collect()
}

fn handle_tuple(buffer: &mut String, tuple: ExprTuple) {
    let mut id = 0i32;
    let mut name = String::new();
    for item in tuple.elems {
        if let Expr::Lit(ExprLit { lit, .. }) = item {
            match lit {
                Lit::Int(int) => id = int.base10_parse().unwrap(),
                Lit::Str(s) => name = s.value(),
                _ => {}
            }
        }
    }
    let const_name = to_const_name(&name);
    writeln!(buffer, "    pub const {const_name}: i32 = {id};").unwrap();
}

// fn _warn<T: Into<String>>(s: T) {
//     println!("cargo:warning={}", s.into());
// }

fn main() {
    let mut properties_out_str = String::new();
    let mut objects_out_str = String::new();
    let mut group_property_ids = Vec::new();
    let other_file = fs::read_to_string("src/gdobj/lookup.rs").unwrap();
    let file = fs::read_to_string("src/gdobj/mod.rs").unwrap();

    let ast: syn::File = syn::parse_str(&file).unwrap();
    for item in ast.items {
        if let Item::Const(c) = item
            && let Expr::Reference(expr_ref) = *c.expr
            && let Expr::Array(ExprArray { elems, .. }) = *expr_ref.expr
            && c.ident == "OBJECT_NAMES"
        {
            objects_out_str = String::with_capacity(elems.len() * 48);
            for elem in elems {
                if let Expr::Tuple(tuple) = elem {
                    handle_tuple(&mut objects_out_str, tuple);
                }
            }
        }

        // warn(format!("found const {}", c.ident.to_string().as_str()));
    }

    let mut seen_map = false;
    for line in other_file.split('\n') {
        if seen_map {
            if line.starts_with("};") {
                break;
            }

            let mut split = line.trim().split(" => (");
            let id = split.next().unwrap();
            let mut tuple_split = split.next().unwrap().split(", ");
            let desc = tuple_split.next().unwrap();
            let const_name = to_const_name(desc);
            let prop_type = tuple_split.next().unwrap_or_default();

            writeln!(
                properties_out_str,
                "    pub const {const_name}: u16 = {id};"
            )
            .unwrap();

        if prop_type.contains("GDObjPropType::Group") {
                group_property_ids.push(id.to_string());
            }
        } else if line.starts_with(
            "pub static PROPERTY_TABLE: Map<u16, (&'static str, GDObjPropType)> = phf_map!",
        ) {
            seen_map = true;
        }
    }

    let gids_len = group_property_ids.len();
    let group_ids_literal = group_property_ids.join(", ");

    let out_str = format!(
        "\
/// Object ids submodule
pub mod objects {{
{objects_out_str}}}

/// Property ids submodule
pub mod properties {{
{properties_out_str}}}

/// Property metadata submodule
pub mod metadata {{
    pub static GROUP_PROPERTY_IDS: &[u16; {gids_len}] = &[{group_ids_literal}];
}}
"
    );

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = "ids.rs";
    fs::write(Path::new(&out_dir).join(out_path), out_str).unwrap();
}
