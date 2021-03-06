#[macro_use] extern crate cpython;

use cpython::{Python, PyResult, PyDict, ToPyObject, PyObject};
use std::collections::HashMap;

py_module_initializer!(conllrust, initconllrust, PyInit_conllrust, |py, m | {
    try!(m.add(py, "__doc__", "conll stats"));
    try!(m.add(py, "frequencies", py_fn!(py, frequencies(text: Vec<String>, c: i32, exclude: Vec<String>, exclude_column: i32, pos_column: i32))));
    try!(m.add(py, "nq", py_fn!(py, nq(item: PyObject))));
    Ok(())
});

fn nq(_py: Python, item: PyObject) -> PyResult<PyDict>{
    let mut hm = HashMap::new();

    let aa = item.size();
    *hm.entry(String::from("test")).or_insert(1);
    let pydict = hm.to_py_object(_py);
    Ok(pydict)
}

fn frequencies(_py: Python, text: Vec<String>, c: i32, exclude: Vec<String>, exclude_c: i32, pos_c: i32) -> PyResult<PyDict> {
    // Returns frequencies (such as parts of speech or words)

    // c determines which column to search

    // exclude is a list of things to exclude, if they appear in the column
    // given by exclude_c

    let column = c as usize;
    let exclude_column = exclude_c as usize;
    let pos_column = pos_c as usize;

    let mut sum = 0.0;
    let mut hm = HashMap::new();

    for line in &text {
        let split = line.split("\t");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() - 1 >= column && vec.len() - 1 >= exclude_column {
            let mut target = String::from(vec[column]);
            target.push_str("_");
            target.push_str(&String::from(vec[pos_column]));
            let target_exclude = vec[exclude_column];
            if !exclude.contains(&String::from(target_exclude)){
                *hm.entry(target).or_insert(0.0) += 1.0;
                sum += 1.0;
            }
        }
    }
    *hm.entry(String::from("TOKEN_COUNT")).or_insert(sum);
    let dict = hm.to_py_object(_py);
    Ok(dict)
}
