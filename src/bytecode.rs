use super::vm::{
    Const, Value, CALL, CREATE_CONTEXT, END, GET_GLOBAL, GET_MEMBER, PUSH_CONST, PUSH_FALSE,
    PUSH_TRUE,
};

#[derive(Debug, Clone)]
pub struct ByteCodeGen {
    pub consts: Const,
}

pub type ByteCode = Vec<u8>;

impl ByteCodeGen {
    pub fn new() -> ByteCodeGen {
        ByteCodeGen {
            consts: Const::new(),
        }
    }
}

impl ByteCodeGen {
    pub fn end(&self, insts: &mut ByteCode) {
        insts.push(END);
    }

    pub fn call(&self, argc: u32, insts: &mut ByteCode) {
        insts.push(CALL);
        self.gen_int32(argc as i32, insts);
    }

    pub fn create_context(&self, n: usize, argc: usize, insts: &mut ByteCode) {
        insts.push(CREATE_CONTEXT);
        self.gen_int32(n as i32, insts);
        self.gen_int32(argc as i32, insts);
    }
    pub fn push_const(&mut self, val: Value, insts: &mut ByteCode) {
        insts.push(PUSH_CONST);
        let id = self.consts.value.len();
        self.consts.value.push(val);
        self.gen_int32(id as i32, insts);
    }

    pub fn push_bool(&self, b: bool, insts: &mut ByteCode) {
        insts.push(if b { PUSH_TRUE } else { PUSH_FALSE })
    }

    pub fn get_member(&self, insts: &mut ByteCode) {
        insts.push(GET_MEMBER);
    }

    pub fn get_global(&mut self, name: String, insts: &mut ByteCode) {
        insts.push(GET_GLOBAL);
        let id = self.consts.string.len();
        self.consts.string.push(name);
        self.gen_int32(id as i32, insts);
    }

    pub fn gen_int32(&self, n: i32, insts: &mut ByteCode) {
        insts.push(((n >> 0) & 0xff as i32) as u8);
        insts.push(((n >> 8) & 0xff as i32) as u8);
        insts.push(((n >> 16) & 0xff as i32) as u8);
        insts.push(((n >> 24) & 0xff as i32) as u8);
    }
}
