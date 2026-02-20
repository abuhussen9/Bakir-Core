use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        display_menu();
        return;
    }
    if args[1] == "-i" && args.len() == 3 {
        install_tool(&args[2]);
    } else {
        println!("⚠️  استخدام غير صحيح. جرب: bakir-store -i <اسم_الأداة>");
    }
}

fn display_menu() {
    println!("==========================================");
    println!("    ⚔️  BAKIR-STORE: SOVEREIGN ARMORY  ⚔️");
    println!("==========================================");
    println!("1. bakir-opt   - [أداة تحسين النظام والإنترنت]");
    println!("2. bakir-shield- [قادم قريباً: الحماية السيادية]");
    println!("------------------------------------------");
    println!("💡 للتثبيت: bakir-store -i <اسم_الأداة>");
    println!("==========================================");
}

fn install_tool(tool_name: &str) {
    let url = format!("https://github.com/abuhussen9/Bakir-Core/raw/main/{}", tool_name);
    let dest = format!("/usr/bin/{}", tool_name);
    println!("📥 جاري جلب السلاح [{}] من الترسانة السحابية...", tool_name);
    let status = Command::new("sudo").args(&["curl", "-L", &url, "-o", &dest]).status();
    if status.is_ok() && status.unwrap().success() {
        println!("🔐 تفعيل الصلاحيات العسكرية للأداة...");
        Command::new("sudo").args(&["chmod", "+x", &dest]).status().unwrap();
        println!("✅ تم التثبيت بنجاح! جرب الآن كتابة: {}", tool_name);
    } else {
        println!("❌ فشل التحميل. تأكد من اسم الأداة.");
    }
}
