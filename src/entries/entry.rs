#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: u32,
    pub content: String,
}

fn new_entry(content: String) -> Entry {
    Entry {
        id: rand::random::<u32>(),
        content: content
    }
}

impl Entry {
    pub fn to_string(&self) -> String {
        let string = "\n{\n".to_string()
            + &format!(
                "    id: '{0}', \n    content: '{1}' \n",
                self.id.to_string(),
                self.content
            )
            .to_string()
            .replace("'", r#"""#)
            + "},";

        string
    }
}

pub trait EntryConversions {
    fn to_entry(&self) -> Entry;
}

impl EntryConversions for String {
    fn to_entry(&self) -> Entry {
        let result = new_entry(self.to_string());

        result 
    }
}