use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // الرابط المباشر لمستودعك على GitHub
    let repo_url = "https://raw.githubusercontent.com/abuhussen9/bakir-store-project/main/"; 

    if args.len() < 2 {
        display_tools(repo_url);
        return;
    }

    if args[1] == "-i" && args.len() > 2 {
        let tool = &args[2];
        println!("📥 جاري جلب {} من ترسنة abuhussen9...", tool);
        
        if tool == "bakir-dark-mode" {
            Command::new("wget").arg("-q").arg(format!("{}{}.tar.gz", repo_url, tool)).status().unwrap();
            Command::new("wget").arg("-q").arg(format!("{}install-theme.sh", repo_url)).status().unwrap();
            Command::new("tar").args(&["-xzvf", "bakir-dark-mode.tar.gz"]).status().unwrap();
            Command::new("bash").arg("install-theme.sh").status().unwrap();
        } else {
            let output = Command::new("sudo").args(&["wget", "-q", "-O", &format!("/usr/bin/{}", tool), &format!("{}{}", repo_url, tool)]).status().unwrap();
            if output.success() {
                Command::new("sudo").args(&["chmod", "+x", &format!("/usr/bin/{}", tool)]).status().unwrap();
            }
        }
        println!("✅ انتهت العملية بنجاح.");
    }
}

fn display_tools(url: &str) {
    println!("📦 قائمة أدوات باكير المتوفرة (مستودع abuhussen9):");
    let list_url = format!("{}tools.list", url);
    let output = Command::new("curl").arg("-s").arg(list_url).output().expect("Failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
