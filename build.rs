use std::process::Command;

fn main() {
    panic!("{:?}", Command::new("git").args(["describe", "--tags"]).output());
    // if let Ok(output) =  {
    //     if output.status.success() {
    //         let version = String::from_utf8_lossy(&output.stdout);
    //         let version = version.trim();
    //         if !version.is_empty() {
    //             println!("cargo:rustc-env=PROJECT_VERSION={}", version);
    //         }
    //     }
    // }
}
