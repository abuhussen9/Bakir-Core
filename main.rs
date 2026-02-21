use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        display_remote_menu();
        return;
    }
    if args[1] == "-i" && args.len() == 3 {
        install_tool(&args[2]);
    }
}

fn display_remote_menu() {
    println!("==========================================");
    println!("    ⚔️  BAKIR-STORE: LIVE CLOUD ARMORY  ⚔️");
    println!("==========================================");
    
    // جلب قائمة الأسلحة من جيت هب مباشرة
    let output = Command::new("curl")
        .args(&["-s", "https://github.com/abuhussen9/Bakir-Core/raw/main/tools.list"])
        .output();

    if let Ok(out) = output {
        let list = String::from_utf8_lossy(&out.stdout);
        for (i, line) in list.lines().enumerate() {
            if !line.is_empty() {
                println!("{}. {}  - [جاهز للتحميل]", i + 1, line);
            }
        }
    }
    println!("------------------------------------------");
    println!("💡 للتثبيت: bakir-store -i <اسم_الأداة>");
    println!("==========================================");
}

fn install_tool(tool_name: &str) {
    let url = format!("https://github.com/abuhussen9/Bakir-Core/raw/main/{}", tool_name);
    let dest = format!("/usr/bin/{}", tool_name);
    println!("📥 جاري جلب [{}] من السحاب...", tool_name);
    let status = Command::new("sudo").args(&["curl", "-L", &url, "-o", &dest]).status();
    if status.is_ok() && status.unwrap().success() {
        Command::new("sudo").args(&["chmod", "+x", &dest]).status().unwrap();
        println!("✅ تم التثبيت بنجاح! جرب الآن: {}", tool_name);
    }
}
