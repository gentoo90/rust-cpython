#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;

py_module_initializer!(inheritance, |_py, m| {
    try!(m.add("__doc__", "Module documentation string"));
    let base_class = try!(
        m.add_type::<()>("BaseClass")
        .doc("Type doc string")
        .finish());
    for i in 1..10 {
        try!(
            m.add_type::<()>(&format!("C{}", i))
            .base(&base_class)
            .doc(&format!("Derived class #{}", i))
            .finish());
    }
    Ok(())
});

