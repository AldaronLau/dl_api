use heck::CamelCase;
use muon_rs as muon;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

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
    bytes: Option<String>,
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
    def: String,
    r#mod: Vec<String>,
    doc: Option<String>,
    pat: Vec<String>,
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

/// A C Prototype
#[derive(Clone)]
struct Prototype {
    /// 
    ret: String,
    ///
    name: String,
    ///
    pars: Vec<(String, String)>,
}

/// Safe C Function
#[derive(Clone)]
struct CFunc {
    proto: Prototype,
    doc: Option<String>,
    pats: Vec<String>,
}

// Get the type of a formal C parameter.
fn get_index(params: &Vec<(String, String)>, name: &str) -> usize {
    for par in 0..params.len() {
        if params[par].1 == name {
            return par;
        }
    }
    panic!("parameter {} not found", name)
}

// Get the type of a formal C parameter.
fn get_formal_type(params: &Vec<(String, String)>, name: &str) -> String {
    for par in params {
        if par.1 == name {
            return par.0.clone();
        }
    }
    panic!("parameter {} not found", name)
}

// Check if a struct is an address
fn address_exists(addresses: &Vec<Address>, name: &str) -> bool {
    for adr in addresses {
        if adr.name == name {
            return true;
        }
    }

    false
}

impl FromStr for Prototype {
    type Err = ();

    fn from_str(string: &str) -> Result<Prototype, Self::Err> {
        if string.chars().last() != Some(')') {
            return Err(());
        }
        let split: Vec<&str> = string[..(string.len()-1)].split('(').collect();
        if split.len() != 2 {
            return Err(());
        }
        let mut func: Vec<&str> = split[0].split_ascii_whitespace().collect();
        if func.len() < 2 {
            return Err(());
        }
        let name = func.pop().unwrap().to_string();
        let ctype = func.pop().unwrap();
        let mut ret = String::new();
        for modifier in func {
            ret.push_str(modifier);
            ret.push_str(" ");
        }
        ret.push_str(ctype);

        let mut pars = Vec::new();

        for par in split[1].split(',') {
            let mut par: Vec<&str> = par.trim().split_ascii_whitespace().collect();
            if par.len() < 2 {
                return Err(());
            }
            let name = par.pop().unwrap().to_string();
            let ctype = par.pop().unwrap();
            let mut ty = String::new();
            for modifier in par {
                ty.push_str(modifier);
                ty.push_str(" ");
            }
            ty.push_str(ctype);

            pars.push((ty, name));
        }

        Ok(Prototype { ret, name, pars })
    }
}

fn fail() -> String {
    eprintln!("Usage:");
    eprintln!("    dl_api ffi/libname.muon src/ffi/libname.rs");
    eprintln!();
    std::process::exit(1);
}

fn c_binding_into_rust(input: &str) -> Option<String> {
    Some(match input {
        "()" => "()".to_string(),
        "bool" => "bool".to_string(),
        "u8" => "u8".to_string(),
        "i8" => "i8".to_string(),
        "u8" => "u8".to_string(),
        "i16" => "i16".to_string(),
        "u32" => "u32".to_string(),
        "i32" => "i32".to_string(),
        "u64" => "u64".to_string(),
        "i64" => "i64".to_string(),
        "std::os::raw::c_char" => "char".to_string(),
        "std::os::raw::c_uchar" => "u8".to_string(),
        "std::os::raw::c_schar" => "i8".to_string(),
        "std::os::raw::c_ushort" => "u16".to_string(),
        "std::os::raw::c_short" => "i16".to_string(),
        "std::os::raw::c_uint" => "u32".to_string(),
        "std::os::raw::c_int" => "i32".to_string(),
        "std::os::raw::c_ulonglong" => "u64".to_string(),
        "std::os::raw::c_longlong" => "i64".to_string(),
        "std::os::raw::c_ulong" => "usize".to_string(),
        "std::os::raw::c_long" => "isize".to_string(),
        "std::os::raw::c_double" => "f64".to_string(),
        "std::os::raw::c_float" => "f32".to_string(),
        "std::os::raw::c_void" => return None,
        other => other.to_string(),
    })
}

fn c_type_as_binding(input: &str, addresses: &Vec<Address>) -> String {
    match input.trim_matches(|c: char| c.is_ascii_whitespace()) {
        "uint8_t" => "u8".to_string(),
        "int8_t" => "i8".to_string(),
        "uint16_t" => "u16".to_string(),
        "int16_t" => "i16".to_string(),
        "uint32_t" => "u32".to_string(),
        "int32_t" => "i32".to_string(),
        "uint64_t" => "u64".to_string(),
        "int64_t" => "i64".to_string(),
        "char" => "std::os::raw::c_char".to_string(),
        "unsigned char" => "std::os::raw::c_uchar".to_string(),
        "signed char" => "std::os::raw::c_schar".to_string(),
        "unsigned short" | "unsigned short int" => "std::os::raw::c_ushort".to_string(),
        "short" | "signed short" | "short int" | "signed short int" => "std::os::raw::c_short".to_string(),
        "unsigned int" | "unsigned" => "std::os::raw::c_uint".to_string(),
        "int" | "signed int" | "signed" => "std::os::raw::c_int".to_string(),
        "unsigned long long int" | "unsigned long long" => "std::os::raw::c_ulonglong".to_string(),
        "long long int" | "long long" | "signed long long int" | "signed long long" => "std::os::raw::c_longlong".to_string(),
        "unsigned long int" | "unsigned long" => "std::os::raw::c_ulong".to_string(),
        "long int" | "long" => "std::os::raw::c_long".to_string(),
        "double" => "std::os::raw::c_double".to_string(),
        "float" => "std::os::raw::c_float".to_string(),
        "bool" => "bool".to_string(),
        "void" => "()".to_string(),
        // FIXME Long Double
        other => if address_exists(addresses, other) {
            "std::os::raw::c_void".to_string()
        } else {
            if other.ends_with("_t") {
                other[..other.len() - 2].to_camel_case()
            } else {
                other.to_camel_case()
            }
        }
    }
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
        out.push_str("#[repr(C)]\n#[non_exhaustive]\n#[derive(Copy, Clone, Debug, PartialEq)]\npub enum ");
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
        let name = if ad.name.ends_with("_t") {
            ad.name[..ad.name.len() - 2].to_camel_case()
        } else {
            ad.name.to_camel_case()
        };

        if let Some(ref doc) = ad.doc {
            out.push_str("/// ");
            out.push_str(&doc.replace("\n", "\n/// "));
            out.push_str("\n");
        }

        out.push_str("pub struct ");
        out.push_str(&name);
        out.push_str("(*mut ");
        if let Some(ref record) = ad.r#struct {
            out.push_str(record);
        } else {
            out.push_str("std::os::raw::c_void");
        }
        out.push_str(");\n\n");
    }

    // FIXME: Structs

    // Functions
    struct Module {
        // The name of the struct that represents the module.
        name: String,
        // Rust extern "C" functions' names, types and symbol.
        c_fn: Vec<CFunc>,
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
        let proto = func.def.parse::<Prototype>().unwrap(); // FIXME unwrap

        let global = {
            let mut temp = proto.name.clone();
            temp.make_ascii_uppercase();
            temp
        };

        out.push_str("static mut FN_");
        out.push_str(&global);
        out.push_str(":\n    std::mem::MaybeUninit<extern fn(\n");

        for param in &proto.pars {
            let (orig_type, is_const) = if param.0.starts_with("const ") {
                (&param.0["const ".len()..], true)
            } else {
                (&param.0[..], false)
            };
            let trim_type = orig_type.trim_end_matches("*");

            let num_ptr = orig_type.len() - trim_type.len();

            let mut string = "".to_string();

            let ctype = c_type_as_binding(trim_type, &spec.address);

            out.push_str("        ");
            out.push_str(&param.1);
            out.push_str(": ");
            for _ in 0..num_ptr {
                if is_const {
                    out.push_str("*const ");
                } else {
                    out.push_str("*mut ");
                }
            }
            out.push_str(ctype.as_str());
            out.push_str(",\n");
        }

        let (orig_type, is_const) = if proto.ret.starts_with("const ") {
            (&proto.ret["const ".len()..], true)
        } else {
            (&proto.ret[..], false)
        };
        let trim_type = orig_type.trim_end_matches("*");

        let num_ptr = orig_type.len() - trim_type.len();

        let mut string = "".to_string();

        let ctype = c_type_as_binding(trim_type, &spec.address);

        out.push_str("    ) -> ");
        for _ in 0..num_ptr {
            if is_const {
                out.push_str("*const ");
            } else {
                out.push_str("*mut ");
            }
        }
        out.push_str(&ctype);
        out.push_str("> = std::mem::MaybeUninit::uninit();\n");

        for module in &func.r#mod {
            if let Ok(index) = mods.binary_search(&Module { name: module.clone(), c_fn: vec![]}) {
                mods[index].c_fn.push(CFunc {
                    proto: proto.clone(),
                    doc: func.doc.clone(),
                    pats: func.pat.clone(),
                });
            } else {
                mods.push(Module {
                    name: module.clone(),
                    c_fn: vec![CFunc {
                        proto: proto.clone(),
                        doc: func.doc.clone(),
                        pats: func.pat.clone(),
                    }],
                });
                mods.sort_unstable();
            }
        }
    }

    for module in mods {
        out.push_str("\n/// A module contains functions.\npub struct ");
        out.push_str(&module.name);
        out.push_str("(std::marker::PhantomData<*mut u8>);\n\n");

        out.push_str("impl ");
        out.push_str(&module.name);
        out.push_str(" {\n");
        out.push_str("    /// Load a module.\n");
        out.push_str("    pub fn new() -> Option<Self> {\n");
        out.push_str("        unsafe {\n");
        out.push_str("            let dll = check_thread()?;\n");
        for cfunc in &module.c_fn {
            let global = {
                let mut temp = cfunc.proto.name.clone();
                temp.make_ascii_uppercase();
                temp
            };
            out.push_str("            FN_");
            out.push_str(&global);
            out.push_str(" = std::mem::MaybeUninit::new(std::mem::transmute(sym(dll, b\"");
            out.push_str(&cfunc.proto.name);
            out.push_str("\\0\")?.as_ptr()));\n");
        }
        out.push_str("            Some(Self(std::marker::PhantomData))\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        for cfunc in &module.c_fn {
            let global = {
                let mut temp = cfunc.proto.name.clone();
                temp.make_ascii_uppercase();
                temp
            };
            let method = {
                let mut temp = cfunc.proto.name.clone();
                temp.make_ascii_lowercase();
                temp
            };

            if let Some(ref doc) = cfunc.doc {
                out.push_str("    /// ");
                out.push_str(&doc.replace("\n", "\n    /// "));
                out.push_str("\n");
            }
            out.push_str("    pub fn ");
            out.push_str(&method);
            out.push_str("(&self,\n");

            // let mut new = None;
            let mut pre = "".to_string();
            let mut post = "".to_string();
            let mut tuple = vec![];
            let mut result = false;
            let mut function_params: Vec<String> = vec![];

            for pattern in &cfunc.pats {
                let mut iter = pattern.split(' ');
                let rule = iter.next().unwrap();

                match rule {
                    "NEW" => { // Return, Pre uninit
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);

                        pre.push_str("            let mut ");
                        pre.push_str(&name);
                        pre.push_str(" = std::mem::MaybeUninit::uninit();\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("{}.as_mut_ptr()", name);
                        post.push_str("            let ");
                        post.push_str(&name);
                        post.push_str(" = ");
                        post.push_str(&name);
                        post.push_str(".assume_init();\n");
                        tuple.push(name);
                    }
                    "STR" => { // Parameter &CStr
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": &std::ffi::CStr,\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("{}.as_ptr()", name);
                    }
                    "RAW" => { // Parameter (if object => ref)
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": *mut std::os::raw::c_void,\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("{}", name);
                    }
                    "VAL" => { // Parameter (if object => ref)
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        let octype = get_formal_type(&cfunc.proto.pars, name);
                        let (octype, is_const) = if octype.starts_with("const ") {
                            (&octype["const ".len()..], true)
                        } else {
                            (&octype[..], false)
                        };
                        let octype = octype.trim_end_matches("*");
                        let ctype = c_type_as_binding(&octype, &spec.address);
                        let (ctype, adr) = if let Some(c) = c_binding_into_rust(&ctype)
                        {
                            (c, false)
                        } else {
                            (if octype.ends_with("_t") {
                                octype[..octype.len() - 2].to_camel_case()
                            } else {
                                octype.to_camel_case()
                            }, true)
                        };

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": ");
                        if adr {
                            out.push_str("&");
                        }
                        out.push_str(&ctype);
                        out.push_str(",\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        if adr {
                            function_params[num] = format!("{}.0", name);
                        } else {
                            function_params[num] = format!("{} as _", name);
                        }
                    }
                    "OK" => { // Return, Post if -> result
                        post.push_str("            if __ret ");
                        post.push_str(match iter.next().unwrap() {
                            "=" => "!=",
                            "<" => ">=",
                            ">" => "<=",
                            "<=" => ">",
                            ">=" => "<",
                            "!=" => "==",
                            a => panic!("Unknown operator in OK {}!", a),
                        });
                        post.push_str(" ");
                        post.push_str(iter.next().unwrap());
                        post.push_str(" { return Err(__ret as _) };\n");
                        if result {
                            panic!("Can't use OK for results twice");
                        }
                        result = true;
                    }
                    "MUT" => { // Parameter, Pre cast, Post cast
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        let octype = get_formal_type(&cfunc.proto.pars, name);
                        if octype.starts_with("const ") {
                            panic!("MUT Should not be const!!!");
                        }
                        let octype = octype.trim_end_matches("*");
                        let ctype = c_type_as_binding(&octype, &spec.address);
                        let (ctype, adr) = if let Some(c) = c_binding_into_rust(&ctype)
                        {
                            (c, false)
                        } else {
                            panic!("Mutable reference to address not supported.  Use NEW");
                        };

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": &mut ");
                        out.push_str(&ctype);
                        out.push_str(",\n");
                        pre.push_str("            let mut __");
                        pre.push_str(&name);
                        pre.push_str(": _ = *");
                        pre.push_str(&name);
                        pre.push_str(" as _;\n");
                        post.push_str("            *");
                        post.push_str(&name);
                        post.push_str(" = __");
                        post.push_str(&name);
                        post.push_str(" as _;\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("&mut __{}", name);
                    }
                    "OPT_MUT" => { // Parameter, Pre cast, Post cast
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        let octype = get_formal_type(&cfunc.proto.pars, name);
                        if octype.starts_with("const ") {
                            panic!("MUT Should not be const!!!");
                        }
                        let octype = octype.trim_end_matches("*");
                        let ctype = c_type_as_binding(&octype, &spec.address);
                        let (ctype, adr) = if let Some(c) = c_binding_into_rust(&ctype)
                        {
                            (c, false)
                        } else {
                            panic!("Mutable reference to address not supported.  Use NEW");
                        };

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": Option<&mut ");
                        out.push_str(&ctype);
                        out.push_str(">,\n");
                        pre.push_str("            let mut __");
                        pre.push_str(&name);
                        pre.push_str(": _ = if let Some(_temp) = ");
                        pre.push_str(&name);
                        pre.push_str(".iter().next() { Some(**_temp as _) } else { None };\n");
                        post.push_str("            if let Some(_temp) = ");
                        post.push_str(&name);
                        post.push_str(" { *_temp = __");
                        post.push_str(&name);
                        post.push_str(".unwrap() as _; }\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("if let Some(ref mut _temp) = __{} {{ _temp }} else {{ std::ptr::null_mut() }}", name);
                    }
                    "OLD" => { // Parameter object => move
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        let octype = get_formal_type(&cfunc.proto.pars, name);
                        if octype.starts_with("const ") {
                            panic!("MUT Should not be const!!!");
                        }
                        let octype = octype.trim_end_matches("*");
                        let ctype = c_type_as_binding(&octype, &spec.address);
                        let ctype = if let Some(c) = c_binding_into_rust(&ctype)
                        {
                            panic!("Can't destroy non-object {}!", c);
                        } else {
                            if octype.ends_with("_t") {
                                octype[..octype.len() - 2].to_camel_case()
                            } else {
                                octype.to_camel_case()
                            }
                        };

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": &mut ");
                        out.push_str(&ctype);
                        out.push_str(",\n");
                        pre.push_str("            if ");
                        pre.push_str(name);
                        pre.push_str(".0.is_null() { panic!(\"Object free'd twice!\") }\n");
                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("{}.0", name);
                        post.push_str("            ");
                        post.push_str(name);
                        post.push_str(".0 = std::ptr::null_mut();\n");
                    }
                    "SLICE" => { // Parameter &[]
                        let namelen = iter.next().unwrap();
                        let numlen = get_index(&cfunc.proto.pars, namelen);
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        let octype = get_formal_type(&cfunc.proto.pars, name);
                        let (octype, is_const) = if octype.starts_with("const ") {
                            (&octype["const ".len()..], true)
                        } else {
                            (&octype[..], false)
                        };
                        let octype = octype.trim_end_matches("*");
                        let ctype = c_type_as_binding(&octype, &spec.address);
                        let ctype = if let Some(c) = c_binding_into_rust(&ctype)
                        {
                            c
                        } else {
                            panic!("Vec of objects not supported");
                        };

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": &[");
                        out.push_str(&ctype);
                        out.push_str("],\n");

                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("{}.as_ptr()", name);

                        function_params.resize(function_params.len().max(numlen + 1), String::new());
                        function_params[numlen] = format!("{}.len() as _", name);
                    }
                    "OUT" => { // Return
                        if let Some(name) = iter.next() {
                            let num = get_index(&cfunc.proto.pars, name);

                            pre.push_str("            let mut ");
                            pre.push_str(&name);
                            pre.push_str(" = std::mem::MaybeUninit::uninit();\n");

                            function_params.resize(function_params.len().max(num + 1), String::new());
                            function_params[num] = format!("{}.as_mut_ptr()", name);

                            post.push_str("            let ");
                            post.push_str(&name);
                            post.push_str(" = ");
                            post.push_str(&name);
                            post.push_str(".assume_init();\n");

                            tuple.push(name);
                        } else {
                            tuple.push("__ret");
                        }
                    }
                    "OUT_VEC" => { // Parameter Vec
                        let namelen = iter.next().unwrap();
                        let numlen = get_index(&cfunc.proto.pars, namelen);
                        let name = iter.next().unwrap();
                        let num = get_index(&cfunc.proto.pars, name);
                        let octype = get_formal_type(&cfunc.proto.pars, name);
                        let octype = octype.trim_end_matches("*");
                        let ctype = c_type_as_binding(&octype, &spec.address);
                        let ctype = if let Some(c) = c_binding_into_rust(&ctype)
                        {
                            c
                        } else {
                            panic!("Vec of objects not supported");
                        };

                        out.push_str("        ");
                        out.push_str(name);
                        out.push_str(": &mut Vec<");
                        out.push_str(&ctype);
                        out.push_str(">,\n");

                        post.push_str("            ");
                        post.push_str(name);
                        post.push_str(".set_len(");
                        if let Some(par) = iter.next() {
                            todo!(); // FIXME: Add out parameter support
                        } else {
                            post.push_str("__ret");
                        }
                        post.push_str(" as _);\n");

                        function_params.resize(function_params.len().max(num + 1), String::new());
                        function_params[num] = format!("{}.as_mut_ptr()", name);
                        function_params.resize(function_params.len().max(numlen + 1), String::new());
                        function_params[numlen] = format!("{}.capacity() as _", name);
                    }
                    unknown => panic!("Unknown pattern: {}", unknown),
                }
            }

            out.push_str("    ) -> ");
            if result {
                out.push_str("Result<");
            }
            let length = tuple.len();
            if length != 1 {
                out.push_str("(");
            }
            for name in tuple.iter() {
                let octype = if name == &"__ret" {
                    cfunc.proto.ret.clone()
                } else {
                    get_formal_type(&cfunc.proto.pars, name)
                };

                if octype.ends_with("*") && octype.starts_with("void") {
                    out.push_str("*mut std::os::raw::c_void");

                    if length != 1 {
                        out.push_str(",")
                    }
                    continue;
                }
                let octype = octype.trim_end_matches("*");
                let ctype = c_type_as_binding(&octype, &spec.address);
                let (ctype, adr) = if let Some(c) = c_binding_into_rust(&ctype)
                {
                    (c, false)
                } else {
                    (if octype.ends_with("_t") {
                        octype[..octype.len() - 2].to_camel_case()
                    } else {
                        octype.to_camel_case()
                    }, true)
                };
                out.push_str(&ctype);

                if length != 1 {
                    out.push_str(",")
                }
            }
            if length != 1 {
                out.push_str(")");
            }
            if result {
                let octype = cfunc.proto.ret.trim_end_matches("*");
                let ctype = c_type_as_binding(&octype, &spec.address);
                let (ctype, adr) = if let Some(c) = c_binding_into_rust(&ctype)
                {
                    (c, false)
                } else {
                    (if octype.ends_with("_t") {
                        octype[..octype.len() - 2].to_camel_case()
                    } else {
                        octype.to_camel_case()
                    }, true)
                };

                out.push_str(", ");
                out.push_str(&ctype); // TODO: From parameters
                out.push_str(">");
            }

            out.push_str("\n    {\n");
            out.push_str("        unsafe {\n");
            out.push_str(&pre);
            out.push_str("            let __ret = ");
            out.push_str(&format!("((FN_{}).assume_init())(\n", global));
            for param in function_params {
                out.push_str("                ");
                out.push_str(&param);
                out.push_str(",\n");
            }
            out.push_str("            );\n");
            out.push_str(&post);
            out.push_str("            ");
            if result {
                out.push_str("Ok(");
            }
            if length != 1 {
                out.push_str("(");
            }
            for name in tuple.iter() {
                let octype = if name == &"__ret" {
                    cfunc.proto.ret.clone()
                } else {
                    get_formal_type(&cfunc.proto.pars, name)
                };

                let octype = octype.trim_end_matches("*");
                let ctype = c_type_as_binding(&octype, &spec.address);
                let (ctype, adr) = if let Some(c) = c_binding_into_rust(&ctype)
                {
                    (c, false)
                } else {
                    (if octype.ends_with("_t") {
                        octype[..octype.len() - 2].to_camel_case()
                    } else {
                        octype.to_camel_case()
                    }, true)
                };
                if adr {
                    out.push_str(&ctype);
                    out.push_str("(");                    
                }
                out.push_str(&name);
                if adr {
                    out.push_str(")");
                }
                out.push_str(" as _");

                if length != 1 {
                    out.push_str(",")
                }
            }
            if length != 1 {
                out.push_str(")");
            }
            if result {
                out.push_str(")");
            }
            out.push_str("\n        }\n");

            out.push_str("    }\n");
        }
        out.push_str("}\n");
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
