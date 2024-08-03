use std::{io::Read, process::{Command, Stdio}};

pub fn ls()
{
    let system_command = "echo";
    let argument = "Hello World";
    
    let echo_hello: std::process::Output = Command::new(system_command)
                            .arg(argument)
                            //.current_dir(TEST_PATH)
                            // Stdio::piped() creates pipes to capture stdout and stderr of a child process which is created by Command. 
                            // we are telling the Command to create a pipe to capture the standard output (stdout) of the child process.
                            .stdout(Stdio::piped())
                            // creating a pipe to capture the standard error (stderr) of the child process.
                            .stderr(Stdio::piped())
                            // output() executes the child process synchronously and captures its output. 
                            //It returns a std::process::Output struct containing information about the process's exit status, stdout, and stderr.
                            .output()
                            .expect("Failed to echo");

    println!("status: {}", echo_hello.status);
    println!("stdout: {}", String::from_utf8_lossy(&echo_hello.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&echo_hello.stderr));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ls();
    }

}