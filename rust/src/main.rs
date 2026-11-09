use macroquad::prelude::*;
use std::f32::consts::PI;

const WORLD_WIDTH: f32 = 2400.0;
const WORLD_HEIGHT: f32 = 2400.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "CYBER SURVIVOR (Rust Native High-Performance 2D Shooter)".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Playing,
    Upgrade,
    Paused,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Runner,
    Spitter,
    Kamikaze,
    Brute,
    Boss,
}

#[derive(Clone, Copy, PartialEq)]
enum PickupType {
    Medkit,
    Ammo,
    Nuke,
    Overcharge,
}

#[derive(Clone)]
struct Weapon {
    name: &'static str,
    damage: f32,
    fire_rate: f32,
    spread: f32,
    speed: f32,
    pellets: u32,
    clip_size: u32,
    current_clip: u32,
    reserve_ammo: i32,
    reload_duration: f32,
    is_rocket: bool,
    pierce: u32,
    color: Color,
    screen_shake: f32,
}

impl Weapon {
    fn default_arsenal() -> Vec<Weapon> {
        vec![
            Weapon {
                name: "PISTOL",
                damage: 28.0,
                fire_rate: 0.16,
                spread: 0.04,
                speed: 1150.0,
                pellets: 1,
                clip_size: 15,
                current_clip: 15,
                reserve_ammo: -1, // Infinite
                reload_duration: 1.1,
                is_rocket: false,
                pierce: 0,
                color: Color::new(0.0, 0.94, 1.0, 1.0),
                screen_shake: 2.0,
            },
            Weapon {
                name: "SHOTGUN",
                damage: 19.0,
                fire_rate: 0.65,
                spread: 0.28,
                speed: 980.0,
                pellets: 7,
                clip_size: 8,
                current_clip: 8,
                reserve_ammo: 48,
                reload_duration: 1.8,
                is_rocket: false,
                pierce: 0,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                screen_shake: 6.0,
            },
            Weapon {
                name: "RIFLE",
                damage: 23.0,
                fire_rate: 0.09,
                spread: 0.08,
                speed: 1300.0,
                pellets: 1,
                clip_size: 30,
                current_clip: 30,
                reserve_ammo: 180,
                reload_duration: 1.4,
                is_rocket: false,
                pierce: 0,
                color: Color::new(1.0, 0.84, 0.0, 1.0),
                screen_shake: 2.5,
            },
            Weapon {
                name: "PLASMA",
                damage: 52.0,
                fire_rate: 0.28,
                spread: 0.02,
                speed: 850.0,
                pellets: 1,
                clip_size: 18,
                current_clip: 18,
                reserve_ammo: 72,
                reload_duration: 1.6,
                is_rocket: false,
                pierce: 2,
                color: Color::new(0.66, 0.33, 0.97, 1.0),
                screen_shake: 4.0,
            },
            Weapon {
                name: "RPG",
                damage: 200.0,
                fire_rate: 0.9,
                spread: 0.01,
                speed: 680.0,
                pellets: 1,
                clip_size: 4,
                current_clip: 4,
                reserve_ammo: 12,
                reload_duration: 2.2,
                is_rocket: true,
                pierce: 0,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                screen_shake: 14.0,
            },
        ]
    }
}

struct Perks {
    bouncing_bullets: bool,
    pierce_count: u32,
    vampiric: bool,
    dash_fire_trail: bool,
    dmg_multi: f32,
    dash_cd_multi: f32,
    turret_overclock: bool,
}

struct Player {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    angle: f32,
    speed: f32,
    hp: f32,
    max_hp: f32,
    shield: f32,
    max_shield: f32,
    shield_regen_timer: f32,
    stamina: f32,
    max_stamina: f32,
    dash_cd: f32,
    dash_max_cd: f32,
    is_dashing: bool,
    dash_duration: f32,
    bullet_time: bool,
    active_overcharge: bool,
    grenade_cd: f32,
    grenade_max_cd: f32,
    turret_cd: f32,
    turret_max_cd: f32,
    perks: Perks,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    damage: f32,
    color: Color,
    radius: f32,
    is_rocket: bool,
    explosion_radius: f32,
    pierce: u32,
    bounces: u32,
    life: f32,
}

struct EnemyBullet {
    pos: Vec2,
    vel: Vec2,
    damage: f32,
    radius: f32,
    color: Color,
    life: f32,
}

struct Enemy {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    hp: f32,
    max_hp: f32,
    speed: f32,
    color: Color,
    enemy_type: EnemyType,
    shoot_timer: f32,
    charge_timer: f32,
    is_charging: bool,
    charge_angle: f32,
    boss_attack_timer: f32,
    boss_phase: usize,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    radius: f32,
    life: f32,
    max_life: f32,
}

struct Decal {
    pos: Vec2,
    radius: f32,
    color: Color,
    life: f32,
    is_damage_zone: bool,
}

struct Casing {
    pos: Vec2,
    vel: Vec2,
    rotation: f32,
    v_rot: f32,
    life: f32,
}

struct Turret {
    pos: Vec2,
    angle: f32,
    fire_timer: f32,
    life: f32,
}

struct Grenade {
    pos: Vec2,
    vel: Vec2,
    fuse: f32,
    radius: f32,
}

struct Pickup {
    pos: Vec2,
    pickup_type: PickupType,
    radius: f32,
    pulse: f32,
}

struct FloatText {
    pos: Vec2,
    text: String,
    color: Color,
    size: f32,
    life: f32,
}

struct ActiveReload {
    is_reloading: bool,
    timer: f32,
    duration: f32,
}

#[derive(Clone)]
struct UpgradeOption {
    id: &'static str,
    title: &'static str,
    icon: &'static str,
    desc: &'static str,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::Start;

    let mut player = Player {
        pos: vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0),
        vel: Vec2::ZERO,
        radius: 18.0,
        angle: 0.0,
        speed: 320.0,
        hp: 100.0,
        max_hp: 100.0,
        shield: 50.0,
        max_shield: 50.0,
        shield_regen_timer: 0.0,
        stamina: 100.0,
        max_stamina: 100.0,
        dash_cd: 0.0,
        dash_max_cd: 1.2,
        is_dashing: false,
        dash_duration: 0.0,
        bullet_time: false,
        active_overcharge: false,
        grenade_cd: 0.0,
        grenade_max_cd: 8.0,
        turret_cd: 0.0,
        turret_max_cd: 16.0,
        perks: Perks {
            bouncing_bullets: false,
            pierce_count: 0,
            vampiric: false,
            dash_fire_trail: false,
            dmg_multi: 1.0,
            dash_cd_multi: 1.0,
            turret_overclock: false,
        },
    };

    let mut weapons = Weapon::default_arsenal();
    let mut current_weapon_idx = 0; // 0 = Pistol, 1 = Shotgun, 2 = Rifle, 3 = Plasma, 4 = RPG
    let mut fire_timer = 0.0f32;
    let mut reload_state = ActiveReload {
        is_reloading: false,
        timer: 0.0,
        duration: 1.0,
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemy_bullets: Vec<EnemyBullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut decals: Vec<Decal> = Vec::new();
    let mut casings: Vec<Casing> = Vec::new();
    let mut turrets: Vec<Turret> = Vec::new();
    let mut grenades: Vec<Grenade> = Vec::new();
    let mut pickups: Vec<Pickup> = Vec::new();
    let mut float_texts: Vec<FloatText> = Vec::new();

    let mut camera_pos = player.pos;
    let mut screen_shake = 0.0f32;

    let mut wave = 1u32;
    let mut score = 0u64;
    let mut combo = 1u32;
    let mut combo_timer = 0.0f32;
    let mut total_kills = 0u32;
    let mut shots_fired = 0u64;
    let mut shots_hit = 0u64;
    let mut enemies_to_spawn = 12u32;
    let mut enemies_spawned = 0u32;
    let mut spawn_timer = 0.0f32;

    let mut current_upgrade_choices: Vec<UpgradeOption> = Vec::new();

    loop {
        let dt = get_frame_time().min(0.05);

        // Global Keys
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
            if state == GameState::Playing {
                state = GameState::Paused;
            } else if state == GameState::Paused {
                state = GameState::Playing;
            }
        }

        match state {
            GameState::Start => {
                clear_background(Color::new(0.04, 0.05, 0.08, 1.0));
                let cx = screen_width() / 2.0;
                let cy = screen_height() / 2.0;

                draw_text("CYBER SURVIVOR", cx - 240.0, cy - 120.0, 56.0, Color::new(0.0, 0.94, 1.0, 1.0));
                draw_text("RUST NATIVE TACTICAL TOP-DOWN SHOOTER", cx - 210.0, cy - 80.0, 22.0, Color::new(0.7, 0.8, 0.9, 1.0));

                draw_text("BAN PHIM: [W][A][S][D] Di chuyen  |  [SPACE] Dash ne don (I-frames)", cx - 280.0, cy - 20.0, 20.0, WHITE);
                draw_text("[R] Active Reload (Nhan dung vach de tang 40% dmg) | [SHIFT] Bullet Time", cx - 280.0, cy + 15.0, 20.0, Color::new(0.0, 1.0, 0.4, 1.0));
                draw_text("[1..5] Doi vu khi  |  [Q] Luu dan no chum  |  [E] Dat tru sung tu dong", cx - 280.0, cy + 50.0, 20.0, Color::new(1.0, 0.85, 0.0, 1.0));
                draw_text("CHUOT: [Chuot Trai] Ban 360  |  [Chuot Phai] Chem dao day lui", cx - 280.0, cy + 85.0, 20.0, Color::new(1.0, 0.4, 0.4, 1.0));

                let btn_hover = mouse_position().0 > cx - 140.0 && mouse_position().0 < cx + 140.0
                    && mouse_position().1 > cy + 130.0 && mouse_position().1 < cy + 185.0;
                let btn_col = if btn_hover { Color::new(0.0, 1.0, 0.8, 1.0) } else { Color::new(0.0, 0.8, 1.0, 1.0) };
                draw_rectangle(cx - 140.0, cy + 130.0, 280.0, 55.0, btn_col);
                draw_text("BAT DAU CHIEN DAU [ENTER]", cx - 120.0, cy + 165.0, 20.0, BLACK);

                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) || (btn_hover && is_mouse_button_pressed(MouseButton::Left)) {
                    state = GameState::Playing;
                    enemies_to_spawn = 12;
                    enemies_spawned = 0;
                }
            }

            GameState::Playing => {
                // Time dilation from Bullet Time
                let time_scale = if player.bullet_time && player.stamina > 0.0 {
                    player.stamina = (player.stamina - dt * 45.0).max(0.0);
                    0.4
                } else {
                    player.bullet_time = false;
                    player.stamina = (player.stamina + dt * 18.0).min(player.max_stamina);
                    1.0
                };

                let sim_dt = dt * time_scale;

                // Screen shake decay
                if screen_shake > 0.0 {
                    screen_shake = (screen_shake - dt * 12.0).max(0.0);
                }

                // Mouse in World Space
                let mouse_screen = vec2(mouse_position().0, mouse_position().1);
                let mouse_world = mouse_screen + camera_pos - vec2(screen_width() / 2.0, screen_height() / 2.0);
                player.angle = (mouse_world.y - player.pos.y).atan2(mouse_world.x - player.pos.x);

                // Movement & Dash
                if player.dash_cd > 0.0 {
                    player.dash_cd -= dt;
                }
                if player.grenade_cd > 0.0 {
                    player.grenade_cd -= dt;
                }
                if player.turret_cd > 0.0 {
                    player.turret_cd -= dt;
                }

                // Shield Regen
                player.shield_regen_timer += dt;
                if player.shield_regen_timer > 4.0 && player.shield < player.max_shield {
                    player.shield = (player.shield + dt * 25.0).min(player.max_shield);
                }

                // Bullet-Time Key
                player.bullet_time = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);

                // Hotkeys 1..5
                if is_key_pressed(KeyCode::Key1) { current_weapon_idx = 0; reload_state.is_reloading = false; }
                if is_key_pressed(KeyCode::Key2) { current_weapon_idx = 1; reload_state.is_reloading = false; }
                if is_key_pressed(KeyCode::Key3) { current_weapon_idx = 2; reload_state.is_reloading = false; }
                if is_key_pressed(KeyCode::Key4) { current_weapon_idx = 3; reload_state.is_reloading = false; }
                if is_key_pressed(KeyCode::Key5) { current_weapon_idx = 4; reload_state.is_reloading = false; }

                // Dash (Space)
                if is_key_pressed(KeyCode::Space) && player.dash_cd <= 0.0 && !player.is_dashing {
                    let mut move_dir = Vec2::ZERO;
                    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { move_dir.y -= 1.0; }
                    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { move_dir.y += 1.0; }
                    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { move_dir.x -= 1.0; }
                    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { move_dir.x += 1.0; }

                    let dash_dir = if move_dir != Vec2::ZERO {
                        move_dir.normalize()
                    } else {
                        vec2(player.angle.cos(), player.angle.sin())
                    };

                    player.is_dashing = true;
                    player.dash_duration = 0.22;
                    player.dash_cd = player.dash_max_cd * player.perks.dash_cd_multi;
                    player.vel = dash_dir * 800.0;
                    screen_shake += 4.0;

                    for _ in 0..12 {
                        particles.push(Particle {
                            pos: player.pos,
                            vel: -dash_dir * 200.0 + vec2(rand::gen_range(-100.0, 100.0), rand::gen_range(-100.0, 100.0)),
                            color: Color::new(1.0, 0.8, 0.0, 1.0),
                            radius: rand::gen_range(2.0, 5.0),
                            life: 0.3,
                            max_life: 0.3,
                        });
                    }
                }

                if player.is_dashing {
                    player.dash_duration -= dt;
                    player.pos += player.vel * dt;
                    if player.perks.dash_fire_trail {
                        decals.push(Decal {
                            pos: player.pos,
                            radius: 16.0,
                            color: Color::new(1.0, 0.3, 0.0, 0.6),
                            life: 3.0,
                            is_damage_zone: true,
                        });
                    }
                    if player.dash_duration <= 0.0 {
                        player.is_dashing = false;
                        player.vel = Vec2::ZERO;
                    }
                } else {
                    let mut move_dir = Vec2::ZERO;
                    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { move_dir.y -= 1.0; }
                    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { move_dir.y += 1.0; }
                    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { move_dir.x -= 1.0; }
                    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { move_dir.x += 1.0; }

                    if move_dir != Vec2::ZERO {
                        player.pos += move_dir.normalize() * player.speed * dt;
                    }
                }

                player.pos.x = player.pos.x.clamp(player.radius, WORLD_WIDTH - player.radius);
                player.pos.y = player.pos.y.clamp(player.radius, WORLD_HEIGHT - player.radius);

                // Active Reload (R)
                let w = &mut weapons[current_weapon_idx];
                if is_key_pressed(KeyCode::R) {
                    if !reload_state.is_reloading && w.current_clip < w.clip_size && w.reserve_ammo != 0 {
                        reload_state.is_reloading = true;
                        reload_state.timer = 0.0;
                        reload_state.duration = w.reload_duration;
                    } else if reload_state.is_reloading {
                        let progress = reload_state.timer / reload_state.duration;
                        if progress >= 0.58 && progress <= 0.82 {
                            // Perfect Active Reload
                            player.active_overcharge = true;
                            let needed = w.clip_size - w.current_clip;
                            let can_load = if w.reserve_ammo < 0 { needed } else { needed.min(w.reserve_ammo as u32) };
                            w.current_clip += can_load;
                            if w.reserve_ammo > 0 { w.reserve_ammo -= can_load as i32; }
                            reload_state.is_reloading = false;
                            float_texts.push(FloatText {
                                pos: player.pos - vec2(0.0, 30.0),
                                text: "PERFECT RELOAD! +40% DMG".to_string(),
                                color: Color::new(0.0, 1.0, 0.4, 1.0),
                                size: 24.0,
                                life: 1.2,
                            });
                        } else {
                            // Jammed
                            reload_state.timer = (reload_state.timer - 0.5).max(0.0);
                            float_texts.push(FloatText {
                                pos: player.pos - vec2(0.0, 30.0),
                                text: "RELOAD JAMMED!".to_string(),
                                color: Color::new(1.0, 0.2, 0.2, 1.0),
                                size: 20.0,
                                life: 1.0,
                            });
                        }
                    }
                }

                if reload_state.is_reloading {
                    reload_state.timer += dt;
                    if reload_state.timer >= reload_state.duration {
                        let needed = w.clip_size - w.current_clip;
                        let can_load = if w.reserve_ammo < 0 { needed } else { needed.min(w.reserve_ammo as u32) };
                        w.current_clip += can_load;
                        if w.reserve_ammo > 0 { w.reserve_ammo -= can_load as i32; }
                        reload_state.is_reloading = false;
                        player.active_overcharge = false;
                    }
                }

                // Skills: Grenade (Q) & Turret (E)
                if is_key_pressed(KeyCode::Q) && player.grenade_cd <= 0.0 {
                    player.grenade_cd = player.grenade_max_cd;
                    let dir = (mouse_world - player.pos).normalize_or_zero();
                    grenades.push(Grenade {
                        pos: player.pos,
                        vel: dir * 460.0,
                        fuse: 1.3,
                        radius: 6.0,
                    });
                }

                if is_key_pressed(KeyCode::E) && player.turret_cd <= 0.0 {
                    player.turret_cd = player.turret_max_cd;
                    turrets.push(Turret {
                        pos: player.pos,
                        angle: player.angle,
                        fire_timer: 0.0,
                        life: if player.perks.turret_overclock { 35.0 } else { 20.0 },
                    });
                }

                // Melee (Right Click)
                if is_mouse_button_pressed(MouseButton::Right) {
                    screen_shake += 5.0;
                    let slash_angle = player.angle;
                    for enemy in enemies.iter_mut() {
                        let to_enemy = enemy.pos - player.pos;
                        let dist = to_enemy.length();
                        if dist < 85.0 + enemy.radius {
                            let angle_diff = (slash_angle - to_enemy.y.atan2(to_enemy.x)).abs();
                            if angle_diff < 0.9 || angle_diff > (PI * 2.0 - 0.9) {
                                enemy.hp -= 70.0;
                                enemy.vel += to_enemy.normalize() * 500.0;
                                float_texts.push(FloatText {
                                    pos: enemy.pos,
                                    text: "70 CRIT!".to_string(),
                                    color: Color::new(1.0, 0.84, 0.0, 1.0),
                                    size: 20.0,
                                    life: 0.8,
                                });
                            }
                        }
                    }
                }

                // Weapon Firing
                if fire_timer > 0.0 { fire_timer -= dt; }
                if is_mouse_button_down(MouseButton::Left) && fire_timer <= 0.0 && !reload_state.is_reloading {
                    let w = &mut weapons[current_weapon_idx];
                    if w.current_clip > 0 {
                        w.current_clip -= 1;
                        fire_timer = w.fire_rate;
                        screen_shake += w.screen_shake;
                        shots_fired += 1;

                        let mut dmg = w.damage * player.perks.dmg_multi;
                        if player.active_overcharge { dmg *= 1.4; }

                        for _ in 0..w.pellets {
                            let spread_angle = player.angle + rand::gen_range(-w.spread, w.spread);
                            let dir = vec2(spread_angle.cos(), spread_angle.sin());
                            bullets.push(Bullet {
                                pos: player.pos + dir * (player.radius + 6.0),
                                vel: dir * w.speed,
                                damage: dmg,
                                color: if player.active_overcharge { Color::new(0.0, 1.0, 0.4, 1.0) } else { w.color },
                                radius: if w.is_rocket { 8.0 } else { 3.5 },
                                is_rocket: w.is_rocket,
                                explosion_radius: if w.is_rocket { 140.0 } else { 0.0 },
                                pierce: w.pierce + player.perks.pierce_count,
                                bounces: if player.perks.bouncing_bullets { 2 } else { 0 },
                                life: 2.2,
                            });
                        }

                        // Shell casing
                        casings.push(Casing {
                            pos: player.pos,
                            vel: vec2((player.angle - PI / 2.0).cos(), (player.angle - PI / 2.0).sin()) * rand::gen_range(80.0, 150.0),
                            rotation: rand::gen_range(0.0, PI * 2.0),
                            v_rot: rand::gen_range(-10.0, 10.0),
                            life: 5.0,
                        });
                    } else if !reload_state.is_reloading && w.reserve_ammo != 0 {
                        // Auto-start reload
                        reload_state.is_reloading = true;
                        reload_state.timer = 0.0;
                        reload_state.duration = w.reload_duration;
                    }
                }

                // Bullets Physics & Collision
                for b in bullets.iter_mut() {
                    b.pos += b.vel * sim_dt;
                    b.life -= sim_dt;

                    if b.bounces > 0 {
                        if b.pos.x < 0.0 || b.pos.x > WORLD_WIDTH { b.vel.x *= -1.0; b.bounces -= 1; }
                        if b.pos.y < 0.0 || b.pos.y > WORLD_HEIGHT { b.vel.y *= -1.0; b.bounces -= 1; }
                    }

                    for e in enemies.iter_mut() {
                        if (e.pos - b.pos).length() < e.radius + b.radius && b.life > 0.0 {
                            shots_hit += 1;
                            if b.is_rocket {
                                b.life = 0.0;
                                screen_shake += 12.0;
                                // Area explosion
                                for oe in enemies.iter_mut() {
                                    let dist = (oe.pos - b.pos).length();
                                    if dist < b.explosion_radius + oe.radius {
                                        oe.hp -= b.damage * (1.0 - dist / (b.explosion_radius + oe.radius) * 0.5);
                                        oe.vel += (oe.pos - b.pos).normalize_or_zero() * 450.0;
                                    }
                                }
                                break;
                            } else {
                                e.hp -= b.damage;
                                e.vel += b.vel.normalize_or_zero() * 60.0;
                                float_texts.push(FloatText {
                                    pos: e.pos,
                                    text: format!("{:.0}", b.damage),
                                    color: WHITE,
                                    size: 14.0,
                                    life: 0.6,
                                });
                                if b.pierce > 0 {
                                    b.pierce -= 1;
                                } else {
                                    b.life = 0.0;
                                    break;
                                }
                            }
                        }
                    }
                }
                bullets.retain(|b| b.life > 0.0 && b.pos.x >= 0.0 && b.pos.x <= WORLD_WIDTH && b.pos.y >= 0.0 && b.pos.y <= WORLD_HEIGHT);

                // Enemy Spawns
                spawn_timer += sim_dt;
                let spawn_interval = (1.5 - (wave as f32) * 0.1).max(0.4);
                if spawn_timer >= spawn_interval && enemies_spawned < enemies_to_spawn {
                    spawn_timer = 0.0;
                    let spawn_angle = rand::gen_range(0.0, PI * 2.0);
                    let spawn_dist = rand::gen_range(650.0, 850.0);
                    let ex = (player.pos.x + spawn_angle.cos() * spawn_dist).clamp(60.0, WORLD_WIDTH - 60.0);
                    let ey = (player.pos.y + spawn_angle.sin() * spawn_dist).clamp(60.0, WORLD_HEIGHT - 60.0);

                    let r = rand::gen_range(0.0, 1.0);
                    let (etype, hp, spd, rad, col) = if wave >= 5 && enemies_spawned == 0 {
                        (EnemyType::Boss, 1400.0 + (wave as f32) * 400.0, 120.0, 45.0, Color::new(1.0, 0.0, 0.35, 1.0))
                    } else if wave >= 4 && r < 0.2 {
                        (EnemyType::Brute, 200.0 + (wave as f32) * 30.0, 110.0, 24.0, Color::new(0.66, 0.33, 0.97, 1.0))
                    } else if wave >= 3 && r < 0.45 {
                        (EnemyType::Kamikaze, 35.0, 260.0, 13.0, Color::new(1.0, 0.66, 0.0, 1.0))
                    } else if wave >= 2 && r < 0.75 {
                        (EnemyType::Spitter, 65.0 + (wave as f32) * 10.0, 140.0, 16.0, Color::new(0.0, 1.0, 0.4, 1.0))
                    } else {
                        (EnemyType::Runner, 50.0 + (wave as f32) * 8.0, 195.0, 14.0, Color::new(1.0, 0.2, 0.27, 1.0))
                    };

                    enemies.push(Enemy {
                        pos: vec2(ex, ey),
                        vel: Vec2::ZERO,
                        radius: rad,
                        hp,
                        max_hp: hp,
                        speed: spd,
                        color: col,
                        enemy_type: etype,
                        shoot_timer: rand::gen_range(0.5, 2.0),
                        charge_timer: 0.0,
                        is_charging: false,
                        charge_angle: 0.0,
                        boss_attack_timer: 0.0,
                        boss_phase: 0,
                    });
                    enemies_spawned += 1;
                }

                // Enemy AI & Updates
                for e in enemies.iter_mut() {
                    // Fire trail damage
                    for d in decals.iter() {
                        if d.is_damage_zone && (e.pos - d.pos).length() < d.radius + e.radius {
                            e.hp -= 30.0 * sim_dt;
                        }
                    }

                    e.pos += e.vel * sim_dt;
                    e.vel *= 0.88;

                    let to_player = player.pos - e.pos;
                    let dist = to_player.length();
                    let dir = to_player.normalize_or_zero();

                    match e.enemy_type {
                        EnemyType::Runner => {
                            e.pos += dir * e.speed * sim_dt;
                            if dist < player.radius + e.radius && !player.is_dashing {
                                damage_player(&mut player, 16.0, &mut screen_shake);
                                e.vel -= dir * 200.0;
                            }
                        }
                        EnemyType::Spitter => {
                            if dist < 280.0 { e.pos -= dir * e.speed * sim_dt; }
                            else if dist > 420.0 { e.pos += dir * e.speed * sim_dt; }
                            e.shoot_timer += sim_dt;
                            if e.shoot_timer >= 2.0 {
                                e.shoot_timer = 0.0;
                                enemy_bullets.push(EnemyBullet {
                                    pos: e.pos,
                                    vel: dir * 380.0,
                                    damage: 20.0,
                                    radius: 6.0,
                                    color: Color::new(0.0, 1.0, 0.4, 1.0),
                                    life: 3.0,
                                });
                            }
                        }
                        EnemyType::Kamikaze => {
                            e.pos += dir * e.speed * sim_dt;
                            if dist < 40.0 {
                                e.hp = 0.0;
                                damage_player(&mut player, 40.0, &mut screen_shake);
                            }
                        }
                        EnemyType::Brute => {
                            e.charge_timer += sim_dt;
                            if e.is_charging {
                                let cdir = vec2(e.charge_angle.cos(), e.charge_angle.sin());
                                e.pos += cdir * 520.0 * sim_dt;
                                if dist < player.radius + e.radius && !player.is_dashing {
                                    damage_player(&mut player, 35.0, &mut screen_shake);
                                    e.is_charging = false;
                                }
                                if e.charge_timer > 0.8 { e.is_charging = false; e.charge_timer = 0.0; }
                            } else {
                                e.pos += dir * e.speed * sim_dt;
                                if e.charge_timer >= 3.5 && dist < 450.0 {
                                    e.is_charging = true;
                                    e.charge_angle = to_player.y.atan2(to_player.x);
                                    e.charge_timer = 0.0;
                                }
                            }
                        }
                        EnemyType::Boss => {
                            e.pos += dir * e.speed * sim_dt;
                            e.boss_attack_timer += sim_dt;
                            if e.boss_attack_timer >= 1.8 {
                                e.boss_attack_timer = 0.0;
                                e.boss_phase = (e.boss_phase + 1) % 2;
                                if e.boss_phase == 0 {
                                    for a in 0..12 {
                                        let angle = (a as f32) * (PI * 2.0 / 12.0);
                                        enemy_bullets.push(EnemyBullet {
                                            pos: e.pos,
                                            vel: vec2(angle.cos(), angle.sin()) * 320.0,
                                            damage: 25.0,
                                            radius: 7.0,
                                            color: Color::new(1.0, 0.0, 0.35, 1.0),
                                            life: 3.5,
                                        });
                                    }
                                } else {
                                    for o in [-0.25, 0.0, 0.25] {
                                        let angle = to_player.y.atan2(to_player.x) + o;
                                        enemy_bullets.push(EnemyBullet {
                                            pos: e.pos,
                                            vel: vec2(angle.cos(), angle.sin()) * 420.0,
                                            damage: 30.0,
                                            radius: 8.0,
                                            color: Color::new(1.0, 0.84, 0.0, 1.0),
                                            life: 3.0,
                                        });
                                    }
                                }
                            }
                            if dist < player.radius + e.radius && !player.is_dashing {
                                damage_player(&mut player, 40.0, &mut screen_shake);
                            }
                        }
                    }
                }

                // Check dead enemies
                let mut new_pickups: Vec<Pickup> = Vec::new();
                for e in enemies.iter() {
                    if e.hp <= 0.0 {
                        total_kills += 1;
                        combo += 1;
                        combo_timer = 3.5;
                        score += ((if e.enemy_type == EnemyType::Boss { 5000 } else { 100 }) * combo) as u64;

                        if player.perks.vampiric && player.hp < player.max_hp {
                            player.hp = (player.hp + 2.0).min(player.max_hp);
                        }

                        if rand::gen_range(0.0, 1.0) < 0.16 || e.enemy_type == EnemyType::Boss {
                            let ptypes = [PickupType::Medkit, PickupType::Ammo, PickupType::Nuke, PickupType::Overcharge];
                            new_pickups.push(Pickup {
                                pos: e.pos,
                                pickup_type: ptypes[rand::gen_range(0, ptypes.len())],
                                radius: 12.0,
                                pulse: 0.0,
                            });
                        }
                    }
                }
                pickups.append(&mut new_pickups);
                enemies.retain(|e| e.hp > 0.0);

                // Pickups
                for p in pickups.iter_mut() {
                    p.pulse += dt * 5.0;
                    if (p.pos - player.pos).length() < p.radius + player.radius {
                        match p.pickup_type {
                            PickupType::Medkit => {
                                player.hp = (player.hp + 35.0).min(player.max_hp);
                                float_texts.push(FloatText { pos: player.pos, text: "+35 HP".to_string(), color: GREEN, size: 20.0, life: 1.0 });
                            }
                            PickupType::Ammo => {
                                weapons[1].reserve_ammo = (weapons[1].reserve_ammo + 24).min(96);
                                weapons[2].reserve_ammo = (weapons[2].reserve_ammo + 90).min(300);
                                weapons[3].reserve_ammo = (weapons[3].reserve_ammo + 36).min(120);
                                weapons[4].reserve_ammo = (weapons[4].reserve_ammo + 6).min(20);
                                float_texts.push(FloatText { pos: player.pos, text: "MAX AMMO REFILLED!".to_string(), color: GOLD, size: 20.0, life: 1.0 });
                            }
                            PickupType::Nuke => {
                                screen_shake += 18.0;
                                for e in enemies.iter_mut() {
                                    if e.enemy_type != EnemyType::Boss { e.hp = 0.0; }
                                    else { e.hp -= 400.0; }
                                }
                                float_texts.push(FloatText { pos: player.pos, text: "NUKE ACTIVATED!".to_string(), color: RED, size: 28.0, life: 1.5 });
                            }
                            PickupType::Overcharge => {
                                player.active_overcharge = true;
                                float_texts.push(FloatText { pos: player.pos, text: "OVERCHARGE BUFF!".to_string(), color: SKYBLUE, size: 24.0, life: 1.2 });
                            }
                        }
                    }
                }
                pickups.retain(|p| (p.pos - player.pos).length() >= p.radius + player.radius);

                // Enemy Bullets
                for eb in enemy_bullets.iter_mut() {
                    eb.pos += eb.vel * sim_dt;
                    eb.life -= sim_dt;
                    if (eb.pos - player.pos).length() < eb.radius + player.radius && !player.is_dashing {
                        damage_player(&mut player, eb.damage, &mut screen_shake);
                        eb.life = 0.0;
                    }
                }
                enemy_bullets.retain(|eb| eb.life > 0.0 && eb.pos.x >= 0.0 && eb.pos.x <= WORLD_WIDTH && eb.pos.y >= 0.0 && eb.pos.y <= WORLD_HEIGHT);

                // Turrets
                for t in turrets.iter_mut() {
                    t.life -= sim_dt;
                    t.fire_timer += sim_dt;
                    let fire_interval = if player.perks.turret_overclock { 0.12 } else { 0.22 };

                    let mut nearest_enemy: Option<Vec2> = None;
                    let mut min_dist = 450.0;
                    for e in enemies.iter() {
                        let d = (e.pos - t.pos).length();
                        if d < min_dist { min_dist = d; nearest_enemy = Some(e.pos); }
                    }

                    if let Some(target) = nearest_enemy {
                        t.angle = (target.y - t.pos.y).atan2(target.x - t.pos.x);
                        if t.fire_timer >= fire_interval {
                            t.fire_timer = 0.0;
                            let dir = vec2(t.angle.cos(), t.angle.sin());
                            bullets.push(Bullet {
                                pos: t.pos + dir * 18.0,
                                vel: dir * 1150.0,
                                damage: 22.0,
                                color: Color::new(0.0, 0.94, 1.0, 1.0),
                                radius: 3.0,
                                is_rocket: false,
                                explosion_radius: 0.0,
                                pierce: 0,
                                bounces: 0,
                                life: 1.5,
                            });
                        }
                    }
                }
                turrets.retain(|t| t.life > 0.0);

                // Grenades
                for g in grenades.iter_mut() {
                    g.pos += g.vel * sim_dt;
                    g.vel *= 0.94;
                    g.fuse -= sim_dt;
                    if g.fuse <= 0.0 {
                        screen_shake += 10.0;
                        for e in enemies.iter_mut() {
                            let dist = (e.pos - g.pos).length();
                            if dist < 140.0 + e.radius {
                                e.hp -= 160.0;
                                e.vel += (e.pos - g.pos).normalize_or_zero() * 450.0;
                            }
                        }
                    }
                }
                grenades.retain(|g| g.fuse > 0.0);

                // Decals & Casings
                for d in decals.iter_mut() { d.life -= dt; }
                decals.retain(|d| d.life > 0.0);
                for c in casings.iter_mut() {
                    c.pos += c.vel * dt;
                    c.vel *= 0.9;
                    c.rotation += c.v_rot * dt;
                    c.life -= dt;
                }
                casings.retain(|c| c.life > 0.0);

                // Particles & Float Texts
                for p in particles.iter_mut() { p.pos += p.vel * dt; p.life -= dt; }
                particles.retain(|p| p.life > 0.0);
                for f in float_texts.iter_mut() { f.pos.y -= 30.0 * dt; f.life -= dt; }
                float_texts.retain(|f| f.life > 0.0);

                // Combo Timer
                if combo_timer > 0.0 {
                    combo_timer -= dt;
                    if combo_timer <= 0.0 { combo = 1; }
                }

                // Check Wave Completion
                if enemies_spawned >= enemies_to_spawn && enemies.is_empty() {
                    state = GameState::Upgrade;
                    current_upgrade_choices = get_random_upgrades();
                }

                // Check Game Over
                if player.hp <= 0.0 {
                    state = GameState::GameOver;
                }

                // Camera follow
                camera_pos += (player.pos - camera_pos) * 10.0 * dt;
            }

            GameState::Upgrade => {
                clear_background(Color::new(0.06, 0.08, 0.12, 1.0));
                let cx = screen_width() / 2.0;
                let cy = screen_height() / 2.0;

                draw_text("NANG CAP CONG NGHE ROGUELITE", cx - 230.0, cy - 140.0, 36.0, Color::new(0.0, 0.94, 1.0, 1.0));
                draw_text("Chon 1 trong 3 nang cap de tiep tuc Wave tiep theo:", cx - 210.0, cy - 100.0, 18.0, Color::new(0.7, 0.8, 0.9, 1.0));

                let card_w = 260.0;
                let card_h = 200.0;
                let start_x = cx - (3.0 * card_w + 2.0 * 24.0) / 2.0;

                for (i, opt) in current_upgrade_choices.iter().enumerate() {
                    let rx = start_x + (i as f32) * (card_w + 24.0);
                    let ry = cy - 60.0;
                    let hover = mouse_position().0 > rx && mouse_position().0 < rx + card_w
                        && mouse_position().1 > ry && mouse_position().1 < ry + card_h;

                    let bg_col = if hover { Color::new(0.0, 0.3, 0.45, 0.9) } else { Color::new(0.08, 0.12, 0.18, 0.9) };
                    let border_col = if hover { Color::new(0.0, 1.0, 0.8, 1.0) } else { Color::new(0.0, 0.6, 0.8, 0.5) };

                    draw_rectangle(rx, ry, card_w, card_h, bg_col);
                    draw_rectangle_lines(rx, ry, card_w, card_h, 2.0, border_col);

                    draw_text(opt.icon, rx + 16.0, ry + 40.0, 32.0, WHITE);
                    draw_text(opt.title, rx + 16.0, ry + 75.0, 20.0, Color::new(1.0, 0.84, 0.0, 1.0));
                    draw_text(opt.desc, rx + 16.0, ry + 110.0, 14.0, Color::new(0.8, 0.9, 1.0, 1.0));

                    if hover && is_mouse_button_pressed(MouseButton::Left) {
                        apply_upgrade(&mut player, opt.id);
                        wave += 1;
                        enemies_to_spawn = 10 + wave * 6;
                        enemies_spawned = 0;
                        state = GameState::Playing;
                    }
                }
            }

            GameState::Paused => {
                let cx = screen_width() / 2.0;
                let cy = screen_height() / 2.0;
                draw_rectangle(cx - 200.0, cy - 80.0, 400.0, 160.0, Color::new(0.05, 0.07, 0.12, 0.95));
                draw_rectangle_lines(cx - 200.0, cy - 80.0, 400.0, 160.0, 2.0, Color::new(0.0, 0.94, 1.0, 1.0));
                draw_text("TAM DUNG TRAN DAU", cx - 130.0, cy - 20.0, 28.0, Color::new(0.0, 0.94, 1.0, 1.0));
                draw_text("Bam [ESC] hoac [P] de tiep tuc", cx - 110.0, cy + 25.0, 18.0, WHITE);
            }

            GameState::GameOver => {
                clear_background(Color::new(0.08, 0.02, 0.04, 1.0));
                let cx = screen_width() / 2.0;
                let cy = screen_height() / 2.0;

                draw_text("MISSION FAILED", cx - 190.0, cy - 100.0, 52.0, Color::new(1.0, 0.1, 0.2, 1.0));
                draw_text(&format!("WAVE DAT DUOC: {}", wave), cx - 120.0, cy - 30.0, 24.0, WHITE);
                draw_text(&format!("TONG DIEM: {}", score), cx - 120.0, cy + 5.0, 24.0, Color::new(1.0, 0.84, 0.0, 1.0));
                draw_text(&format!("QUAI DA HA: {}", total_kills), cx - 120.0, cy + 40.0, 24.0, Color::new(0.0, 0.94, 1.0, 1.0));

                let acc = if shots_fired > 0 { (shots_hit as f32 / shots_fired as f32 * 100.0) as u32 } else { 0 };
                draw_text(&format!("DO CHINH XAC: {}%", acc), cx - 120.0, cy + 75.0, 24.0, Color::new(0.0, 1.0, 0.4, 1.0));

                draw_text("Bam [ENTER] de choi lai", cx - 110.0, cy + 140.0, 22.0, WHITE);

                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R) {
                    player.hp = player.max_hp;
                    player.shield = player.max_shield;
                    player.pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
                    weapons = Weapon::default_arsenal();
                    bullets.clear();
                    enemy_bullets.clear();
                    enemies.clear();
                    particles.clear();
                    decals.clear();
                    turrets.clear();
                    grenades.clear();
                    pickups.clear();
                    score = 0;
                    total_kills = 0;
                    wave = 1;
                    enemies_to_spawn = 12;
                    enemies_spawned = 0;
                    state = GameState::Playing;
                }
            }
        }

        // ==========================================
        // RENDERING PIPELINE FOR PLAYING STATE
        // ==========================================
        if state == GameState::Playing || state == GameState::Paused {
            clear_background(Color::new(0.05, 0.06, 0.08, 1.0));

            let shake_offset = if screen_shake > 0.0 {
                vec2(rand::gen_range(-screen_shake, screen_shake), rand::gen_range(-screen_shake, screen_shake))
            } else {
                Vec2::ZERO
            };

            let cam_offset = vec2(screen_width() / 2.0, screen_height() / 2.0) - camera_pos + shake_offset;

            // 1. Grid
            let grid_size = 80.0;
            let start_x = (cam_offset.x % grid_size) - grid_size;
            let start_y = (cam_offset.y % grid_size) - grid_size;
            let grid_col = Color::new(0.0, 0.8, 1.0, 0.04);
            let mut x = start_x;
            while x < screen_width() + grid_size {
                draw_line(x, 0.0, x, screen_height(), 1.0, grid_col);
                x += grid_size;
            }
            let mut y = start_y;
            while y < screen_height() + grid_size {
                draw_line(0.0, y, screen_width(), y, 1.0, grid_col);
                y += grid_size;
            }

            // World Boundary
            let bound_r = Rect::new(cam_offset.x, cam_offset.y, WORLD_WIDTH, WORLD_HEIGHT);
            draw_rectangle_lines(bound_r.x, bound_r.y, bound_r.w, bound_r.h, 4.0, Color::new(0.0, 0.94, 1.0, 0.8));

            // Decals
            for d in decals.iter() {
                let sp = d.pos + cam_offset;
                draw_circle(sp.x, sp.y, d.radius, d.color);
            }

            // Casings
            for c in casings.iter() {
                let sp = c.pos + cam_offset;
                draw_circle(sp.x, sp.y, 2.0, Color::new(0.85, 0.7, 0.2, 0.8));
            }

            // Turrets
            for t in turrets.iter() {
                let sp = t.pos + cam_offset;
                draw_circle(sp.x, sp.y, 14.0, Color::new(0.12, 0.16, 0.24, 1.0));
                draw_circle_lines(sp.x, sp.y, 14.0, 2.0, Color::new(0.0, 0.94, 1.0, 1.0));
                let dir = vec2(t.angle.cos(), t.angle.sin());
                draw_line(sp.x, sp.y, sp.x + dir.x * 18.0, sp.y + dir.y * 18.0, 4.0, Color::new(0.6, 0.7, 0.8, 1.0));
            }

            // Pickups
            for p in pickups.iter() {
                let sp = p.pos + cam_offset;
                let col = match p.pickup_type {
                    PickupType::Medkit => Color::new(1.0, 0.2, 0.2, 1.0),
                    PickupType::Ammo => Color::new(1.0, 0.84, 0.0, 1.0),
                    PickupType::Nuke => Color::new(1.0, 0.5, 0.0, 1.0),
                    PickupType::Overcharge => Color::new(0.0, 0.94, 1.0, 1.0),
                };
                draw_circle(sp.x, sp.y, p.radius, col);
            }

            // Grenades
            for g in grenades.iter() {
                let sp = g.pos + cam_offset;
                draw_circle(sp.x, sp.y, g.radius, Color::new(1.0, 0.2, 0.27, 1.0));
                draw_circle_lines(sp.x, sp.y, g.radius, 1.5, WHITE);
            }

            // Particles
            for p in particles.iter() {
                let sp = p.pos + cam_offset;
                let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
                draw_circle(sp.x, sp.y, p.radius, Color::new(p.color.r, p.color.g, p.color.b, alpha));
            }

            // Enemies
            for e in enemies.iter() {
                let sp = e.pos + cam_offset;
                draw_circle(sp.x, sp.y, e.radius, e.color);
                draw_circle(sp.x, sp.y, e.radius * 0.4, BLACK);

                // Health bar
                if e.hp < e.max_hp && e.enemy_type != EnemyType::Boss {
                    let bw = e.radius * 2.0;
                    draw_rectangle(sp.x - bw / 2.0, sp.y - e.radius - 8.0, bw, 3.0, Color::new(0.0, 0.0, 0.0, 0.6));
                    draw_rectangle(sp.x - bw / 2.0, sp.y - e.radius - 8.0, bw * (e.hp / e.max_hp), 3.0, Color::new(0.0, 1.0, 0.4, 1.0));
                }
            }

            // Player
            let p_sp = player.pos + cam_offset;
            if player.shield > 0.0 {
                draw_circle_lines(p_sp.x, p_sp.y, player.radius + 4.0, 2.0, Color::new(0.0, 0.94, 1.0, 0.5));
            }
            draw_circle(p_sp.x, p_sp.y, player.radius, Color::new(0.0, 0.94, 1.0, 1.0));
            draw_circle(p_sp.x, p_sp.y, 8.0, Color::new(0.06, 0.09, 0.16, 1.0));
            let gun_dir = vec2(player.angle.cos(), player.angle.sin());
            draw_line(p_sp.x, p_sp.y, p_sp.x + gun_dir.x * 24.0, p_sp.y + gun_dir.y * 24.0, 5.0, Color::new(0.4, 0.5, 0.6, 1.0));

            // Bullets
            for b in bullets.iter() {
                let sp = b.pos + cam_offset;
                draw_circle(sp.x, sp.y, b.radius, b.color);
            }
            for eb in enemy_bullets.iter() {
                let sp = eb.pos + cam_offset;
                draw_circle(sp.x, sp.y, eb.radius, eb.color);
            }

            // Float Texts
            for f in float_texts.iter() {
                let sp = f.pos + cam_offset;
                draw_text(&f.text, sp.x, sp.y, f.size, f.color);
            }

            // HUD OVERLAY (Screen space)
            draw_hud(&player, &weapons[current_weapon_idx], &reload_state, wave, score, combo, (enemies_to_spawn - enemies_spawned) + (enemies.len() as u32));
        }

        next_frame().await;
    }
}

fn damage_player(player: &mut Player, mut amount: f32, shake: &mut f32) {
    player.shield_regen_timer = 0.0;
    *shake += 6.0;
    if player.shield > 0.0 {
        if player.shield >= amount {
            player.shield -= amount;
            amount = 0.0;
        } else {
            amount -= player.shield;
            player.shield = 0.0;
        }
    }
    player.hp = (player.hp - amount).max(0.0);
}

fn draw_hud(player: &Player, w: &Weapon, reload: &ActiveReload, wave: u32, score: u64, combo: u32, enemies_left: u32) {
    // Top Bar
    draw_rectangle(20.0, 20.0, 140.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, 20.0, 140.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("WAVE", 30.0, 40.0, 16.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", wave), 30.0, 62.0, 22.0, Color::new(0.0, 0.94, 1.0, 1.0));

    draw_rectangle(170.0, 20.0, 220.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(170.0, 20.0, 220.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("SCORE", 180.0, 40.0, 16.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", score), 180.0, 62.0, 22.0, Color::new(1.0, 0.84, 0.0, 1.0));
    draw_text(&format!("x{}", combo), 330.0, 62.0, 18.0, Color::new(0.0, 1.0, 0.4, 1.0));

    draw_rectangle(400.0, 20.0, 150.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(400.0, 20.0, 150.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("ENEMIES", 410.0, 40.0, 16.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", enemies_left), 410.0, 62.0, 22.0, Color::new(1.0, 0.2, 0.27, 1.0));

    // Bottom Left: Vitals
    let by = screen_height() - 110.0;
    draw_rectangle(20.0, by, 280.0, 90.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, by, 280.0, 90.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));

    // HP Bar
    draw_text(&format!("HP: {:.0} / {:.0}", player.hp, player.max_hp), 30.0, by + 22.0, 14.0, WHITE);
    draw_rectangle(30.0, by + 28.0, 260.0, 8.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(30.0, by + 28.0, 260.0 * (player.hp / player.max_hp), 8.0, Color::new(1.0, 0.2, 0.27, 1.0));

    // Shield Bar
    draw_text(&format!("SHIELD: {:.0} / {:.0}", player.shield, player.max_shield), 30.0, by + 50.0, 14.0, WHITE);
    draw_rectangle(30.0, by + 56.0, 260.0, 8.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(30.0, by + 56.0, 260.0 * (player.shield / player.max_shield), 8.0, Color::new(0.0, 0.94, 1.0, 1.0));

    // Dash & Bullet Time
    let dash_pct = if player.dash_cd <= 0.0 { 1.0 } else { 1.0 - (player.dash_cd / (player.dash_max_cd * player.perks.dash_cd_multi)) };
    draw_text("DASH [SPACE]", 30.0, by + 78.0, 12.0, Color::new(1.0, 0.8, 0.0, 1.0));
    draw_rectangle(30.0, by + 82.0, 120.0, 5.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(30.0, by + 82.0, 120.0 * dash_pct, 5.0, Color::new(1.0, 0.8, 0.0, 1.0));

    draw_text("TIME [SHIFT]", 165.0, by + 78.0, 12.0, Color::new(0.7, 0.4, 1.0, 1.0));
    draw_rectangle(165.0, by + 82.0, 125.0, 5.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(165.0, by + 82.0, 125.0 * (player.stamina / player.max_stamina), 5.0, Color::new(0.7, 0.4, 1.0, 1.0));

    // Bottom Right: Ammo & Active Reload
    let bx = screen_width() - 250.0;
    draw_rectangle(bx, by, 230.0, 90.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(bx, by, 230.0, 90.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));

    draw_text(w.name, bx + 16.0, by + 26.0, 20.0, Color::new(0.0, 0.94, 1.0, 1.0));
    let reserve_str = if w.reserve_ammo < 0 { "∞".to_string() } else { format!("{}", w.reserve_ammo) };
    draw_text(&format!("{} / {}", w.current_clip, reserve_str), bx + 16.0, by + 56.0, 28.0, WHITE);

    // Active Reload Bar
    if reload.is_reloading {
        let rw = 198.0;
        let progress = (reload.timer / reload.duration).clamp(0.0, 1.0);
        draw_rectangle(bx + 16.0, by + 68.0, rw, 12.0, Color::new(0.1, 0.15, 0.2, 1.0));
        // Sweet spot
        draw_rectangle(bx + 16.0 + rw * 0.58, by + 68.0, rw * 0.24, 12.0, Color::new(0.0, 1.0, 0.4, 0.5));
        // Indicator
        draw_rectangle(bx + 16.0 + rw * progress, by + 66.0, 4.0, 16.0, WHITE);
    } else {
        draw_text("[R] NAP DAN", bx + 16.0, by + 80.0, 13.0, Color::new(0.6, 0.7, 0.8, 1.0));
    }

    // Skills
    draw_text(&format!("[Q] GRENADE: {}", if player.grenade_cd <= 0.0 { "READY" } else { "..." }), screen_width() / 2.0 - 160.0, screen_height() - 30.0, 14.0, Color::new(0.0, 0.94, 1.0, 1.0));
    draw_text(&format!("[E] TURRET: {}", if player.turret_cd <= 0.0 { "READY" } else { "..." }), screen_width() / 2.0 + 20.0, screen_height() - 30.0, 14.0, Color::new(0.0, 0.94, 1.0, 1.0));
}

fn get_random_upgrades() -> Vec<UpgradeOption> {
    vec![
        UpgradeOption { id: "bounce", title: "DAN PHAN XA", icon: "o", desc: "Dan nay lai 2 lan khi va vao tuong dau truong." },
        UpgradeOption { id: "pierce", title: "DAN XUYEN THAU", icon: ">", desc: "Moi loai sung tang them +1 lan ban xuyen muc tieu." },
        UpgradeOption { id: "vamp", title: "HUT NANG LUONG", icon: "+", desc: "Hoi +2 HP ngay lap tuc moi khi tieu diet duoc quai." },
    ]
}

fn apply_upgrade(player: &mut Player, id: &str) {
    if id == "bounce" { player.perks.bouncing_bullets = true; }
    else if id == "pierce" { player.perks.pierce_count += 1; }
    else if id == "vamp" { player.perks.vampiric = true; }
}
