/**
 * CYBER SURVIVOR - 2D Top-Down Tactical Action Shooter
 * Built with HTML5 Canvas & Web Audio API
 */

// ==========================================
// 1. PROCEDURAL WEB AUDIO SYNTHESIZER
// ==========================================
class SoundFX {
  constructor() {
    this.ctx = null;
    this.masterGain = null;
    this.muted = false;
  }

  init() {
    if (this.ctx) return;
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    this.ctx = new AudioContext();
    this.masterGain = this.ctx.createGain();
    this.masterGain.gain.setValueAtTime(0.35, this.ctx.currentTime);
    this.masterGain.connect(this.ctx.destination);
  }

  resume() {
    if (this.ctx && this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  shootPistol() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(600, t);
    osc.frequency.exponentialRampToValueAtTime(100, t + 0.08);

    gain.gain.setValueAtTime(0.4, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.08);

    osc.connect(gain);
    gain.connect(this.masterGain);

    osc.start(t);
    osc.stop(t + 0.08);
  }

  shootShotgun() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    // Low punch
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'triangle';
    osc.frequency.setValueAtTime(160, t);
    osc.frequency.exponentialRampToValueAtTime(30, t + 0.18);
    gain.gain.setValueAtTime(0.8, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.18);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.18);

    // Noise blast
    this.playNoise(0.14, 800, 0.6);
  }

  shootRifle() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'square';
    osc.frequency.setValueAtTime(450, t);
    osc.frequency.exponentialRampToValueAtTime(80, t + 0.07);
    gain.gain.setValueAtTime(0.35, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.07);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.07);
  }

  shootPlasma() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(900, t);
    osc.frequency.exponentialRampToValueAtTime(200, t + 0.14);
    gain.gain.setValueAtTime(0.5, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.14);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.14);
  }

  shootRPG() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(200, t);
    osc.frequency.linearRampToValueAtTime(500, t + 0.1);
    gain.gain.setValueAtTime(0.5, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.15);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.15);
  }

  playNoise(duration, filterFreq, volume) {
    if (!this.ctx || this.muted) return;
    const bufferSize = this.ctx.sampleRate * duration;
    const buffer = this.ctx.createBuffer(1, bufferSize, this.ctx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < bufferSize; i++) {
      data[i] = Math.random() * 2 - 1;
    }
    const noise = this.ctx.createBufferSource();
    noise.buffer = buffer;

    const filter = this.ctx.createBiquadFilter();
    filter.type = 'lowpass';
    filter.frequency.setValueAtTime(filterFreq, this.ctx.currentTime);

    const gain = this.ctx.createGain();
    gain.gain.setValueAtTime(volume, this.ctx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + duration);

    noise.connect(filter);
    filter.connect(gain);
    gain.connect(this.masterGain);

    noise.start();
  }

  explosion(scale = 1) {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    // Sub rumble
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'triangle';
    osc.frequency.setValueAtTime(100, t);
    osc.frequency.exponentialRampToValueAtTime(20, t + 0.5 * scale);
    gain.gain.setValueAtTime(0.9, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.5 * scale);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.5 * scale);

    this.playNoise(0.4 * scale, 400, 0.8);
  }

  hit(isCrit = false) {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = isCrit ? 'square' : 'triangle';
    osc.frequency.setValueAtTime(isCrit ? 800 : 350, t);
    osc.frequency.exponentialRampToValueAtTime(120, t + 0.05);
    gain.gain.setValueAtTime(0.3, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.05);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.05);
  }

  playerHurt() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(150, t);
    osc.frequency.exponentialRampToValueAtTime(60, t + 0.2);
    gain.gain.setValueAtTime(0.7, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.2);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.2);
  }

  dash() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(250, t);
    osc.frequency.exponentialRampToValueAtTime(650, t + 0.12);
    gain.gain.setValueAtTime(0.4, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.12);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.12);
  }

  melee() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(400, t);
    osc.frequency.exponentialRampToValueAtTime(100, t + 0.15);
    gain.gain.setValueAtTime(0.6, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.15);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.15);
  }

  activeReloadSuccess() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    [523.25, 659.25, 783.99, 1046.50].forEach((freq, idx) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'sine';
      osc.frequency.setValueAtTime(freq, t + idx * 0.05);
      gain.gain.setValueAtTime(0.3, t + idx * 0.05);
      gain.gain.exponentialRampToValueAtTime(0.01, t + idx * 0.05 + 0.15);
      osc.connect(gain);
      gain.connect(this.masterGain);
      osc.start(t + idx * 0.05);
      osc.stop(t + idx * 0.05 + 0.15);
    });
  }

  activeReloadFail() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(120, t);
    osc.frequency.setValueAtTime(90, t + 0.1);
    gain.gain.setValueAtTime(0.4, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.25);
    osc.connect(gain);
    gain.connect(this.masterGain);
    osc.start(t);
    osc.stop(t + 0.25);
  }

  powerup() {
    if (!this.ctx || this.muted) return;
    const t = this.ctx.currentTime;
    [330, 440, 550, 660, 880].forEach((freq, i) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, t + i * 0.04);
      gain.gain.setValueAtTime(0.3, t + i * 0.04);
      gain.gain.exponentialRampToValueAtTime(0.01, t + i * 0.04 + 0.12);
      osc.connect(gain);
      gain.connect(this.masterGain);
      osc.start(t + i * 0.04);
      osc.stop(t + i * 0.04 + 0.12);
    });
  }
}

// ==========================================
// 2. WEAPONS DEFINITION
// ==========================================
const WEAPONS = {
  1: {
    id: 1,
    name: 'PISTOL',
    damage: 26,
    fireRate: 0.16, // seconds between shots
    spread: 0.04,
    speed: 1100,
    bulletRadius: 3.5,
    bulletColor: '#00f0ff',
    clipSize: 15,
    currentClip: 15,
    reserveAmmo: Infinity,
    reloadDuration: 1.1,
    burstCount: 1,
    screenShake: 2,
    recoil: 20
  },
  2: {
    id: 2,
    name: 'SHOTGUN',
    damage: 18,
    pellets: 7,
    fireRate: 0.65,
    spread: 0.28,
    speed: 950,
    bulletRadius: 3,
    bulletColor: '#ffaa00',
    clipSize: 8,
    currentClip: 8,
    reserveAmmo: 48,
    reloadDuration: 1.8,
    screenShake: 6,
    recoil: 80
  },
  3: {
    id: 3,
    name: 'RIFLE',
    damage: 22,
    fireRate: 0.09,
    spread: 0.08,
    speed: 1250,
    bulletRadius: 3.5,
    bulletColor: '#ffd700',
    clipSize: 30,
    currentClip: 30,
    reserveAmmo: 180,
    reloadDuration: 1.4,
    screenShake: 2.5,
    recoil: 35
  },
  4: {
    id: 4,
    name: 'PLASMA',
    damage: 48,
    fireRate: 0.28,
    spread: 0.02,
    speed: 800,
    bulletRadius: 7,
    bulletColor: '#a855f7',
    clipSize: 18,
    currentClip: 18,
    reserveAmmo: 72,
    reloadDuration: 1.6,
    pierce: 2,
    screenShake: 4,
    recoil: 40
  },
  5: {
    id: 5,
    name: 'RPG',
    damage: 180,
    fireRate: 0.9,
    spread: 0.01,
    speed: 650,
    bulletRadius: 8,
    bulletColor: '#ff3344',
    isRocket: true,
    explosionRadius: 130,
    clipSize: 4,
    currentClip: 4,
    reserveAmmo: 12,
    reloadDuration: 2.2,
    screenShake: 12,
    recoil: 120
  }
};

// ==========================================
// 3. MAIN GAME ENGINE
// ==========================================
class Game {
  constructor() {
    this.canvas = document.getElementById('gameCanvas');
    this.ctx = this.canvas.getContext('2d');
    this.minimapCanvas = document.getElementById('minimapCanvas');
    this.mctx = this.minimapCanvas.getContext('2d');

    this.sfx = new SoundFX();

    // Map & World Dimensions
    this.worldWidth = 2600;
    this.worldHeight = 2600;

    // Viewport & Camera
    this.camera = { x: 0, y: 0, shake: 0, shakeDuration: 0 };

    // Player Object
    this.player = {
      x: this.worldWidth / 2,
      y: this.worldHeight / 2,
      vx: 0,
      vy: 0,
      radius: 18,
      angle: 0,
      speed: 310,
      hp: 100,
      maxHp: 100,
      shield: 50,
      maxShield: 50,
      shieldRegenTimer: 0,
      stamina: 100,
      maxStamina: 100,
      dashCooldown: 0,
      dashMaxCd: 1.2,
      isDashing: false,
      dashDuration: 0,
      dashTrailTimer: 0,
      activeOvercharge: false, // from active reload
      bulletTime: false,
      
      // Skills
      grenadeCd: 0,
      grenadeMaxCd: 8,
      turretCd: 0,
      turretMaxCd: 16,

      // Upgrades / Perks
      perks: {
        bouncingBullets: false,
        pierceCount: 0,
        vampiric: false,
        dashFireTrail: false,
        bulletDamageMulti: 1.0,
        dashCdMulti: 1.0,
        turretOverclock: false
      }
    };

    // Weapons
    this.weapons = JSON.parse(JSON.stringify(WEAPONS));
    this.currentWeaponId = 1;
    this.fireTimer = 0;
    this.isReloading = false;
    this.reloadTimer = 0;
    this.reloadSweetSpotHit = false;

    // Mouse & Keys State
    this.keys = {};
    this.mouse = { x: 0, y: 0, worldX: 0, worldY: 0, down: false };

    // Entities
    this.bullets = [];
    this.enemyBullets = [];
    this.enemies = [];
    this.particles = [];
    this.casings = [];
    this.decals = []; // Blood stains, scorch marks on floor
    this.turrets = [];
    this.grenades = [];
    this.pickups = [];
    this.floatTexts = [];

    // Wave / Game States
    this.gameState = 'START'; // START, PLAYING, UPGRADE, GAMEOVER, PAUSED
    this.wave = 1;
    this.score = 0;
    this.combo = 1;
    this.comboTimer = 0;
    this.totalKills = 0;
    this.shotsFired = 0;
    this.shotsHit = 0;
    this.enemiesSpawnedInWave = 0;
    this.enemiesToSpawn = 12;
    this.spawnTimer = 0;
    this.activeBoss = null;

    // Time Tracking
    this.lastTime = performance.now();

    // DOM Elements Cache
    this.dom = {
      waveNum: document.getElementById('wave-num'),
      scoreVal: document.getElementById('score-val'),
      comboDisplay: document.getElementById('combo-display'),
      enemiesNum: document.getElementById('enemies-num'),
      hpBar: document.getElementById('hp-bar'),
      hpText: document.getElementById('hp-text'),
      shieldBar: document.getElementById('shield-bar'),
      shieldText: document.getElementById('shield-text'),
      dashBar: document.getElementById('dash-bar'),
      adrenBar: document.getElementById('adren-bar'),
      curClip: document.getElementById('cur-clip'),
      maxClip: document.getElementById('max-clip'),
      reserveAmmo: document.getElementById('reserve-ammo'),
      activeReloadWrap: document.getElementById('active-reload-wrap'),
      reloadIndicator: document.getElementById('reload-indicator'),
      cdGrenade: document.getElementById('cd-grenade'),
      cdTurret: document.getElementById('cd-turret'),
      bossHud: document.getElementById('boss-hud'),
      bossName: document.getElementById('boss-name'),
      bossHpFill: document.getElementById('boss-hp-fill'),
      combatLog: document.getElementById('combat-log'),
      startOverlay: document.getElementById('start-overlay'),
      upgradeOverlay: document.getElementById('upgrade-overlay'),
      upgradeCards: document.getElementById('upgrade-cards'),
      gameoverOverlay: document.getElementById('gameover-overlay'),
      pauseOverlay: document.getElementById('pause-overlay'),
      finalWave: document.getElementById('final-wave'),
      finalScore: document.getElementById('final-score'),
      finalKills: document.getElementById('final-kills'),
      finalAcc: document.getElementById('final-acc')
    };

    this.initEventListeners();
    this.resizeCanvas();
    window.addEventListener('resize', () => this.resizeCanvas());

    // Start Animation Loop
    requestAnimationFrame((t) => this.loop(t));
  }

  resizeCanvas() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  initEventListeners() {
    // Keyboard Down
    window.addEventListener('keydown', (e) => {
      const code = e.code;
      this.keys[code] = true;

      if (this.gameState === 'START') {
        if (code === 'Enter' || code === 'Space') this.startGame();
        return;
      }

      if (this.gameState === 'GAMEOVER') {
        if (code === 'KeyR' || code === 'Enter' || code === 'Space') this.restartGame();
        return;
      }

      if (this.gameState === 'PLAYING') {
        // Weapon hotkeys
        if (e.key >= '1' && e.key <= '5') {
          this.switchWeapon(parseInt(e.key));
        }

        // Active Reload Key (R)
        if (code === 'KeyR') {
          this.handleReloadKey();
        }

        // Dash Key (Space)
        if (code === 'Space') {
          e.preventDefault();
          this.performDash();
        }

        // Skills
        if (code === 'KeyQ') {
          this.useGrenade();
        }
        if (code === 'KeyE') {
          this.useTurret();
        }

        // Bullet time (Shift)
        if (code === 'ShiftLeft' || code === 'ShiftRight') {
          this.player.bulletTime = true;
        }

        // Pause
        if (code === 'Escape' || code === 'KeyP') {
          this.togglePause();
        }
      } else if (this.gameState === 'PAUSED') {
        if (code === 'Escape' || code === 'KeyP') {
          this.togglePause();
        }
      }
    });

    // Keyboard Up
    window.addEventListener('keyup', (e) => {
      this.keys[e.code] = false;
      if (e.code === 'ShiftLeft' || e.code === 'ShiftRight') {
        this.player.bulletTime = false;
      }
    });

    // Mouse Move
    window.addEventListener('mousemove', (e) => {
      this.mouse.x = e.clientX;
      this.mouse.y = e.clientY;
    });

    // Mouse Down
    window.addEventListener('mousedown', (e) => {
      this.sfx.init();
      this.sfx.resume();

      if (this.gameState !== 'PLAYING') return;

      if (e.button === 0) { // Left Click
        this.mouse.down = true;
      } else if (e.button === 2) { // Right Click (Melee)
        e.preventDefault();
        this.performMelee();
      }
    });

    window.addEventListener('mouseup', (e) => {
      if (e.button === 0) {
        this.mouse.down = false;
      }
    });

    // Mouse Wheel weapon cycle
    window.addEventListener('wheel', (e) => {
      if (this.gameState !== 'PLAYING') return;
      if (e.deltaY > 0) {
        let next = this.currentWeaponId + 1;
        if (next > 5) next = 1;
        this.switchWeapon(next);
      } else {
        let prev = this.currentWeaponId - 1;
        if (prev < 1) prev = 5;
        this.switchWeapon(prev);
      }
    });

    // Prevent context menu
    window.addEventListener('contextmenu', (e) => e.preventDefault());

    // UI Buttons
    document.getElementById('btn-start').addEventListener('click', () => this.startGame());
    document.getElementById('btn-restart').addEventListener('click', () => this.restartGame());
    document.getElementById('btn-resume').addEventListener('click', () => this.togglePause());

    // Weapon slot clicks
    document.querySelectorAll('.weapon-slot').forEach(el => {
      el.addEventListener('click', () => {
        const slot = parseInt(el.dataset.slot);
        if (slot) this.switchWeapon(slot);
      });
    });
  }

  startGame() {
    this.sfx.init();
    this.sfx.resume();
    this.dom.startOverlay.classList.add('hidden');
    this.gameState = 'PLAYING';
    this.startWave(1);
    this.logCombat('HỆ THỐNG ĐÃ KÍCH HOẠT. TIÊU DIỆT TẤT CẢ KẺ ĐỊCH!');
  }

  restartGame() {
    this.dom.gameoverOverlay.classList.add('hidden');
    // Reset player stats
    this.player.x = this.worldWidth / 2;
    this.player.y = this.worldHeight / 2;
    this.player.hp = 100;
    this.player.shield = 50;
    this.player.stamina = 100;
    this.player.perks = {
      bouncingBullets: false,
      pierceCount: 0,
      vampiric: false,
      dashFireTrail: false,
      bulletDamageMulti: 1.0,
      dashCdMulti: 1.0,
      turretOverclock: false
    };

    // Reset weapons
    this.weapons = JSON.parse(JSON.stringify(WEAPONS));
    this.currentWeaponId = 1;
    this.isReloading = false;
    this.reloadTimer = 0;
    this.dom.activeReloadWrap.classList.add('hidden');

    // Clear entities
    this.bullets = [];
    this.enemyBullets = [];
    this.enemies = [];
    this.particles = [];
    this.casings = [];
    this.decals = [];
    this.turrets = [];
    this.grenades = [];
    this.pickups = [];
    this.floatTexts = [];

    // Stats
    this.score = 0;
    this.combo = 1;
    this.totalKills = 0;
    this.shotsFired = 0;
    this.shotsHit = 0;

    this.gameState = 'PLAYING';
    this.startWave(1);
    this.updateHUD();
  }

  togglePause() {
    if (this.gameState === 'PLAYING') {
      this.gameState = 'PAUSED';
      this.dom.pauseOverlay.classList.remove('hidden');
    } else if (this.gameState === 'PAUSED') {
      this.gameState = 'PLAYING';
      this.dom.pauseOverlay.classList.add('hidden');
    }
  }

  switchWeapon(id) {
    if (this.currentWeaponId === id) return;
    this.currentWeaponId = id;
    this.isReloading = false;
    this.dom.activeReloadWrap.classList.add('hidden');

    document.querySelectorAll('.weapon-slot').forEach(el => {
      if (parseInt(el.dataset.slot) === id) {
        el.classList.add('active');
      } else {
        el.classList.remove('active');
      }
    });

    const w = this.weapons[id];
    this.logCombat(`ĐÃ CHUYỂN SANG VŨ KHÍ: [${w.name}]`);
    this.updateAmmoDisplay();
  }

  handleReloadKey() {
    const w = this.weapons[this.currentWeaponId];
    if (w.currentClip >= w.clipSize) return; // already full
    if (w.reserveAmmo <= 0) return; // out of ammo

    if (!this.isReloading) {
      // Start Reloading
      this.isReloading = true;
      this.reloadTimer = 0;
      this.reloadSweetSpotHit = false;
      this.dom.activeReloadWrap.classList.remove('hidden');
    } else {
      // Active reload attempt
      const progress = this.reloadTimer / w.reloadDuration;
      // Sweet spot is between 60% and 80%
      if (progress >= 0.58 && progress <= 0.82) {
        // SUCCESS ACTIVE RELOAD!
        this.sfx.activeReloadSuccess();
        this.finishReload(true);
        this.addFloatText(this.player.x, this.player.y - 30, 'PERFECT RELOAD! +40% DMG', '#00ff66', 22);
        this.logCombat('NẠP ĐẠN HOÀN HẢO! TĂNG 40% SÁT THƯƠNG ĐỢT NÀY!');
      } else {
        // Failed / jammed active reload
        this.sfx.activeReloadFail();
        this.reloadTimer -= 0.5; // penalty delay
        this.addFloatText(this.player.x, this.player.y - 30, 'RELOAD JAMMED!', '#ff3344', 18);
      }
    }
  }

  finishReload(isPerfect = false) {
    const w = this.weapons[this.currentWeaponId];
    const needed = w.clipSize - w.currentClip;
    const canLoad = Math.min(needed, w.reserveAmmo);

    w.currentClip += canLoad;
    if (w.reserveAmmo !== Infinity) {
      w.reserveAmmo -= canLoad;
    }

    this.player.activeOvercharge = isPerfect;
    this.isReloading = false;
    this.dom.activeReloadWrap.classList.add('hidden');
    this.updateAmmoDisplay();
  }

  performDash() {
    if (this.player.dashCooldown > 0 || this.player.isDashing) return;

    // Determine direction from keys or player angle
    let dx = 0, dy = 0;
    if (this.keys['KeyW'] || this.keys['ArrowUp']) dy -= 1;
    if (this.keys['KeyS'] || this.keys['ArrowDown']) dy += 1;
    if (this.keys['KeyA'] || this.keys['ArrowLeft']) dx -= 1;
    if (this.keys['KeyD'] || this.keys['ArrowRight']) dx += 1;

    let angle = this.player.angle;
    if (dx !== 0 || dy !== 0) {
      angle = Math.atan2(dy, dx);
    }

    this.player.isDashing = true;
    this.player.dashDuration = 0.22;
    this.player.dashCooldown = this.player.dashMaxCd * this.player.perks.dashCdMulti;
    this.player.vx = Math.cos(angle) * 780;
    this.player.vy = Math.sin(angle) * 780;

    this.sfx.dash();
    this.addScreenShake(3);

    // Dash particle burst
    for (let i = 0; i < 15; i++) {
      this.particles.push({
        x: this.player.x,
        y: this.player.y,
        vx: -Math.cos(angle) * 200 + (Math.random() - 0.5) * 150,
        vy: -Math.sin(angle) * 200 + (Math.random() - 0.5) * 150,
        radius: Math.random() * 4 + 2,
        color: '#ffcc00',
        alpha: 1,
        life: 0.35
      });
    }
  }

  performMelee() {
    this.sfx.melee();
    this.addScreenShake(4);

    const slashAngle = this.player.angle;
    const slashRange = 75;

    // Melee swing particle arc
    for (let a = -0.6; a <= 0.6; a += 0.15) {
      const pAngle = slashAngle + a;
      this.particles.push({
        x: this.player.x + Math.cos(pAngle) * 35,
        y: this.player.y + Math.sin(pAngle) * 35,
        vx: Math.cos(pAngle) * 280,
        vy: Math.sin(pAngle) * 280,
        radius: 4,
        color: '#00f0ff',
        alpha: 1,
        life: 0.18
      });
    }

    // Damage and pushback enemies
    this.enemies.forEach(e => {
      const dist = Math.hypot(e.x - this.player.x, e.y - this.player.y);
      if (dist < slashRange + e.radius) {
        const angleToEnemy = Math.atan2(e.y - this.player.y, e.x - this.player.x);
        let angleDiff = Math.abs(slashAngle - angleToEnemy);
        while (angleDiff > Math.PI) angleDiff = Math.abs(angleDiff - Math.PI * 2);

        if (angleDiff < 0.9) {
          // Hit enemy
          const dmg = 65;
          e.hp -= dmg;
          e.vx += Math.cos(angleToEnemy) * 450;
          e.vy += Math.sin(angleToEnemy) * 450;
          this.sfx.hit(true);
          this.addFloatText(e.x, e.y, `${dmg} CRIT!`, '#ffd700', 18);
          this.createBloodSplatter(e.x, e.y, 10, '#00ff66');
        }
      }
    });
  }

  useGrenade() {
    if (this.player.grenadeCd > 0) return;
    this.player.grenadeCd = this.player.grenadeMaxCd;

    const angle = this.player.angle;
    const distToMouse = Math.min(Math.hypot(this.mouse.worldX - this.player.x, this.mouse.worldY - this.player.y), 450);
    const speed = 480;
    const travelTime = distToMouse / speed;

    this.grenades.push({
      x: this.player.x,
      y: this.player.y,
      targetX: this.player.x + Math.cos(angle) * distToMouse,
      targetY: this.player.y + Math.sin(angle) * distToMouse,
      vx: Math.cos(angle) * speed,
      vy: Math.sin(angle) * speed,
      fuse: travelTime + 0.35,
      radius: 6
    });

    this.logCombat('ĐÃ NÉM LỰU ĐẠN NỔ CHÙM!');
  }

  useTurret() {
    if (this.player.turretCd > 0) return;
    this.player.turretCd = this.player.turretMaxCd;

    this.turrets.push({
      x: this.player.x,
      y: this.player.y,
      angle: 0,
      fireTimer: 0,
      life: 25,
      maxLife: 25,
      hp: 150
    });

    this.logCombat('ĐÃ TRIỂN KHAI TRỤ SÚNG TỰ ĐỘNG!');
  }

  startWave(waveNum) {
    this.wave = waveNum;
    this.enemiesSpawnedInWave = 0;
    this.enemiesToSpawn = 10 + waveNum * 6;
    this.spawnTimer = 0;

    this.dom.waveNum.textContent = waveNum;
    this.dom.enemiesNum.textContent = this.enemiesToSpawn;

    // Check Boss Wave
    if (waveNum % 5 === 0) {
      this.spawnBoss(waveNum);
    } else {
      this.activeBoss = null;
      this.dom.bossHud.classList.add('hidden');
    }

    this.addFloatText(this.player.x, this.player.y - 60, `WAVE ${waveNum}`, '#00f0ff', 36);
  }

  spawnBoss(waveNum) {
    const angle = Math.random() * Math.PI * 2;
    const dist = 700;
    const boss = {
      x: this.player.x + Math.cos(angle) * dist,
      y: this.player.y + Math.sin(angle) * dist,
      vx: 0,
      vy: 0,
      radius: 44,
      hp: 1200 + waveNum * 500,
      maxHp: 1200 + waveNum * 500,
      speed: 120,
      type: 'boss',
      color: '#ff0055',
      attackTimer: 0,
      attackPhase: 0
    };

    this.activeBoss = boss;
    this.enemies.push(boss);

    this.dom.bossHud.classList.remove('hidden');
    this.dom.bossName.textContent = `CYBER TITAN - MK ${waveNum / 5}`;
    this.dom.bossHpFill.style.width = '100%';
    this.logCombat('CẢNH BÁO: BOSS KHỦNG ĐÃ XUẤT HIỆN!');
    this.addScreenShake(8);
  }

  spawnEnemy() {
    if (this.enemiesSpawnedInWave >= this.enemiesToSpawn) return;

    // Spawn off-screen around player
    const angle = Math.random() * Math.PI * 2;
    const dist = 650 + Math.random() * 250;
    const ex = Math.max(50, Math.min(this.worldWidth - 50, this.player.x + Math.cos(angle) * dist));
    const ey = Math.max(50, Math.min(this.worldHeight - 50, this.player.y + Math.sin(angle) * dist));

    // Enemy Types: runner, spitter, brute, kamikaze
    const r = Math.random();
    let type = 'runner';
    let hp = 45 + this.wave * 8;
    let speed = 190 + Math.random() * 40;
    let radius = 14;
    let color = '#ff3344';

    if (this.wave >= 2 && r > 0.7) {
      type = 'spitter';
      hp = 60 + this.wave * 10;
      speed = 130;
      radius = 16;
      color = '#00ff66';
    } else if (this.wave >= 3 && r > 0.5 && r <= 0.7) {
      type = 'kamikaze';
      hp = 30;
      speed = 260;
      radius = 12;
      color = '#ffaa00';
    } else if (this.wave >= 4 && r < 0.2) {
      type = 'brute';
      hp = 180 + this.wave * 25;
      speed = 110;
      radius = 24;
      color = '#a855f7';
    }

    this.enemies.push({
      x: ex,
      y: ey,
      vx: 0,
      vy: 0,
      type: type,
      hp: hp,
      maxHp: hp,
      speed: speed,
      radius: radius,
      color: color,
      shootTimer: Math.random() * 2,
      chargeTimer: 0,
      isCharging: false
    });

    this.enemiesSpawnedInWave++;
  }

  // ==========================================
  // 4. MAIN GAME LOOP & UPDATES
  // ==========================================
  loop(timestamp) {
    let dt = (timestamp - this.lastTime) / 1000;
    this.lastTime = timestamp;

    if (dt > 0.1) dt = 0.1; // clamp lag spikes

    if (this.gameState === 'PLAYING') {
      // Time scale for bullet time
      let timeScale = 1.0;
      if (this.player.bulletTime && this.player.stamina > 0) {
        timeScale = 0.4;
        this.player.stamina = Math.max(0, this.player.stamina - dt * 45);
      } else {
        this.player.bulletTime = false;
        this.player.stamina = Math.min(this.player.maxStamina, this.player.stamina + dt * 18);
      }

      this.update(dt, timeScale);
      this.render();
    } else if (this.gameState === 'UPGRADE' || this.gameState === 'PAUSED' || this.gameState === 'START') {
      this.render();
    }

    requestAnimationFrame((t) => this.loop(t));
  }

  update(dt, timeScale) {
    this.updateCamera(dt);
    this.updatePlayer(dt);
    this.updateWeapons(dt);
    this.updateBullets(dt * timeScale);
    this.updateEnemies(dt * timeScale);
    this.updateEnemyBullets(dt * timeScale);
    this.updateTurrets(dt * timeScale);
    this.updateGrenades(dt * timeScale);
    this.updatePickups(dt);
    this.updateParticles(dt);
    this.updateFloatTexts(dt);
    this.updateSpawns(dt * timeScale);
    this.updateCombo(dt);
    this.updateHUD();
  }

  updateCamera(dt) {
    // Smooth camera tracking
    const targetX = this.player.x - this.canvas.width / 2;
    const targetY = this.player.y - this.canvas.height / 2;

    this.camera.x += (targetX - this.camera.x) * 10 * dt;
    this.camera.y += (targetY - this.camera.y) * 10 * dt;

    // Screen Shake
    if (this.camera.shakeDuration > 0) {
      this.camera.shakeDuration -= dt;
      this.camera.shake = Math.sin(performance.now() * 0.05) * this.camera.shakeDuration * 8;
    } else {
      this.camera.shake = 0;
    }

    // Convert mouse screen pos to world pos
    this.mouse.worldX = this.mouse.x + this.camera.x;
    this.mouse.worldY = this.mouse.y + this.camera.y;

    this.player.angle = Math.atan2(this.mouse.worldY - this.player.y, this.mouse.worldX - this.player.x);
  }

  addScreenShake(amount) {
    this.camera.shakeDuration = Math.min(0.4, this.camera.shakeDuration + amount * 0.03);
  }

  updatePlayer(dt) {
    // Cooldowns
    if (this.player.dashCooldown > 0) this.player.dashCooldown -= dt;
    if (this.player.grenadeCd > 0) this.player.grenadeCd -= dt;
    if (this.player.turretCd > 0) this.player.turretCd -= dt;

    // Shield Regen (if untouched for 4 seconds)
    this.player.shieldRegenTimer += dt;
    if (this.player.shieldRegenTimer > 4.0 && this.player.shield < this.player.maxShield) {
      this.player.shield = Math.min(this.player.maxShield, this.player.shield + dt * 25);
    }

    // Dash state
    if (this.player.isDashing) {
      this.player.dashDuration -= dt;
      this.player.x += this.player.vx * dt;
      this.player.y += this.player.vy * dt;

      // Trail particles
      this.player.dashTrailTimer += dt;
      if (this.player.dashTrailTimer > 0.03) {
        this.player.dashTrailTimer = 0;
        this.particles.push({
          x: this.player.x,
          y: this.player.y,
          vx: 0,
          vy: 0,
          radius: this.player.radius,
          color: 'rgba(0, 240, 255, 0.4)',
          alpha: 0.8,
          life: 0.2
        });

        // Fire trail perk
        if (this.player.perks.dashFireTrail) {
          this.decals.push({
            x: this.player.x,
            y: this.player.y,
            radius: 16,
            color: 'rgba(255, 80, 0, 0.7)',
            life: 3.5,
            isDamageZone: true
          });
        }
      }

      if (this.player.dashDuration <= 0) {
        this.player.isDashing = false;
        this.player.vx = 0;
        this.player.vy = 0;
      }
    } else {
      // Normal WASD movement
      let moveX = 0, moveY = 0;
      if (this.keys['KeyW'] || this.keys['ArrowUp']) moveY -= 1;
      if (this.keys['KeyS'] || this.keys['ArrowDown']) moveY += 1;
      if (this.keys['KeyA'] || this.keys['ArrowLeft']) moveX -= 1;
      if (this.keys['KeyD'] || this.keys['ArrowRight']) moveX += 1;

      if (moveX !== 0 && moveY !== 0) {
        moveX *= 0.7071;
        moveY *= 0.7071;
      }

      this.player.x += moveX * this.player.speed * dt;
      this.player.y += moveY * this.player.speed * dt;
    }

    // World bounds clamping
    this.player.x = Math.max(this.player.radius, Math.min(this.worldWidth - this.player.radius, this.player.x));
    this.player.y = Math.max(this.player.radius, Math.min(this.worldHeight - this.player.radius, this.player.y));
  }

  damagePlayer(amount) {
    if (this.player.isDashing) return; // Invincible during dash

    this.player.shieldRegenTimer = 0;
    this.sfx.playerHurt();
    this.addScreenShake(6);

    // Shield absorbs first
    if (this.player.shield > 0) {
      if (this.player.shield >= amount) {
        this.player.shield -= amount;
        amount = 0;
      } else {
        amount -= this.player.shield;
        this.player.shield = 0;
      }
    }

    if (amount > 0) {
      this.player.hp -= amount;
      this.createBloodSplatter(this.player.x, this.player.y, 8, '#ff3344');
    }

    if (this.player.hp <= 0) {
      this.player.hp = 0;
      this.triggerGameOver();
    }
  }

  updateWeapons(dt) {
    const w = this.weapons[this.currentWeaponId];

    // Reload progress
    if (this.isReloading) {
      this.reloadTimer += dt;
      const progress = Math.min(1, this.reloadTimer / w.reloadDuration);
      this.dom.reloadIndicator.style.left = `${progress * 100}%`;

      if (this.reloadTimer >= w.reloadDuration) {
        this.finishReload(false);
      }
    }

    // Firing
    if (this.fireTimer > 0) this.fireTimer -= dt;

    if (this.mouse.down && this.fireTimer <= 0 && !this.isReloading) {
      if (w.currentClip > 0) {
        this.fireWeapon(w);
      } else {
        // Auto trigger reload on empty
        this.handleReloadKey();
      }
    }
  }

  fireWeapon(w) {
    this.fireTimer = w.fireRate;
    w.currentClip--;
    this.shotsFired++;

    this.addScreenShake(w.screenShake);

    // Audio
    if (w.id === 1) this.sfx.shootPistol();
    else if (w.id === 2) this.sfx.shootShotgun();
    else if (w.id === 3) this.sfx.shootRifle();
    else if (w.id === 4) this.sfx.shootPlasma();
    else if (w.id === 5) this.sfx.shootRPG();

    // Damage multiplier (Active reload overcharge + perks)
    let dmg = w.damage * this.player.perks.bulletDamageMulti;
    if (this.player.activeOvercharge) dmg *= 1.4;

    const count = w.pellets || 1;
    for (let i = 0; i < count; i++) {
      const spreadAngle = this.player.angle + (Math.random() - 0.5) * w.spread;
      this.bullets.push({
        x: this.player.x + Math.cos(this.player.angle) * (this.player.radius + 6),
        y: this.player.y + Math.sin(this.player.angle) * (this.player.radius + 6),
        vx: Math.cos(spreadAngle) * w.speed,
        vy: Math.sin(spreadAngle) * w.speed,
        radius: w.bulletRadius,
        damage: dmg,
        color: this.player.activeOvercharge ? '#00ff66' : w.bulletColor,
        isRocket: w.isRocket || false,
        explosionRadius: w.explosionRadius || 0,
        pierce: (w.pierce || 0) + this.player.perks.pierceCount,
        bounces: this.player.perks.bouncingBullets ? 2 : 0,
        life: 2.2
      });
    }

    // Eject shell casing
    const casingAngle = this.player.angle - Math.PI / 2 + (Math.random() - 0.5) * 0.4;
    this.casings.push({
      x: this.player.x,
      y: this.player.y,
      vx: Math.cos(casingAngle) * (Math.random() * 80 + 80),
      vy: Math.sin(casingAngle) * (Math.random() * 80 + 80),
      rotation: Math.random() * Math.PI * 2,
      vRot: (Math.random() - 0.5) * 15,
      life: 5.0
    });

    this.updateAmmoDisplay();
  }

  updateBullets(dt) {
    for (let i = this.bullets.length - 1; i >= 0; i--) {
      const b = this.bullets[i];
      b.x += b.vx * dt;
      b.y += b.vy * dt;
      b.life -= dt;

      // Rocket smoke particle trail
      if (b.isRocket) {
        this.particles.push({
          x: b.x,
          y: b.y,
          vx: (Math.random() - 0.5) * 20,
          vy: (Math.random() - 0.5) * 20,
          radius: Math.random() * 4 + 2,
          color: '#8899aa',
          alpha: 0.6,
          life: 0.25
        });
      }

      // Check boundary bouncing
      if (b.bounces > 0) {
        if (b.x < 0 || b.x > this.worldWidth) {
          b.vx *= -1;
          b.bounces--;
        }
        if (b.y < 0 || b.y > this.worldHeight) {
          b.vy *= -1;
          b.bounces--;
        }
      }

      // Check hit with enemies
      let hitEnemy = false;
      for (let j = this.enemies.length - 1; j >= 0; j--) {
        const e = this.enemies[j];
        const dist = Math.hypot(e.x - b.x, e.y - b.y);

        if (dist < e.radius + b.radius) {
          hitEnemy = true;
          this.shotsHit++;

          if (b.isRocket) {
            this.explodeRocket(b.x, b.y, b.explosionRadius, b.damage);
            b.life = 0;
            break;
          } else {
            // Normal bullet impact
            e.hp -= b.damage;
            e.vx += b.vx * 0.05;
            e.vy += b.vy * 0.05;

            const isCrit = Math.random() < 0.2;
            this.sfx.hit(isCrit);
            this.addFloatText(e.x, e.y, Math.round(b.damage), isCrit ? '#ffd700' : '#ffffff', isCrit ? 18 : 14);
            this.createBloodSplatter(e.x, e.y, 4, e.color);

            if (b.pierce > 0) {
              b.pierce--;
            } else {
              b.life = 0;
              break;
            }
          }
        }
      }

      if (b.life <= 0 || b.x < 0 || b.x > this.worldWidth || b.y < 0 || b.y > this.worldHeight) {
        this.bullets.splice(i, 1);
      }
    }
  }

  explodeRocket(x, y, radius, damage) {
    this.sfx.explosion(1.3);
    this.addScreenShake(12);

    // Scorch mark
    this.decals.push({
      x: x,
      y: y,
      radius: radius * 0.8,
      color: 'rgba(20, 20, 25, 0.75)',
      life: 15
    });

    // Fire & shockwave particles
    for (let i = 0; i < 40; i++) {
      const angle = Math.random() * Math.PI * 2;
      const speed = Math.random() * 350 + 50;
      this.particles.push({
        x: x,
        y: y,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        radius: Math.random() * 8 + 4,
        color: Math.random() < 0.5 ? '#ff3344' : '#ffaa00',
        alpha: 1,
        life: 0.5
      });
    }

    // Damage area
    this.enemies.forEach(e => {
      const d = Math.hypot(e.x - x, e.y - y);
      if (d < radius + e.radius) {
        const falloff = 1 - (d / (radius + e.radius)) * 0.5;
        const totalDmg = damage * falloff;
        e.hp -= totalDmg;
        const angle = Math.atan2(e.y - y, e.x - x);
        e.vx += Math.cos(angle) * 550;
        e.vy += Math.sin(angle) * 550;
        this.addFloatText(e.x, e.y, Math.round(totalDmg), '#ff3344', 22);
      }
    });
  }

  updateEnemies(dt) {
    for (let i = this.enemies.length - 1; i >= 0; i--) {
      const e = this.enemies[i];

      // Damage zones check (Fire trail from dash)
      this.decals.forEach(d => {
        if (d.isDamageZone) {
          if (Math.hypot(e.x - d.x, e.y - d.y) < d.radius + e.radius) {
            e.hp -= 25 * dt;
          }
        }
      });

      // Death check
      if (e.hp <= 0) {
        this.killEnemy(e);
        this.enemies.splice(i, 1);
        continue;
      }

      // Physics friction
      e.x += e.vx * dt;
      e.y += e.vy * dt;
      e.vx *= 0.88;
      e.vy *= 0.88;

      // AI Logic according to type
      const angleToPlayer = Math.atan2(this.player.y - e.y, this.player.x - e.x);
      const distToPlayer = Math.hypot(this.player.x - e.x, this.player.y - e.y);

      if (e.type === 'runner') {
        // Direct swarm
        e.x += Math.cos(angleToPlayer) * e.speed * dt;
        e.y += Math.sin(angleToPlayer) * e.speed * dt;

        if (distToPlayer < this.player.radius + e.radius) {
          this.damagePlayer(16);
          e.vx -= Math.cos(angleToPlayer) * 180;
          e.vy -= Math.sin(angleToPlayer) * 180;
        }
      } else if (e.type === 'spitter') {
        // Keeps distance around 350px and shoots acid globs
        if (distToPlayer < 280) {
          // retreat
          e.x -= Math.cos(angleToPlayer) * e.speed * dt;
          e.y -= Math.sin(angleToPlayer) * e.speed * dt;
        } else if (distToPlayer > 400) {
          // approach
          e.x += Math.cos(angleToPlayer) * e.speed * dt;
          e.y += Math.sin(angleToPlayer) * e.speed * dt;
        }

        e.shootTimer += dt;
        if (e.shootTimer >= 2.0) {
          e.shootTimer = 0;
          this.enemyBullets.push({
            x: e.x,
            y: e.y,
            vx: Math.cos(angleToPlayer) * 380,
            vy: Math.sin(angleToPlayer) * 380,
            radius: 6,
            damage: 22,
            color: '#00ff66',
            life: 3.0
          });
        }
      } else if (e.type === 'kamikaze') {
        // Fast rush, flashes, then self-destructs
        e.x += Math.cos(angleToPlayer) * e.speed * dt;
        e.y += Math.sin(angleToPlayer) * e.speed * dt;

        if (distToPlayer < 40) {
          this.explodeRocket(e.x, e.y, 90, 45);
          e.hp = 0;
        }
      } else if (e.type === 'brute') {
        // Heavy tank that charges periodically
        e.chargeTimer += dt;
        if (e.isCharging) {
          e.x += Math.cos(e.chargeAngle) * 550 * dt;
          e.y += Math.sin(e.chargeAngle) * 550 * dt;
          if (distToPlayer < this.player.radius + e.radius) {
            this.damagePlayer(35);
            e.isCharging = false;
          }
          if (e.chargeTimer > 0.8) {
            e.isCharging = false;
            e.chargeTimer = 0;
          }
        } else {
          e.x += Math.cos(angleToPlayer) * e.speed * dt;
          e.y += Math.sin(angleToPlayer) * e.speed * dt;
          if (e.chargeTimer >= 3.5 && distToPlayer < 400) {
            e.isCharging = true;
            e.chargeAngle = angleToPlayer;
            e.chargeTimer = 0;
            this.addFloatText(e.x, e.y - 20, '⚠️ CHARGING!', '#ff0055', 16);
          }
        }
      } else if (e.type === 'boss') {
        // Multi-phase Boss
        e.attackTimer += dt;
        e.x += Math.cos(angleToPlayer) * e.speed * dt;
        e.y += Math.sin(angleToPlayer) * e.speed * dt;

        // Boss attacks
        if (e.attackTimer > 1.8) {
          e.attackTimer = 0;
          e.attackPhase = (e.attackPhase + 1) % 3;

          if (e.attackPhase === 0) {
            // Spiral bullet hell
            for (let a = 0; a < Math.PI * 2; a += Math.PI / 6) {
              this.enemyBullets.push({
                x: e.x,
                y: e.y,
                vx: Math.cos(a) * 320,
                vy: Math.sin(a) * 320,
                radius: 7,
                damage: 25,
                color: '#ff0055',
                life: 3.5
              });
            }
          } else if (e.attackPhase === 1) {
            // Triple spread towards player
            for (let offset = -0.3; offset <= 0.3; offset += 0.3) {
              this.enemyBullets.push({
                x: e.x,
                y: e.y,
                vx: Math.cos(angleToPlayer + offset) * 440,
                vy: Math.sin(angleToPlayer + offset) * 440,
                radius: 8,
                damage: 30,
                color: '#ffd700',
                life: 3.0
              });
            }
          } else if (e.attackPhase === 2) {
            // Summon mini drones
            for (let k = 0; k < 3; k++) {
              this.enemies.push({
                x: e.x + (Math.random() - 0.5) * 80,
                y: e.y + (Math.random() - 0.5) * 80,
                vx: 0,
                vy: 0,
                type: 'runner',
                hp: 40,
                maxHp: 40,
                speed: 210,
                radius: 12,
                color: '#ff3344'
              });
            }
          }
        }

        // Update Boss Health Bar
        const pct = Math.max(0, (e.hp / e.maxHp) * 100);
        this.dom.bossHpFill.style.width = `${pct}%`;

        if (distToPlayer < this.player.radius + e.radius) {
          this.damagePlayer(40);
        }
      }
    }
  }

  killEnemy(e) {
    this.totalKills++;
    this.combo++;
    this.comboTimer = 3.5; // keep combo alive
    const pts = (e.type === 'boss' ? 5000 : 100) * this.combo;
    this.score += pts;

    this.addFloatText(e.x, e.y, `+${pts}`, '#ffd700', 16);

    // Sound & Blood
    this.createBloodSplatter(e.x, e.y, 16, e.color);

    // Vampiric Perk
    if (this.player.perks.vampiric && this.player.hp < this.player.maxHp) {
      this.player.hp = Math.min(this.player.maxHp, this.player.hp + 2);
    }

    // Drop Pickups (15% chance, 100% on Boss)
    if (Math.random() < 0.16 || e.type === 'boss') {
      this.spawnPickup(e.x, e.y);
    }

    if (e.type === 'boss') {
      this.dom.bossHud.classList.add('hidden');
      this.logCombat('BẠN ĐÃ TIÊU DIỆT ĐƯỢC BOSS THÀNH CÔNG!');
    }
  }

  createBloodSplatter(x, y, count, color) {
    // Floor stain
    this.decals.push({
      x: x + (Math.random() - 0.5) * 20,
      y: y + (Math.random() - 0.5) * 20,
      radius: Math.random() * 14 + 10,
      color: color,
      alpha: 0.6,
      life: 25
    });

    // Flying gore particles
    for (let i = 0; i < count; i++) {
      const angle = Math.random() * Math.PI * 2;
      const speed = Math.random() * 180 + 40;
      this.particles.push({
        x: x,
        y: y,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        radius: Math.random() * 3.5 + 1.5,
        color: color,
        alpha: 0.9,
        life: 0.4
      });
    }
  }

  spawnPickup(x, y) {
    const types = ['medkit', 'ammo', 'nuke', 'quad_damage'];
    const type = types[Math.floor(Math.random() * types.length)];
    this.pickups.push({
      x: x,
      y: y,
      type: type,
      radius: 12,
      pulse: 0
    });
  }

  updatePickups(dt) {
    for (let i = this.pickups.length - 1; i >= 0; i--) {
      const p = this.pickups[i];
      p.pulse += dt * 5;

      const dist = Math.hypot(p.x - this.player.x, p.y - this.player.y);
      if (dist < p.radius + this.player.radius) {
        this.sfx.powerup();
        if (p.type === 'medkit') {
          this.player.hp = Math.min(this.player.maxHp, this.player.hp + 35);
          this.addFloatText(this.player.x, this.player.y - 20, '+35 HP', '#00ff66', 20);
        } else if (p.type === 'ammo') {
          // Refill all reserves
          this.weapons[2].reserveAmmo = Math.min(96, this.weapons[2].reserveAmmo + 24);
          this.weapons[3].reserveAmmo = Math.min(300, this.weapons[3].reserveAmmo + 90);
          this.weapons[4].reserveAmmo = Math.min(120, this.weapons[4].reserveAmmo + 36);
          this.weapons[5].reserveAmmo = Math.min(20, this.weapons[5].reserveAmmo + 6);
          this.addFloatText(this.player.x, this.player.y - 20, 'MAX AMMO REFILLED!', '#ffd700', 20);
        } else if (p.type === 'nuke') {
          this.sfx.explosion(2.0);
          this.addScreenShake(18);
          this.enemies.forEach(e => {
            if (e.type !== 'boss') e.hp = 0;
            else e.hp -= 400;
          });
          this.addFloatText(this.player.x, this.player.y - 20, '💥 NUKE ACTIVATED!', '#ff0055', 28);
        } else if (p.type === 'quad_damage') {
          this.player.activeOvercharge = true;
          this.addFloatText(this.player.x, this.player.y - 20, '⚡ OVERCHARGE BUFF!', '#00f0ff', 24);
        }

        this.pickups.splice(i, 1);
      }
    }
  }

  updateEnemyBullets(dt) {
    for (let i = this.enemyBullets.length - 1; i >= 0; i--) {
      const b = this.enemyBullets[i];
      b.x += b.vx * dt;
      b.y += b.vy * dt;
      b.life -= dt;

      // Hit player
      const dist = Math.hypot(b.x - this.player.x, b.y - this.player.y);
      if (dist < b.radius + this.player.radius) {
        this.damagePlayer(b.damage);
        this.enemyBullets.splice(i, 1);
        continue;
      }

      if (b.life <= 0 || b.x < 0 || b.x > this.worldWidth || b.y < 0 || b.y > this.worldHeight) {
        this.enemyBullets.splice(i, 1);
      }
    }
  }

  updateTurrets(dt) {
    for (let i = this.turrets.length - 1; i >= 0; i--) {
      const t = this.turrets[i];
      t.life -= dt;
      t.fireTimer += dt;

      // Find nearest enemy
      let nearest = null;
      let minD = 480;
      this.enemies.forEach(e => {
        const d = Math.hypot(e.x - t.x, e.y - t.y);
        if (d < minD) {
          minD = d;
          nearest = e;
        }
      });

      if (nearest) {
        t.angle = Math.atan2(nearest.y - t.y, nearest.x - t.x);
        const rate = this.player.perks.turretOverclock ? 0.12 : 0.22;
        if (t.fireTimer >= rate) {
          t.fireTimer = 0;
          this.sfx.shootRifle();
          this.bullets.push({
            x: t.x + Math.cos(t.angle) * 16,
            y: t.y + Math.sin(t.angle) * 16,
            vx: Math.cos(t.angle) * 1100,
            vy: Math.sin(t.angle) * 1100,
            radius: 3,
            damage: 20,
            color: '#00f0ff',
            life: 1.5
          });
        }
      }

      if (t.life <= 0) {
        this.turrets.splice(i, 1);
      }
    }
  }

  updateGrenades(dt) {
    for (let i = this.grenades.length - 1; i >= 0; i--) {
      const g = this.grenades[i];
      g.x += g.vx * dt;
      g.y += g.vy * dt;
      g.vx *= 0.94;
      g.vy *= 0.94;
      g.fuse -= dt;

      if (g.fuse <= 0) {
        // Main explosion
        this.explodeRocket(g.x, g.y, 140, 160);

        // Spawn 4 mini cluster bombs
        for (let k = 0; k < 4; k++) {
          const a = (Math.PI * 2 * k) / 4 + Math.random() * 0.4;
          const dist = 60 + Math.random() * 40;
          setTimeout(() => {
            this.explodeRocket(g.x + Math.cos(a) * dist, g.y + Math.sin(a) * dist, 70, 80);
          }, 150 + k * 80);
        }

        this.grenades.splice(i, 1);
      }
    }
  }

  updateParticles(dt) {
    for (let i = this.particles.length - 1; i >= 0; i--) {
      const p = this.particles[i];
      p.x += p.vx * dt;
      p.y += p.vy * dt;
      p.alpha -= dt / p.life;
      if (p.alpha <= 0) {
        this.particles.splice(i, 1);
      }
    }

    // Casings
    for (let i = this.casings.length - 1; i >= 0; i--) {
      const c = this.casings[i];
      c.x += c.vx * dt;
      c.y += c.vy * dt;
      c.vx *= 0.9;
      c.vy *= 0.9;
      c.rotation += c.vRot * dt;
      c.life -= dt;
      if (c.life <= 0) {
        this.casings.splice(i, 1);
      }
    }

    // Decals
    for (let i = this.decals.length - 1; i >= 0; i--) {
      const d = this.decals[i];
      d.life -= dt;
      if (d.life <= 0) {
        this.decals.splice(i, 1);
      }
    }
  }

  updateFloatTexts(dt) {
    for (let i = this.floatTexts.length - 1; i >= 0; i--) {
      const f = this.floatTexts[i];
      f.y -= 35 * dt;
      f.alpha -= dt * 1.2;
      if (f.alpha <= 0) {
        this.floatTexts.splice(i, 1);
      }
    }
  }

  addFloatText(x, y, text, color = '#fff', size = 16) {
    this.floatTexts.push({
      x: x + (Math.random() - 0.5) * 15,
      y: y,
      text: text,
      color: color,
      size: size,
      alpha: 1.0
    });
  }

  updateSpawns(dt) {
    this.spawnTimer += dt;
    // Spawn rate speeds up with waves
    const interval = Math.max(0.4, 1.6 - this.wave * 0.12);
    if (this.spawnTimer >= interval && this.enemiesSpawnedInWave < this.enemiesToSpawn) {
      this.spawnTimer = 0;
      this.spawnEnemy();
    }

    // Wave completed check
    if (this.enemiesSpawnedInWave >= this.enemiesToSpawn && this.enemies.length === 0) {
      this.waveCompleted();
    }
  }

  waveCompleted() {
    this.gameState = 'UPGRADE';
    this.logCombat(`HOÀN THÀNH WAVE ${this.wave}! HÃY CHỌN NÂNG CẤP!`);
    this.showUpgradeMenu();
  }

  showUpgradeMenu() {
    const allPerks = [
      {
        id: 'bouncing',
        title: 'ĐẠN PHẢN XẠ',
        icon: '🔄',
        desc: 'Đạn nảy lại 2 lần khi va vào tường đấu trường, tăng độ sát thương diện rộng.'
      },
      {
        id: 'pierce',
        title: 'ĐẠN XUYÊN THẤU',
        icon: '🏹',
        desc: 'Mọi loại vũ khí tăng thêm +1 lần bắn xuyên qua mục tiêu.'
      },
      {
        id: 'vampiric',
        title: 'HÚT NĂNG LƯỢNG',
        icon: '🩸',
        desc: 'Hồi phục +2 HP ngay lập tức mỗi khi tiêu diệt được một kẻ địch.'
      },
      {
        id: 'fire_dash',
        title: 'LƯỚT LỬA TỰ ĐỘNG',
        icon: '🔥',
        desc: 'Mỗi cú Lướt (Space) để lại một vệt lửa plasma thiêu đốt quái vật chạy qua.'
      },
      {
        id: 'heavy_cal',
        title: 'ĐẠN HẠNG NẶNG',
        icon: '💥',
        desc: 'Tăng vĩnh viễn +25% toàn bộ sát thương của tất cả các dòng súng.'
      },
      {
        id: 'rapid_dash',
        title: 'LƯỚT SIÊU NHẸ',
        icon: '⚡',
        desc: 'Giảm 30% thời gian hồi chiêu Dash (Space), tăng tốc độ né đòn dồn dập.'
      },
      {
        id: 'nano_shield',
        title: 'KHIÊN NANO',
        icon: '🛡️',
        desc: 'Tăng dung lượng khiên tối đa thêm +30 và hồi phục nhanh gấp đôi.'
      },
      {
        id: 'overclock_turret',
        title: 'ÉP XUNG TRỤ SÚNG',
        icon: '🤖',
        desc: 'Trụ súng tự động [E] bắn nhanh gấp đôi và tăng thêm 15 giây tồn tại.'
      }
    ];

    // Pick 3 random
    const shuffled = allPerks.sort(() => 0.5 - Math.random()).slice(0, 3);
    this.dom.upgradeCards.innerHTML = '';

    shuffled.forEach(perk => {
      const card = document.createElement('div');
      card.className = 'upgrade-card';
      card.innerHTML = `
        <div class="card-icon">${perk.icon}</div>
        <div class="card-title">${perk.title}</div>
        <div class="card-desc">${perk.desc}</div>
      `;
      card.addEventListener('click', () => this.applyPerk(perk.id));
      this.dom.upgradeCards.appendChild(card);
    });

    this.dom.upgradeOverlay.classList.remove('hidden');
  }

  applyPerk(perkId) {
    if (perkId === 'bouncing') this.player.perks.bouncingBullets = true;
    else if (perkId === 'pierce') this.player.perks.pierceCount += 1;
    else if (perkId === 'vampiric') this.player.perks.vampiric = true;
    else if (perkId === 'fire_dash') this.player.perks.dashFireTrail = true;
    else if (perkId === 'heavy_cal') this.player.perks.bulletDamageMulti += 0.25;
    else if (perkId === 'rapid_dash') this.player.perks.dashCdMulti *= 0.7;
    else if (perkId === 'nano_shield') {
      this.player.maxShield += 30;
      this.player.shield = this.player.maxShield;
    } else if (perkId === 'overclock_turret') this.player.perks.turretOverclock = true;

    this.dom.upgradeOverlay.classList.add('hidden');
    this.gameState = 'PLAYING';
    this.startWave(this.wave + 1);
  }

  updateCombo(dt) {
    if (this.comboTimer > 0) {
      this.comboTimer -= dt;
      if (this.comboTimer <= 0) {
        this.combo = 1;
      }
    }
  }

  updateHUD() {
    this.dom.scoreVal.textContent = this.score.toLocaleString();
    this.dom.comboDisplay.textContent = `COMBO x${this.combo}`;
    this.dom.enemiesNum.textContent = (this.enemiesToSpawn - this.enemiesSpawnedInWave) + this.enemies.length;

    // Vitals
    const hpPct = Math.max(0, (this.player.hp / this.player.maxHp) * 100);
    this.dom.hpBar.style.width = `${hpPct}%`;
    this.dom.hpText.textContent = `${Math.ceil(this.player.hp)} / ${this.player.maxHp}`;

    const shieldPct = Math.max(0, (this.player.shield / this.player.maxShield) * 100);
    this.dom.shieldBar.style.width = `${shieldPct}%`;
    this.dom.shieldText.textContent = `${Math.ceil(this.player.shield)} / ${this.player.maxShield}`;

    // Dash Bar
    const dashPct = this.player.dashCooldown <= 0 ? 100 : Math.max(0, 100 - (this.player.dashCooldown / (this.player.dashMaxCd * this.player.perks.dashCdMulti)) * 100);
    this.dom.dashBar.style.width = `${dashPct}%`;

    // Adrenaline Bar
    const adrenPct = (this.player.stamina / this.player.maxStamina) * 100;
    this.dom.adrenBar.style.width = `${adrenPct}%`;

    // Skills CD
    this.dom.cdGrenade.textContent = this.player.grenadeCd <= 0 ? 'READY' : `${this.player.grenadeCd.toFixed(1)}s`;
    this.dom.cdTurret.textContent = this.player.turretCd <= 0 ? 'READY' : `${this.player.turretCd.toFixed(1)}s`;

    this.updateAmmoDisplay();
  }

  updateAmmoDisplay() {
    const w = this.weapons[this.currentWeaponId];
    this.dom.curClip.textContent = w.currentClip;
    this.dom.maxClip.textContent = w.clipSize;
    this.dom.reserveAmmo.textContent = w.reserveAmmo === Infinity ? '∞' : w.reserveAmmo;

    // Hotbar numbers
    document.getElementById('slot-2-ammo').textContent = this.weapons[2].reserveAmmo;
    document.getElementById('slot-3-ammo').textContent = this.weapons[3].reserveAmmo;
    document.getElementById('slot-4-ammo').textContent = this.weapons[4].reserveAmmo;
    document.getElementById('slot-5-ammo').textContent = this.weapons[5].reserveAmmo;
  }

  logCombat(text) {
    const el = document.createElement('div');
    el.className = 'log-entry';
    el.textContent = text;
    this.dom.combatLog.prepend(el);
    setTimeout(() => {
      if (el.parentNode) el.parentNode.removeChild(el);
    }, 4000);
  }

  triggerGameOver() {
    this.gameState = 'GAMEOVER';
    this.dom.finalWave.textContent = this.wave;
    this.dom.finalScore.textContent = this.score.toLocaleString();
    this.dom.finalKills.textContent = this.totalKills;
    const acc = this.shotsFired > 0 ? Math.round((this.shotsHit / this.shotsFired) * 100) : 0;
    this.dom.finalAcc.textContent = `${acc}%`;
    this.dom.gameoverOverlay.classList.remove('hidden');
  }

  // ==========================================
  // 5. RENDERING PIPELINE (CANVAS & MINIMAP)
  // ==========================================
  render() {
    const ctx = this.ctx;
    const w = this.canvas.width;
    const h = this.canvas.height;

    ctx.save();
    ctx.clearRect(0, 0, w, h);

    // Apply Camera translation + Shake
    ctx.translate(-this.camera.x + this.camera.shake, -this.camera.y + this.camera.shake);

    // 1. Arena Grid & Background
    this.renderArena(ctx);

    // 2. Decals (Blood splatters, scorch marks, fire zones)
    this.renderDecals(ctx);

    // 3. Shell Casings
    this.renderCasings(ctx);

    // 4. Turrets
    this.renderTurrets(ctx);

    // 5. Pickups
    this.renderPickups(ctx);

    // 6. Grenades
    this.renderGrenades(ctx);

    // 7. Enemies
    this.renderEnemies(ctx);

    // 8. Player
    this.renderPlayer(ctx);

    // 9. Bullets (Player & Enemy)
    this.renderBullets(ctx);

    // 10. Particles
    this.renderParticles(ctx);

    // 11. Flashlight / Dynamic Darkness Overlay
    this.renderDarknessAndLighting(ctx);

    // 12. Floating Damage Numbers
    this.renderFloatTexts(ctx);

    ctx.restore();

    // 13. Minimap (Screen-Space)
    this.renderMinimap();
  }

  renderArena(ctx) {
    // Dark metallic industrial floor
    ctx.fillStyle = '#0b0e14';
    ctx.fillRect(0, 0, this.worldWidth, this.worldHeight);

    // Grid lines
    ctx.strokeStyle = 'rgba(0, 240, 255, 0.04)';
    ctx.lineWidth = 1;
    const gridSize = 80;

    const startX = 0;
    const endX = this.worldWidth;
    const startY = 0;
    const endY = this.worldHeight;

    ctx.beginPath();
    for (let x = startX; x <= endX; x += gridSize) {
      ctx.moveTo(x, startY);
      ctx.lineTo(x, endY);
    }
    for (let y = startY; y <= endY; y += gridSize) {
      ctx.moveTo(startX, y);
      ctx.lineTo(endX, y);
    }
    ctx.stroke();

    // Outer Boundary Neon Fence
    ctx.strokeStyle = '#00f0ff';
    ctx.lineWidth = 4;
    ctx.shadowColor = '#00f0ff';
    ctx.shadowBlur = 15;
    ctx.strokeRect(0, 0, this.worldWidth, this.worldHeight);
    ctx.shadowBlur = 0;
  }

  renderDecals(ctx) {
    this.decals.forEach(d => {
      ctx.beginPath();
      ctx.arc(d.x, d.y, d.radius, 0, Math.PI * 2);
      ctx.fillStyle = d.color;
      ctx.fill();
    });
  }

  renderCasings(ctx) {
    ctx.fillStyle = '#d4af37';
    this.casings.forEach(c => {
      ctx.save();
      ctx.translate(c.x, c.y);
      ctx.rotate(c.rotation);
      ctx.fillRect(-3, -1, 6, 2.5);
      ctx.restore();
    });
  }

  renderTurrets(ctx) {
    this.turrets.forEach(t => {
      ctx.save();
      ctx.translate(t.x, t.y);

      // Base tripod
      ctx.fillStyle = '#1e293b';
      ctx.beginPath();
      ctx.arc(0, 0, 14, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = '#00f0ff';
      ctx.lineWidth = 2;
      ctx.stroke();

      // Rotating barrel
      ctx.rotate(t.angle);
      ctx.fillStyle = '#475569';
      ctx.fillRect(0, -3, 18, 6);

      ctx.restore();
    });
  }

  renderPickups(ctx) {
    this.pickups.forEach(p => {
      ctx.save();
      ctx.translate(p.x, p.y);
      const scale = 1 + Math.sin(p.pulse) * 0.15;
      ctx.scale(scale, scale);

      ctx.beginPath();
      ctx.arc(0, 0, p.radius, 0, Math.PI * 2);
      if (p.type === 'medkit') {
        ctx.fillStyle = '#ff3344';
        ctx.fill();
        ctx.fillStyle = '#ffffff';
        ctx.fillRect(-2, -6, 4, 12);
        ctx.fillRect(-6, -2, 12, 4);
      } else if (p.type === 'ammo') {
        ctx.fillStyle = '#ffd700';
        ctx.fill();
        ctx.fillStyle = '#000000';
        ctx.font = 'bold 10px Orbitron';
        ctx.textAlign = 'center';
        ctx.fillText('AM', 0, 4);
      } else if (p.type === 'nuke') {
        ctx.fillStyle = '#ffaa00';
        ctx.fill();
        ctx.fillStyle = '#000000';
        ctx.font = 'bold 12px Orbitron';
        ctx.textAlign = 'center';
        ctx.fillText('☢', 0, 4);
      } else if (p.type === 'quad_damage') {
        ctx.fillStyle = '#00f0ff';
        ctx.fill();
        ctx.fillStyle = '#000000';
        ctx.font = 'bold 12px Orbitron';
        ctx.textAlign = 'center';
        ctx.fillText('⚡', 0, 4);
      }

      ctx.restore();
    });
  }

  renderGrenades(ctx) {
    this.grenades.forEach(g => {
      ctx.save();
      ctx.translate(g.x, g.y);
      ctx.beginPath();
      ctx.arc(0, 0, g.radius, 0, Math.PI * 2);
      ctx.fillStyle = '#ff3344';
      ctx.fill();
      ctx.strokeStyle = '#ffffff';
      ctx.lineWidth = 1.5;
      ctx.stroke();
      ctx.restore();
    });
  }

  renderEnemies(ctx) {
    this.enemies.forEach(e => {
      ctx.save();
      ctx.translate(e.x, e.y);

      // Body
      ctx.beginPath();
      ctx.arc(0, 0, e.radius, 0, Math.PI * 2);
      ctx.fillStyle = e.color;
      ctx.shadowColor = e.color;
      ctx.shadowBlur = 8;
      ctx.fill();
      ctx.shadowBlur = 0;

      // Inner details
      ctx.fillStyle = '#000';
      ctx.beginPath();
      ctx.arc(0, 0, e.radius * 0.45, 0, Math.PI * 2);
      ctx.fill();

      // Health bar above enemy
      if (e.hp < e.maxHp && e.type !== 'boss') {
        const barW = e.radius * 2;
        const barH = 3;
        const pct = Math.max(0, e.hp / e.maxHp);
        ctx.fillStyle = 'rgba(0,0,0,0.6)';
        ctx.fillRect(-barW / 2, -e.radius - 8, barW, barH);
        ctx.fillStyle = '#00ff66';
        ctx.fillRect(-barW / 2, -e.radius - 8, barW * pct, barH);
      }

      ctx.restore();
    });
  }

  renderPlayer(ctx) {
    ctx.save();
    ctx.translate(this.player.x, this.player.y);
    ctx.rotate(this.player.angle);

    // Shield Aura
    if (this.player.shield > 0) {
      ctx.beginPath();
      ctx.arc(0, 0, this.player.radius + 4, 0, Math.PI * 2);
      ctx.strokeStyle = 'rgba(0, 240, 255, 0.4)';
      ctx.lineWidth = 2;
      ctx.stroke();
    }

    // Gun Barrel
    ctx.fillStyle = '#64748b';
    ctx.fillRect(8, 4, 18, 5);

    // Player Body
    ctx.beginPath();
    ctx.arc(0, 0, this.player.radius, 0, Math.PI * 2);
    ctx.fillStyle = '#00f0ff';
    ctx.shadowColor = '#00f0ff';
    ctx.shadowBlur = 12;
    ctx.fill();
    ctx.shadowBlur = 0;

    // Tactical Armor Core
    ctx.beginPath();
    ctx.arc(0, 0, 8, 0, Math.PI * 2);
    ctx.fillStyle = '#0f172a';
    ctx.fill();

    // Visor Eye
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(4, -3, 6, 6);

    ctx.restore();
  }

  renderBullets(ctx) {
    // Player Bullets
    this.bullets.forEach(b => {
      ctx.save();
      ctx.beginPath();
      ctx.arc(b.x, b.y, b.radius, 0, Math.PI * 2);
      ctx.fillStyle = b.color;
      ctx.shadowColor = b.color;
      ctx.shadowBlur = 10;
      ctx.fill();
      ctx.shadowBlur = 0;
      ctx.restore();
    });

    // Enemy Bullets
    this.enemyBullets.forEach(b => {
      ctx.save();
      ctx.beginPath();
      ctx.arc(b.x, b.y, b.radius, 0, Math.PI * 2);
      ctx.fillStyle = b.color;
      ctx.shadowColor = b.color;
      ctx.shadowBlur = 8;
      ctx.fill();
      ctx.shadowBlur = 0;
      ctx.restore();
    });
  }

  renderParticles(ctx) {
    this.particles.forEach(p => {
      ctx.save();
      ctx.globalAlpha = p.alpha;
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
      ctx.fillStyle = p.color;
      ctx.fill();
      ctx.restore();
    });
  }

  renderDarknessAndLighting(ctx) {
    // Flashlight cone & Ambient Lighting
    const lightRadius = 450;
    const gradient = ctx.createRadialGradient(
      this.player.x, this.player.y, 10,
      this.player.x, this.player.y, lightRadius
    );
    gradient.addColorStop(0, 'rgba(0, 240, 255, 0.08)');
    gradient.addColorStop(1, 'rgba(0, 0, 0, 0.0)');

    ctx.fillStyle = gradient;
    ctx.beginPath();
    ctx.arc(this.player.x, this.player.y, lightRadius, 0, Math.PI * 2);
    ctx.fill();

    // Crosshair in World Space
    ctx.save();
    ctx.strokeStyle = '#00f0ff';
    ctx.lineWidth = 1.5;
    const mx = this.mouse.worldX;
    const my = this.mouse.worldY;
    ctx.beginPath();
    ctx.arc(mx, my, 8, 0, Math.PI * 2);
    ctx.moveTo(mx - 12, my);
    ctx.lineTo(mx + 12, my);
    ctx.moveTo(mx, my - 12);
    ctx.lineTo(mx, my + 12);
    ctx.stroke();
    ctx.restore();
  }

  renderFloatTexts(ctx) {
    this.floatTexts.forEach(f => {
      ctx.save();
      ctx.globalAlpha = f.alpha;
      ctx.font = `bold ${f.size}px Orbitron, sans-serif`;
      ctx.fillStyle = f.color;
      ctx.textAlign = 'center';
      ctx.shadowColor = f.color;
      ctx.shadowBlur = 6;
      ctx.fillText(f.text, f.x, f.y);
      ctx.shadowBlur = 0;
      ctx.restore();
    });
  }

  renderMinimap() {
    const mctx = this.mctx;
    const mw = this.minimapCanvas.width;
    const mh = this.minimapCanvas.height;

    mctx.clearRect(0, 0, mw, mh);

    // Map Background
    mctx.fillStyle = 'rgba(8, 12, 20, 0.9)';
    mctx.fillRect(0, 0, mw, mh);

    // Ratio
    const scaleX = mw / this.worldWidth;
    const scaleY = mh / this.worldHeight;

    // Enemies
    this.enemies.forEach(e => {
      mctx.fillStyle = e.type === 'boss' ? '#ff0055' : '#ff3344';
      mctx.beginPath();
      mctx.arc(e.x * scaleX, e.y * scaleY, e.type === 'boss' ? 4 : 2, 0, Math.PI * 2);
      mctx.fill();
    });

    // Player
    mctx.fillStyle = '#00f0ff';
    mctx.beginPath();
    mctx.arc(this.player.x * scaleX, this.player.y * scaleY, 3.5, 0, Math.PI * 2);
    mctx.fill();
  }
}

// Start Game on load
window.addEventListener('load', () => {
  window.cyberSurvivor = new Game();
});
