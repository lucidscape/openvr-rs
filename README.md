# Rust Bindings for the OpenVR Library

**This library is a work in progress**

    /bindgen - generates the ffi interface
    /example - minimal example of using the lib
    /openvr - submodule containing the api definition and binaries   


- [OpenVR Source](https://github.com/ValveSoftware/openvr)
- [OpenVR Documentation](https://github.com/ValveSoftware/openvr/wiki/API-Documentation)

## Usage
```
[dependencies]
openvr = "*"
```

```
extern crate openvr;
use openvr::ffi;

...

// Initialize the library
let mut context = openvr::initialize().expect("OpenVR init");

// Get the current sensor state
let poses = context.compositor.wait_get_poses();

// Submit eye textures
context.compositor.submit(...);

```

- Copy openvr_api.dll to your application's target directory

## Roadmap

- Bind remaining API
- Cross platform support
- Docs
- Expanded example
