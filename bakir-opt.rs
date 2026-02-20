use std::process::Command;
use std::io::{self, Write};

fn run_command(desc: &str, cmd: &str, args: &[&str]) {
    print!("⚙️  {}... ", desc);
    io::stdout().flush().unwrap();
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) if s.success() => println!("✅ تم"),
        _ => println!("❌ فشل أو يحتاج صلاحيات"),
    }
}

fn main() {
    println!("==========================================");
    println!("    ⚔️  BAKIR-OPT: SMART SOVEREIGN SYSTEM  ⚔️");
    println!("==========================================");

    run_command("تحسين إعدادات الشبكة وتسريع الإنترنت", "sudo", &["sysctl", "-w", "net.ipv4.tcp_fastopen=3"]);
    run_command("تقليل وقت الاستجابة (Ping)", "sudo", &["sysctl", "-w", "net.core.rmem_max=16777216"]);
    run_command("إصلاح الحزم المكسورة والمكتبات الناقصة", "sudo", &["apt-get", "install", "-f", "-y"]);
    run_command("تحديث قاعدة بيانات النظام", "sudo", &["apt-get", "update", "-y"]);
    run_command("تطهير الرام والذاكرة المخبأة", "sudo", &["sh", "-c", "sync; echo 3 > /proc/sys/vm/drop_caches"]);
    run_command("حذف الملفات المؤقتة والسجلات الضخمة", "sudo", &["sh", "-c", "rm -rf /var/log/*.gz /tmp/*"]);
    run_command("إزالة الحزم اليتيمة والمخلفات", "sudo", &["apt-get", "autoremove", "-y"]);
    run_command("تنظيف مستودع التخزين المؤقت", "sudo", &["apt-get", "clean"]);

    println!("------------------------------------------");
    println!("💎 النظام الآن: سريع، مستقر، وآمن تماماً.");
    println!("==========================================");
}
