use std::env;
use dirs::home_dir;

pub struct HostProps {
    // See https://github.com/dart-lang/ffigen/blob/6e10689c0e1a510f47d2e81540678771bf560250/lib/src/strings.dart#L158-L170
    pub llvm_paths: Vec<String>,
}

const LINUX_LLVM_PATHS: [&str; 7] = [
    "/usr/lib/llvm-6.0/lib/libclang.so",
    "/usr/lib/llvm-9/lib/libclang.so",
    "/usr/lib/llvm-10/lib/libclang.so",
    "/usr/lib/llvm-11/lib/libclang.so",
    "/usr/lib/libclang.so",
    "/usr/lib64/libclang.so",
    "/usr/lib/llvm-6.0/lib/libclang.so"
];

const MACOS_LLVM_PATHS: [&str; 2] = [
    "/usr/local/opt/llvm/lib/",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr"];

const WINDOWS_LLVM_PATHS: [&str; 1] = [r#"C:\Program Files\LLVM\bin\"#];

impl HostProps {
    pub fn new() -> Self {
        let mut llvm_paths: Vec<String> = match env::consts::OS {
            "linux" => LINUX_LLVM_PATHS.map(String::from).to_vec(),
            "macos" => MACOS_LLVM_PATHS.map(String::from).to_vec(),
            "windows" => WINDOWS_LLVM_PATHS.map(String::from).to_vec(),
            "android" | "ios" | "freebsd" | "dragonfly" | "netbsd"
            | "openbsd" | "solaris" | _ => {
                panic!("rid cli cannot run on {}", env::consts::OS)
            }
        };

        match env::var("LIBCLANG_PATH") {
            Err(_) => {
                match (env::consts::OS, home_dir()) {
                    ("macos", Some(mut path)) => {
                        path.push("homebrew/opt/llvm");
                        match path.into_os_string().into_string() {
                            Err(_) => (),
                            Ok(path) => {
                                llvm_paths.push(path.to_owned());
                            }
                        }
                    },
                    (_, _) => ()
                };
                Self {
                    llvm_paths: llvm_paths
                }
            },
            Ok(path) => { 
                Self {
                    llvm_paths: vec![path]
                }
            }
        }
    }
}
