// all the operations command will be executed using the following functions
use std::process::Command;
pub fn execute(arguments:&str) -> String {
    let output = Command::new("powershell").args([arguments]).output().expect("Failed to execute process");
    let execution_sucess = output.stdout;
    let execution_error = output.stderr;
    //String::from_utf8(response).unwrap()
    if output.status.success() {
        String::from_utf8(execution_sucess).unwrap()
    }
    else {
        String::from_utf8(execution_error).unwrap()
    }
}