use heck::CamelCase;
use muon_rs as muon;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct UnionVariant {
    r#type: String,
}

#[derive(Serialize, Deserialize)]
struct Union {
    name: String,
    doc: Option<String>,
    r#enum: String,
    variant: Vec<UnionVariant>,
}

#[derive(Serialize, Deserialize)]
struct EnumVariant {
    name: String,
    doc: Option<String>,
    value: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Enum {
    name: String,
    doc: Option<String>,
    r#type: Option<String>,
    variant: Vec<EnumVariant>,
}

#[derive(Serialize, Deserialize)]
struct Address {
    name: String,
    doc: Option<String>,
    r#struct: Option<String>,
    old: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Param {
    name: String,
    attr: Vec<String>,
    r#type: String,
}

#[derive(Serialize, Deserialize)]
struct Struct {
    name: String,
    doc: Option<String>,
    field: Vec<Param>,
}

#[derive(Serialize, Deserialize)]
struct Err {
    r#type: String,
    success: String,
}

#[derive(Serialize, Deserialize)]
struct Func {
    sym: String,
    r#mod: Vec<String>,
    doc: Option<String>,
    ret: Option<String>,
    err: Option<Err>,
    par: Vec<Param>,
}

/// The main struct for a safe FFI specification.
#[derive(Serialize, Deserialize)]
struct SafeFFI {
    r#union: Vec<Union>,
    r#enum: Vec<Enum>,
    address: Vec<Address>,
    r#struct: Vec<Struct>,
    func: Vec<Func>,
}

fn fail() -> String {
    eprintln!("Usage:");
    eprintln!("    dl_api ffi/libname.muon src/ffi/libname.rs");
    eprintln!();
    std::process::exit(1);
}

fn convert(spec: &SafeFFI, mut out: String) -> String {
    use std::fmt::Write;

    // FIXME: Unions

    // Enums
    for en in &spec.r#enum {
        if let Some(ref doc) = en.doc {
            out.push_str("/// ");
            out.push_str(&doc.replace("\n", "\n/// "));
            out.push_str("\n");
        }
        if let Some(ref ty) = en.r#type {
            out.push_str("#[repr(");
            out.push_str(ty);
            out.push_str(")]\n");
        }
        out.push_str("#[repr(C)]\n#[non_exhaustive]\npub(crate) enum ");
        if en.name.ends_with("_t") {
            out.push_str(&en.name[..en.name.len() - 2].to_camel_case());
        } else {
            out.push_str(&en.name.to_camel_case());
        }
        out.push_str(" {\n");

        for variant in &en.variant {
            if let Some(ref doc) = variant.doc {
                out.push_str("    /// ");
                out.push_str(&doc.replace("\n", "\n    /// "));
                out.push_str("\n");
            }
            out.push_str("    ");
            out.push_str(&variant.name.to_camel_case());
            if let Some(ref value) = variant.value {
                out.push_str(" = ");
                write!(out, "{}", value).unwrap();
            }
            out.push_str(",\n");
        }

        out.push_str("}\n\n");
    }

    // Addresses
    for ad in &spec.address {
        if let Some(ref doc) = ad.doc {
            out.push_str("/// ");
            out.push_str(&doc.replace("\n", "\n/// "));
            out.push_str("\n");
        }

        out.push_str("#[repr(C)]\npub struct ");
        out.push_str(&ad.name);
        out.push_str("(*mut ");
        if let Some(ref record) = ad.r#struct {
            out.push_str(record);
        } else {
            out.push_str("c_void");
        }
        out.push_str(");\n\n");

        if let Some(ref old) = ad.old {
            out.push_str("impl Drop for ");
            out.push_str(&ad.name);
            out.push_str(" {\n    fn drop(&mut self) {\n        ");
            out.push_str(old);
            out.push_str("(self.0);\n    }\n}\n\n");
        }
    }

    // FIXME: Structs

    // FIXME: Functions

    out
}

fn main() {
    let mut args = std::env::args().skip(1);

    let src = args.next().unwrap_or_else(fail);
    let dst = args.next().unwrap_or_else(fail);

    let cx: SafeFFI = muon::from_reader(File::open(&src).unwrap_or_else(|e| {
        eprintln!("Couldn't open file: '{}': {}!", src, e);
        std::process::exit(1);
    }))
    .unwrap_or_else(|e| {
        eprintln!("Invalid file format: \"{}\"", e);
        std::process::exit(2);
    });

    println!("Converting SafeFFI file: {} into a Rust file: {}", src, dst);

    let out = convert(&cx, include_str!("../res/header.rs").to_string());

    let dst = Path::new(&dst);

    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            eprintln!("Couldn't create folder: '{}': {}!", parent.display(), e);
            std::process::exit(3);
        });
    }

    std::fs::write(dst, out).unwrap_or_else(|e| {
        eprintln!("Couldn't save file: '{}': {}!", dst.display(), e);
        std::process::exit(4);
    });
}
