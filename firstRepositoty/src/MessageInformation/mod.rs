pub struct Message {
    author: String,
    release_time: String,
    text: String
}


impl Message {
    pub fn print_info(&self) {
        println!("{} at {}:\n\t{}", self.author, self.release_time, self.text);
    }


    pub fn create_message(author: String, release_time: String, text: String) -> Message {
        let result: Message = Message { author: author, release_time: release_time, text: text };
        return result;
    }
}