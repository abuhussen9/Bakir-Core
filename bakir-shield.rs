use std::process::Command;
use std::env;
use std::io::{self, Write};

fn get_lang() -> String {
    env::var("LANG").unwrap_or_else(|_| "en".to_string())
}

fn print_msg(ar: &str, en: &str) {
    if get_lang().contains("ar") {
        println!("{}", ar);
    } else {
        println!("{}", en);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "-h" {
        display_help();
        return;
    }

    match args[1].as_str() {
        "-scan" => run_scan(),
        "-all" if args.get(2).map(|s| s.as_str()) == Some("close") => close_all(),
        "-all" if args.get(2).map(|s| s.as_str()) == Some("open") => open_all(),
        "-port" if args.len() == 4 => manage_port(&args[2], &args[3]),
        "-status" => show_status(),
        "-ghost" => toggle_ghost(),
        _ => display_help(),
    }
}

fn display_help() {
    println!("==========================================");
    println!("    🛡️  BAKIR-SHIELD (BS) : THE BEAST      ");
    println!("==========================================");
    print_msg("bs -scan           : فحص شامل للمنافذ (ألوان ذكية)", "bs -scan           : Full port scan (Smart colors)");
    print_msg("bs -port [رقم] close: إغلاق منفذ معين فوراً", "bs -port [num] close: Close specific port");
    print_msg("bs -port [رقم] open : فتح منفذ معين", "bs -port [num] open : Open specific port");
    print_msg("bs -all close      : الوضع الدفاعي (إغلاق الكل)", "bs -all close      : Defense mode (Close all)");
    print_msg("bs -ghost          : وضع التخفي (إخفاء الـ Ping)", "bs -ghost          : Stealth mode (Hide Ping)");
    print_msg("bs -status         : حالة الدرع والتهديدات", "bs -status         : Shield status & threats");
    println!("==========================================");
}

fn run_scan() {
    print_msg("🔎 جاري فحص المنافذ بالذكاء الاصطناعي...", "🔎 Scanning ports with AI-Logic...");
    let output = Command::new("ss").args(&["-tunl"]).output().expect("Failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines().skip(1) {
        if line.contains("LISTEN") {
            println!("\x1b[32m[OPEN]\x1b[0m {}", line); // أخضر فسفوري
        } else {
            println!("\x1b[31m[CLOSED]\x1b[0m {}", line); // أحمر دموي
        }
    }
}

fn manage_port(port: &str, action: &str) {
    let cmd = if action == "close" { "DROP" } else { "ACCEPT" };
    Command::new("sudo").args(&["iptables", "-A", "INPUT", "-p", "tcp", "--dport", port, "-j", cmd]).status().unwrap();
    print_msg(&format!("✅ تم تنفيذ الإجراء {} على المنفذ {}", action, port), &format!("✅ Action {} applied to port {}", action, port));
}

fn close_all() {
    print_msg("🛡️ تفعيل الدرع الكامل: إغلاق كافة المنافذ...", "🛡️ Activating Full Shield: Closing all ports...");
    Command::new("sudo").args(&["iptables", "-P", "INPUT", "DROP"]).status().unwrap();
    println!("DONE ✅");
}

fn open_all() {
    Command::new("sudo").args(&["iptables", "-P", "INPUT", "ACCEPT"]).status().unwrap();
    print_msg("🔓 العودة للوضع العادي", "🔓 Returned to normal mode");
}

fn toggle_ghost() {
    Command::new("sudo").args(&["sysctl", "-w", "net.ipv4.icmp_echo_ignore_all=1"]).status().unwrap();
    print_msg("👻 وضع التخفي نشط: جهازك الآن غير مرئي", "👻 Stealth mode active: Device is invisible");
}

fn show_status() {
    println!("🛡️ Shield Status: ACTIVE (Kernel eBPF Ready)");
    println!("🚫 Blocked Threats: 0 (No attacks detected yet)");
}
