#[macro_use] extern crate cpython;

use cpython::{Python, PyResult, PyDict, ToPyObject};
use std::collections::HashMap;

py_module_initializer!(conllrust, initconllrust, PyInit_conllrust, |py, m | {
    try!(m.add(py, "__doc__", "conll stats"));
    try!(m.add(py, "frequencies", py_fn!(py, frequencies(text: Vec<String>, c: i32, exclude: Vec<String>, exclude_column: i32))));
    Ok(())
});

fn frequencies(_py: Python, text: Vec<String>, c: i32, exclude: Vec<String>, exclude_column: i32) -> PyResult<PyDict> {
    // Returns frequencies (such as parts of speech or words)

    // c determines which column to search

    // exclude is a list of things to exclude, if they appear in the column
    // given by exclude_column

    let column = c as usize;

    let mut hm = HashMap::new();
    
    for line in &text {
        let split = line.split("\t");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() - 1 >= column && vec.len() - 1 >= exclude_column {
            let target = vec[column];
            let target_exclude = vec[exclude_column]
            if !exclude.contains(&String::from(target_exclude)){
                *hm.entry(target).or_insert(0.0) += 1.0;
            }
        }
    }

    let dict = hm.to_py_object(_py);
    Ok(dict)
}
