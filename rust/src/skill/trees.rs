use macroquad::prelude::*;
use crate::core::i18n::tr;
use crate::core::types::{PlayerModifiers, UpgradeOption};

/// Thông tin tên và màu sắc biểu trưng của 8 Hệ Nâng Cấp
pub const TREE_META: [(&str, &str, Color); 8] = [
    ("1. TRỌNG PHÁO", "ĐẠN TO", Color::new(1.0, 0.35, 0.2, 1.0)),
    ("2. LIÊN THANH", "BẮN NHANH", Color::new(1.0, 0.85, 0.1, 1.0)),
    ("3. TIẾP VẬN", "NẠP NHANH", Color::new(0.0, 0.95, 0.65, 1.0)),
    ("4. NGUYÊN TỐ", "LỬA/SÉT/BĂNG", Color::new(0.2, 0.85, 1.0, 1.0)),
    ("5. CƠ ĐỘNG", "LƯỚT/THỜI GIAN", Color::new(0.85, 0.35, 1.0, 1.0)),
    ("6. CÔNG NGHỆ", "KHIÊN/DRONE", Color::new(0.35, 1.0, 0.35, 1.0)),
    ("7. TỪ TRƯỜNG", "HÚT KINH NGHIỆM", Color::new(0.0, 1.0, 0.9, 1.0)),
    ("8. ĐA HƯỚNG", "BẮN 360°/PHÒNG THỦ", Color::new(1.0, 0.4, 0.85, 1.0)),
];

pub fn tree_meta_localized(tree_id: usize) -> (&'static str, &'static str, Color) {
    match tree_id {
        0 => (
            tr("1. TRỌNG PHÁO", "1. HEAVY CALIBER"),
            tr("ĐẠN TO", "BIG BULLETS"),
            Color::new(1.0, 0.35, 0.2, 1.0),
        ),
        1 => (
            tr("2. LIÊN THANH", "2. RAPID FIRE"),
            tr("BẮN NHANH", "HIGH ROF"),
            Color::new(1.0, 0.85, 0.1, 1.0),
        ),
        2 => (
            tr("3. TIẾP VẬN", "3. FAST RELOAD"),
            tr("NẠP NHANH", "QUICK MAG"),
            Color::new(0.0, 0.95, 0.65, 1.0),
        ),
        3 => (
            tr("4. NGUYÊN TỐ", "4. ELEMENTAL"),
            tr("LỬA/SÉT/BĂNG", "FIRE/SHOCK/ICE"),
            Color::new(0.2, 0.85, 1.0, 1.0),
        ),
        4 => (
            tr("5. CƠ ĐỘNG", "5. PHANTOM"),
            tr("LƯỚT/THỜI GIAN", "DASH/TIME"),
            Color::new(0.85, 0.35, 1.0, 1.0),
        ),
        5 => (
            tr("6. CÔNG NGHỆ", "6. CYBER AEGIS"),
            tr("KHIÊN/DRONE", "SHIELD/DRONE"),
            Color::new(0.35, 1.0, 0.35, 1.0),
        ),
        6 => (
            tr("7. TỪ TRƯỜNG", "7. MAGNETISM"),
            tr("HÚT KINH NGHIỆM", "EXP MAGNET"),
            Color::new(0.0, 1.0, 0.9, 1.0),
        ),
        _ => (
            tr("8. ĐA HƯỚNG", "8. OMNI-FIRE"),
            tr("BẮN 360°/PHÒNG THỦ", "360° BURST/DEFENSE"),
            Color::new(1.0, 0.4, 0.85, 1.0),
        ),
    }
}

/// Danh sách 28 Kỹ Năng thuộc 7 Hệ (Mỗi hệ: 1 Cấp 1 -> 2 Cấp 2 -> 1 Cấp 3)
pub fn all_tree_nodes() -> Vec<UpgradeOption> {
    vec![
        // =====================================================
        // HỆ 0: TRỌNG PHÁO (HEAVY CALIBER - Đạn To & Công Phá)
        // =====================================================
        UpgradeOption {
            id: "heavy_1",
            tree_id: 0,
            tier: 1,
            category_tag: tr("HỆ TRỌNG PHÁO - CẤP 1", "HEAVY CALIBER - TIER 1"),
            title: tr("ĐẠN CỠ LỚN (CALIBER+)", "HEAVY CALIBER ROUNDS"),
            icon: "[O+]",
            desc: tr(
                "+35% kích thước viên đạn & +20% Sát thương.",
                "+35% Bullet Size & +20% Weapon Damage.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Trọng Pháo Cấp 2",
                "Unlocks Heavy Caliber Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "heavy_2a",
            tree_id: 0,
            tier: 2,
            category_tag: tr("HỆ TRỌNG PHÁO - CẤP 2A", "HEAVY CALIBER - TIER 2A"),
            title: tr("ĐẠN XUYÊN GIÁP (PIERCE+)", "ARMOR-PIERCING CORE"),
            icon: "[>>]",
            desc: tr(
                "Đạn bắn xuyên thủng thêm +2 mục tiêu.",
                "Bullets pierce through +2 additional targets.",
            ),
            req_desc: tr(
                "Yêu cầu: Đạn Cỡ Lớn (Cấp 1)",
                "Requires: Heavy Caliber (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "heavy_2b",
            tree_id: 0,
            tier: 2,
            category_tag: tr("HỆ TRỌNG PHÁO - CẤP 2B", "HEAVY CALIBER - TIER 2B"),
            title: tr("ĐẦU ĐẠN CHẤN ĐỘNG (BLAST)", "CONCUSSIVE WARHEADS"),
            icon: "[*!]",
            desc: tr(
                "Đạn chạm mục tiêu phát nổ lan 55px & đẩy lùi.",
                "Bullets explode on impact (55px AoE) & knockback.",
            ),
            req_desc: tr(
                "Yêu cầu: Đạn Cỡ Lớn (Cấp 1)",
                "Requires: Heavy Caliber (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "heavy_3",
            tree_id: 0,
            tier: 3,
            category_tag: tr("* HỆ TRỌNG PHÁO - CẤP 3 *", "* HEAVY CALIBER - TIER 3 *"),
            title: tr("ĐẠI PHÁO KHỔNG LỒ (COLOSSAL)", "COLOSSAL ORDNANCE"),
            icon: "[*O*]",
            desc: tr(
                "Đạn to gấp 2.2 lần, +40% Sát thương & xuyên phá.",
                "2.2x Bullet Size, +40% Damage & extra pierce.",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG!",
                "KEY PIECE FOR ULTIMATE SYNERGY SETS!",
            ),
        },

        // =====================================================
        // HỆ 1: LIÊN THANH (RAPID FIRE - Bắn Nhanh & Mưa Đạn)
        // =====================================================
        UpgradeOption {
            id: "rapid_1",
            tree_id: 1,
            tier: 1,
            category_tag: tr("HỆ LIÊN THANH - CẤP 1", "RAPID FIRE - TIER 1"),
            title: tr("GIA TỐC NÒNG (RAPID+)", "ACCELERATED RECEIVER"),
            icon: "[~>]",
            desc: tr(
                "+28% Tốc độ bắn (Fire Rate) cho mọi vũ khí.",
                "+28% Fire Rate for all weapons.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Liên Thanh Cấp 2",
                "Unlocks Rapid Fire Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "rapid_2a",
            tree_id: 1,
            tier: 2,
            category_tag: tr("HỆ LIÊN THANH - CẤP 2A", "RAPID FIRE - TIER 2A"),
            title: tr("NÒNG KÉP TÁCH ĐẠN (TWIN)", "TWIN-LINKED BARREL"),
            icon: "[=>]",
            desc: tr(
                "Mỗi phát bắn phóng thêm +1 tia đạn song song.",
                "Every shot fires +1 additional parallel projectile.",
            ),
            req_desc: tr(
                "Yêu cầu: Gia Tốc Nòng (Cấp 1)",
                "Requires: Accelerated Receiver (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "rapid_2b",
            tree_id: 1,
            tier: 2,
            category_tag: tr("HỆ LIÊN THANH - CẤP 2B", "RAPID FIRE - TIER 2B"),
            title: tr("ĐẠN PHẢN XẠ (RICOCHET)", "RICOCHET MATRIX"),
            icon: "[<>]",
            desc: tr(
                "Đạn nảy lại 2 lần khi va chạm tường chắn.",
                "Bullets ricochet up to 2 times off walls.",
            ),
            req_desc: tr(
                "Yêu cầu: Gia Tốc Nòng (Cấp 1)",
                "Requires: Accelerated Receiver (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "rapid_3",
            tree_id: 1,
            tier: 3,
            category_tag: tr("* HỆ LIÊN THANH - CẤP 3 *", "* RAPID FIRE - TIER 3 *"),
            title: tr("BÃO ĐẠN SIÊU THANH (GATLING)", "SUPERSONIC GATLING STORM"),
            icon: "[*~*]",
            desc: tr(
                "+45% Tốc độ bắn & thêm +1 tia đạn mỗi phát!",
                "+45% Fire Rate & +1 extra projectile per shot!",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG!",
                "KEY PIECE FOR ULTIMATE SYNERGY SETS!",
            ),
        },

        // =====================================================
        // HỆ 2: TIẾP VẬN (FAST RELOAD - Nạp Đạn Nhanh & Băng Đạn)
        // =====================================================
        UpgradeOption {
            id: "reload_1",
            tree_id: 2,
            tier: 1,
            category_tag: tr("HỆ TIẾP VẬN - CẤP 1", "FAST RELOAD - TIER 1"),
            title: tr("BĂNG ĐẠN MỞ RỘNG (MAG+)", "EXTENDED DRUM MAG"),
            icon: "[||]",
            desc: tr(
                "+50% Băng đạn tối đa & +25% Tốc độ nạp đạn.",
                "+50% Magazine Capacity & +25% Reload Speed.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Tiếp Vận Cấp 2",
                "Unlocks Fast Reload Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "reload_2a",
            tree_id: 2,
            tier: 2,
            category_tag: tr("HỆ TIẾP VẬN - CẤP 2A", "FAST RELOAD - TIER 2A"),
            title: tr("NẠP ĐẠN THẦN TỐC (QUICK)", "SLEIGHT OF HAND"),
            icon: "[+R]",
            desc: tr(
                "+45% Tốc độ nạp & Vạch Xanh trên đầu rộng gấp đôi!",
                "+45% Reload Speed & 2x wider Active Reload zone!",
            ),
            req_desc: tr(
                "Yêu cầu: Băng Đạn Mở Rộng (Cấp 1)",
                "Requires: Extended Drum Mag (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "reload_2b",
            tree_id: 2,
            tier: 2,
            category_tag: tr("HỆ TIẾP VẬN - CẤP 2B", "FAST RELOAD - TIER 2B"),
            title: tr("THU HỒI ĐẠN (SCAVENGER)", "AMMO SCAVENGER"),
            icon: "[+||]",
            desc: tr(
                "Mỗi quái bị hạ hồi ngay +2 viên vào băng đạn.",
                "Each enemy kill instantly restores +2 rounds to clip.",
            ),
            req_desc: tr(
                "Yêu cầu: Băng Đạn Mở Rộng (Cấp 1)",
                "Requires: Extended Drum Mag (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "reload_3",
            tree_id: 2,
            tier: 3,
            category_tag: tr("* HỆ TIẾP VẬN - CẤP 3 *", "* FAST RELOAD - TIER 3 *"),
            title: tr("VÔ TẬN TIẾP VẬN (OVERFEED)", "ENDLESS OVERFEED"),
            icon: "[*R*]",
            desc: tr(
                "Nạp đạn xong: 3.5s bắn KHÔNG TỐN ĐẠN + Overcharge!",
                "After reload: 3.5s INFINITE AMMO + Overcharge!",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG!",
                "KEY PIECE FOR ULTIMATE SYNERGY SETS!",
            ),
        },

        // =====================================================
        // HỆ 3: NGUYÊN TỐ (ELEMENTAL - Lửa, Băng & Sét)
        // =====================================================
        UpgradeOption {
            id: "elem_1",
            tree_id: 3,
            tier: 1,
            category_tag: tr("HỆ NGUYÊN TỐ - CẤP 1", "ELEMENTAL - TIER 1"),
            title: tr("LÕI NHIỆT PLASMA (BURN)", "PLASMA INCENDIARY CORE"),
            icon: "[LUA]",
            desc: tr(
                "Đạn thiêu đốt quái gây 35 Sát thương/giây trong 3s.",
                "Bullets ignite enemies for 35 Burn DMG/s over 3s.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Nguyên Tố Cấp 2",
                "Unlocks Elemental Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "elem_2a",
            tree_id: 3,
            tier: 2,
            category_tag: tr("HỆ NGUYÊN TỐ - CẤP 2A", "ELEMENTAL - TIER 2A"),
            title: tr("TỤ ĐIỆN CAO THẾ (SHOCK)", "ARC CAPACITOR COIL"),
            icon: "[SET]",
            desc: tr(
                "Đạn bắn trúng phóng sét giật lan 2 mục tiêu.",
                "Bullet hits discharge chain lightning to 2 targets.",
            ),
            req_desc: tr(
                "Yêu cầu: Lõi Nhiệt Plasma (Cấp 1)",
                "Requires: Plasma Incendiary Core (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "elem_2b",
            tree_id: 3,
            tier: 2,
            category_tag: tr("HỆ NGUYÊN TỐ - CẤP 2B", "ELEMENTAL - TIER 2B"),
            title: tr("NITƠ LỎNG CRYO (FREEZE)", "CRYO NITROGEN MATRIX"),
            icon: "[BANG]",
            desc: tr(
                "Đạn đóng băng làm chậm 45% tốc độ chạy của quái.",
                "Bullets chill enemies, slowing movement by 45%.",
            ),
            req_desc: tr(
                "Yêu cầu: Lõi Nhiệt Plasma (Cấp 1)",
                "Requires: Plasma Incendiary Core (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "elem_3",
            tree_id: 3,
            tier: 3,
            category_tag: tr("* HỆ NGUYÊN TỐ - CẤP 3 *", "* ELEMENTAL - TIER 3 *"),
            title: tr("ĐẠI HỒNG THỦY NGUYÊN TỐ", "ELEMENTAL MAELSTROM"),
            icon: "[*E*]",
            desc: tr(
                "Kết hợp Lửa + Băng + Sét giật 4 mục tiêu & +25% DMG!",
                "Combines Fire + Ice + 4-target Chain Lightning & +25% DMG!",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG!",
                "KEY PIECE FOR ULTIMATE SYNERGY SETS!",
            ),
        },

        // =====================================================
        // HỆ 4: CƠ ĐỘNG & HƯ KHÔNG (PHANTOM MOBILITY)
        // =====================================================
        UpgradeOption {
            id: "mob_1",
            tree_id: 4,
            tier: 1,
            category_tag: tr("HỆ CƠ ĐỘNG - CẤP 1", "PHANTOM MOBILITY - TIER 1"),
            title: tr("ĐỘNG CƠ PHẢN LỰC (JET+)", "ION THRUSTER BOOTS"),
            icon: "[>>]",
            desc: tr(
                "+20% Tốc chạy, -25% Hồi chiêu Dash & +40% Tầm hút EXP.",
                "+20% Move Speed, -25% Dash CD & +40% EXP Pickup Range.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Cơ Động Cấp 2",
                "Unlocks Phantom Mobility Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "mob_2a",
            tree_id: 4,
            tier: 2,
            category_tag: tr("HỆ CƠ ĐỘNG - CẤP 2A", "PHANTOM MOBILITY - TIER 2A"),
            title: tr("VẾT LỬA PHÂN THÂN (TRAIL)", "BLAZING AFTERIMAGE"),
            icon: "[~>]",
            desc: tr(
                "Lướt Dash để lại thảm lửa thiêu đốt quái bám đuổi.",
                "Dashing leaves a scorching fire trail behind you.",
            ),
            req_desc: tr(
                "Yêu cầu: Động Cơ Phản Lực (Cấp 1)",
                "Requires: Ion Thruster Boots (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "mob_2b",
            tree_id: 4,
            tier: 2,
            category_tag: tr("HỆ CƠ ĐỘNG - CẤP 2B", "PHANTOM MOBILITY - TIER 2B"),
            title: tr("THẤU THỊ BULLET-TIME", "CHRONO DILATION CORE"),
            icon: "[TIME]",
            desc: tr(
                "+60% Năng lượng làm chậm thời gian [SHIFT] & +20% DMG.",
                "+60% Bullet-Time Energy [SHIFT] & +20% Weapon DMG.",
            ),
            req_desc: tr(
                "Yêu cầu: Động Cơ Phản Lực (Cấp 1)",
                "Requires: Ion Thruster Boots (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "mob_3",
            tree_id: 4,
            tier: 3,
            category_tag: tr("* HỆ CƠ ĐỘNG - CẤP 3 *", "* PHANTOM MOBILITY - TIER 3 *"),
            title: tr("BÓNG MA THỜI KHÔNG (PHANTOM)", "PHANTOM BLADE STEP"),
            icon: "[*P*]",
            desc: tr(
                "Mỗi lần Dash phóng 12 lưỡi kiếm năng lượng 360 độ!",
                "Every Dash unleashes 12 energy blades in a 360° nova!",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG!",
                "KEY PIECE FOR ULTIMATE SYNERGY SETS!",
            ),
        },

        // =====================================================
        // HỆ 5: CÔNG NGHỆ & SINH TỒN (CYBER AEGIS)
        // =====================================================
        UpgradeOption {
            id: "aegis_1",
            tree_id: 5,
            tier: 1,
            category_tag: tr("HỆ CÔNG NGHỆ - CẤP 1", "CYBER AEGIS - TIER 1"),
            title: tr("GIÁP NANO HÚT MÁU (VAMP)", "NANO SIPHON PLATING"),
            icon: "[+HP]",
            desc: tr(
                "+45 Max Shield & Hạ quái hồi +3 HP/Shield.",
                "+45 Max Shield & Enemy kills restore +3 HP/Shield.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Công Nghệ Cấp 2",
                "Unlocks Cyber Aegis Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "aegis_2a",
            tree_id: 5,
            tier: 2,
            category_tag: tr("HỆ CÔNG NGHỆ - CẤP 2A", "CYBER AEGIS - TIER 2A"),
            title: tr("QUÁ TẢI CÔNG TRÌNH (TURRET+)", "OVERCLOCKED SENTRY"),
            icon: "[+T]",
            desc: tr(
                "Tăng gấp đôi tốc bắn của Ụ Súng & gọi ngay 1 Turret.",
                "Doubles Turret Fire Rate & deploys 1 Sentry Turret now.",
            ),
            req_desc: tr(
                "Yêu cầu: Giáp Nano Hút Máu (Cấp 1)",
                "Requires: Nano Siphon Plating (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "aegis_2b",
            tree_id: 5,
            tier: 2,
            category_tag: tr("HỆ CÔNG NGHỆ - CẤP 2B", "CYBER AEGIS - TIER 2B"),
            title: tr("KHIÊN PHẢN PHÁO (REFLECT)", "REACTIVE EMP AEGIS"),
            icon: "[EMP]",
            desc: tr(
                "Khi bị trúng đạn, phóng Vòng Xung Kích & Sét trả đũa.",
                "Taking hits triggers a retaliatory EMP Shockwave & Arc.",
            ),
            req_desc: tr(
                "Yêu cầu: Giáp Nano Hút Máu (Cấp 1)",
                "Requires: Nano Siphon Plating (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "aegis_3",
            tree_id: 5,
            tier: 3,
            category_tag: tr("* HỆ CÔNG NGHỆ - CẤP 3 *", "* CYBER AEGIS - TIER 3 *"),
            title: tr("TƯ LỆNH BINH ĐOÀN (OMEGA)", "OMEGA DRONE COMMANDER"),
            icon: "[*D*]",
            desc: tr(
                "Gọi 2 Drone Hộ Vệ vĩnh viễn & +60 Max HP/Shield!",
                "Summons 2 permanent Attack Drones & +60 Max HP/Shield!",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG!",
                "KEY PIECE FOR ULTIMATE SYNERGY SETS!",
            ),
        },

        // =====================================================
        // HỆ 6: TỪ TRƯỜNG THU HỒI (MAGNETIC HARVESTER - Hút EXP Rộng)
        // =====================================================
        UpgradeOption {
            id: "magnet_1",
            tree_id: 6,
            tier: 1,
            category_tag: tr("HỆ TỪ TRƯỜNG - CẤP 1", "MAGNETISM - TIER 1"),
            title: tr("NAM CHÂM LƯỢNG TỬ (MAGNET+)", "QUANTUM MAGNET COIL"),
            icon: "[((U))]",
            desc: tr(
                "+85% Phạm vi hút Kinh Nghiệm (EXP) & +15% EXP nhận được.",
                "+85% EXP Pickup Radius & +15% EXP Gain.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Từ Trường Cấp 2",
                "Unlocks Magnetism Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "magnet_2a",
            tree_id: 6,
            tier: 2,
            category_tag: tr("HỆ TỪ TRƯỜNG - CẤP 2A", "MAGNETISM - TIER 2A"),
            title: tr("TRƯỜNG HẤP DẪN SIÊU CẤP", "GRAVITON ATTRACTOR"),
            icon: "[((O))]",
            desc: tr(
                "+120% Phạm vi hút EXP. Mỗi viên EXP nhặt hồi +2 Shield!",
                "+120% EXP Pickup Radius. Each EXP orb restores +2 Shield!",
            ),
            req_desc: tr(
                "Yêu cầu: Nam Châm Lượng Tử (Cấp 1)",
                "Requires: Quantum Magnet Coil (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "magnet_2b",
            tree_id: 6,
            tier: 2,
            category_tag: tr("HỆ TỪ TRƯỜNG - CẤP 2B", "MAGNETISM - TIER 2B"),
            title: tr("TINH THỂ CỘNG HƯỞNG ĐIỆN", "RESONANT EXP CRYSTALS"),
            icon: "[+EXP]",
            desc: tr(
                "+75% Phạm vi hút EXP & +30% EXP. Nhặt EXP giật sét quái gần!",
                "+75% Pickup Radius & +30% EXP. Orbs zap nearby enemies!",
            ),
            req_desc: tr(
                "Yêu cầu: Nam Châm Lượng Tử (Cấp 1)",
                "Requires: Quantum Magnet Coil (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "magnet_3",
            tree_id: 6,
            tier: 3,
            category_tag: tr("* HỆ TỪ TRƯỜNG - CẤP 3 *", "* MAGNETISM - TIER 3 *"),
            title: tr("HỐ ĐEN THU HỒI TOÀN CẦU", "GLOBAL SINGULARITY HARVESTER"),
            icon: "[*U*]",
            desc: tr(
                "+220% Phạm vi hút EXP cực rộng & +35% Sát thương vũ khí!",
                "+220% Massive EXP Pickup Radius & +35% Weapon Damage!",
            ),
            req_desc: tr(
                "MẢNH GHÉP THU HỒI TỐI THƯỢNG!",
                "ULTIMATE HARVESTER KEYSTONE!",
            ),
        },

        // =====================================================
        // HỆ 7: ĐA HƯỚNG & HỎA LỰC PHÒNG THỦ (OMNI-DIRECTIONAL & DEFENSE)
        // =====================================================
        UpgradeOption {
            id: "omni_1",
            tree_id: 7,
            tier: 1,
            category_tag: tr("HỆ ĐA HƯỚNG - CẤP 1", "OMNI-FIRE - TIER 1"),
            title: tr("PHÁO ĐUÔI PHẢN LỰC (REAR CANNON)", "REAR THRUSTER CANNONS"),
            icon: "[<<-]",
            desc: tr(
                "Bắn thêm 1 luồng đạn nặng ra sau lưng (180°) đẩy lùi quái & +15% Sát thương.",
                "Fires heavy shots backwards (180°) knocking back pursuers & +15% DMG.",
            ),
            req_desc: tr(
                "Mở khóa nhánh Đa Hướng Cấp 2",
                "Unlocks Omni-Fire Tier 2 branch",
            ),
        },
        UpgradeOption {
            id: "omni_2a",
            tree_id: 7,
            tier: 2,
            category_tag: tr("HỆ ĐA HƯỚNG - CẤP 2A", "OMNI-FIRE - TIER 2A"),
            title: tr("PHÁO MẠN CÁNH TẢ HỮU (FLANK BATTERY)", "TWIN FLANK BATTERIES"),
            icon: "[<^>]",
            desc: tr(
                "Mỗi phát bắn xả thêm 2 luồng đạn sang 2 bên mạn sườn (±90°) & +20% Tốc bắn.",
                "Fires twin projectiles to both flanks (±90°) & +20% Fire Rate.",
            ),
            req_desc: tr(
                "Yêu cầu: Pháo Đuôi Phản Lực (Cấp 1)",
                "Requires: Rear Thruster Cannons (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "omni_2b",
            tree_id: 7,
            tier: 2,
            category_tag: tr("HỆ ĐA HƯỚNG - CẤP 2B", "OMNI-FIRE - TIER 2B"),
            title: tr("CẦU PLASMA HỘ THỂ (ORBITAL PLASMAS)", "ORBITAL PLASMA SPHERES"),
            icon: "[*O*]",
            desc: tr(
                "Triệu hồi 3 quả cầu Plasma xoay quanh người chơi thiêu đốt mọi quái áp sát!",
                "Summons 3 orbiting Plasma Spheres incinerating enemies in contact range!",
            ),
            req_desc: tr(
                "Yêu cầu: Pháo Đuôi Phản Lực (Cấp 1)",
                "Requires: Rear Thruster Cannons (Tier 1)",
            ),
        },
        UpgradeOption {
            id: "omni_3",
            tree_id: 7,
            tier: 3,
            category_tag: tr("* HỆ ĐA HƯỚNG - CẤP 3 *", "* OMNI-FIRE - TIER 3 *"),
            title: tr("BÃO ĐẠN HOA SEN 360° (OMNI STARBURST)", "OMNI STARBURST MATRIX"),
            icon: "[*8*]",
            desc: tr(
                "Cứ mỗi 4 phát bắn bung tỏa 8 hướng toàn diện (360°) với đạn nổ lan 55px!",
                "Every 4th shot unleashes an 8-way 360° starburst barrage with 55px blast!",
            ),
            req_desc: tr(
                "MẢNH GHÉP MỞ KHÓA BỘ TỐI THƯỢNG ĐA HƯỚNG!",
                "KEY PIECE FOR OMNI ULTIMATE SETS!",
            ),
        },
    ]
}

/// Danh sách 12 Bộ Kỹ Năng Tối Thượng (12 Ultimate Synergy Sets)
/// Mỗi phần tử gồm: (Điều kiện các ID Cấp 3 cần có, Thông tin Thẻ Tối Thượng)
pub fn all_ultimate_sets() -> Vec<(&'static [&'static str], UpgradeOption)> {
    vec![
        // 1. Đạn To (3) + Bắn Nhanh (3) + Nạp Đạn Nhanh (3)
        (
            &["heavy_3", "rapid_3", "reload_3"],
            UpgradeOption {
                id: "ult_apocalypse_gatling",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 1/12 (3 HỆ) **", "** ULTIMATE SET 1/12 (3 TREES) **"),
                title: tr("THẦN PHÁO VÔ TẬN (APOCALYPSE)", "APOCALYPSE GATLING"),
                icon: "[ULT-1]",
                desc: tr(
                    "Đạn Siêu To x3 + Tốc bắn Cực Đại + VÔ HẠN ĐẠN + Nổ Lan!",
                    "3x Colossal Rounds + Max Fire Rate + INFINITE AMMO + Blast!",
                ),
                req_desc: tr(
                    "Set: Trọng Pháo 3 + Liên Thanh 3 + Tiếp Vận 3",
                    "Set: Heavy 3 + Rapid 3 + Reload 3",
                ),
            },
        ),
        // 2. Đạn To (3) + Bắn Nhanh (3) + Nguyên Tố (3)
        (
            &["heavy_3", "rapid_3", "elem_3"],
            UpgradeOption {
                id: "ult_cataclysm_barrage",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 2/12 (3 HỆ) **", "** ULTIMATE SET 2/12 (3 TREES) **"),
                title: tr("SIÊU BÃO NGUYÊN TỐ (CATACLYSM)", "CATACLYSM BARRAGE"),
                icon: "[ULT-2]",
                desc: tr(
                    "Đạn bắn trúng tự động giáng Thiên Thạch & Bão Sét!",
                    "Bullet hits automatically call down Meteors & Lightning!",
                ),
                req_desc: tr(
                    "Set: Trọng Pháo 3 + Liên Thanh 3 + Nguyên Tố 3",
                    "Set: Heavy 3 + Rapid 3 + Elemental 3",
                ),
            },
        ),
        // 3. Nạp Nhanh (3) + Cơ Động (3) + Công Nghệ (3)
        (
            &["reload_3", "mob_3", "aegis_3"],
            UpgradeOption {
                id: "ult_chrono_legion",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 3/12 (3 HỆ) **", "** ULTIMATE SET 3/12 (3 TREES) **"),
                title: tr("THỜI KHÔNG QUÂN ĐOÀN (LEGION)", "CHRONO MECH LEGION"),
                icon: "[ULT-3]",
                desc: tr(
                    "-55% Hồi chiêu kỹ năng & gọi Robot Mech mỗi khi nạp đạn!",
                    "-55% Skill Cooldowns & spawns a Combat Mech on reload!",
                ),
                req_desc: tr(
                    "Set: Tiếp Vận 3 + Cơ Động 3 + Công Nghệ 3",
                    "Set: Reload 3 + Mobility 3 + Cyber Aegis 3",
                ),
            },
        ),
        // 4. Đạn To (3) + Nguyên Tố (3)
        (
            &["heavy_3", "elem_3"],
            UpgradeOption {
                id: "ult_hellfire_artillery",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 4/12 **", "** ULTIMATE SET 4/12 **"),
                title: tr("PHÁO ĐÀI ĐỊA NGỤC (HELLFIRE)", "HELLFIRE ARTILLERY"),
                icon: "[ULT-4]",
                desc: tr(
                    "Đạn cỡ lớn tạo hồ dung nham cháy rực & nổ nhiệt 110px!",
                    "Heavy rounds create burning lava pools & 110px thermal blasts!",
                ),
                req_desc: tr(
                    "Set: Trọng Pháo 3 + Nguyên Tố 3",
                    "Set: Heavy 3 + Elemental 3",
                ),
            },
        ),
        // 5. Bắn Nhanh (3) + Nguyên Tố (3)
        (
            &["rapid_3", "elem_3"],
            UpgradeOption {
                id: "ult_volt_overdrive",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 5/12 **", "** ULTIMATE SET 5/12 **"),
                title: tr("TIA CHỚP VĨNH CỬU (OVERDRIVE)", "VOLT OVERDRIVE"),
                icon: "[ULT-5]",
                desc: tr(
                    "Mỗi 4 phát bắn phóng Tia Sét Dây Chuyền giật 8 mục tiêu!",
                    "Every 4th shot unleashes an 8-target Chain Lightning arc!",
                ),
                req_desc: tr(
                    "Set: Liên Thanh 3 + Nguyên Tố 3",
                    "Set: Rapid 3 + Elemental 3",
                ),
            },
        ),
        // 6. Bắn Nhanh (3) + Nạp Nhanh (3)
        (
            &["rapid_3", "reload_3"],
            UpgradeOption {
                id: "ult_bullet_hurricane",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 6/12 **", "** ULTIMATE SET 6/12 **"),
                title: tr("BÃO ĐẠN VÔ CỰC (HURRICANE)", "BULLET HURRICANE"),
                icon: "[ULT-6]",
                desc: tr(
                    "+2 tia đạn mỗi phát & tự phóng Vòng Đạn Xoay 360 độ!",
                    "+2 projectiles per shot & periodic 360° Bullet Spirals!",
                ),
                req_desc: tr(
                    "Set: Liên Thanh 3 + Tiếp Vận 3",
                    "Set: Rapid 3 + Reload 3",
                ),
            },
        ),
        // 7. Đạn To (3) + Công Nghệ (3)
        (
            &["heavy_3", "aegis_3"],
            UpgradeOption {
                id: "ult_dreadnought_titan",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 7/12 **", "** ULTIMATE SET 7/12 **"),
                title: tr("PHÁO ĐÀI THIẾT GIÁP (TITAN)", "DREADNOUGHT TITAN"),
                icon: "[ULT-7]",
                desc: tr(
                    "+120 HP/Shield & Vai giáp tự bắn 2 Tên Lửa Tầm Nhiệt!",
                    "+120 HP/Shield & Shoulder Pods auto-fire 2 Homing Rockets!",
                ),
                req_desc: tr(
                    "Set: Trọng Pháo 3 + Công Nghệ 3",
                    "Set: Heavy 3 + Cyber Aegis 3",
                ),
            },
        ),
        // 8. Bắn Nhanh (3) + Cơ Động (3)
        (
            &["rapid_3", "mob_3"],
            UpgradeOption {
                id: "ult_phantom_trigger",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 8/12 **", "** ULTIMATE SET 8/12 **"),
                title: tr("XẠ THỦ BÓNG MA (PHANTOM)", "PHANTOM TRIGGER"),
                icon: "[ULT-8]",
                desc: tr(
                    "+35% Tốc chạy; mỗi khi Dash tự xả 16 viên đạn & Overcharge!",
                    "+35% Speed; Dashing fires a 16-round burst & Overcharge!",
                ),
                req_desc: tr(
                    "Set: Liên Thanh 3 + Cơ Động 3",
                    "Set: Rapid 3 + Mobility 3",
                ),
            },
        ),
        // 9. Nạp Nhanh (3) + Nguyên Tố (3)
        (
            &["reload_3", "elem_3"],
            UpgradeOption {
                id: "ult_hyper_reactor",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 9/12 **", "** ULTIMATE SET 9/12 **"),
                title: tr("LÒ PHẢN ỨNG SIÊU NẠP (REACTOR)", "HYPER REACTOR"),
                icon: "[ULT-9]",
                desc: tr(
                    "Nạp đạn & mỗi 3.5s phát nổ Vòng Xung Kích EMP + Lửa!",
                    "Reloading & every 3.5s triggers a massive EMP + Fire Nova!",
                ),
                req_desc: tr(
                    "Set: Tiếp Vận 3 + Nguyên Tố 3",
                    "Set: Reload 3 + Elemental 3",
                ),
            },
        ),
        // 10. Nạp Nhanh (3) + Công Nghệ (3)
        (
            &["reload_3", "aegis_3"],
            UpgradeOption {
                id: "ult_autonomous_arsenal",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 10/12 **", "** ULTIMATE SET 10/12 **"),
                title: tr("CHIẾN GIÁP TỰ HÀNH (AUTONOMOUS)", "AUTONOMOUS ARSENAL"),
                icon: "[ULT-10]",
                desc: tr(
                    "Hết đạn TỰ ĐỘNG NẠP TỨC THÌ (0s) & gọi 2 Ụ Súng Laser!",
                    "INSTANT 0s AUTO-RELOAD on empty clip & spawns 2 Laser Turrets!",
                ),
                req_desc: tr(
                    "Set: Tiếp Vận 3 + Công Nghệ 3",
                    "Set: Reload 3 + Cyber Aegis 3",
                ),
            },
        ),
        // 11. Nguyên Tố (3) + Cơ Động (3)
        (
            &["elem_3", "mob_3"],
            UpgradeOption {
                id: "ult_singularity_walker",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 11/12 **", "** ULTIMATE SET 11/12 **"),
                title: tr("BƯỚC NHẢY LƯỢNG TỬ (SINGULARITY)", "SINGULARITY WALKER"),
                icon: "[ULT-11]",
                desc: tr(
                    "Mỗi khi Dash hoặc hạ 12 quái tạo Hố Đen hút gom quái!",
                    "Dashing or every 12 kills spawns a gravity Black Hole!",
                ),
                req_desc: tr(
                    "Set: Nguyên Tố 3 + Cơ Động 3",
                    "Set: Elemental 3 + Mobility 3",
                ),
            },
        ),
        // 12. Đạn To (3) + Cơ Động (3)
        (
            &["heavy_3", "mob_3"],
            UpgradeOption {
                id: "ult_valkyrie_impact",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 12/16 **", "** ULTIMATE SET 12/16 **"),
                title: tr("ĐỘT KÍCH XUNG KÍCH (VALKYRIE)", "VALKYRIE IMPACT"),
                icon: "[ULT-12]",
                desc: tr(
                    "Lướt Dash nghiền nát quái & tăng +65% Sát thương Chí Mạng!",
                    "Dashing crushes enemies in path & grants +65% Weapon DMG!",
                ),
                req_desc: tr(
                    "Set: Trọng Pháo 3 + Cơ Động 3",
                    "Set: Heavy 3 + Mobility 3",
                ),
            },
        ),
        // 13. Đa Hướng (3) + Nguyên Tố (3)
        (
            &["omni_3", "elem_3"],
            UpgradeOption {
                id: "ult_orbital_supernova",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 13/16 **", "** ULTIMATE SET 13/16 **"),
                title: tr("SIÊU TÂN TINH QUỸ ĐẠO (SUPERNOVA)", "ORBITAL SUPERNOVA"),
                icon: "[ULT-13]",
                desc: tr(
                    "Tăng lên 6 Cầu Plasma quay siêu tốc & mỗi 2.8s kích nổ Siêu Tân Tinh 350px!",
                    "Spawns 6 High-speed Plasmas & triggers 350px Supernova blast every 2.8s!",
                ),
                req_desc: tr(
                    "Set: Đa Hướng 3 + Nguyên Tố 3",
                    "Set: Omni 3 + Elemental 3",
                ),
            },
        ),
        // 14. Đa Hướng (3) + Bắn Nhanh (3) + Đạn To (3)
        (
            &["omni_3", "rapid_3", "heavy_3"],
            UpgradeOption {
                id: "ult_omni_annihilation",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 14/16 (3 HỆ) **", "** ULTIMATE SET 14/16 (3 TREES) **"),
                title: tr("MA TRẬN HỦY DIỆT TOÀN DIỆN (ANNIHILATION)", "OMNI ANNIHILATION MATRIX"),
                icon: "[ULT-14]",
                desc: tr(
                    "MỌI PHÁT BẮN bắn đồng thời 16 hướng 360° với đạn cỡ lớn xuyên phá nổ cực mạnh!",
                    "EVERY SHOT unleashes a 16-way 360° barrage with Colossal piercing blast rounds!",
                ),
                req_desc: tr(
                    "Set: Đa Hướng 3 + Liên Thanh 3 + Trọng Pháo 3",
                    "Set: Omni 3 + Rapid 3 + Heavy 3",
                ),
            },
        ),
        // 15. Đa Hướng (3) + Cơ Động (3) + Từ Trường (3)
        (
            &["omni_3", "mob_3", "magnet_3"],
            UpgradeOption {
                id: "ult_chronos_singularity",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 15/16 (3 HỆ) **", "** ULTIMATE SET 15/16 (3 TREES) **"),
                title: tr("HỐ ĐEN THỜI KHÔNG (SINGULARITY)", "CHRONOS SINGULARITY"),
                icon: "[ULT-15]",
                desc: tr(
                    "Mỗi 3.5s triệu hồi Hố Đen Không Thời Gian (hút 450px) gom nghiền nát ngàn quái!",
                    "Every 3.5s spawns a 450px Supermassive Black Hole crushing thousands of enemies!",
                ),
                req_desc: tr(
                    "Set: Đa Hướng 3 + Cơ Động 3 + Từ Trường 3",
                    "Set: Omni 3 + Mobility 3 + Magnetism 3",
                ),
            },
        ),
        // 16. Đa Hướng (3) + Nguyên Tố (3) + Công Nghệ (3)
        (
            &["omni_3", "elem_3", "aegis_3"],
            UpgradeOption {
                id: "ult_divine_wrath",
                tree_id: 99,
                tier: 4,
                category_tag: tr("** BỘ TỐI THƯỢNG 16/16 (3 HỆ) **", "** ULTIMATE SET 16/16 (3 TREES) **"),
                title: tr("THIÊN PHẠT SẤM TRUYỀN (DIVINE WRATH)", "DIVINE LIGHTNING MATRIX"),
                icon: "[ULT-16]",
                desc: tr(
                    "Liên tục giáng Thiên Phạt sấm sét xuống 6 mục tiêu mỗi 0.25s & phóng giáp sét phản đòn!",
                    "Calls down continuous Thunderbolts on 6 targets every 0.25s & shock armor!",
                ),
                req_desc: tr(
                    "Set: Đa Hướng 3 + Nguyên Tố 3 + Công Nghệ 3",
                    "Set: Omni 3 + Elemental 3 + Cyber Aegis 3",
                ),
            },
        ),
    ]
}

/// Kiểm tra xem người chơi còn thẻ nâng cấp Cây Kỹ Năng hoặc Bộ Tối Thượng nào chưa mở khóa không
pub fn has_available_unique_upgrades(mods: &PlayerModifiers) -> bool {
    // 1. Còn Bộ Tối Thượng nào đã đủ điều kiện nhưng chưa chọn không?
    for (reqs, ult_opt) in all_ultimate_sets() {
        if !mods.has(ult_opt.id) && reqs.iter().all(|&r| mods.has(r)) {
            return true;
        }
    }
    // 2. Còn Node nào trong 8 Hệ mà người chơi đủ điều kiện nhận không?
    for node in all_tree_nodes() {
        if mods.has(node.id) {
            continue;
        }
        let current_tier = mods.tree_tier(node.tree_id);
        let eligible = match node.tier {
            1 => current_tier == 0,
            2 => current_tier >= 1,
            3 => current_tier >= 2,
            _ => false,
        };
        if eligible {
            return true;
        }
    }
    false
}

/// Rút ngẫu nhiên `count` (4) thẻ nâng cấp hợp lệ dựa trên tiến độ Cây Kỹ Năng hiện tại của người chơi
pub fn roll_upgrade_choices(mods: &PlayerModifiers, count: usize) -> Vec<UpgradeOption> {
    let mut result: Vec<UpgradeOption> = Vec::new();

    // 1. Kiểm tra các Bộ Kỹ Năng Tối Thượng (12 Ultimate Sets) đã đủ điều kiện mở khóa chưa
    let mut unlocked_ultimates: Vec<UpgradeOption> = Vec::new();
    for (reqs, ult_opt) in all_ultimate_sets() {
        if !mods.has(ult_opt.id) && reqs.iter().all(|&r| mods.has(r)) {
            unlocked_ultimates.push(ult_opt);
        }
    }

    // Ưu tiên đưa tối đa 2 Thẻ Tối Thượng vào danh sách chọn ngay khi vừa đủ bộ!
    while !unlocked_ultimates.is_empty() && result.len() < 2 {
        let idx = rand::gen_range(0, unlocked_ultimates.len());
        result.push(unlocked_ultimates.swap_remove(idx));
    }

    // 2. Lọc các Node hợp lệ trong 7 Hệ Nâng Cấp (Tier 1 -> Tier 2A/2B -> Tier 3)
    let mut eligible_nodes: Vec<UpgradeOption> = Vec::new();
    for node in all_tree_nodes() {
        if mods.has(node.id) {
            continue;
        }
        let current_tier = mods.tree_tier(node.tree_id);
        let eligible = match node.tier {
            1 => current_tier == 0,
            2 => current_tier >= 1,
            3 => current_tier >= 2,
            _ => false,
        };
        if eligible {
            eligible_nodes.push(node);
        }
    }

    // Ưu tiên trọng số cao hơn cho các thẻ Cấp 2 và Cấp 3 của hệ người chơi đã đầu tư
    eligible_nodes.sort_by_key(|n| {
        let bonus = if n.tier == 3 {
            0
        } else if n.tier == 2 {
            1
        } else {
            2
        };
        bonus + rand::gen_range(0, 4)
    });

    for node in eligible_nodes {
        if result.len() >= count {
            break;
        }
        result.push(node);
    }

    // 3. Dự phòng nếu người chơi đã mở khóa gần hết toàn bộ cây kỹ năng
    let fallbacks = [
        UpgradeOption {
            id: "boost_dmg",
            tree_id: 0,
            tier: 1,
            category_tag: tr("CƯỜNG HÓA VŨ KHÍ", "WEAPON AUGMENT"),
            title: tr("LÕI HỎA LỰC (+25% DMG)", "FIREPOWER CORE (+25% DMG)"),
            icon: "[+DMG]",
            desc: tr(
                "Tăng vĩnh viễn +25% Sát thương cho vũ khí.",
                "Permanently increases Weapon Damage by +25%.",
            ),
            req_desc: tr("Cộng dồn không giới hạn", "Stacks infinitely"),
        },
        UpgradeOption {
            id: "boost_rof",
            tree_id: 1,
            tier: 1,
            category_tag: tr("CƯỜNG HÓA TỐC BẮN", "FIRE RATE AUGMENT"),
            title: tr("BỘ GIA TỐC (+20% ROF)", "OVERCLOCKED TRIGGER (+20% ROF)"),
            icon: "[+ROF]",
            desc: tr(
                "Tăng vĩnh viễn +20% Tốc độ bắn cho vũ khí.",
                "Permanently increases Weapon Fire Rate by +20%.",
            ),
            req_desc: tr("Cộng dồn không giới hạn", "Stacks infinitely"),
        },
        UpgradeOption {
            id: "boost_hp",
            tree_id: 5,
            tier: 1,
            category_tag: tr("CƯỜNG HÓA GIÁP", "ARMOR AUGMENT"),
            title: tr("GIÁP TITAN (+50 HP & SHIELD)", "TITAN PLATING (+50 HP & SHIELD)"),
            icon: "[+HP]",
            desc: tr(
                "Tăng +50 Máu & Giáp tối đa, hồi đầy ngay lập tức.",
                "Grants +50 Max HP & Shield and fully restores both.",
            ),
            req_desc: tr("Cộng dồn không giới hạn", "Stacks infinitely"),
        },
        UpgradeOption {
            id: "boost_spd",
            tree_id: 4,
            tier: 1,
            category_tag: tr("CƯỜNG HÓA CƠ ĐỘNG", "MOBILITY AUGMENT"),
            title: tr("ĐỘNG CƠ ION (+15% SPEED)", "ION DRIVE (+15% SPEED)"),
            icon: "[+SPD]",
            desc: tr(
                "Tăng +15% Tốc độ di chuyển & hồi đầy Thể lực.",
                "Grants +15% Move Speed & refills Stamina.",
            ),
            req_desc: tr("Cộng dồn không giới hạn", "Stacks infinitely"),
        },
        UpgradeOption {
            id: "boost_magnet",
            tree_id: 6,
            tier: 1,
            category_tag: tr("CƯỜNG HÓA TỪ TRƯỜNG", "MAGNET AUGMENT"),
            title: tr("KHUẾCH ĐẠI NAM CHÂM (+60% HÚT EXP)", "MAGNET AMPLIFIER (+60% RANGE)"),
            icon: "[+MAG]",
            desc: tr(
                "Tăng +60% Phạm vi hút Kinh Nghiệm & +15% EXP nhận được.",
                "Increases EXP Pickup Range by +60% & EXP Gain by +15%.",
            ),
            req_desc: tr("Cộng dồn không giới hạn", "Stacks infinitely"),
        },
    ];

    let mut fb_idx = 0;
    while result.len() < count {
        result.push(fallbacks[fb_idx % fallbacks.len()].clone());
        fb_idx += 1;
    }

    result
}
