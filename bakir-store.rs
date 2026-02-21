use std::process::Command;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let repo_url = "https://raw.githubusercontent.com/username/bakir-store-project/main/"; // استبدل username باسم حسابك

    if args.len() < 2 {
        display_tools();
        return;
    }

    if args[1] == "-i" && args.len() > 2 {
        let tool = &args[2];
        println!("📥 جاري جلب {} من الترسانة السحابية...", tool);

        if tool == "bakir-dark-mode" {
            // معالجة خاصة للثيم الشامل
            Command::new("wget").arg(format!("{}{}.tar.gz", repo_url, tool)).status().unwrap();
            Command::new("wget").arg(format!("{}install-theme.sh", repo_url)).status().unwrap();
            Command::new("tar").args(&["-xzvf", "bakir-dark-mode.tar.gz"]).status().unwrap();
            Command::new("bash").arg("install-theme.sh").status().unwrap();
            println!("✅ تم تثبيت وتفعيل الثيم بنجاح!");
        } else {
            // معالجة الأدوات العادية
            let output = Command::new("sudo").args(&["wget", "-O", &format!("/usr/bin/{}", tool), &format!("{}{}", repo_url, tool)]).status().unwrap();
            if output.success() {
                Command::new("sudo").args(&["chmod", "+x", &format!("/usr/bin/{}", tool)]).status().unwrap();
                println!("✅ تم تثبيت {} بنجاح.", tool);
            }
        }
    }
}

fn display_tools() {
    println!("📦 قائمة أدوات باكير المتوفرة:");
    let output = Command::new("curl").arg("https://raw.githubusercontent.com/username/bakir-store-project/main/tools.list").output().expect("Failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
