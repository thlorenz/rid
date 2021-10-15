use std::env;
use dirs::home_dir;

pub struct HostProps {
    // See https://github.com/dart-lang/ffigen/blob/6e10689c0e1a510f47d2e81540678771bf560250/lib/src/strings.dart#L158-L170
    pub llvm_paths: Vec<String>,
}

impl HostProps {
    pub fn new() -> Self {
        let linux_llvm_paths = [
            "/usr/lib/llvm-6.0/lib/libclang.so".to_owned(),
            "/usr/lib/llvm-9/lib/libclang.so".to_owned(),
            "/usr/lib/llvm-10/lib/libclang.so".to_owned(),
            "/usr/lib/llvm-11/lib/libclang.so".to_owned(),
            "/usr/lib/libclang.so".to_owned(),
            "/usr/lib64/libclang.so".to_owned(),
            "/usr/lib/llvm-6.0/lib/libclang.so".to_owned()
        ];

        let mut macos_llvm_paths = [
            "/usr/local/opt/llvm/lib/".to_owned(),
            "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr".to_owned()].to_vec();

        match home_dir() {
            None => (),
            Some(mut path) => {
                path.push("homebrew/opt/llvm");
                match path.into_os_string().into_string() {
                    Err(_) => (),
                    Ok(path) => {
                        macos_llvm_paths.push(path.to_owned())
                    }
                }
            }
        }

        let windows_llvm_paths = [r#"C:\Program Files\LLVM\bin\"#.to_owned()];

        let llvm_paths: Vec<String> = match env::consts::OS {
            "linux" => linux_llvm_paths.to_vec(),
            "macos" => macos_llvm_paths.to_vec(),
            "windows" => windows_llvm_paths.to_vec(),
            "android" | "ios" | "freebsd" | "dragonfly" | "netbsd"
            | "openbsd" | "solaris" | _ => {
                panic!("rid cli cannot run on {}", env::consts::OS)
            }
        };

        match env::var("LIBCLANG_PATH") {
            Err(_) => Self {
                llvm_paths: llvm_paths
            },
            Ok(paths) => { 
                let split_paths = paths.split(":");
                Self {
                    llvm_paths: split_paths.map(str::to_owned).collect()
                }
            }
        }
    }
}
