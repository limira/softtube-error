#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Search {
    Limit(i32),
    Text(String)
}

pub struct Videos {
}

    
impl Videos {
    pub fn search(search : Option<Search>) {
    }
}
