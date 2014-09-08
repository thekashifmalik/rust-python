
extern crate python;

use python::Python;


fn main() {
    println!("Running python interpreter...");
    spawn(proc() {
        let python_interpreter = Python::new("rust-python-test");
        python_interpreter.run("from time import time,ctime\nprint 'Today is',ctime(time())\n");
        python_interpreter.run("import sys\nprint '{}'.format(time())\n");
    });
}