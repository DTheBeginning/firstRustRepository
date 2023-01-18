mod MessageList;
mod MessageInformation;


fn main() {
    let mut stack = MessageList::Messages::create_stack();
    stack.create_message(entering_information(), entering_information(), entering_information());
    stack.create_message(entering_information(), entering_information(), entering_information());
    stack.print_stack();
    stack.delete_message(1);
    stack.print_stack();
}


fn entering_information() -> String {
    let mut value = String::new();
    std::io::stdin().read_line(&mut value).expect("pizda");
    let value: String = value.trim().parse().expect("zalupa");
    return value;
}