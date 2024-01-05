use std::error::Error;
use std::process::Command;
use std::process::Stdio;
use execute::Execute;
use std::path::PathBuf;
use std::env;
use std::path::Path;
use std::fs;
use glob::glob;
fn main() -> Result<(),Box<dyn Error>> {
    let srcdir = PathBuf::from("tools/Appacker_v1.3.11.exe");
    let rel_path = Path::new("Release");
    fs::create_dir_all(rel_path)?;
    let packer_path= fs::canonicalize(&srcdir).unwrap();
    let out_dir = env!("CRATE_OUT_DIR");
    let mut name = String::new();
    for entry in glob(&format!("{}/*.exe",out_dir)).unwrap(){
        let exec = entry.unwrap();
        fs::copy(&exec,format!("tools/Release/{}",exec.file_name().unwrap().to_str().unwrap()))?;
        name = exec.file_name().unwrap().to_string_lossy().to_string();
        break;
    }
    let path = env::current_dir()?;
    let mut first_command = Command::new(packer_path);
    first_command.arg("-q");
    first_command.arg("--fd \"Rust App\"");
    first_command.arg("-d");
    first_command.arg(format!("Release\\{}",name));
    first_command.arg("-e");
    first_command.arg(format!("{}\\tools\\Release\\{}",path.display(),name));
    first_command.arg("-s");
    first_command.arg("tools\\Release");
    first_command.stdout(Stdio::piped());
    let output = first_command.execute_input_output("2^99\n").unwrap();    
    println!("{}", String::from_utf8(output.stdout).unwrap().trim_end());
    println!("{}", String::from_utf8(output.stderr).unwrap().trim_end());
    Ok(())
}