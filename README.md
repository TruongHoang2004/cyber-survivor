# ⚡ CYBER SURVIVOR: 2D Top-Down Tactical Action Shooter

> Một game hành động bắn súng góc nhìn từ trên xuống (2D Top-Down Action Shooter) trên nền web, kết hợp đồ họa Neon Cyberpunk, âm thanh tổng hợp đa âm bằng Web Audio API và cơ chế điều khiển bàn phím tốc độ cao (High APM).

---

## 🎮 Tính năng nổi bật

- 🕹️ **Thao tác phím dồn dập (High APM):** Di chuyển 8 hướng, lướt né đạn (Dash I-frames), làm chậm thời gian (Bullet Time), ném lựu đạn nổ chùm, triển khai trụ súng tự động.
- 🎯 **Cơ chế Nạp đạn Chủ động (Active Reload - Gears of War Style):** Bấm `R` đúng vùng sáng (Sweet Spot) để nạp đạn tức thì và nhận buff **+40% Overcharge Damage**.
- 🔫 **Kho vũ khí đa dạng:**
  - `[1] PISTOL`: Vô hạn đạn, tốc độ bắn linh hoạt.
  - `[2] SHOTGUN`: Bắn tỏa chùm 7 viên đạn uy lực cực lớn ở cự ly gần.
  - `[3] ASSAULT RIFLE`: Tốc độ xả đạn liên thanh dồn dập.
  - `[4] PLASMA CANNON`: Bắn đạn năng lượng xuyên mục tiêu.
  - `[5] RPG`: Bắn tên lửa nổ diện rộng, hất tung kẻ địch.
- 👾 **Kẻ địch & Boss phong phú:**
  - `Swarm Runner`: Quái áp sát bầy đàn tốc độ cao.
  - `Acid Spitter`: Quái giữ khoảng cách và phun cầu độc.
  - `Kamikaze Drone`: Drone phát nổ tự sát.
  - `Brute Tank`: Quái trâu bò tích điện lao húc dữ dội.
  - `Cyber Titan Boss (Mỗi 5 wave)`: Thanh máu lớn, bắn đạn nan quạt xoay tròn và triệu hồi quái đệ tử.
- 🧬 **Hệ thống Nâng cấp Công nghệ (Roguelite):** Sau mỗi Wave, người chơi được lựa chọn 1 trong 3 mô-đun nâng cấp (Đạn nảy tường, Đạn xuyên thấu, Hút máu theo mạng hạ gục, Lướt lửa thiêu đốt...).
- 🔊 **Web Audio API Pure Synthesis:** Không cần tải bất kỳ file mp3/wav ngoài nào, toàn bộ âm thanh (tiếng súng các loại, nổ bom, chém dao, lướt né, cảnh báo) đều được tổng hợp thời gian thực.
- 🎨 **Đồ họa Neon & Hiệu ứng chân thực:** Máu vương trên sàn đấu, vỏ đạn kim loại rơi leng keng, đèn pin rọi theo chuột, rung màn hình (Screen Shake) uy lực.

---

## ⌨️ Phím điều khiển

| Phím | Thao tác |
| :---: | :--- |
| <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd> | Di chuyển chiến thuật 8 hướng |
| <kbd>SPACE</kbd> | Dash / Lướt né đòn (bất tử trong tích tắc) |
| <kbd>R</kbd> | Active Reload (Căn vạch nạp nhanh & nhận Overcharge buff) |
| <kbd>1</kbd> - <kbd>5</kbd> / Cuộn chuột | Đổi vũ khí |
| <kbd>Q</kbd> | Ném lựu đạn nổ chùm (Cluster Grenade) |
| <kbd>E</kbd> | Đặt ụ súng máy tự động (Auto-Turret) |
| <kbd>SHIFT</kbd> | Bullet Time (Làm chậm thời gian Ma Trận) |
| 🖱️ Chuột trái | Ngắm và bắn súng 360° |
| 🖱️ Chuột phải | Chém dao cận chiến (Melee Slash hất lùi quái) |
| <kbd>ESC</kbd> / <kbd>P</kbd> | Tạm dừng trận đấu |

---

## 🚀 Cách chạy game

Game được viết hoàn toàn bằng **HTML5, CSS3 và Vanilla JavaScript**, không cần cài đặt thư viện phụ thuộc (`npm`, `yarn`...).

### Cách 1: Mở trực tiếp
Mở file `index.html` bằng trình duyệt web bất kỳ (Chrome, Edge, Firefox, Safari).

### Cách 2: Chạy qua HTTP Server cục bộ
```bash
# Sử dụng Python 3
python3 -m http.server 8080

# Hoặc dùng npx serve
npx serve .
```
Sau đó truy cập: `http://localhost:8080`

---

## 📁 Cấu trúc dự án
```
topdown-shooter/
├── index.html     # Khung giao diện HTML5 Canvas, HUD và Modal
├── style.css      # Phong cách đồ họa Neon Cyberpunk, bố cục HUD
├── game.js        # Logic game engine, Web Audio synth, AI, vật lý và rendering
└── README.md      # Tài liệu hướng dẫn
```

---
*Created with ❤️ by TruongHoang2004*
