use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn FetchFromBakirRepo(url: *const c_char) -> *mut c_char;
}

fn main() {
    println!("🚀 Bakir-Store: Initializing Secure Connection...");
    
    let repo_url = CString::new("https://raw.githubusercontent.com/abuhussen9/Bakir-Core/main/repo/tools.list").unwrap();
    
    unsafe {
        let result_ptr = FetchFromBakirRepo(repo_url.as_ptr());
        if !result_ptr.is_null() {
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            println!("📦 Available Tools in Arsenal:\n------------------------------\n{}", result);
            println!("------------------------------");
        } else {
            println!("❌ Error: Could not fetch repository data.");
        }
    }
}
