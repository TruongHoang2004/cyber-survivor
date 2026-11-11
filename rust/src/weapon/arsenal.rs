use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::i18n::tr;
use crate::core::props::WeaponRarity;
use crate::core::types::{PlayerModifiers, StatusEffect};
use crate::weapon::projectile::Projectile;
use crate::weapon::traits::{Weapon, WeaponBehavior, WeaponStats};

pub fn weapon_name_by_id(id: usize) -> &'static str {
    weapon_form_name(id, 1)
}

pub fn evolved_weapon_name_by_id(id: usize) -> &'static str {
    weapon_form_name(id, 100)
}

/// Trả về tên Form tương ứng với cấp độ (1-99: Dạng 1, 100-199: Dạng 2, 200-299: Dạng 3, 300-399: Dạng 4, 400-499: Dạng 5, 500: Tối Thượng)
pub fn weapon_form_name(id: usize, level: u32) -> &'static str {
    match id {
        0 => {
            // PISTOL
            if level >= 500 {
                tr("THẦN PHÁO VÔ CỰC OMEGA [TỐI THƯỢNG]", "OMEGA INFINITY [ULTIMATE]")
            } else if level >= 400 {
                tr("PHÁN QUYẾT HƯ KHÔNG [FORM 5]", "VOID EXECUTIONER [FORM 5]")
            } else if level >= 300 {
                tr("SIÊU PHÁO HYPER REVOLVER [FORM 4]", "HYPER REVOLVER [FORM 4]")
            } else if level >= 200 {
                tr("PHÁO PLASMA XUNG KÍCH [FORM 3]", "PLASMA PULSE [FORM 3]")
            } else if level >= 100 {
                tr("SONG PHÁO MAGNUM [FORM 2]", "TWIN MAGNUM [FORM 2]")
            } else {
                tr("LỤC ĐIỆN TỪ PISTOL", "CYBER PISTOL")
            }
        }
        1 => {
            // SHOTGUN
            if level >= 500 {
                tr("NHẬT THỰC THÁI DƯƠNG [TỐI THƯỢNG]", "SOLAR ECLIPSE [ULTIMATE]")
            } else if level >= 400 {
                tr("SIÊU BÃO HỎA CẢI [FORM 5]", "SUPERCELL FLAK [FORM 5]")
            } else if level >= 300 {
                tr("PHÁO LỬA ĐỊA NGỤC [FORM 4]", "HELLFIRE CANNON [FORM 4]")
            } else if level >= 200 {
                tr("PHÁ GIÁP TITAN [FORM 3]", "TITAN BREACHER [FORM 3]")
            } else if level >= 100 {
                tr("HỎA LONG SHOTGUN [FORM 2]", "DRAGON FLAK [FORM 2]")
            } else {
                tr("PHÁO HOA CẢI SHOTGUN", "COMBAT SHOTGUN")
            }
        }
        2 => {
            // VULCAN MINIGUN
            if level >= 500 {
                tr("CUỒNG PHONG THỜI KHÔNG [TỐI THƯỢNG]", "CHRONO TYPHOON [ULTIMATE]")
            } else if level >= 400 {
                tr("TẬN THẾ CƠ GIỚI [FORM 5]", "CYBER DOOMSDAY [FORM 5]")
            } else if level >= 300 {
                tr("SỨ GIẢ BÃO TỐ [FORM 4]", "STORM BRINGER [FORM 4]")
            } else if level >= 200 {
                tr("GATLING CỰC HẠN [FORM 3]", "GATLING APEX [FORM 3]")
            } else if level >= 100 {
                tr("SIÊU BÃO VULCAN [FORM 2]", "HYPER-VULCAN [FORM 2]")
            } else {
                tr("SÚNG MÁY 6 NÒNG VULCAN", "VULCAN MINIGUN")
            }
        }
        3 => {
            // TESLA
            if level >= 500 {
                tr("THIÊN KIẾP THẦN SÉT ZEUS [TỐI THƯỢNG]", "ZEUS CATACLYSM [ULTIMATE]")
            } else if level >= 400 {
                tr("LÔI KIẾP HOÀNG HÔN [FORM 5]", "RAGNAROK LIGHTNING [FORM 5]")
            } else if level >= 300 {
                tr("BÃO ION SIÊU DẪN [FORM 4]", "IONIC TEMPEST [FORM 4]")
            } else if level >= 200 {
                tr("MA TRẬN ĐIỆN ÁP [FORM 3]", "VOLTAIC MATRIX [FORM 3]")
            } else if level >= 100 {
                tr("LÔI THẦN TESLA [FORM 2]", "THUNDER TESLA [FORM 2]")
            } else {
                tr("SÚNG PHÓNG SÉT TESLA", "TESLA CHAIN-GUN")
            }
        }
        4 => {
            // RAILGUN
            if level >= 500 {
                tr("KHỞI NGUYÊN DIỆT VONG [TỐI THƯỢNG]", "GENESIS OBLIVION [ULTIMATE]")
            } else if level >= 400 {
                tr("TIA SÁNG TUYỆT CHỦNG [FORM 5]", "EXTINCTION RAY [FORM 5]")
            } else if level >= 300 {
                tr("MÁY GIA TỐC HẠT [FORM 4]", "PARTICLE ACCELERATOR [FORM 4]")
            } else if level >= 200 {
                tr("NGŨ HÀNH PHÁO QUANG [FORM 3]", "QUINT BEAM [FORM 3]")
            } else if level >= 100 {
                tr("TAM XÀ RAILGUN [FORM 2]", "TRIPLE RAIL [FORM 2]")
            } else {
                tr("PHÁO ĐIỆN TỪ RAILGUN", "ORBITAL RAILGUN")
            }
        }
        5 => {
            // SINGULARITY
            if level >= 500 {
                tr("VỤ NỔ LỚN TÁI SINH [TỐI THƯỢNG]", "BIG BANG REBIRTH [ULTIMATE]")
            } else if level >= 400 {
                tr("TINH VÂN SỤP ĐỔ [FORM 5]", "NEBULA COLLAPSE [FORM 5]")
            } else if level >= 300 {
                tr("XOÁY LỐC VŨ TRỤ [FORM 4]", "COSMIC VORTEX [FORM 4]")
            } else if level >= 200 {
                tr("CHÂN TRỜI SỰ KIỆN [FORM 3]", "EVENT HORIZON [FORM 3]")
            } else if level >= 100 {
                tr("SIÊU TÂN TINH [FORM 2]", "SUPERNOVA [FORM 2]")
            } else {
                tr("PHÁO HỐ ĐEN SINGULARITY", "SINGULARITY CANNON")
            }
        }
        6 => {
            // CLUSTER RPG
            if level >= 500 {
                tr("NGÀY PHÁN XÉT ARMAGEDDON [TỐI THƯỢNG]", "ARMAGEDDON MIRV [ULTIMATE]")
            } else if level >= 400 {
                tr("PHÁO KÍCH QUỸ ĐẠO [FORM 5]", "ORBITAL BOMBARDMENT [FORM 5]")
            } else if level >= 300 {
                tr("ĐẦU ĐẠN NHIỆT HẠCH [FORM 4]", "NUCLEAR TITAN [FORM 4]")
            } else if level >= 200 {
                tr("MƯA TÊN LỬA TẬN THẾ [FORM 3]", "APOCALYPSE BARRAGE [FORM 3]")
            } else if level >= 100 {
                tr("PHÁO ĐÀI TÊN LỬA [FORM 2]", "MIRV LAUNCHER [FORM 2]")
            } else {
                tr("TÊN LỬA CHÙM CLUSTER RPG", "CLUSTER RPG")
            }
        }
        _ => tr("VŨ KHÍ TỐI THƯỢNG", "ULTIMATE WEAPON"),
    }
}

fn resolve_status(mods: &PlayerModifiers) -> Option<StatusEffect> {
    if mods.incendiary_rounds {
        Some(StatusEffect::Burn {
            dps: 22.0,
            duration: 3.0,
        })
    } else if mods.cryo_rounds {
        Some(StatusEffect::Slow {
            factor: 0.45,
            duration: 2.5,
        })
    } else {
        None
    }
}

fn calc_damage(stats: &WeaponStats, mods: &PlayerModifiers, overcharged: bool) -> f32 {
    let level_bonus = 1.0 + ((stats.level.saturating_sub(1)) as f32) * 0.009;
    let rarity_bonus = stats.rarity.damage_bonus();
    let mut dmg = stats.damage * mods.dmg_multi * level_bonus * rarity_bonus;
    if overcharged {
        dmg *= 1.4;
    }
    dmg
}

fn bullet_color(stats: &WeaponStats, mods: &PlayerModifiers, overcharged: bool) -> Color {
    if overcharged {
        Color::new(0.0, 1.0, 0.4, 1.0)
    } else if mods.incendiary_rounds {
        Color::new(1.0, 0.35, 0.0, 1.0)
    } else if mods.cryo_rounds {
        Color::new(0.3, 0.85, 1.0, 1.0)
    } else if stats.level >= 500 {
        Color::new(0.3, 1.0, 0.85, 1.0)
    } else if stats.level >= 400 {
        Color::new(1.0, 0.25, 0.35, 1.0)
    } else if stats.level >= 300 {
        Color::new(1.0, 0.85, 0.1, 1.0)
    } else if stats.level >= 200 {
        Color::new(0.8, 0.3, 1.0, 1.0)
    } else {
        stats.color
    }
}

// =========================================================
// 1. STANDARD / MINIGUN BALLISTIC BEHAVIOR (Lv.1 -> Lv.500 [TỐI THƯỢNG])
// =========================================================
#[derive(Clone)]
pub struct StandardBallisticBehavior;

impl WeaponBehavior for StandardBallisticBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let base_streams = if stats.level >= 500 {
            6
        } else if stats.level >= 400 {
            5
        } else if stats.level >= 300 {
            4
        } else if stats.level >= 200 {
            3
        } else if stats.level >= 100 {
            2
        } else {
            1
        };
        let streams = base_streams + mods.multishot_bonus;
        let dmg = calc_damage(stats, mods, overcharged);
        let col = bullet_color(stats, mods, overcharged);
        let evo_pierce = if stats.level >= 500 {
            6
        } else if stats.level >= 400 {
            4
        } else if stats.level >= 300 {
            3
        } else if stats.level >= 200 {
            2
        } else if stats.level >= 100 {
            1
        } else {
            0
        };
        let evo_bounce = if stats.level >= 500 {
            3
        } else if stats.level >= 300 {
            2
        } else if stats.level >= 100 {
            1
        } else {
            0
        };
        let base_rad = if stats.level >= 500 {
            6.5
        } else if stats.level >= 300 {
            5.4
        } else if stats.level >= 100 {
            4.6
        } else {
            3.8
        };
        let blast = if stats.level >= 500 {
            Some(95.0)
        } else if stats.level >= 400 {
            Some(75.0)
        } else if stats.level >= 300 {
            Some(60.0)
        } else if stats.level >= 200 {
            Some(48.0)
        } else if stats.level >= 100 {
            Some(36.0)
        } else if mods.impact_blast_radius > 0.0 {
            Some(mods.impact_blast_radius)
        } else {
            None
        };

        for s in 0..streams {
            let offset_angle = if streams > 1 {
                ((s as f32) - ((streams - 1) as f32) * 0.5) * 0.065
            } else {
                0.0
            };
            let angle = aim_angle + offset_angle + rand::gen_range(-stats.spread, stats.spread);
            let dir = vec2(angle.cos(), angle.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + dir * 24.0,
                vel: dir * stats.speed,
                damage: dmg,
                color: col,
                radius: base_rad * mods.bullet_size_multi,
                explosion_radius: blast,
                spawns_black_hole: false,
                pierce: mods.pierce_count + evo_pierce,
                bounces: (if mods.bouncing_bullets { 2 } else { 0 }) + evo_bounce,
                life: 2.2,
                status_effect: resolve_status(mods),
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 2. BUCKSHOT SPREAD BEHAVIOR (Shotgun -> Lv.500 [TỐI THƯỢNG])
// =========================================================
#[derive(Clone)]
pub struct BuckshotSpreadBehavior {
    pub base_pellets: u32,
}

impl WeaponBehavior for BuckshotSpreadBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let extra_pellets = if stats.level >= 500 {
            44
        } else if stats.level >= 400 {
            32
        } else if stats.level >= 300 {
            24
        } else if stats.level >= 200 {
            16
        } else if stats.level >= 100 {
            8
        } else {
            (stats.level.saturating_sub(1)) / 12
        };
        let pellets = self.base_pellets + extra_pellets + mods.multishot_bonus * 2;
        let dmg = calc_damage(stats, mods, overcharged);
        let col = if stats.level >= 500 {
            Color::new(1.0, 0.9, 0.2, 1.0)
        } else if stats.level >= 400 {
            Color::new(1.0, 0.2, 0.35, 1.0)
        } else if stats.level >= 200 {
            Color::new(1.0, 0.45, 0.05, 1.0)
        } else if stats.level >= 100 {
            Color::new(1.0, 0.6, 0.1, 1.0)
        } else {
            bullet_color(stats, mods, overcharged)
        };
        let status = if stats.level >= 500 {
            Some(StatusEffect::Burn {
                dps: 180.0,
                duration: 4.5,
            })
        } else if stats.level >= 400 {
            Some(StatusEffect::Burn {
                dps: 110.0,
                duration: 3.8,
            })
        } else if stats.level >= 300 {
            Some(StatusEffect::Burn {
                dps: 80.0,
                duration: 3.5,
            })
        } else if stats.level >= 200 {
            Some(StatusEffect::Burn {
                dps: 55.0,
                duration: 3.2,
            })
        } else if stats.level >= 100 {
            Some(StatusEffect::Burn {
                dps: 38.0,
                duration: 3.0,
            })
        } else {
            resolve_status(mods)
        };
        let base_rad = if stats.level >= 500 {
            5.8
        } else if stats.level >= 300 {
            4.8
        } else if stats.level >= 100 {
            4.0
        } else {
            3.2
        };
        let blast = if stats.level >= 500 {
            Some(90.0)
        } else if stats.level >= 400 {
            Some(75.0)
        } else if stats.level >= 300 {
            Some(60.0)
        } else if stats.level >= 200 {
            Some(50.0)
        } else if stats.level >= 100 {
            Some(40.0)
        } else if mods.impact_blast_radius > 0.0 {
            Some(mods.impact_blast_radius * 0.75)
        } else {
            None
        };
        let pierce = mods.pierce_count
            + (if stats.level >= 500 {
                4
            } else if stats.level >= 300 {
                3
            } else if stats.level >= 200 {
                2
            } else if stats.level >= 100 {
                1
            } else {
                0
            });

        for _ in 0..pellets {
            let angle = aim_angle + rand::gen_range(-stats.spread, stats.spread);
            let dir = vec2(angle.cos(), angle.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + dir * 24.0,
                vel: dir * stats.speed * rand::gen_range(0.9, 1.12),
                damage: dmg,
                color: col,
                radius: base_rad * mods.bullet_size_multi,
                explosion_radius: blast,
                spawns_black_hole: false,
                pierce,
                bounces: if stats.level >= 400 || mods.bouncing_bullets { 2 } else { 0 },
                life: 1.45,
                status_effect: status,
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 3. TESLA CHAIN-LIGHTNING BEHAVIOR (Sét Liên Hoàn -> Lv.500 [TỐI THƯỢNG])
// =========================================================
#[derive(Clone)]
pub struct TeslaChainBehavior {
    pub base_jumps: usize,
}

impl WeaponBehavior for TeslaChainBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let (arcs, jumps, jump_range) = if stats.level >= 500 {
            (6, 55, 540.0)
        } else if stats.level >= 400 {
            (5, 38, 480.0)
        } else if stats.level >= 300 {
            (4, 28, 430.0)
        } else if stats.level >= 200 {
            (3, 20, 380.0)
        } else if stats.level >= 100 {
            (2, 14, 340.0)
        } else {
            (1, self.base_jumps + ((stats.level as usize).saturating_sub(1)) / 10, 310.0)
        };
        let dmg = calc_damage(stats, mods, overcharged);

        if arcs > 1 {
            let total_arcs = arcs + mods.multishot_bonus as usize;
            for i in 0..total_arcs {
                let offset = ((i as f32) - ((total_arcs - 1) as f32) * 0.5) * 0.12;
                let a = aim_angle + offset;
                let dir = vec2(a.cos(), a.sin());
                ctx.chain_lightning(origin + dir * 20.0, dir, jumps, jump_range, dmg * 0.92);
            }
        } else {
            let dir = vec2(aim_angle.cos(), aim_angle.sin());
            ctx.chain_lightning(origin + dir * 20.0, dir, jumps, jump_range, dmg);
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 4. ORBITAL RAILGUN BEHAVIOR (Tia Điện Từ -> Lv.500 [TỐI THƯỢNG])
// =========================================================
#[derive(Clone)]
pub struct RailgunBeamBehavior;

impl WeaponBehavior for RailgunBeamBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let dir = vec2(aim_angle.cos(), aim_angle.sin());
        let dmg = calc_damage(stats, mods, overcharged);
        let (beams, beam_spread, base_w) = if stats.level >= 500 {
            (9, 0.12, 60.0)
        } else if stats.level >= 400 {
            (7, 0.10, 50.0)
        } else if stats.level >= 300 {
            (5, 0.08, 42.0)
        } else if stats.level >= 200 {
            (4, 0.07, 36.0)
        } else if stats.level >= 100 {
            (3, 0.06, 30.0)
        } else {
            (1, 0.0, 24.0)
        };
        let width = (base_w + (stats.level as f32) * 0.12) * mods.bullet_size_multi;

        if beams > 1 {
            let total_beams = beams + mods.multishot_bonus * 2;
            for i in 0..total_beams {
                let offset_angle = ((i as f32) - ((total_beams - 1) as f32) * 0.5) * beam_spread;
                let b_angle = aim_angle + offset_angle;
                let b_dir = vec2(b_angle.cos(), b_angle.sin());
                let start = origin + b_dir * 22.0;
                let end = start + b_dir * 1700.0;
                ctx.railgun_strike(start, end, width * 0.85, dmg * 0.82);
            }
        } else {
            let start = origin + dir * 22.0;
            let end = origin + dir * 1500.0;
            ctx.railgun_strike(start, end, width, dmg);
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 5. SINGULARITY CANNON BEHAVIOR (Súng Hố Đen -> Lv.500 [TỐI THƯỢNG])
// =========================================================
#[derive(Clone)]
pub struct SingularityCannonBehavior;

impl WeaponBehavior for SingularityCannonBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let dmg = calc_damage(stats, mods, overcharged);
        let (count, pull_rad, bullet_rad) = if stats.level >= 500 {
            (5, 360.0, 20.0)
        } else if stats.level >= 400 {
            (4, 300.0, 18.0)
        } else if stats.level >= 300 {
            (3, 260.0, 16.0)
        } else if stats.level >= 200 {
            (3, 220.0, 14.0)
        } else if stats.level >= 100 {
            (2, 180.0, 12.0)
        } else {
            (1, 130.0 + (stats.level as f32) * 0.5, 10.0)
        };

        let total_count = count + mods.multishot_bonus;
        for i in 0..total_count {
            let spread_offset = if total_count > 1 {
                ((i as f32) - ((total_count - 1) as f32) * 0.5) * 0.16
            } else {
                0.0
            };
            let a = aim_angle + spread_offset;
            let d = vec2(a.cos(), a.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + d * 24.0,
                vel: d * stats.speed,
                damage: dmg,
                color: if stats.level >= 500 {
                    Color::new(0.3, 1.0, 0.9, 1.0)
                } else if stats.level >= 300 {
                    Color::new(0.9, 0.2, 1.0, 1.0)
                } else {
                    Color::new(0.75, 0.2, 1.0, 1.0)
                },
                radius: bullet_rad,
                explosion_radius: Some(pull_rad),
                spawns_black_hole: true,
                pierce: 0,
                bounces: 0,
                life: 2.4,
                status_effect: None,
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 6. EXPLOSIVE ROCKET BEHAVIOR (RPG -> Lv.500 [TỐI THƯỢNG])
// =========================================================
#[derive(Clone)]
pub struct ExplosiveRocketBehavior {
    pub blast_radius: f32,
}

impl WeaponBehavior for ExplosiveRocketBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let (rockets, blast) = if stats.level >= 500 {
            (10, 360.0)
        } else if stats.level >= 400 {
            (8, 300.0)
        } else if stats.level >= 300 {
            (6, 260.0)
        } else if stats.level >= 200 {
            (4, 220.0)
        } else if stats.level >= 100 {
            (3, 190.0)
        } else {
            (1, self.blast_radius + (stats.level as f32) * 0.5)
        };

        let total_rockets = rockets + mods.multishot_bonus;
        let dmg = calc_damage(stats, mods, overcharged);
        let col = bullet_color(stats, mods, overcharged);

        for r in 0..total_rockets {
            let spread_offset = if total_rockets > 1 {
                ((r as f32) - ((total_rockets - 1) as f32) * 0.5) * 0.12
            } else {
                0.0
            };
            let angle =
                aim_angle + spread_offset + rand::gen_range(-stats.spread, stats.spread);
            let dir = vec2(angle.cos(), angle.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + dir * 24.0,
                vel: dir * stats.speed,
                damage: dmg,
                color: col,
                radius: if stats.level >= 500 { 12.0 } else { 8.0 },
                explosion_radius: Some(blast),
                spawns_black_hole: false,
                pierce: 0,
                bounces: 0,
                life: 2.6,
                status_effect: resolve_status(mods),
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

pub fn default_arsenal() -> Vec<Weapon> {
    vec![
        // [1] PISTOL - Mở khóa sẵn từ đầu
        Weapon::new(
            WeaponStats {
                name: "PISTOL",
                damage: 30.0,
                fire_rate: 0.15,
                spread: 0.04,
                speed: 1180.0,
                clip_size: 16,
                current_clip: 16,
                reserve_ammo: -1,
                reload_duration: 1.0,
                color: Color::new(0.0, 0.94, 1.0, 1.0),
                screen_shake: 2.0,
                unlocked: true,
                level: 1,
                rarity: WeaponRarity::Common,
            },
            Box::new(StandardBallisticBehavior),
        ),
        // [2] SHOTGUN - Khẩu pháo hoa cải cận chiến chùm rộng
        Weapon::new(
            WeaponStats {
                name: "SHOTGUN",
                damage: 24.0,
                fire_rate: 0.52,
                spread: 0.30,
                speed: 1020.0,
                clip_size: 10,
                current_clip: 10,
                reserve_ammo: -1,
                reload_duration: 1.45,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                screen_shake: 6.5,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Common,
            },
            Box::new(BuckshotSpreadBehavior { base_pellets: 8 }),
        ),
        // [3] VULCAN MINIGUN - Súng máy 6 nòng xả đạn cực nhanh
        Weapon::new(
            WeaponStats {
                name: "VULCAN MINIGUN",
                damage: 20.0,
                fire_rate: 0.045,
                spread: 0.14,
                speed: 1350.0,
                clip_size: 80,
                current_clip: 80,
                reserve_ammo: -1,
                reload_duration: 1.75,
                color: Color::new(1.0, 0.88, 0.15, 1.0),
                screen_shake: 2.2,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Rare,
            },
            Box::new(StandardBallisticBehavior),
        ),
        // [4] TESLA CHAIN-GUN - Phóng sét nhảy qua 6-15 mục tiêu
        Weapon::new(
            WeaponStats {
                name: "TESLA CHAIN-GUN",
                damage: 48.0,
                fire_rate: 0.30,
                spread: 0.02,
                speed: 1000.0,
                clip_size: 18,
                current_clip: 18,
                reserve_ammo: -1,
                reload_duration: 1.45,
                color: Color::new(0.25, 0.9, 1.0, 1.0),
                screen_shake: 4.5,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Epic,
            },
            Box::new(TeslaChainBehavior { base_jumps: 6 }),
        ),
        // [5] ORBITAL RAILGUN - Tia chớp xuyên thủng toàn bản đồ
        Weapon::new(
            WeaponStats {
                name: "ORBITAL RAILGUN",
                damage: 240.0,
                fire_rate: 0.72,
                spread: 0.0,
                speed: 2000.0,
                clip_size: 6,
                current_clip: 6,
                reserve_ammo: -1,
                reload_duration: 1.7,
                color: Color::new(0.0, 1.0, 0.85, 1.0),
                screen_shake: 12.0,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Epic,
            },
            Box::new(RailgunBeamBehavior),
        ),
        // [6] SINGULARITY CANNON - Bắn ra Hố Đen hút hàng trăm quái vật
        Weapon::new(
            WeaponStats {
                name: "SINGULARITY",
                damage: 125.0,
                fire_rate: 1.05,
                spread: 0.01,
                speed: 560.0,
                clip_size: 3,
                current_clip: 3,
                reserve_ammo: -1,
                reload_duration: 2.0,
                color: Color::new(0.75, 0.2, 1.0, 1.0),
                screen_shake: 14.0,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Legendary,
            },
            Box::new(SingularityCannonBehavior),
        ),
        // [7] CLUSTER RPG - Tên lửa hạng nặng nổ lan diện rộng
        Weapon::new(
            WeaponStats {
                name: "CLUSTER RPG",
                damage: 220.0,
                fire_rate: 0.80,
                spread: 0.01,
                speed: 720.0,
                clip_size: 4,
                current_clip: 4,
                reserve_ammo: -1,
                reload_duration: 1.9,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                screen_shake: 15.0,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Rare,
            },
            Box::new(ExplosiveRocketBehavior {
                blast_radius: 155.0,
            }),
        ),
    ]
}
