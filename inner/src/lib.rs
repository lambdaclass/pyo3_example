pub struct Inner {
    field: Vec<usize>,
}

impl Inner {
    pub fn new() -> Inner {
        Inner { field: vec![0; 10] }
    }
}
