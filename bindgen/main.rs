use std::collections::BTreeMap;
use std::io::Write;
use std::fs::File;

// use std::ffi::CString;
// use std::os::raw::c_char;

extern crate libc;
extern crate rustc_serialize;

use rustc_serialize::json::Json;

fn map_type(ty: &str) -> String {

    match ty {
        "uint8_t"       => "u8".into(),
        "int8_t"        => "i8".into(),
        "uint32_t"      => "u32".into(),
        "int32_t"       => "i32".into(),
        "float"         => "f32".into(),
        "double"        => "f64".into(),

        "unsigned short" => "u16".into(),

        "char *"        => "*mut c_char".into(),
        "const char *"  => "*const c_char".into(),

        "uint8_t *"     => "*mut u8".into(),
        "int8_t *"      => "*mut i8".into(),
        "uint32_t *"    => "*mut u32".into(),
        "int32_t *"     => "*mut i32".into(),
        "float *"       => "*mut f32".into(),

        "void *"        => "*mut libc::c_void".into(),
        "void **"        => "*mut *mut libc::c_void".into(),

        "const uint8_t *"   => "*const u8".into(),
        "const uint16_t *"   => "*const u16".into(),

        _               => {
            let mut ty =
                ty
                .replace("vr::", "")
                .replace("struct ", "")
                .replace("class ", "")
                .replace("const", "");

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

fn main() {
    // let mut w   = &mut io::stdout();
    let mut w   = File::create("../src/ffi.rs").unwrap();
    let data    = Json::from_str(include_str!("openvr_api.json")).expect("failed to parse");
    let obj     = data.as_object().expect("is object");

    writeln!(&mut w, "{}", include_str!("ffi_header.rs")).unwrap();

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

    for (classname, methods) in classes {
        let struct_name = classname.replace("vr::", "");

        writeln!(&mut w, "struct {} {{", struct_name).unwrap();
        writeln!(&mut w, "    ptr: *mut {}", struct_name).unwrap();
        writeln!(&mut w, "}}\n").unwrap();

        writeln!(&mut w, "impl {} {{", struct_name).unwrap();

        for method in methods {
            write!(&mut w, "    unsafe fn {}(&self, ", method["methodname"].as_string().unwrap()).unwrap();
            match method.find("params") {
                Some(&Json::Array(ref params)) => {
                    for (i, param) in params.iter().enumerate() {
                        write!(&mut w, "{}: {}",
                            map_ident(&param["paramname"].as_string().unwrap()),
                            map_type(&param["paramtype"].as_string().unwrap())
                        ).unwrap();

                        if i < params.len() - 1 {
                            write!(&mut w, ", ").unwrap();
                        }
                    }
                }
                _ => {}
            }

            // Call the internal function
            writeln!(&mut w, ") {{").unwrap();
            write!(&mut w, "        {}(self.ptr, ", method["methodname"].as_string().unwrap()).unwrap();
            match method.find("params") {
                Some(&Json::Array(ref params)) => {
                    for (i, param) in params.iter().enumerate() {
                        write!(&mut w, "{}",
                            map_ident(&param["paramname"].as_string().unwrap())
                        ).unwrap();

                        if i < params.len() - 1 {
                            write!(&mut w, ", ").unwrap();
                        }
                    }
                }
                _ => {}
            }
            writeln!(&mut w, ")").unwrap();
            writeln!(&mut w, "    }}\n").unwrap();
        }

        writeln!(&mut w, "}}\n").unwrap();
        // writeln!(&mut w, "--> {}", classname);
    }

    match &obj["enums"] {
        &Json::Array(ref arr) => {
            for en in arr {
                writeln!(&mut w, "enum {} {{", en["enumname"].as_string().unwrap().replace("vr::", "")).unwrap();
                match &en["values"] {
                    &Json::Array(ref arr) => {
                        for val in arr {
                            writeln!(&mut w, "    {} = {},",
                                val["name"].as_string().unwrap(),
                                val["value"].as_string().unwrap()
                            ).unwrap();
                        }
                    }
                    _ => panic!("expected array of values")
                }
                writeln!(&mut w, "}}\n").unwrap();
            }
        }
        _ => panic!("expected array")
    }

    match &obj["consts"] {
        &Json::Array(ref arr) => {
            for c in arr {
                let ty = map_type(&c["consttype"].as_string().unwrap());
                let value = c["constval"].as_string().unwrap();
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
                        ty
                    };

                writeln!(&mut w, "const {}: {} = {};",
                    c["constname"].as_string().unwrap(),
                    ty,
                    value
                ).unwrap();
            }
        }
        _ => panic!("expected array")
    }

    writeln!(&mut w, "\n").unwrap();

    match &obj["structs"] {
        &Json::Array(ref arr) => {
            for st in arr {
                writeln!(&mut w, "struct {} {{", st["struct"].as_string().unwrap().replace("vr::", "")).unwrap();
                match &st["fields"] {
                    &Json::Array(ref arr) => {
                        for field in arr {
                            writeln!(&mut w, "    {}: {},",
                                field["fieldname"].as_string().unwrap(),
                                map_type(&field["fieldtype"].as_string().unwrap())
                            ).unwrap();
                        }
                    }
                    _ => panic!("expected array of values")
                }
                writeln!(&mut w, "}}\n").unwrap();
            }
        }
        _ => panic!("expected array")
    }

}
