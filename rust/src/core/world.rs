use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::character::classes::CyberMarine;
use crate::character::player::Player;
use crate::core::context::{CombatContext, WorldCommand};
use crate::core::types::*;
use crate::enemy::spawner::EnemyFactory;
use crate::enemy::traits::EnemyEntity;
use crate::skill::deployables::Deployable;
use crate::ui::hud::draw_hud;
use crate::ui::menus::*;
use crate::weapon::projectile::Projectile;

pub struct GameWorld {
    pub state: GameState,
    pub player: Player,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub enemies: Vec<EnemyEntity>,
    pub deployables: Vec<Deployable>,
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
        Self {
            state: GameState::Start,
            player,
            player_projectiles: Vec::new(),
            enemy_projectiles: Vec::new(),
            enemies: Vec::new(),
            deployables: Vec::new(),
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
            enemies_to_spawn: 12,
            enemies_spawned: 0,
            spawn_timer: 0.0,
            current_upgrade_choices: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        let start_pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        self.player.reset(start_pos);
        self.player_projectiles.clear();
        self.enemy_projectiles.clear();
        self.enemies.clear();
        self.deployables.clear();
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
        self.enemies_to_spawn = 12;
        self.enemies_spawned = 0;
        self.spawn_timer = 0.0;
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
                WorldCommand::SpawnDeployable(dep) => {
                    self.deployables.push(dep);
                }
                WorldCommand::TriggerExplosion {
                    pos,
                    radius,
                    damage,
                } => {
                    self.screen_shake += 10.0;
                    self.decals.push(Decal {
                        pos,
                        radius: radius * 0.75,
                        color: Color::new(0.08, 0.08, 0.1, 0.7),
                        life: 12.0,
                        is_damage_zone: false,
                    });

                    for _ in 0..30 {
                        let a = rand::gen_range(0.0, PI * 2.0);
                        let spd = rand::gen_range(50.0, 320.0);
                        self.particles.push(Particle {
                            pos,
                            vel: vec2(a.cos(), a.sin()) * spd,
                            color: if rand::gen_range(0, 2) == 0 {
                                Color::new(1.0, 0.25, 0.15, 1.0)
                            } else {
                                Color::new(1.0, 0.7, 0.0, 1.0)
                            },
                            radius: rand::gen_range(3.0, 7.0),
                            life: 0.45,
                            max_life: 0.45,
                        });
                    }

                    for e in self.enemies.iter_mut() {
                        let dist = (e.body.pos - pos).length();
                        if dist < radius + e.body.radius {
                            let falloff = 1.0 - (dist / (radius + e.body.radius)) * 0.5;
                            let total_dmg = damage * falloff;
                            e.body.hp -= total_dmg;
                            e.body.vel += (e.body.pos - pos).normalize_or_zero() * 480.0;
                            self.float_texts.push(FloatText {
                                pos: e.body.pos,
                                text: format!("{:.0}", total_dmg),
                                color: Color::new(1.0, 0.3, 0.2, 1.0),
                                size: 20.0,
                                life: 0.75,
                            });
                        }
                    }
                }
                WorldCommand::DamagePlayer(dmg) => {
                    self.screen_shake += self.player.take_damage(dmg);
                }
                WorldCommand::AddScreenShake(s) => {
                    self.screen_shake += s;
                }
                WorldCommand::SpawnParticle(p) => {
                    self.particles.push(p);
                }
                WorldCommand::SpawnDecal(d) => {
                    self.decals.push(d);
                }
                WorldCommand::SpawnFloatText(f) => {
                    self.float_texts.push(f);
                }
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
            if self.state == GameState::Playing {
                self.state = GameState::Paused;
            } else if self.state == GameState::Paused {
                self.state = GameState::Playing;
            }
        }

        match self.state {
            GameState::Start => {
                if draw_start_menu(self.player.class.name(), self.player.class.description()) {
                    self.state = GameState::Playing;
                }
            }
            GameState::Upgrade => {
                if let Some(chosen_id) = draw_upgrade_menu(&self.current_upgrade_choices) {
                    self.apply_upgrade(chosen_id);
                    self.wave += 1;
                    self.enemies_to_spawn = 10 + self.wave * 6;
                    self.enemies_spawned = 0;
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
                self.render_world();
                draw_pause_menu();
            }
            GameState::Playing => {
                self.update_playing(dt);
                self.render_world();
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
        // Bullet-Time Stamina
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
            self.screen_shake = (self.screen_shake - dt * 14.0).max(0.0);
        }

        // Aiming
        let mouse_screen = vec2(mouse_position().0, mouse_position().1);
        let mouse_world = mouse_screen + self.camera_pos
            - vec2(screen_width() / 2.0, screen_height() / 2.0);
        self.player.body.angle =
            (mouse_world.y - self.player.body.pos.y).atan2(mouse_world.x - self.player.body.pos.x);

        // Shield Regen
        self.player.body.shield_regen_timer += dt;
        if self.player.body.shield_regen_timer > 4.0
            && self.player.body.shield < self.player.body.max_shield
        {
            self.player.body.shield =
                (self.player.body.shield + dt * 25.0).min(self.player.body.max_shield);
        }

        // Weapon Switching (1..5)
        for (idx, key) in [
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::Key5,
        ]
        .iter()
        .enumerate()
        {
            if is_key_pressed(*key) && idx < self.player.weapons.len() {
                self.player.current_weapon_idx = idx;
                self.player.reload_state.is_reloading = false;
            }
        }

        // Input Movement Vector
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

        // Skills cooldown & activation
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

        // Player Movement & Dash Physics
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

        // Update Deployables
        let mut dep_ctx = self.make_context();
        self.deployables
            .retain_mut(|d| d.update(&mut dep_ctx, sim_dt));
        self.flush_commands(dep_ctx.commands);

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

            for e in self.enemies.iter_mut() {
                if (e.body.pos - b.pos).length() < e.body.radius + b.radius && b.life > 0.0 {
                    self.shots_hit += 1;
                    if let Some(blast_rad) = b.explosion_radius {
                        b.life = 0.0;
                        proj_ctx.trigger_explosion(b.pos, blast_rad, b.damage);
                        break;
                    } else {
                        let knockback = b.vel.normalize_or_zero() * 65.0;
                        e.take_damage(b.damage, knockback, b.status_effect, &mut proj_ctx);
                        self.float_texts.push(FloatText {
                            pos: e.body.pos,
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
        self.player_projectiles.retain(|b| {
            b.life > 0.0
                && b.pos.x >= 0.0
                && b.pos.x <= WORLD_WIDTH
                && b.pos.y >= 0.0
                && b.pos.y <= WORLD_HEIGHT
        });
        self.flush_commands(proj_ctx.commands);

        // Spawn Enemies via EnemyFactory
        self.spawn_timer += sim_dt;
        let spawn_interval = (1.5 - (self.wave as f32) * 0.1).max(0.4);
        if self.spawn_timer >= spawn_interval && self.enemies_spawned < self.enemies_to_spawn {
            self.spawn_timer = 0.0;
            let spawn_angle = rand::gen_range(0.0, PI * 2.0);
            let spawn_dist = rand::gen_range(650.0, 850.0);
            let ex = (self.player.body.pos.x + spawn_angle.cos() * spawn_dist)
                .clamp(60.0, WORLD_WIDTH - 60.0);
            let ey = (self.player.body.pos.y + spawn_angle.sin() * spawn_dist)
                .clamp(60.0, WORLD_HEIGHT - 60.0);

            let enemy =
                EnemyFactory::spawn_for_wave(vec2(ex, ey), self.wave, self.enemies_spawned);
            self.enemies.push(enemy);
            self.enemies_spawned += 1;
        }

        // Update Enemies & Damage Zones
        let mut enemy_ctx = self.make_context();
        for e in self.enemies.iter_mut() {
            for d in self.decals.iter() {
                if d.is_damage_zone && (e.body.pos - d.pos).length() < d.radius + e.body.radius {
                    e.body.hp -= 32.0 * sim_dt;
                }
            }
            e.tick(&mut enemy_ctx, sim_dt);
        }
        self.flush_commands(enemy_ctx.commands);

        // Process Dead Enemies (including chain reactions like SplitterSlime or KamikazeDrone)
        self.cleanup_dead_enemies();

        // Update Pickups
        self.update_pickups(dt);

        // Update Enemy Projectiles
        for eb in self.enemy_projectiles.iter_mut() {
            eb.pos += eb.vel * sim_dt;
            eb.life -= sim_dt;
            if (eb.pos - self.player.body.pos).length() < eb.radius + self.player.body.radius
                && !self.player.body.is_dashing
            {
                self.screen_shake += self.player.take_damage(eb.damage);
                eb.life = 0.0;
            }
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
            f.pos.y -= 30.0 * dt;
            f.life -= dt;
        }
        self.float_texts.retain(|f| f.life > 0.0);

        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 1;
            }
        }

        // Check Wave Clear
        if self.enemies_spawned >= self.enemies_to_spawn && self.enemies.is_empty() {
            self.state = GameState::Upgrade;
            self.current_upgrade_choices = get_random_upgrades();
        }

        // Check Game Over
        if self.player.body.hp <= 0.0 {
            self.state = GameState::GameOver;
        }

        self.camera_pos += (self.player.body.pos - self.camera_pos) * 10.0 * dt;
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
        self.screen_shake += 5.0;
        let slash_angle = self.player.body.angle;
        let p_pos = self.player.body.pos;
        let mut ctx = self.make_context();

        for e in self.enemies.iter_mut() {
            let to_enemy = e.body.pos - p_pos;
            let dist = to_enemy.length();
            if dist < 88.0 + e.body.radius {
                let angle_diff = (slash_angle - to_enemy.y.atan2(to_enemy.x)).abs();
                if angle_diff < 0.9 || angle_diff > (PI * 2.0 - 0.9) {
                    e.take_damage(
                        75.0,
                        to_enemy.normalize_or_zero() * 520.0,
                        Some(StatusEffect::Stun { duration: 0.45 }),
                        &mut ctx,
                    );
                    self.float_texts.push(FloatText {
                        pos: e.body.pos,
                        text: "75 MELEE CRIT!".to_string(),
                        color: Color::new(1.0, 0.84, 0.0, 1.0),
                        size: 20.0,
                        life: 0.8,
                    });
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
            self.screen_shake += shake;
            self.shots_fired += 1;

            self.casings.push(Casing {
                pos: origin,
                vel: vec2((aim - PI / 2.0).cos(), (aim - PI / 2.0).sin())
                    * rand::gen_range(80.0, 150.0),
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

                // Blood decal
                self.decals.push(Decal {
                    pos: dead.body.pos,
                    radius: dead.body.radius * 0.9,
                    color: Color::new(
                        dead.body.color.r,
                        dead.body.color.g,
                        dead.body.color.b,
                        0.35,
                    ),
                    life: 16.0,
                    is_damage_zone: false,
                });

                if rand::gen_range(0.0, 1.0) < 0.16 || dead.body.is_boss {
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
                        if self.player.weapons.len() >= 5 {
                            self.player.weapons[1].stats.reserve_ammo =
                                (self.player.weapons[1].stats.reserve_ammo + 24).min(96);
                            self.player.weapons[2].stats.reserve_ammo =
                                (self.player.weapons[2].stats.reserve_ammo + 90).min(300);
                            self.player.weapons[3].stats.reserve_ammo =
                                (self.player.weapons[3].stats.reserve_ammo + 36).min(120);
                            self.player.weapons[4].stats.reserve_ammo =
                                (self.player.weapons[4].stats.reserve_ammo + 6).min(20);
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
                        self.screen_shake += 18.0;
                        for e in self.enemies.iter_mut() {
                            if !e.body.is_boss {
                                e.body.hp = 0.0;
                            } else {
                                e.body.hp -= 450.0;
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
        self.pickups
            .retain(|p| (p.pos - p_pos).length() >= p.radius + p_rad);
    }

    pub fn render_world(&self) {
        clear_background(Color::new(0.05, 0.06, 0.08, 1.0));

        let shake_offset = if self.screen_shake > 0.0 {
            vec2(
                rand::gen_range(-self.screen_shake, self.screen_shake),
                rand::gen_range(-self.screen_shake, self.screen_shake),
            )
        } else {
            Vec2::ZERO
        };

        let cam_offset =
            vec2(screen_width() / 2.0, screen_height() / 2.0) - self.camera_pos + shake_offset;

        // 1. Arena Grid
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

        // Deployables
        for dep in &self.deployables {
            dep.draw(cam_offset);
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

        // Enemies
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
            Color::new(0.4, 0.5, 0.6, 1.0),
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

        // Find active boss if any
        let active_boss = self
            .enemies
            .iter()
            .find(|e| e.body.is_boss)
            .map(|b| (b.body.name, b.body.hp, b.body.max_hp));

        draw_hud(
            &self.player,
            self.wave,
            self.score,
            self.combo,
            (self.enemies_to_spawn - self.enemies_spawned) + (self.enemies.len() as u32),
            active_boss,
        );
    }
}
