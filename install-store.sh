#!/bin/bash
echo "⚔️  جاري تثبيت متجر باكير السيادي في نظامك..."
sudo curl -L "https://github.com/abuhussen9/Bakir-Core/raw/main/bakir-store" -o /usr/bin/bakir-store
sudo curl -L "https://github.com/abuhussen9/Bakir-Core/raw/main/libnetwork.so" -o /usr/lib/libnetwork.so
sudo chmod +x /usr/bin/bakir-store
sudo ldconfig
echo "✅ تم التثبيت بنجاح! جرب الآن كتابة الأمر: bakir-store"
