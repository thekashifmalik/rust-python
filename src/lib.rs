#![feature(unsafe_destructor)]


#[link(name = "python")]
extern {
    fn Py_SetProgramName(name: &str);
    fn Py_Initialize();
    fn PyRun_SimpleString(string: &str);
    fn Py_Finalize();
}


pub struct Python<'a> {
    name: &'a str
}

impl<'a, 'b> Python<'a> {
    pub fn new(name: &'a str) -> Python {
        unsafe {
            Py_SetProgramName(name);
            Py_Initialize();
        }
        Python{name: name}
    }

    pub fn run(&self, string: &'b str) {
        unsafe {
            PyRun_SimpleString(string);
        }
    }

}

#[unsafe_destructor]
impl<'a> Drop for Python<'a> {
    fn drop(&mut self) {
        unsafe {
            Py_Finalize();
        }
    }
}