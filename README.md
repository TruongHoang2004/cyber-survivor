# ⚡ CYBER SURVIVOR: Grand-Scale 2D Tactical Action Shooter (Rust + Web)

> Một tựa game hành động bắn súng góc nhìn từ trên xuống (2D Top-Down Tactical Action Shooter) với các trận đại chiến quy mô lớn, được thiết kế theo **kiến trúc hướng Trait (Trait-Based Modular Architecture)** bằng **Rust (`macroquad`)** và phiên bản Web (**HTML5 Canvas + Web Audio**).

---

## 🔥 Tính năng nổi bật (Phiên bản Rust — Grand Battle Edition)

- ⚔️ **Trận chiến quy mô lớn (Grand-Scale Battles):**
  - Biển quái vật xuất hiện theo từng tiểu đội vây ráp từ mọi hướng.
  - **Tiểu đội Đồng Minh AI (`AlliedBehavior`)**: Lính Thủy Quân Lục Chiến (`Allied Marine`), Robot Hạng Nặng (`Heavy Mech`) bắn pháo kép + tên lửa, và Drone Hộ Vệ (`Orbiting Combat Drone`) bay quanh người chơi.
  - **Thùng nhiên liệu nổ dây chuyền (`Explosive Barrel`)**: Tạo hiệu ứng nổ Domino quét sạch cả góc bản đồ.
- 🎁 **Hệ thống Nhặt Súng Dưới Đất & Độ Hiếm (`WeaponDrop` & `WeaponRarity`):**
  - Khởi đầu với súng lục cơ bản `[1] PISTOL`.
  - Khoang tiếp tế từ quỹ đạo (`Orbital Supply Pod`), Quái Tinh Anh và các bộ phận Boss khi bị phá hủy sẽ rơi ra hòm vũ khí phát sáng theo 4 cấp độ hiếm: **Common**, **Rare**, **Epic**, **Legendary**.
  - Đi qua hoặc bấm `[F]` để nhặt và mở khóa súng vào các ô `[1]`–`[7]`. Nhặt trùng súng đã có sẽ **Nâng Cấp Level (`Lv.2`, `Lv.3+`)** tăng mạnh sát thương, số tia đạn và tốc độ bắn!
- 🔫 **7 Vũ Khí Hạng Nặng & Hủy Diệt (`WeaponBehavior`):**
  - `[1] PISTOL` — Súng lục chiến thuật vô hạn đạn.
  - `[2] SHOTGUN` — Súng hoa cải bắn tỏa chùm đạn (tăng số viên theo Level).
  - `[3] VULCAN MINIGUN` — Súng máy 6 nòng tốc độ xả đạn siêu thanh.
  - `[4] TESLA CHAIN-GUN` — Súng phóng hồ quang điện giật dây chuyền qua nhiều mục tiêu kèm làm choáng.
  - `[5] ORBITAL RAILGUN` — Pháo điện từ bắn tia năng lượng xuyên thấu toàn bộ kẻ địch trên đường thẳng.
  - `[6] SINGULARITY CANNON` — Pháo bắn ra **Hố Đen Vũ Trụ (`SingularityBlackHoleEvent`)** hút mọi quái vật xung quanh vào tâm rồi phát nổ.
  - `[7] CLUSTER RPG` — Tên lửa hạng nặng nổ mảnh bom chùm diện rộng.
- 👾 **Đại Boss Pháo Đài Nhiều Bộ Phận (`ColossalBehemothBoss` & `BossPartBehavior`):**
  - Boss khổng lồ sở hữu các mô-đun vũ khí độc lập gắn quanh thân: Tay Pháo Cối (`Mortar Artillery`), Tháp Súng Gatling (`Gatling Turret`), Lõi Khiên Năng Lượng (`Shield Generator`), và Khoang Ấp Drone (`Drone Hatchery`).
  - Người chơi có thể bắn nổ từng bộ phận để tước vũ khí của Boss và nhặt súng phẩm chất cao rơi ra ngay trong trận đánh.
- ⚙️ **Menu Chiến Thuật & Bảng Tuỳ Chỉnh Cài Đặt (`GameSettings`):**
  - Điều chỉnh trực tiếp bằng thanh trượt kéo chuột (`Slider`) hoặc nút bật/tắt (`Toggle`) ngay trong trận (`[ESC]` hoặc `[O]`):
    - **Độ rung màn hình (`Screen Shake`)**: Dao động điều hòa mượt (`0%` tắt hẳn đến `100%`, mặc định `15%`).
    - **Mật độ hiệu ứng hạt cháy nổ (`VFX Density`)**: Giữ tầm nhìn luôn rõ ràng trong đại chiến.
    - **Độ sáng lưới nền (`Grid Opacity`)** & **Quy mô đàn quái (`Swarm Scale`: `50%` – `200%`)**.
    - Bật/tắt **Số sát thương (`Damage Numbers`)**, **Tự động nhặt súng (`Auto-Pickup`)**, **Tự động cầm súng mới (`Auto-Equip`)**, và **Thanh máu đồng minh (`Ally HP Bars`)**.

---

## 🧱 Kiến trúc Trừu Tượng Hóa (Rust Trait Architecture)

Toàn bộ các thực thể và sự kiện có tính chất tương đồng được trừu tượng hóa qua các `Trait` kèm cơ chế hàng đợi lệnh `CombatContext` + `WorldCommand`:

| Trait | File | Vai trò & Các lớp triển khai |
| :--- | :--- | :--- |
| `BattlefieldEvent` | `rust/src/core/events.rs` | Các sự kiện không gian/thời gian trên chiến trường: `ShockwaveRingEvent`, `SingularityBlackHoleEvent`, `CarpetBombingRunEvent`, `OrbitalLaserStrikeEvent`, `MeteorWarningStrikeEvent`, `BeamArcVisualEvent`, `SupplyPodDropEvent` |
| `AlliedBehavior` | `rust/src/ally/mod.rs` | Đơn vị đồng minh AI: `AlliedMarine`, `AlliedHeavyMech`, `OrbitingCombatDrone` |
| `BossPartBehavior` | `rust/src/enemy/traits.rs` | Các bộ phận độc lập trên thân Boss: `MortarArtilleryArmPart`, `GatlingLaserTurretPart`, `ShieldGeneratorPart`, `DroneHatcheryBayPart` |
| `EnemyBehavior` | `rust/src/enemy/traits.rs` | Hành vi AI quái vật: `SwarmRunner`, `AcidSpitter`, `KamikazeDrone`, `BruteTank`, `PhantomAssassin`, `NecroSummoner`, `ColossalBehemothBoss` |
| `WeaponBehavior` | `rust/src/weapon/traits.rs` | Hành vi khai hỏa vũ khí: `StandardBallisticBehavior`, `BuckshotSpreadBehavior`, `TeslaChainBehavior`, `RailgunBeamBehavior`, `SingularityCannonBehavior`, `ClusterRocketBehavior` |
| `SkillBehavior` | `rust/src/skill/traits.rs` | Kỹ năng kích hoạt: `TacticalDashSkill` (`[SPACE]`), `TacticalAirstrikeSkill` (`[Q]`), `ReinforcementsDropSkill` (`[E]`) |
| `CharacterClass` | `rust/src/character/traits.rs` | Lớp nhân vật người chơi: `CyberMarine`, `VoidStalker`, `TechEngineer` |

---

## ⌨️ Phím điều khiển

| Phím | Thao tác |
| :---: | :--- |
| <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd> | Di chuyển chiến thuật 8 hướng |
| 🖱️ Chuột trái / phải | Bắn vũ khí 360° / Chém dao cận chiến đẩy lùi |
| <kbd>1</kbd> – <kbd>7</kbd> | Chuyển đổi giữa 7 loại vũ khí đã mở khóa |
| <kbd>F</kbd> | Nhặt súng rơi dưới đất (hoặc tự động nhặt khi lại gần) |
| <kbd>R</kbd> | Active Reload (Bấm đúng vạch xanh nhận **+40% Overcharge DMG**) |
| <kbd>SPACE</kbd> | Lướt chiến thuật (Dash I-frames + để lại mìn xung kích) |
| <kbd>SHIFT</kbd> | Bullet Time (Làm chậm thời gian chiến trường) |
| <kbd>Q</kbd> | **Tactical Airstrike**: Rải thảm bom liên hoàn + Tia Laser Quỹ Đạo |
| <kbd>E</kbd> | **Reinforcements Drop**: Gọi Lính Thủy Quân + Robot Hạng Nặng + Drone + Khoang Tiếp Tế |
| <kbd>G</kbd> | Gọi ngay một Khoang Tiếp Tế Súng (`Orbital Supply Pod`) tại vị trí chuột |
| <kbd>B</kbd> | Triệu hồi ngay Đại Boss Pháo Đài (`Colossal Behemoth Boss`) |
| <kbd>ESC</kbd> / <kbd>P</kbd> / <kbd>O</kbd> | Tạm dừng trận đấu & Mở Bảng Tuỳ Chỉnh Cài Đặt (`Settings`) |

---

## 🚀 Cách chạy game

### 🦀 Phiên bản Rust Native (Khuyên dùng — 60-120+ FPS)

```bash
cd rust

# Build và chạy bản tối ưu hóa cao nhất (Release)
cargo run --release
```

### 🌐 Phiên bản Web (HTML5 Canvas + Web Audio)

```bash
python3 -m http.server 8080
```
Sau đó mở trình duyệt tại `http://localhost:8080` hoặc trải nghiệm trên [GitHub Pages](https://truonghoang2004.github.io/cyber-survivor/).

---

## 📁 Cấu trúc thư mục

```text
topdown-shooter/
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # Entry point & vòng lặp game
│       ├── ally/              # Trait AlliedBehavior & các đơn vị đồng minh AI
│       ├── character/         # Trait CharacterClass & thông số Player
│       ├── core/              # GameWorld, CombatContext, BattlefieldEvent, Props, GameSettings
│       ├── enemy/             # Trait EnemyBehavior, BossPartBehavior, Quái & Đại Boss
│       ├── skill/             # Trait SkillBehavior, Không kích, Tiếp viện & Deployables
│       ├── ui/                # HUD chiến trường, Start/Pause Menu & Settings Modal
│       └── weapon/            # Trait WeaponBehavior, 7 loại súng & đạn đạo
├── index.html                 # Phiên bản Web HTML5 Canvas
├── style.css
├── game.js
└── README.md
```

---
*Created with ❤️ by TruongHoang2004*
