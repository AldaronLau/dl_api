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

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
struct Err {
    r#type: String,
    success: String,
}

#[derive(Serialize, Deserialize, Clone)]
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

fn convert(spec: &SafeFFI, mut out: String, so_name: &str) -> String {
    use std::fmt::Write;

    out.push_str("const DL_API_SHARED_OBJECT_NAME: &[u8] = b\"lib");
    out.push_str(so_name);
    out.push_str("\\0\";\n\n");

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

    struct Module {
        // The name of the struct that represents the module.
        name: String,
        // Rust extern "C" functions' names, types and symbol.
        c_fn: Vec<Func>,
    }

    impl std::cmp::PartialEq for Module {
        fn eq(&self, other: &Module) -> bool {
            self.name == other.name
        }
    }

    impl std::cmp::Eq for Module {}

    impl std::cmp::PartialOrd for Module {
        fn partial_cmp(&self, other: &Module) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::Ord for Module {
        fn cmp(&self, other: &Module) -> std::cmp::Ordering {
            self.name.cmp(&other.name)
        }
    }

    let mut mods: Vec<Module> = vec![]; // All modules.

    for func in &spec.func {
        let global = {
            let mut temp = func.sym.clone();
            temp.make_ascii_uppercase();
            temp
        };

        out.push_str("static mut FUNC_");
        out.push_str(&global);
        out.push_str(": std::mem::MaybeUninit<extern fn(\n    ");

        let mut new = None;

        for param in &func.par {
            if param.attr.len() == 2 {
                match param.attr.first().unwrap().as_str() {
                    // Modifier on references - use NULL for None.
                    "Opt" => {},
                    // Modifier on references - multiple.
                    "Arr" => {},
                    //
                    a => panic!("Invalid modifier: {}", a),
                }
            }

            match param.attr.last().unwrap().as_str() {
                // Input, pass-by-value (copy).
                "Val" => {
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                },
                // Output, pointer to uninitialized data to be initialized.
                "Out" => new = Some(param.r#type.clone()),
                // Output, pointer to uninitialized pointer to be allocated.
                "New" => new = Some(param.r#type.clone()),
                // Input-Output, initialized reference that may change.
                "Mut" => {
                    out.push_str("*mut ");
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                }
                // Input, pass-by-reference, initialized memory that won't change.
                "Ref" => {
                    out.push_str("*const ");
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                }
                // Input, pass-by-reference, and free all.
                "Old" => {
                    out.push_str("*mut ");
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                },
                // Input, pass-by-value (copy), and free all.
                "Eat" => {
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                },
                // Input, pass-by-reference, and free fields but not struct itself.
                "Inv" => {
                    out.push_str("*const ");
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                }
                // Input, pass-by-value (must use with Arr).
                "Len" => {
                    out.push_str(&param.r#type);
                    out.push_str(", ");
                },
                // Output, pointer to uninitialized error data to be initialized.
                "Err" => todo!(), // FIXME
                // Use integer value as length for a .text parameter
                "Txt" => todo!(), // FIXME
                //
                a => panic!("Invalid modifier: {}", a),
            }
        }
        out.push_str("\n) -> ");

        if let Some(ref ret) = func.ret {
            out.push_str(ret);
        } else if let Some(ref ret) = func.err {
            out.push_str(&ret.r#type);
        } else {
            out.push_str("()");
        }

        out.push_str("> = std::mem::MaybeUninit::uninit();\n\n");

        for module in &func.r#mod {
            if let Ok(index) = mods.binary_search(&Module { name: module.clone(), c_fn: vec![]}) {
                mods[index].c_fn.push(func.clone());
            } else {
                mods.push(Module {
                    name: module.clone(),
                    c_fn: vec![func.clone()],
                });
                mods.sort_unstable();
            }
        }
    }

    for module in mods {
        out.push_str("/// A module contains functions.\npub struct ");
        out.push_str(&module.name);
        out.push_str(";\n\n");

        out.push_str("impl ");
        out.push_str(&module.name);
        out.push_str(" {\n");
        for cfunc in &module.c_fn {
            let global = {
                let mut temp = cfunc.sym.clone();
                temp.make_ascii_uppercase();
                temp
            };
            let method = {
                let mut temp = cfunc.sym.clone();
                temp.make_ascii_lowercase();
                temp
            };

            if let Some(ref doc) = cfunc.doc {
                out.push_str("    /// ");
                out.push_str(&doc.replace("\n", "\n    /// "));
                out.push_str("\n");
            }
            out.push_str("    fn ");
            out.push_str(&method);
            out.push_str("(");

            let mut new = None;
            let mut function_call = format!("((FUNC_{}).assume_init())(", global);

            for param in &cfunc.par {
                let mut end = ", ";

                if param.attr.len() == 2 {
                    match param.attr.first().unwrap().as_str() {
                        // Modifier on references - use NULL for None.
                        "Opt" => {
                            out.push_str("Option<");
                            end = ">, ";
                        },
                        // Modifier on references - multiple.
                        "Arr" => {
                            out.push_str("&[");
                            end = "], ";
                        },
                        //
                        a => panic!("Invalid modifier: {}", a),
                    }
                }

                match param.attr.last().unwrap().as_str() {
                    // Input, pass-by-value (copy).
                    "Val" => {
                        out.push_str(&param.r#type);
                        out.push_str(end);
                    },
                    // Output, pointer to uninitialized data to be initialized.
                    "Out" => new = Some(param.r#type.clone()),
                    // Output, pointer to uninitialized pointer to be allocated.
                    "New" => new = Some(param.r#type.clone()),
                    // Input-Output, initialized reference that may change.
                    "Mut" => {
                        out.push_str("&mut ");
                        out.push_str(&param.r#type);
                        out.push_str(end);
                    }
                    // Input, pass-by-reference, initialized memory that won't change.
                    "Ref" => {
                        out.push_str("&");
                        out.push_str(&param.r#type);
                        out.push_str(end);
                    }
                    // Input, pass-by-reference, and free all.
                    "Old" => {
                        out.push_str("&mut ");
                        out.push_str(&param.r#type);
                        out.push_str(end);
                    },
                    // Input, pass-by-value (copy), and free all.
                    "Eat" => {
                        out.push_str(&param.r#type);
                        out.push_str(end);
                    }
                    // Input, pass-by-reference, and free fields but not struct itself.
                    "Inv" => {
                        out.push_str("&");
                        out.push_str(&param.r#type);
                        out.push_str(end);
                    }
                    // Input, pass-by-value (must use with Arr).
                    "Len" => {
                        function_call.push_str("DL_API_PLACEHOLDER_LEN.len()");
                        out.push_str(end);
                    }, // FIXME
                    // Output, pointer to uninitialized error data to be initialized.
                    "Err" => todo!(), // FIXME
                    // Use integer value as length for a .text parameter
                    "Txt" => todo!(), // FIXME
                    //
                    a => panic!("Invalid modifier: {}", a),
                }
            }

            out.push_str(") -> ");
            if let Some(ref ret) = cfunc.ret {
                out.push_str(ret);
            } else if let Some(ref ret) = cfunc.err {
                out.push_str("Result<");
                if let Some(new) = new {
                    out.push_str(&new);
                } else {
                    out.push_str("()");
                }
                out.push_str(",");
                out.push_str(&ret.r#type);
                out.push_str(">");
            } else {
                if let Some(new) = new {
                    out.push_str(&new);
                } else {
                    out.push_str("()");
                }
            }
            out.push_str(" {\n");
            out.push_str("        let _ret = unsafe { ");
            out.push_str(&function_call);
            out.push_str(") };\n        ");

            out.push_str("_ret\n");

            out.push_str("    }\n\n");
        }
        out.push_str("}\n\n");
    }

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

    let src = Path::new(&src);
    let so_name = src.file_stem().unwrap().to_str().unwrap().replace(",", ".");

    let out = convert(&cx, include_str!("../res/header.rs").to_string(), &so_name);

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
