use rand::Rng;

pub fn add_task(task: (&str, u8)) -> ( i32, &str, u8, bool ) {  // task, and priority,
                                                                         // returns a tuple
    let mut rng = rand::thread_rng();
    let id: i32 = rng.gen::<i32>();


} // add a task to the list, and assign a random ID

pub fn remove_task() { }

pub fn archive_task() { } // mark a task as done

pub fn unarchive_task() { }

pub fn change_priority() { }

pub fn list_tasks() { }
