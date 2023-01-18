pub struct Messages {
    pub stack: Vec<crate::MessageInformation::Message>
}


impl Messages {
    pub fn print_stack(&self) {
        for item in 0..self.stack.len() {
            print!("[{}]: ", item);
            self.stack[item].print_info();
        }
    }


    pub fn create_message(&self, author: String, release_time: String, text: String) {
        self.stack.push(crate::MessageInformation::Message::create_message(author, release_time, text));
    }


    pub fn delete_message(&self, index: usize) {
        
        self.stack.remove(index);
    }


    pub fn create_stack() -> Messages {
        let result: Vec<crate::MessageInformation::Message> = vec![];
        return crate::MessageList::Messages{stack: result};
    }
}