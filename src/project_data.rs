use std::{cell::RefCell, rc::Rc};

pub type Shared<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct ProjectData {
    pub name: String,
    pub top: String,
    pub part: String,
    pub vhdl: u32,
    pub sv: u32,
    pub testbench: u32,
}

impl ProjectData {
    pub fn new(
        name: String,
        top: String,
        part: String,
        vhdl: u32,
        sv: u32,
        testbench: u32,
    ) -> Self {
        return Self {
            name,
            top,
            part,
            vhdl,
            sv,
            testbench,
        };
    }

    pub fn shared(
        name: String,
        top: String,
        part: String,
        vhdl: u32,
        sv: u32,
        testbench: u32,
    ) -> Shared<Self> {
        return Rc::new(RefCell::new(Self::new(
            name,
            top,
            part,
            vhdl,
            sv,
            testbench,
        )));
    }

    pub fn to_string(&self) -> [String;6] {
        return [
            self.name.clone(), 
            self.vhdl.to_string(), 
            self.sv.to_string(), 
            self.top.clone(), 
            self.testbench.to_string(),
            self.part.clone(), 
        ];
    }
}
