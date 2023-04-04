pub struct Entry {
    pub id: u32,
    pub content: String,
}

impl Entry {
    pub fn to_string(&self) -> String {
        let string = "\n{\n".to_string()
            + &format!("    id: '{0}', \n    content: '{1}' \n", self.id.to_string(), self.content).to_string().replace("'", r#"""#)
            + "},";

        string
    }
}
