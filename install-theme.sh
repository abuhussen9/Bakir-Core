#!/bin/bash
echo "🛡️ جاري تفعيل الهوية البصرية لباكير لينكس..."

# نقل الملفات للمسارات الجذرية
sudo cp -r assets/icons/* /usr/share/icons/ 2>/dev/null
sudo cp -r assets/desktop/* /usr/share/plasma/desktoptheme/ 2>/dev/null
sudo cp -r assets/sddm/* /usr/share/sddm/themes/ 2>/dev/null
sudo cp -r assets/splash/* /usr/share/plasma/look-and-feel/ 2>/dev/null

# أوامر التفعيل الفوري (لبيئة KDE)
lookandfeeltool -a Bakir-Splash-Screen
kwriteconfig5 --file kdeglobals --group Icons --key Theme "Bakir-Icons"
kwriteconfig5 --file plasmarc --group Theme --key name "Bakir-Plasma-Dark"

echo "✅ تم تفعيل الوحش الجمالي بنجاح!"
