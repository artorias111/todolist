fn main() {
    println!("Hello, this program is a simple cli-based to-do list. \n
        You can add items to your to-do list. \n
        You can also give each task a priority on a scale of 1-5, and they're sorted according to priority. \n
        Once you're done with a task, you can add it to an archive list");

    println!("Example usage: \n
        $ todolist add get pencils for tomorrow's exam -p 4 \n
        $ todolist list \n
        $ todolist done -i 1001 // (ID) \n
        $ todolist list -a // list all, including archived");
}
