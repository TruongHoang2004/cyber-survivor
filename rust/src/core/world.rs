use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::ally::{AlliedBody, AlliedEntity, AlliedMarine};
use crate::character::classes::CyberMarine;
use crate::character::player::Player;
use crate::core::context::{CombatContext, WorldCommand};
use crate::core::events::{
    BattlefieldEvent, BeamArcVisualEvent, ShockwaveRingEvent, SingularityBlackHoleEvent,
    SupplyPodDropEvent,
};
use crate::core::props::{ExplosiveBarrel, WeaponDrop, WeaponRarity};
use crate::core::types::*;
use crate::enemy::spawner::EnemyFactory;
use crate::enemy::traits::EnemyEntity;
use crate::skill::deployables::Deployable;
use crate::ui::hud::draw_hud;
use crate::ui::menus::*;
use crate::weapon::projectile::Projectile;

pub struct GameWorld {
    pub state: GameState,
    pub settings: GameSettings,
    pub settings_open: bool,
    pub player: Player,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub enemies: Vec<EnemyEntity>,
    pub allies: Vec<AlliedEntity>,
    pub deployables: Vec<Deployable>,
    pub events: Vec<Box<dyn BattlefieldEvent>>,
    pub weapon_drops: Vec<WeaponDrop>,
    pub barrels: Vec<ExplosiveBarrel>,
    pub particles: Vec<Particle>,
    pub decals: Vec<Decal>,
    pub casings: Vec<Casing>,
    pub pickups: Vec<Pickup>,
    pub float_texts: Vec<FloatText>,
    pub camera_pos: Vec2,
    pub screen_shake: f32,
    pub wave: u32,
    pub score: u64,
    pub combo: u32,
    pub combo_timer: f32,
    pub total_kills: u32,
    pub shots_fired: u64,
    pub shots_hit: u64,
    pub enemies_to_spawn: u32,
    pub enemies_spawned: u32,
    pub spawn_timer: f32,
    pub current_upgrade_choices: Vec<UpgradeOption>,
}

impl GameWorld {
    pub fn new() -> Self {
        let start_pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        let player = Player::new(Box::new(CyberMarine), start_pos);
        let mut world = Self {
            state: GameState::Start,
            settings: GameSettings::default(),
            settings_open: false,
            player,
            player_projectiles: Vec::new(),
            enemy_projectiles: Vec::new(),
            enemies: Vec::new(),
            allies: Vec::new(),
            deployables: Vec::new(),
            events: Vec::new(),
            weapon_drops: Vec::new(),
            barrels: Vec::new(),
            particles: Vec::new(),
            decals: Vec::new(),
            casings: Vec::new(),
            pickups: Vec::new(),
            float_texts: Vec::new(),
            camera_pos: start_pos,
            screen_shake: 0.0,
            wave: 1,
            score: 0,
            combo: 1,
            combo_timer: 0.0,
            total_kills: 0,
            shots_fired: 0,
            shots_hit: 0,
            enemies_to_spawn: 45,
            enemies_spawned: 0,
            spawn_timer: 0.0,
            current_upgrade_choices: Vec::new(),
        };
        world.populate_battlefield_props();
        world
    }

    /// Cộng độ rung màn hình đã được giảm chấn mượt theo `settings.screen_shake_intensity`
    pub fn add_screen_shake(&mut self, raw_amount: f32) {
        let scale = self.settings.screen_shake_intensity;
        if scale <= 0.001 {
            self.screen_shake = 0.0;
            return;
        }
        let max_cap = 7.5 * scale;
        self.screen_shake = (self.screen_shake + raw_amount * 0.32 * scale).min(max_cap);
    }

    pub fn populate_battlefield_props(&mut self) {
        let center = self.player.body.pos;
        // 1. Thả sẵn 2 Khoang Tiếp Tế Vũ Khí ngay gần người chơi khi bắt đầu
        for (i, offset) in [vec2(-130.0, -70.0), vec2(130.0, -70.0)].iter().enumerate() {
            let wid = if i == 0 {
                rand::gen_range(1, 4) // Shotgun, Minigun, Tesla
            } else {
                rand::gen_range(4, 7) // Railgun, Singularity, RPG
            };
            self.events.push(Box::new(SupplyPodDropEvent {
                landing_pos: center + *offset,
                timer: 0.0,
                max_time: 0.9 + (i as f32) * 0.25,
                weapon_id: wid,
                rarity: if i == 1 {
                    WeaponRarity::Epic
                } else {
                    WeaponRarity::Rare
                },
            }));
        }

        // 2. Triển khai sẵn 2 Lính Thủy Quân Lục Chiến Đồng Minh hộ tống
        for (i, offset) in [vec2(-55.0, 50.0), vec2(55.0, 50.0)].iter().enumerate() {
            self.allies.push(AlliedEntity::new(
                AlliedBody {
                    name: "ALLIED MARINE",
                    pos: center + *offset,
                    vel: Vec2::ZERO,
                    angle: 0.0,
                    radius: 13.0,
                    speed: 285.0,
                    color: Color::new(0.15, 0.85, 1.0, 1.0),
                    hp: 160.0,
                    max_hp: 160.0,
                    life: 45.0,
                },
                Box::new(AlliedMarine {
                    formation_offset: *offset,
                    fire_timer: (i as f32) * 0.08,
                }),
            ));
        }

        // 3. Rải 16 Thùng Nhiên Liệu Nổ Dây Chuyền (Explosive Barrels) quanh chiến trường
        for _ in 0..16 {
            let bx = rand::gen_range(250.0, WORLD_WIDTH - 250.0);
            let by = rand::gen_range(250.0, WORLD_HEIGHT - 250.0);
            if (vec2(bx, by) - center).length() > 160.0 {
                self.barrels.push(ExplosiveBarrel::new(vec2(bx, by)));
            }
        }
    }

    pub fn reset(&mut self) {
        let start_pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        self.player.reset(start_pos);
        self.player_projectiles.clear();
        self.enemy_projectiles.clear();
        self.enemies.clear();
        self.allies.clear();
        self.deployables.clear();
        self.events.clear();
        self.weapon_drops.clear();
        self.barrels.clear();
        self.particles.clear();
        self.decals.clear();
        self.casings.clear();
        self.pickups.clear();
        self.float_texts.clear();
        self.camera_pos = start_pos;
        self.screen_shake = 0.0;
        self.wave = 1;
        self.score = 0;
        self.combo = 1;
        self.combo_timer = 0.0;
        self.total_kills = 0;
        self.shots_fired = 0;
        self.shots_hit = 0;
        self.enemies_to_spawn = (45.0 * self.settings.swarm_scale).round() as u32;
        self.enemies_spawned = 0;
        self.spawn_timer = 0.0;
        self.populate_battlefield_props();
        self.state = GameState::Playing;
    }

    pub fn make_context(&self) -> CombatContext {
        let enemy_positions = self.enemies.iter().map(|e| e.body.pos).collect();
        CombatContext::new(
            self.player.body.pos,
            self.player.body.radius,
            self.player.body.is_dashing,
            enemy_positions,
        )
    }

    pub fn flush_commands(&mut self, commands: Vec<WorldCommand>) {
        let mut follow_up_commands: Vec<WorldCommand> = Vec::new();

        for cmd in commands {
            match cmd {
                WorldCommand::SpawnPlayerProjectile(proj) => {
                    self.player_projectiles.push(proj);
                }
                WorldCommand::SpawnEnemyProjectile(proj) => {
                    self.enemy_projectiles.push(proj);
                }
                WorldCommand::SpawnEnemy(enemy) => {
                    self.enemies.push(enemy);
                }
                WorldCommand::SpawnAlly(ally) => {
                    self.allies.push(ally);
                }
                WorldCommand::SpawnDeployable(dep) => {
                    self.deployables.push(dep);
                }
                WorldCommand::SpawnEvent(ev) => {
                    self.events.push(ev);
                }
                WorldCommand::SpawnWeaponDrop(drop) => {
                    self.weapon_drops.push(drop);
                }
                WorldCommand::TriggerExplosion {
                    pos,
                    radius,
                    damage,
                } => {
                    self.add_screen_shake(7.0);
                    self.decals.push(Decal {
                        pos,
                        radius: radius * 0.72,
                        color: Color::new(0.08, 0.08, 0.1, 0.72),
                        life: 12.0,
                        is_damage_zone: false,
                    });

                    let count = (26.0 * self.settings.particle_density).round() as usize;
                    for _ in 0..count {
                        let a = rand::gen_range(0.0, PI * 2.0);
                        let spd = rand::gen_range(60.0, 340.0);
                        self.particles.push(Particle {
                            pos,
                            vel: vec2(a.cos(), a.sin()) * spd,
                            color: if rand::gen_range(0, 2) == 0 {
                                Color::new(1.0, 0.25, 0.15, 1.0)
                            } else {
                                Color::new(1.0, 0.75, 0.0, 1.0)
                            },
                            radius: rand::gen_range(3.0, 7.0),
                            life: 0.45,
                            max_life: 0.45,
                        });
                    }

                    // Kích hoạt nổ dây chuyền các Thùng Nhiên Liệu (Explosive Barrels) gần đó!
                    for barrel in self.barrels.iter_mut() {
                        if barrel.fuse_timer.is_none()
                            && (barrel.pos - pos).length() < radius + barrel.radius
                        {
                            barrel.fuse_timer = Some(0.12);
                        }
                    }

                    let mut sub_ctx = CombatContext::new(
                        self.player.body.pos,
                        self.player.body.radius,
                        self.player.body.is_dashing,
                        Vec::new(),
                    );
                    for e in self.enemies.iter_mut() {
                        let dist = (e.body.pos - pos).length();
                        if dist < radius + e.body.radius + 50.0 {
                            let falloff = (1.0 - (dist / (radius + e.body.radius + 50.0)) * 0.5)
                                .clamp(0.2, 1.0);
                            let total_dmg = damage * falloff;
                            let kb = (e.body.pos - pos).normalize_or_zero() * 520.0;
                            let dealt = e.take_damage_at(pos, total_dmg, kb, None, &mut sub_ctx);
                            if self.settings.show_damage_numbers {
                                self.float_texts.push(FloatText {
                                    pos: e.body.pos,
                                    text: format!("{:.0}", dealt),
                                    color: Color::new(1.0, 0.35, 0.15, 1.0),
                                    size: 20.0,
                                    life: 0.75,
                                });
                            }
                        }
                    }
                    follow_up_commands.append(&mut sub_ctx.commands);
                }
                WorldCommand::ApplyVortexPull {
                    center,
                    radius,
                    pull_step,
                    damage_step,
                } => {
                    for e in self.enemies.iter_mut() {
                        let to_center = center - e.body.pos;
                        let dist = to_center.length();
                        if dist < radius + e.body.radius {
                            if pull_step > 0.0 && dist > 14.0 {
                                e.body.pos += to_center.normalize_or_zero() * pull_step;
                            }
                            e.body.hp -= damage_step;
                        }
                    }
                }
                WorldCommand::RailgunStrike {
                    start,
                    end,
                    width,
                    damage,
                } => {
                    self.add_screen_shake(8.0);
                    let seg = end - start;
                    let seg_len_sq = seg.length_squared().max(1.0);

                    let mut sub_ctx = CombatContext::new(
                        self.player.body.pos,
                        self.player.body.radius,
                        self.player.body.is_dashing,
                        Vec::new(),
                    );

                    for e in self.enemies.iter_mut() {
                        let t = ((e.body.pos - start).dot(seg) / seg_len_sq).clamp(0.0, 1.0);
                        let closest = start + seg * t;
                        let dist = (e.body.pos - closest).length();
                        if dist < width + e.body.radius {
                            let kb = seg.normalize_or_zero() * 350.0;
                            let dealt = e.take_damage_at(closest, damage, kb, None, &mut sub_ctx);
                            if self.settings.show_damage_numbers {
                                self.float_texts.push(FloatText {
                                    pos: e.body.pos,
                                    text: format!("{:.0} RAIL!", dealt),
                                    color: Color::new(0.0, 1.0, 0.9, 1.0),
                                    size: 22.0,
                                    life: 0.85,
                                });
                            }
                        }
                    }
                    follow_up_commands.append(&mut sub_ctx.commands);

                    // Hiệu ứng tia Railgun và các vòng sóng xung kích dọc đường đạn
                    self.events.push(Box::new(BeamArcVisualEvent {
                        points: vec![start, end],
                        color: Color::new(0.0, 0.95, 1.0, 1.0),
                        thickness: 6.5,
                        life: 0.28,
                        max_life: 0.28,
                    }));
                    for step in (120..1200).step_by(240) {
                        let ring_pos = start + seg.normalize_or_zero() * (step as f32);
                        self.events.push(Box::new(ShockwaveRingEvent {
                            center: ring_pos,
                            current_radius: 6.0,
                            max_radius: 65.0,
                            speed: 380.0,
                            color: Color::new(0.0, 1.0, 0.9, 0.8),
                            thickness: 3.0,
                        }));
                    }
                }
                WorldCommand::ChainLightning {
                    origin,
                    aim_dir,
                    max_jumps,
                    jump_range,
                    damage,
                } => {
                    let mut chain_points = vec![origin];
                    let mut hit_indices: Vec<usize> = Vec::new();
                    let mut current_pos = origin + aim_dir * 60.0;

                    let mut sub_ctx = CombatContext::new(
                        self.player.body.pos,
                        self.player.body.radius,
                        self.player.body.is_dashing,
                        Vec::new(),
                    );

                    for _ in 0..max_jumps {
                        let mut best_idx = None;
                        let mut best_dist = jump_range;
                        for (idx, e) in self.enemies.iter().enumerate() {
                            if hit_indices.contains(&idx) || e.body.hp <= 0.0 {
                                continue;
                            }
                            let d = (e.body.pos - current_pos).length();
                            if d < best_dist {
                                best_dist = d;
                                best_idx = Some(idx);
                            }
                        }

                        if let Some(idx) = best_idx {
                            hit_indices.push(idx);
                            let target_pos = self.enemies[idx].body.pos;
                            chain_points.push(target_pos);
                            let dealt = self.enemies[idx].take_damage_at(
                                target_pos,
                                damage,
                                Vec2::ZERO,
                                Some(StatusEffect::Stun { duration: 0.18 }),
                                &mut sub_ctx,
                            );
                            if self.settings.show_damage_numbers {
                                self.float_texts.push(FloatText {
                                    pos: target_pos,
                                    text: format!("{:.0} ZAP", dealt),
                                    color: Color::new(0.3, 0.95, 1.0, 1.0),
                                    size: 16.0,
                                    life: 0.65,
                                });
                            }
                            current_pos = target_pos;
                        } else {
                            break;
                        }
                    }

                    if chain_points.len() == 1 {
                        chain_points.push(origin + aim_dir * 240.0);
                    }
                    follow_up_commands.append(&mut sub_ctx.commands);

                    self.events.push(Box::new(BeamArcVisualEvent {
                        points: chain_points,
                        color: Color::new(0.25, 0.9, 1.0, 1.0),
                        thickness: 3.5,
                        life: 0.18,
                        max_life: 0.18,
                    }));
                }
                WorldCommand::DamagePlayer(dmg) => {
                    let shake = self.player.take_damage(dmg);
                    self.add_screen_shake(shake);
                }
                WorldCommand::AddScreenShake(s) => {
                    self.add_screen_shake(s);
                }
                WorldCommand::SpawnParticle(p) => {
                    if rand::gen_range(0.0, 1.0) <= self.settings.particle_density {
                        self.particles.push(p);
                    }
                }
                WorldCommand::SpawnDecal(d) => {
                    self.decals.push(d);
                }
                WorldCommand::SpawnFloatText(f) => {
                    self.float_texts.push(f);
                }
            }
        }

        if !follow_up_commands.is_empty() {
            self.flush_commands(follow_up_commands);
        }
    }

    pub fn step(&mut self, dt: f32) {
        if self.settings_open {
            if self.state == GameState::Paused {
                let _ = self.render_world();
            } else {
                clear_background(Color::new(0.03, 0.04, 0.07, 1.0));
            }
            if draw_settings_modal(&mut self.settings) {
                self.settings_open = false;
            }
            return;
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
            if self.state == GameState::Playing {
                self.state = GameState::Paused;
            } else if self.state == GameState::Paused {
                self.state = GameState::Playing;
            }
        } else if is_key_pressed(KeyCode::O) && self.state == GameState::Playing {
            self.state = GameState::Paused;
            self.settings_open = true;
            return;
        }

        match self.state {
            GameState::Start => {
                match draw_start_menu(
                    self.player.class.name(),
                    self.player.class.description(),
                    &self.settings,
                ) {
                    StartMenuAction::StartGame => {
                        self.enemies_to_spawn =
                            (45.0 * self.settings.swarm_scale).round() as u32;
                        self.state = GameState::Playing;
                    }
                    StartMenuAction::OpenSettings => {
                        self.settings_open = true;
                    }
                    StartMenuAction::None => {}
                }
            }
            GameState::Upgrade => {
                if let Some(chosen_id) = draw_upgrade_menu(&self.current_upgrade_choices) {
                    self.apply_upgrade(chosen_id);
                    self.wave += 1;
                    let base_count = (35 + self.wave * 20) as f32;
                    self.enemies_to_spawn = (base_count * self.settings.swarm_scale).round() as u32;
                    self.enemies_spawned = 0;
                    self.populate_battlefield_props();
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if draw_gameover_menu(
                    self.wave,
                    self.score,
                    self.total_kills,
                    self.shots_fired,
                    self.shots_hit,
                ) {
                    self.reset();
                }
            }
            GameState::Paused => {
                let _ = self.render_world();
                match draw_pause_menu() {
                    PauseMenuAction::Resume => {
                        self.state = GameState::Playing;
                    }
                    PauseMenuAction::OpenSettings => {
                        self.settings_open = true;
                    }
                    PauseMenuAction::Restart => {
                        self.reset();
                    }
                    PauseMenuAction::MainMenu => {
                        self.reset();
                        self.state = GameState::Start;
                    }
                    PauseMenuAction::None => {}
                }
            }
            GameState::Playing => {
                self.update_playing(dt);
                if self.render_world() {
                    self.state = GameState::Paused;
                    self.settings_open = true;
                }
            }
        }
    }

    fn apply_upgrade(&mut self, id: &str) {
        let m = &mut self.player.body.modifiers;
        match id {
            "bounce" => m.bouncing_bullets = true,
            "pierce" => m.pierce_count += 1,
            "vamp" => m.vampiric = true,
            "burn" => m.incendiary_rounds = true,
            "cryo" => m.cryo_rounds = true,
            "heavy" => m.dmg_multi += 0.25,
            "dash_fire" => m.dash_fire_trail = true,
            "overclock" => m.turret_overclock = true,
            _ => {}
        }
    }

    fn update_playing(&mut self, dt: f32) {
        self.player.body.bullet_time =
            is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let time_scale = if self.player.body.bullet_time && self.player.body.stamina > 0.0 {
            self.player.body.stamina = (self.player.body.stamina - dt * 45.0).max(0.0);
            0.4
        } else {
            self.player.body.bullet_time = false;
            self.player.body.stamina =
                (self.player.body.stamina + dt * 18.0).min(self.player.body.max_stamina);
            1.0
        };
        let sim_dt = dt * time_scale;

        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 26.0).max(0.0);
        }

        // Aiming
        let mouse_screen = vec2(mouse_position().0, mouse_position().1);
        let mouse_world = mouse_screen + self.camera_pos
            - vec2(screen_width() / 2.0, screen_height() / 2.0);
        self.player.body.angle =
            (mouse_world.y - self.player.body.pos.y).atan2(mouse_world.x - self.player.body.pos.x);

        // Commander Instant Hotkeys: [G] Drop Supply Pod, [B] Summon Colossus Boss
        if is_key_pressed(KeyCode::G) {
            self.events.push(Box::new(SupplyPodDropEvent {
                landing_pos: mouse_world,
                timer: 0.0,
                max_time: 0.9,
                weapon_id: rand::gen_range(1, 7),
                rarity: WeaponRarity::random(),
            }));
        }
        if is_key_pressed(KeyCode::B) {
            let spawn_pos = (self.player.body.pos + vec2(450.0, -250.0))
                .clamp(vec2(120.0, 120.0), vec2(WORLD_WIDTH - 120.0, WORLD_HEIGHT - 120.0));
            self.enemies
                .push(EnemyFactory::create_boss(spawn_pos, self.wave));
            self.float_texts.push(FloatText {
                pos: self.player.body.pos - vec2(0.0, 45.0),
                text: "WARNING: COLOSSUS FORTRESS DEPLOYED!".to_string(),
                color: Color::new(1.0, 0.1, 0.3, 1.0),
                size: 24.0,
                life: 1.5,
            });
        }

        // Shield Regen
        self.player.body.shield_regen_timer += dt;
        if self.player.body.shield_regen_timer > 4.0
            && self.player.body.shield < self.player.body.max_shield
        {
            self.player.body.shield =
                (self.player.body.shield + dt * 25.0).min(self.player.body.max_shield);
        }

        // Weapon Switching (1..7 - only switches if unlocked!)
        for (idx, key) in [
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::Key5,
            KeyCode::Key6,
            KeyCode::Key7,
        ]
        .iter()
        .enumerate()
        {
            if is_key_pressed(*key)
                && idx < self.player.weapons.len()
                && self.player.weapons[idx].stats.unlocked
            {
                self.player.current_weapon_idx = idx;
                self.player.reload_state.is_reloading = false;
            }
        }

        // Movement Input
        let mut move_dir = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 1.0;
        }

        // Skills
        let mut ctx = self.make_context();
        for skill in self.player.skills.iter_mut() {
            if skill.cooldown > 0.0 {
                skill.cooldown -= dt;
            }
            if is_key_pressed(skill.key) {
                skill.try_activate(&mut self.player.body, mouse_world, move_dir, &mut ctx);
            }
        }
        self.flush_commands(ctx.commands);

        // Player Movement & Dash
        if self.player.body.is_dashing {
            self.player.body.dash_duration -= dt;
            self.player.body.pos += self.player.body.vel * dt;
            if self.player.body.modifiers.dash_fire_trail {
                self.decals.push(Decal {
                    pos: self.player.body.pos,
                    radius: 16.0,
                    color: Color::new(1.0, 0.3, 0.0, 0.6),
                    life: 3.0,
                    is_damage_zone: true,
                });
            }
            if self.player.body.dash_duration <= 0.0 {
                self.player.body.is_dashing = false;
                self.player.body.vel = Vec2::ZERO;
            }
        } else if move_dir != Vec2::ZERO {
            self.player.body.pos += move_dir.normalize() * self.player.body.speed * dt;
        }

        let r = self.player.body.radius;
        self.player.body.pos.x = self.player.body.pos.x.clamp(r, WORLD_WIDTH - r);
        self.player.body.pos.y = self.player.body.pos.y.clamp(r, WORLD_HEIGHT - r);

        // Active Reload (R)
        self.handle_reload(dt);

        // Melee Slash (Right Click)
        if is_mouse_button_pressed(MouseButton::Right) {
            self.perform_melee();
        }

        // Weapon Firing (Left Click)
        if self.player.fire_timer > 0.0 {
            self.player.fire_timer -= dt;
        }
        if is_mouse_button_down(MouseButton::Left)
            && self.player.fire_timer <= 0.0
            && !self.player.reload_state.is_reloading
        {
            self.fire_current_weapon();
        }

        // Update Battlefield Events (Trait BattlefieldEvent)
        let mut ev_ctx = self.make_context();
        let mut active_events = std::mem::take(&mut self.events);
        active_events.retain_mut(|ev| ev.update(&mut ev_ctx, sim_dt));
        self.events.append(&mut active_events);
        self.flush_commands(ev_ctx.commands);

        // Update Allied Units (Trait AlliedBehavior)
        let mut ally_ctx = self.make_context();
        self.allies.retain_mut(|a| a.tick(&mut ally_ctx, sim_dt));
        self.flush_commands(ally_ctx.commands);

        // Update Deployables
        let mut dep_ctx = self.make_context();
        self.deployables
            .retain_mut(|d| d.update(&mut dep_ctx, sim_dt));
        self.flush_commands(dep_ctx.commands);

        // Update Explosive Barrels (Domino Chain Reactions)
        let mut barrel_ctx = self.make_context();
        for barrel in self.barrels.iter_mut() {
            if let Some(ref mut fuse) = barrel.fuse_timer {
                *fuse -= sim_dt;
                if *fuse <= 0.0 {
                    barrel.hp = 0.0;
                    barrel_ctx.trigger_explosion(barrel.pos, 165.0, 240.0);
                    barrel_ctx.spawn_event(Box::new(ShockwaveRingEvent {
                        center: barrel.pos,
                        current_radius: 10.0,
                        max_radius: 175.0,
                        speed: 540.0,
                        color: Color::new(1.0, 0.5, 0.0, 0.9),
                        thickness: 6.0,
                    }));
                }
            }
        }
        self.barrels.retain(|b| b.hp > 0.0);
        self.flush_commands(barrel_ctx.commands);

        // Update Player Projectiles
        let mut proj_ctx = self.make_context();
        for b in self.player_projectiles.iter_mut() {
            b.pos += b.vel * sim_dt;
            b.life -= sim_dt;

            if b.bounces > 0 {
                if b.pos.x < 0.0 || b.pos.x > WORLD_WIDTH {
                    b.vel.x *= -1.0;
                    b.bounces -= 1;
                }
                if b.pos.y < 0.0 || b.pos.y > WORLD_HEIGHT {
                    b.vel.y *= -1.0;
                    b.bounces -= 1;
                }
            }

            // Check hit against Explosive Barrels
            for barrel in self.barrels.iter_mut() {
                if (barrel.pos - b.pos).length() < barrel.radius + b.radius && b.life > 0.0 {
                    barrel.hp -= b.damage;
                    if barrel.hp <= 0.0 && barrel.fuse_timer.is_none() {
                        barrel.fuse_timer = Some(0.02);
                    }
                    if b.pierce == 0 {
                        b.life = 0.0;
                    }
                }
            }

            // Check hit against Enemies & Boss Parts
            for e in self.enemies.iter_mut() {
                if e.check_hit(b.pos, b.radius) && b.life > 0.0 {
                    self.shots_hit += 1;
                    if b.spawns_black_hole {
                        b.life = 0.0;
                        proj_ctx.spawn_event(Box::new(SingularityBlackHoleEvent {
                            pos: b.pos,
                            pull_radius: 260.0,
                            pull_force: 360.0,
                            dps: 95.0,
                            duration: 2.8,
                            max_duration: 2.8,
                            rotation: 0.0,
                        }));
                        break;
                    } else if let Some(blast_rad) = b.explosion_radius {
                        b.life = 0.0;
                        proj_ctx.trigger_explosion(b.pos, blast_rad, b.damage);
                        break;
                    } else {
                        let knockback = b.vel.normalize_or_zero() * 65.0;
                        let dealt =
                            e.take_damage_at(b.pos, b.damage, knockback, b.status_effect, &mut proj_ctx);
                        if self.settings.show_damage_numbers {
                            self.float_texts.push(FloatText {
                                pos: b.pos,
                                text: format!("{:.0}", dealt),
                                color: WHITE,
                                size: 14.0,
                                life: 0.55,
                            });
                        }
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
        self.player_projectiles.retain(|b| {
            b.life > 0.0
                && b.pos.x >= 0.0
                && b.pos.x <= WORLD_WIDTH
                && b.pos.y >= 0.0
                && b.pos.y <= WORLD_HEIGHT
        });
        self.flush_commands(proj_ctx.commands);

        // Massive Swarm Spawner (Spawns squads of 3-5 enemies at a time!)
        self.spawn_timer += sim_dt;
        let spawn_interval = (0.85 - (self.wave as f32) * 0.05).max(0.28);
        if self.spawn_timer >= spawn_interval && self.enemies_spawned < self.enemies_to_spawn {
            self.spawn_timer = 0.0;
            let squad_size = rand::gen_range(2, 5).min(self.enemies_to_spawn - self.enemies_spawned);
            let base_angle = rand::gen_range(0.0, PI * 2.0);
            let base_dist = rand::gen_range(620.0, 820.0);

            for s in 0..squad_size {
                let angle = base_angle + (s as f32) * 0.08;
                let ex = (self.player.body.pos.x + angle.cos() * base_dist)
                    .clamp(60.0, WORLD_WIDTH - 60.0);
                let ey = (self.player.body.pos.y + angle.sin() * base_dist)
                    .clamp(60.0, WORLD_HEIGHT - 60.0);

                let enemy =
                    EnemyFactory::spawn_for_wave(vec2(ex, ey), self.wave, self.enemies_spawned);
                self.enemies.push(enemy);
                self.enemies_spawned += 1;
            }
        }

        // Update Enemies
        let mut enemy_ctx = self.make_context();
        for e in self.enemies.iter_mut() {
            for d in self.decals.iter() {
                if d.is_damage_zone && (e.body.pos - d.pos).length() < d.radius + e.body.radius {
                    e.body.hp -= 35.0 * sim_dt;
                }
            }
            e.tick(&mut enemy_ctx, sim_dt);
        }
        self.flush_commands(enemy_ctx.commands);

        self.cleanup_dead_enemies();
        self.update_weapon_drops(dt);
        self.update_pickups(dt);

        // Update Enemy Projectiles
        let mut hit_shake = 0.0;
        for eb in self.enemy_projectiles.iter_mut() {
            eb.pos += eb.vel * sim_dt;
            eb.life -= sim_dt;
            if (eb.pos - self.player.body.pos).length() < eb.radius + self.player.body.radius
                && !self.player.body.is_dashing
            {
                hit_shake += self.player.take_damage(eb.damage);
                eb.life = 0.0;
            }
        }
        if hit_shake > 0.0 {
            self.add_screen_shake(hit_shake);
        }
        self.enemy_projectiles.retain(|eb| {
            eb.life > 0.0
                && eb.pos.x >= 0.0
                && eb.pos.x <= WORLD_WIDTH
                && eb.pos.y >= 0.0
                && eb.pos.y <= WORLD_HEIGHT
        });

        // Visual FX Updates
        for d in self.decals.iter_mut() {
            d.life -= dt;
        }
        self.decals.retain(|d| d.life > 0.0);

        for c in self.casings.iter_mut() {
            c.pos += c.vel * dt;
            c.vel *= 0.9;
            c.rotation += c.v_rot * dt;
            c.life -= dt;
        }
        self.casings.retain(|c| c.life > 0.0);

        for p in self.particles.iter_mut() {
            p.pos += p.vel * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        for f in self.float_texts.iter_mut() {
            f.pos.y -= 32.0 * dt;
            f.life -= dt;
        }
        self.float_texts.retain(|f| f.life > 0.0);

        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 1;
            }
        }

        if self.enemies_spawned >= self.enemies_to_spawn && self.enemies.is_empty() {
            self.state = GameState::Upgrade;
            self.current_upgrade_choices = get_random_upgrades();
        }

        if self.player.body.hp <= 0.0 {
            self.state = GameState::GameOver;
        }

        self.camera_pos += (self.player.body.pos - self.camera_pos) * 10.0 * dt;
    }

    fn update_weapon_drops(&mut self, dt: f32) {
        let p_pos = self.player.body.pos;
        let press_f = is_key_pressed(KeyCode::F);

        let mut i = 0;
        while i < self.weapon_drops.len() {
            self.weapon_drops[i].bob_phase += dt * 4.5;
            let dist = (self.weapon_drops[i].pos - p_pos).length();

            let should_pickup =
                (self.settings.auto_pickup_weapons && dist < 34.0) || (dist < 95.0 && press_f);

            if should_pickup {
                let drop = self.weapon_drops.swap_remove(i);
                if drop.weapon_id < self.player.weapons.len() {
                    let w = &mut self.player.weapons[drop.weapon_id];
                    let was_unlocked = w.stats.unlocked;
                    w.stats.unlocked = true;
                    if was_unlocked {
                        w.stats.level += 1;
                    }
                    if drop.rarity > w.stats.rarity {
                        w.stats.rarity = drop.rarity;
                    }
                    w.stats.current_clip = w.stats.clip_size;
                    if w.stats.reserve_ammo >= 0 {
                        w.stats.reserve_ammo += (w.stats.clip_size * 5) as i32;
                    }
                    let w_name = w.stats.name;
                    let w_lvl = w.stats.level;
                    let w_col = w.stats.rarity.color();

                    // Tự động chuyển sang khẩu súng mới vừa mở khóa (nếu cài đặt bật)!
                    if !was_unlocked && self.settings.auto_equip_new_weapon {
                        self.player.current_weapon_idx = drop.weapon_id;
                        self.player.reload_state.is_reloading = false;
                    }

                    self.float_texts.push(FloatText {
                        pos: p_pos - vec2(0.0, 35.0),
                        text: if !was_unlocked {
                            format!("UNLOCKED: {} [{}]!", w_name, drop.rarity.label())
                        } else {
                            format!("UPGRADED: {} Lv.{}!", w_name, w_lvl)
                        },
                        color: w_col,
                        size: 22.0,
                        life: 1.4,
                    });
                }
            } else {
                i += 1;
            }
        }
    }

    fn handle_reload(&mut self, dt: f32) {
        let clip = self.player.current_weapon().stats.current_clip;
        let max_clip = self.player.current_weapon().stats.clip_size;
        let reserve = self.player.current_weapon().stats.reserve_ammo;
        let reload_dur = self.player.current_weapon().stats.reload_duration;

        if is_key_pressed(KeyCode::R) {
            if !self.player.reload_state.is_reloading && clip < max_clip && reserve != 0 {
                self.player.reload_state.is_reloading = true;
                self.player.reload_state.timer = 0.0;
                self.player.reload_state.duration = reload_dur;
            } else if self.player.reload_state.is_reloading {
                let progress =
                    self.player.reload_state.timer / self.player.reload_state.duration;
                if (0.58..=0.82).contains(&progress) {
                    self.player.body.active_overcharge = true;
                    let mut ctx = self.make_context();
                    let w = self.player.current_weapon_mut();
                    let needed = w.stats.clip_size - w.stats.current_clip;
                    let can_load = if w.stats.reserve_ammo < 0 {
                        needed
                    } else {
                        needed.min(w.stats.reserve_ammo as u32)
                    };
                    w.stats.current_clip += can_load;
                    if w.stats.reserve_ammo > 0 {
                        w.stats.reserve_ammo -= can_load as i32;
                    }
                    w.behavior.on_perfect_reload(&mut w.stats, &mut ctx);
                    self.player.reload_state.is_reloading = false;
                    self.flush_commands(ctx.commands);
                    self.float_texts.push(FloatText {
                        pos: self.player.body.pos - vec2(0.0, 30.0),
                        text: "PERFECT RELOAD! +40% DMG".to_string(),
                        color: Color::new(0.0, 1.0, 0.4, 1.0),
                        size: 24.0,
                        life: 1.2,
                    });
                } else {
                    self.player.reload_state.timer =
                        (self.player.reload_state.timer - 0.5).max(0.0);
                    self.float_texts.push(FloatText {
                        pos: self.player.body.pos - vec2(0.0, 30.0),
                        text: "RELOAD JAMMED!".to_string(),
                        color: Color::new(1.0, 0.2, 0.2, 1.0),
                        size: 20.0,
                        life: 1.0,
                    });
                }
            }
        }

        if self.player.reload_state.is_reloading {
            self.player.reload_state.timer += dt;
            if self.player.reload_state.timer >= self.player.reload_state.duration {
                let w = self.player.current_weapon_mut();
                let needed = w.stats.clip_size - w.stats.current_clip;
                let can_load = if w.stats.reserve_ammo < 0 {
                    needed
                } else {
                    needed.min(w.stats.reserve_ammo as u32)
                };
                w.stats.current_clip += can_load;
                if w.stats.reserve_ammo > 0 {
                    w.stats.reserve_ammo -= can_load as i32;
                }
                self.player.reload_state.is_reloading = false;
                self.player.body.active_overcharge = false;
            }
        }
    }

    fn perform_melee(&mut self) {
        self.add_screen_shake(4.0);
        let slash_angle = self.player.body.angle;
        let p_pos = self.player.body.pos;
        let mut ctx = self.make_context();

        ctx.spawn_event(Box::new(ShockwaveRingEvent {
            center: p_pos + vec2(slash_angle.cos(), slash_angle.sin()) * 32.0,
            current_radius: 10.0,
            max_radius: 90.0,
            speed: 550.0,
            color: Color::new(0.0, 0.95, 1.0, 0.85),
            thickness: 4.5,
        }));

        for e in self.enemies.iter_mut() {
            let to_enemy = e.body.pos - p_pos;
            let dist = to_enemy.length();
            if dist < 95.0 + e.body.radius {
                let angle_diff = (slash_angle - to_enemy.y.atan2(to_enemy.x)).abs();
                if angle_diff < 0.95 || angle_diff > (PI * 2.0 - 0.95) {
                    let dealt = e.take_damage_at(
                        e.body.pos,
                        85.0,
                        to_enemy.normalize_or_zero() * 560.0,
                        Some(StatusEffect::Stun { duration: 0.45 }),
                        &mut ctx,
                    );
                    if self.settings.show_damage_numbers {
                        self.float_texts.push(FloatText {
                            pos: e.body.pos,
                            text: format!("{:.0} SLASH!", dealt),
                            color: Color::new(1.0, 0.84, 0.0, 1.0),
                            size: 20.0,
                            life: 0.8,
                        });
                    }
                }
            }
        }
        self.flush_commands(ctx.commands);
    }

    fn fire_current_weapon(&mut self) {
        let clip = self.player.current_weapon().stats.current_clip;
        let reserve = self.player.current_weapon().stats.reserve_ammo;
        let reload_dur = self.player.current_weapon().stats.reload_duration;

        if clip > 0 {
            let origin = self.player.body.pos;
            let aim = self.player.body.angle;
            let mods = self.player.body.modifiers.clone();
            let overcharged = self.player.body.active_overcharge;

            let mut ctx = self.make_context();
            let w = self.player.current_weapon_mut();
            w.stats.current_clip -= 1;
            let fire_rate = w.stats.fire_rate;
            let shake = w.stats.screen_shake;
            let stats_clone = w.stats.clone();
            w.behavior
                .fire(origin, aim, &stats_clone, &mods, overcharged, &mut ctx);

            self.player.fire_timer = fire_rate;
            self.add_screen_shake(shake * 0.5);
            self.shots_fired += 1;

            self.casings.push(Casing {
                pos: origin,
                vel: vec2((aim - PI / 2.0).cos(), (aim - PI / 2.0).sin())
                    * rand::gen_range(80.0, 160.0),
                rotation: rand::gen_range(0.0, PI * 2.0),
                v_rot: rand::gen_range(-10.0, 10.0),
                life: 5.0,
            });

            self.flush_commands(ctx.commands);
        } else if !self.player.reload_state.is_reloading && reserve != 0 {
            self.player.reload_state.is_reloading = true;
            self.player.reload_state.timer = 0.0;
            self.player.reload_state.duration = reload_dur;
        }
    }

    fn cleanup_dead_enemies(&mut self) {
        let mut death_ctx = self.make_context();
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].body.hp <= 0.0 {
                let mut dead = self.enemies.swap_remove(i);
                self.total_kills += 1;
                self.combo += 1;
                self.combo_timer = 3.5;
                self.score += dead.body.score_reward * (self.combo as u64);

                self.player
                    .class
                    .on_enemy_killed(&mut self.player.body, &mut death_ctx);
                dead.behavior.on_death(&dead.body, &mut death_ctx);

                // Tỉ lệ 9% quái thường cũng rơi ra Súng trên chiến trường
                if !dead.body.is_boss && rand::gen_range(0.0, 1.0) < 0.09 {
                    death_ctx.spawn_weapon_drop(WeaponDrop::new(
                        dead.body.pos,
                        rand::gen_range(1, 7),
                        WeaponRarity::random(),
                    ));
                }

                self.decals.push(Decal {
                    pos: dead.body.pos,
                    radius: dead.body.radius * 0.9,
                    color: Color::new(
                        dead.body.color.r,
                        dead.body.color.g,
                        dead.body.color.b,
                        0.32,
                    ),
                    life: 14.0,
                    is_damage_zone: false,
                });

                if rand::gen_range(0.0, 1.0) < 0.14 || dead.body.is_boss {
                    let ptypes = [
                        PickupType::Medkit,
                        PickupType::Ammo,
                        PickupType::Nuke,
                        PickupType::Overcharge,
                    ];
                    self.pickups.push(Pickup {
                        pos: dead.body.pos,
                        pickup_type: ptypes[rand::gen_range(0, ptypes.len())],
                        radius: 12.0,
                        pulse: 0.0,
                    });
                }
            } else {
                i += 1;
            }
        }
        if !death_ctx.commands.is_empty() {
            self.flush_commands(death_ctx.commands);
        }
    }

    fn update_pickups(&mut self, dt: f32) {
        let p_pos = self.player.body.pos;
        let p_rad = self.player.body.radius;
        let mut nuke_shake = 0.0;

        for p in self.pickups.iter_mut() {
            p.pulse += dt * 5.0;
            if (p.pos - p_pos).length() < p.radius + p_rad {
                match p.pickup_type {
                    PickupType::Medkit => {
                        self.player.body.hp =
                            (self.player.body.hp + 35.0).min(self.player.body.max_hp);
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: "+35 HP".to_string(),
                            color: GREEN,
                            size: 20.0,
                            life: 1.0,
                        });
                    }
                    PickupType::Ammo => {
                        for w in self.player.weapons.iter_mut() {
                            if w.stats.reserve_ammo >= 0 {
                                w.stats.reserve_ammo += (w.stats.clip_size * 4) as i32;
                            }
                        }
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: "MAX AMMO REFILLED!".to_string(),
                            color: GOLD,
                            size: 20.0,
                            life: 1.0,
                        });
                    }
                    PickupType::Nuke => {
                        nuke_shake += 10.0;
                        for e in self.enemies.iter_mut() {
                            if !e.body.is_boss {
                                e.body.hp = 0.0;
                            } else {
                                e.body.hp -= 500.0;
                            }
                        }
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: "NUKE ACTIVATED!".to_string(),
                            color: RED,
                            size: 28.0,
                            life: 1.5,
                        });
                    }
                    PickupType::Overcharge => {
                        self.player.body.active_overcharge = true;
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: "OVERCHARGE BUFF!".to_string(),
                            color: SKYBLUE,
                            size: 24.0,
                            life: 1.2,
                        });
                    }
                }
            }
        }
        if nuke_shake > 0.0 {
            self.add_screen_shake(nuke_shake);
        }
        self.pickups
            .retain(|p| (p.pos - p_pos).length() >= p.radius + p_rad);
    }

    pub fn render_world(&self) -> bool {
        clear_background(Color::new(0.05, 0.06, 0.08, 1.0));

        // Rung màn hình dạng sóng điều hòa mượt (không nhảy giật ngẫu nhiên gây hoa mắt)
        let shake_offset =
            if self.screen_shake > 0.01 && self.settings.screen_shake_intensity > 0.001 {
                let t = get_time() as f32;
                vec2(
                    (t * 42.0).sin() * self.screen_shake,
                    (t * 53.0).cos() * self.screen_shake,
                )
            } else {
                Vec2::ZERO
            };

        let cam_offset =
            vec2(screen_width() / 2.0, screen_height() / 2.0) - self.camera_pos + shake_offset;

        // 1. Arena Grid
        if self.settings.grid_opacity > 0.01 {
            let grid_size = 80.0;
            let start_x = (cam_offset.x % grid_size) - grid_size;
            let start_y = (cam_offset.y % grid_size) - grid_size;
            let grid_col = Color::new(0.0, 0.8, 1.0, 0.08 * self.settings.grid_opacity);
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
        }

        // Outer Boundary
        draw_rectangle_lines(
            cam_offset.x,
            cam_offset.y,
            WORLD_WIDTH,
            WORLD_HEIGHT,
            4.0,
            Color::new(0.0, 0.94, 1.0, 0.8),
        );

        // Decals
        for d in &self.decals {
            let sp = d.pos + cam_offset;
            draw_circle(sp.x, sp.y, d.radius, d.color);
        }

        // Casings
        for c in &self.casings {
            let sp = c.pos + cam_offset;
            draw_circle(sp.x, sp.y, 2.0, Color::new(0.85, 0.7, 0.2, 0.8));
        }

        // Explosive Barrels
        for b in &self.barrels {
            b.draw(cam_offset);
        }

        // Weapon Drops (Ground Loot with Hologram Beams)
        for wd in &self.weapon_drops {
            wd.draw(cam_offset, self.player.body.pos);
        }

        // Deployables
        for dep in &self.deployables {
            dep.draw(cam_offset);
        }

        // Allied Units
        for ally in &self.allies {
            ally.draw(cam_offset, self.settings.show_ally_bars);
        }

        // Pickups
        for p in &self.pickups {
            let sp = p.pos + cam_offset;
            let col = match p.pickup_type {
                PickupType::Medkit => Color::new(1.0, 0.2, 0.2, 1.0),
                PickupType::Ammo => Color::new(1.0, 0.84, 0.0, 1.0),
                PickupType::Nuke => Color::new(1.0, 0.5, 0.0, 1.0),
                PickupType::Overcharge => Color::new(0.0, 0.94, 1.0, 1.0),
            };
            draw_circle(sp.x, sp.y, p.radius, col);
        }

        // Battlefield Events (Black Holes, Orbital Lasers, Carpet Bombs, Telegraphs, Lightning Arcs)
        for ev in &self.events {
            ev.draw(cam_offset);
        }

        // Particles
        for p in &self.particles {
            let sp = p.pos + cam_offset;
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            draw_circle(
                sp.x,
                sp.y,
                p.radius,
                Color::new(p.color.r, p.color.g, p.color.b, alpha),
            );
        }

        // Enemies & Multi-Part Bosses
        for e in &self.enemies {
            e.draw(cam_offset);
        }

        // Player
        let p_sp = self.player.body.pos + cam_offset;
        if self.player.body.shield > 0.0 {
            draw_circle_lines(
                p_sp.x,
                p_sp.y,
                self.player.body.radius + 4.0,
                2.0,
                Color::new(0.0, 0.94, 1.0, 0.5),
            );
        }
        draw_circle(
            p_sp.x,
            p_sp.y,
            self.player.body.radius,
            Color::new(0.0, 0.94, 1.0, 1.0),
        );
        draw_circle(p_sp.x, p_sp.y, 8.0, Color::new(0.06, 0.09, 0.16, 1.0));
        let gun_dir = vec2(self.player.body.angle.cos(), self.player.body.angle.sin());
        draw_line(
            p_sp.x,
            p_sp.y,
            p_sp.x + gun_dir.x * 24.0,
            p_sp.y + gun_dir.y * 24.0,
            5.0,
            self.player.current_weapon().stats.rarity.color(),
        );

        // Projectiles
        for b in &self.player_projectiles {
            let sp = b.pos + cam_offset;
            draw_circle(sp.x, sp.y, b.radius, b.color);
        }
        for eb in &self.enemy_projectiles {
            let sp = eb.pos + cam_offset;
            draw_circle(sp.x, sp.y, eb.radius, eb.color);
        }

        // Float Texts
        for f in &self.float_texts {
            let sp = f.pos + cam_offset;
            draw_text(&f.text, sp.x, sp.y, f.size, f.color);
        }

        let active_boss = self
            .enemies
            .iter()
            .find(|e| e.body.is_boss)
            .map(|b| (b.body.name, b.body.hp, b.body.max_hp, b.body.parts.len()));

        draw_hud(
            &self.player,
            self.wave,
            self.score,
            self.combo,
            (self.enemies_to_spawn - self.enemies_spawned) + (self.enemies.len() as u32),
            self.allies.len(),
            active_boss,
            &self.settings,
        )
    }
}
