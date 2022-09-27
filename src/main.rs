// Task List
// Task 1. Reading user input (key presses)

// Sub-Task for Task 1
// 1. Raw mode instead of canonical mode (enter sends input to program)
//     using termion (external crate) to allow enter raw mode.
// 2. Error handling
// 3. Ediomatic code (Code that others don't have to think for them to understand.)
// 4. Separate the code into multiple files.

// #task.1.4
mod editor;
use editor::Editor;

fn main() {
    
    let editor = Editor::default();
    editor.run();
    
}
