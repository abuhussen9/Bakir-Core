use std::process::Command;
use std::io::{self, Write};

fn run_command(desc: &str, cmd: &str, args: &[&str]) {
    print!("⚙️  {}... ", desc);
    io::stdout().flush().unwrap();
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) if s.success() => println!("✅ تم"),
        _ => println!("❌ فشل"),
    }
}

fn main() {
    println!("==========================================");
    println!("    ⚔️  BAKIR-OPT: SMART SOVEREIGN SYSTEM  ⚔️");
    println!("==========================================");

    run_command("تسريع الإنترنت (TCP Fast Open)", "sudo", &["sysctl", "-w", "net.ipv4.tcp_fastopen=3"]);
    run_command("إصلاح المكتبات والحزم المكسورة", "sudo", &["apt-get", "install", "-f", "-y"]);
    run_command("تطهير الرام والذاكرة المخبأة", "sudo", &["sh", "-c", "sync; echo 3 > /proc/sys/vm/drop_caches"]);
    run_command("إزالة الحزم اليتيمة والمخلفات", "sudo", &["apt-get", "autoremove", "-y"]);

    println!("------------------------------------------");
    println!("💎 نظام باكير الآن في قمة عطائه!");
    println!("==========================================");
}
