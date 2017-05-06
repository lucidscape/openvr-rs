use std::collections::BTreeMap;
use std::path::Path;
use std::io::{self, Write};
use std::fs::File;

// use std::ffi::CString;
// use std::os::raw::c_char;

extern crate rustc_serialize;

use rustc_serialize::json::Json;

fn map_name(n: &str) -> String {
    const KEYWORDS: [&'static str; 52] = [
        "abstract", "alignof", "as", "become", "box",
        "break", "const", "continue", "crate", "do",
        "else", "enum", "extern", "false", "final",
        "fn", "for", "if", "impl", "in",
        "let", "loop", "macro", "match", "mod",
        "move", "mut", "offsetof", "override", "priv",
        "proc", "pub", "pure", "ref", "return",
        "Self", "self", "sizeof", "static", "struct",
        "super", "trait", "true", "type", "typeof",
        "unsafe", "unsized", "use", "virtual", "where",
        "while", "yield",
    ];

    if KEYWORDS.iter().any(|&x| x == n) {
        let mut out: String = n.into();
        out.push('_');
        out
    } else {
        n.into()
    }
}

fn map_type(ty: &str) -> String {

    match ty {
        "int"           => "c_int".into(),

        "uint8_t"       => "u8".into(),
        "int8_t"        => "i8".into(),
        "uint16_t"      => "u16".into(),
        "int16_t"       => "i16".into(),
        "uint32_t"      => "u32".into(),
        "int32_t"       => "i32".into(),
        "uint64_t"      => "u64".into(),
        "int64_t"       => "i64".into(),
        "float"         => "f32".into(),
        "double"        => "f64".into(),
        "_Bool"         => "bool".into(),

        "unsigned short" => "u16".into(),

        "char *"        => "*mut c_char".into(),
        "const char *"  => "*const c_char".into(),
        "const char *const"  => "*const c_char".into(),

        "uint8_t *"     => "*mut u8".into(),
        "int8_t *"      => "*mut i8".into(),
        "uint32_t *"    => "*mut u32".into(),
        "int32_t *"     => "*mut i32".into(),
        "uint64_t *"    => "*mut u64".into(),
        "int64_t *"     => "*mut i64".into(),
        "float *"       => "*mut f32".into(),

        "void *"        => "*mut c_void".into(),
        "void **"       => "*mut *mut c_void".into(),

        "const uint8_t *"    => "*const u8".into(),
        "const uint16_t *"   => "*const u16".into(),

        "char [8]"      => "[c_char; 8]".into(),
        "float [2]"     => "[f32; 2]".into(),
        "float [3]"     => "[f32; 3]".into(),
        "float [4]"     => "[f32; 4]".into(),
        "double [3]"    => "[f64; 3]".into(),
        "float [3][4]"  => "[[f32; 4]; 3]".into(),
        "float [4][4]"  => "[[f32; 4]; 4]".into(),
        "struct vr::HmdVector3_t [4]" => "[HmdVector3; 4]".into(),
        "struct vr::VRControllerAxis_t [5]" => "[VRControllerAxis; 5]".into(),

        _               => {
            let mut ty =
                ty
                .replace("vr::", "")
                .replace("struct ", "")
                .replace("class ", "")
                .replace("const ", "")
                .replace("enum ", "")
                .replace("union ", "")
                .replace("_t", "");

            if ty.contains(" **") {
                let mut newty = String::new();
                newty.push_str("*mut *mut ");
                newty.push_str(&ty.replace(" **", ""));
                ty = newty
            }
            else if ty.contains(" *") {
                let mut newty = String::new();
                newty.push_str("*mut ");
                newty.push_str(&ty.replace(" *", ""));
                ty = newty
            }

            ty
        }
    }
}

fn map_ident(ident: &str) -> String {
    match ident {
        "type"  => "ty".into(),
        _       => ident.into()
    }
}

fn write_method_params(w: &mut Write, params: &[Json]) {
    for (i, param) in params.iter().enumerate() {
        write!(w, "{}: {}",
            map_ident(&param["paramname"].as_string().unwrap()),
            map_type(&param["paramtype"].as_string().unwrap())
        ).unwrap();

        if i < params.len() - 1 {
            write!(w, ", ").unwrap();
        }
    }
}

pub fn generate(definition: &Path, output: &Path) -> io::Result<()> {
    let mut w   = File::create(output)?;
    let data    = Json::from_reader(&mut File::open(definition)?).expect("failed to parse");
    let obj     = data.as_object().expect("is object");

    writeln!(&mut w, "{}", include_str!("ffi_header.rs"))?;

    match &obj["typedefs"] {
        &Json::Array(ref arr) => {
            for t in arr {
                let typedef = t["typedef"].as_string().unwrap().replace("vr::", "").replace("_t", "");

                let ty = map_type(&t["type"].as_string().unwrap());
                if ty == typedef {
                    continue;
                }

                writeln!(&mut w, "pub type {} = {};",
                    typedef,
                    ty
                )?;
            }
        }
        _ => panic!("expected array")
    }

    match &obj["enums"] {
        &Json::Array(ref arr) => {
            for en in arr {
                let enum_name = en["enumname"].as_string().unwrap().replace("vr::", "");
                writeln!(&mut w, "#[repr(C)]")?;
                writeln!(&mut w, "#[derive(Clone, Copy, Debug, PartialEq)]")?;
                writeln!(&mut w, "pub enum {} {{", enum_name)?;
                match &en["values"] {
                    &Json::Array(ref arr) => {
                        for val in arr {
                            let prefix = &enum_name[1..];
                            let mut name: String = val["name"].as_string().unwrap().into();

                            if name.starts_with(prefix) {
                                name = name.replace(prefix, "")[1..].to_string();
                            }

                            if
                                name == "k_EButton_SteamVR_Touchpad" ||
                                name == "k_EButton_SteamVR_Trigger" ||
                                name == "k_EButton_Dashboard_Back" {
                                println!("WARNING: Skipping duplicate enum value: {}", name);
                                continue;
                            }

                            writeln!(&mut w, "    {} = {},",
                                name,
                                val["value"].as_string().unwrap()
                            )?;
                        }
                    }
                    _ => panic!("expected array of values")
                }
                writeln!(&mut w, "}}\n")?;
            }
        }
        _ => panic!("expected array")
    }

    fn emit_const(w: &mut Write, name: &str, ty: &str, value: &str) -> io::Result<()> {
        if name.contains("_Version") {
            writeln!(w, "pub const {}_FnTable: &'static str = \"FnTable:{}\";",
                name.replace("_Version", ""),
                value
            )?;
        }

        let value: String =
            if ty == "const char *const" {
                format!("\"{}\"", value)
            } else {
                value.into()
            };
        let ty =
            if ty == "const char *const" {
                "&'static str".into()
            } else {
                map_type(&ty.replace("const ", ""))
            };

        writeln!(w, "pub const {}: {} = {};",
            name,
            ty,
            value
        )?;
        Ok(())
    }

    match &obj["consts"] {
        &Json::Array(ref arr) => {
            for c in arr {
                let ty = c["consttype"].as_string().unwrap();
                let value = c["constval"].as_string().unwrap();
                let name = c["constname"].as_string().unwrap();

                emit_const(&mut w, name, ty, value)?;
            }
        }
        _ => panic!("expected array")
    }

    writeln!(&mut w, "\n")?;

    match &obj["structs"] {
        &Json::Array(ref arr) => {
            for st in arr {
                let structname = st["struct"].as_string().unwrap().replace("vr::", "").replace("_t", "");
                if structname == "(anonymous)" {
                    println!("WARNING: Ignoring anonymous struct (probable union)");
                    continue;
                }
                writeln!(&mut w, "#[repr(C)]")?;
                writeln!(&mut w, "#[derive(Clone, Copy)]")?;
                writeln!(&mut w, "pub struct {} {{", structname )?;
                match &st["fields"] {
                    &Json::Array(ref arr) => {
                        for field in arr {
                            writeln!(&mut w, "    pub {}: {},",
                                map_name(field["fieldname"].as_string().unwrap()),
                                map_type(&field["fieldtype"].as_string().unwrap())
                            )?;
                        }
                    }
                    _ => panic!("expected array of values")
                }
                writeln!(&mut w, "}}\n")?;

                writeln!(&mut w, "impl Default for {} {{", structname)?;
                writeln!(&mut w, "    fn default() -> {} {{", structname)?;
                writeln!(&mut w, "        unsafe {{ mem::zeroed() }}")?;
                writeln!(&mut w, "    }}")?;
                writeln!(&mut w, "}}\n")?;
            }
        }
        _ => panic!("expected array")
    }

    let mut classes = BTreeMap::new();

    match &obj["methods"] {
        &Json::Array(ref arr) => {
            for method in arr {
                let classname = method["classname"].as_string().unwrap();
                if classes.contains_key(classname) == false {
                    classes.insert(classname, Vec::new());
                }

                classes.get_mut(&classname).unwrap().push(method)
            }
        }
        _ => panic!("expected array")
    }

    // Interfaces
    for (classname, methods) in classes {
        let struct_name = classname.replace("vr::", "");

        writeln!(&mut w, "#[repr(C)]")?;
        writeln!(&mut w, "pub struct {} {{", struct_name)?;

        for method in methods {
            write!(&mut w, "    pub {}: extern \"C\" fn(", method["methodname"].as_string().unwrap())?;
            match method.find("params") {
                Some(&Json::Array(ref params)) => {
                    write_method_params(&mut w, params);
                }
                _ => {}
            }

            let returntype = method["returntype"].as_string().unwrap();
            if returntype == "void" {
                writeln!(&mut w, "),")?;
            } else {
                writeln!(&mut w, ") -> {},", map_type(&returntype))?;
            }
        }

        writeln!(&mut w, "}}\n")?;
    }

    Ok(())
}
