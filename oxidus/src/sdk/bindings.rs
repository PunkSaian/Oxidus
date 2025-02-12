use libc::c_void;
pub type Vector2 = [f32;2];
pub type Vector3 = [f32;3];
pub type Unknown = [u8;0];
use macros::tf2_struct;

#[tf2_struct(baselcass = WeaponIFMBaseCamera)]
pub struct WeaponIFMSteadyCam;

#[tf2_struct(baselcass = WeaponIFMBase)]
pub struct WeaponIFMBaseCamera {
    #[offset(4312)]
    pub m_flRenderAspectRatio: f32,
    #[offset(4316)]
    pub m_flRenderFOV: f32,
    #[offset(4320)]
    pub m_flRenderArmLength: f32,
    #[offset(4324)]
    pub m_vecRenderPosition: Vector2,
    #[offset(4336)]
    pub m_angRenderAngles: Vector2,
}

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct WeaponIFMBase;

#[tf2_struct(baselcass = BaseEntity)]
pub struct MannVsMachineStats {
    #[offset(1968)]
    pub m_runningTotalWaveStats: CMannVsMachineWaveStats,
    #[offset(2000)]
    pub m_previousWaveStats: CMannVsMachineWaveStats,
    #[offset(2032)]
    pub m_currentWaveStats: CMannVsMachineWaveStats,
    #[offset(2064)]
    pub m_iCurrentWaveIdx: i32,
    #[offset(2068)]
    pub m_iServerWaveID: i32,
    #[offset(2192)]
    pub m_iCurrencyCollectedForRespec: i32,
    #[offset(2196)]
    pub m_nRespecsAwardedInWave: i32,
}

#[tf2_struct(baselcass = TFBaseBoss)]
pub struct TFTankBoss;

#[tf2_struct(baselcass = NextBot)]
pub struct TFBaseBoss {
    #[offset(4632)]
    pub m_lastHealthPercentage: f32,
}

#[tf2_struct(baselcass = NextBot)]
pub struct BossAlpha {
    #[offset(4620)]
    pub m_isNuking: i32,
}

#[tf2_struct(baselcass = BaseCombatCharacter)]
pub struct NextBot;

#[tf2_struct(baselcass = BaseEntity)]
pub struct TFBotHintEngineerNest {
    #[offset(1966)]
    pub m_bHasActiveTeleporter: bool,
}

#[tf2_struct(baselcass = NextBot)]
pub struct BotNPCMinion {
    #[offset(4620)]
    pub m_stunTarget: i32,
}

#[tf2_struct(baselcass = NextBot)]
pub struct BotNPC {
    #[offset(4620)]
    pub m_laserTarget: i32,
    #[offset(4648)]
    pub m_isNuking: i32,
}

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellKartBats;

#[tf2_struct(baselcass = TFProjectile_SpellFireball)]
pub struct TFProjectile_SpellKartOrb;

#[tf2_struct(baselcass = BaseEntity)]
pub struct TFHellZap;

#[tf2_struct(baselcass = TFProjectile_SpellFireball)]
pub struct TFProjectile_SpellLightningOrb;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct SpellTransposeTeleport;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellMeteorShower;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellSpawnBoss;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellMirv;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellPumpkin;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellSpawnHorde;

#[tf2_struct(baselcass = TFProjectile_SpellBats)]
pub struct TFProjectile_SpellSpawnZombie;

#[tf2_struct(baselcass = TFProjectile_Jar)]
pub struct TFProjectile_SpellBats;

#[tf2_struct(baselcass = TFProjectile_Rocket)]
pub struct TFProjectile_SpellFireball;

#[tf2_struct(baselcass = TFWeaponThrowable)]
pub struct TFWeaponSpellBook {
    #[offset(4304)]
    pub m_flTimeNextSpell: f32,
    #[offset(4308)]
    pub m_iSelectedSpellIndex: i32,
    #[offset(4312)]
    pub m_iSpellCharges: i32,
    #[offset(4316)]
    pub m_bFiredAttack: bool,
}

#[tf2_struct(baselcass = TeleportVortex)]
pub struct Hightower_TeleportVortex {
    #[offset(3104)]
    pub m_iState: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TeleportVortex {
    #[offset(3104)]
    pub m_iState: i32,
}

#[tf2_struct(baselcass = NextBot)]
pub struct Zombie {
    #[offset(4620)]
    pub m_flHeadScale: f32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct MerasmusDancer;

#[tf2_struct(baselcass = NextBot)]
pub struct Merasmus {
    #[offset(4697)]
    pub m_bRevealed: bool,
    #[offset(4698)]
    pub m_bIsDoingAOEAttack: bool,
    #[offset(4699)]
    pub m_bStunned: bool,
}

#[tf2_struct(baselcass = NextBot)]
pub struct EyeballBoss {
    #[offset(4620)]
    pub m_lookAtSpot: Vector2,
    #[offset(4632)]
    pub m_attitude: i32,
}

#[tf2_struct(baselcass = NextBot)]
pub struct HeadlessHatman;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponRocketPack {
    #[offset(4264)]
    pub m_flInitLaunchTime: f32,
    #[offset(4268)]
    pub m_flLaunchTime: f32,
    #[offset(4272)]
    pub m_flToggleEndTime: f32,
    #[offset(4277)]
    pub m_bEnabled: bool,
}

#[tf2_struct(baselcass = TFProjectile_Rocket)]
pub struct TFProjectile_MechanicalArmOrb;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct TFMechanicalArm;

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWearableCampaignItem {
    #[offset(3792)]
    pub m_nState: i32,
}

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWearableLevelableItem {
    #[offset(3792)]
    pub m_unLevel: i32,
}

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWearableRazorback;

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWearableDemoShield;

#[tf2_struct(baselcass = WeaponFlareGun)]
pub struct WeaponFlareGun_Revenge {
    #[offset(4276)]
    pub m_fLastExtinguishTime: f32,
}

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponFlareGun {
    #[offset(4248)]
    pub m_flChargeBeginTime: f32,
}

#[tf2_struct(baselcass = BaseProjectile)]
pub struct TFBaseRocket {
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: Vector2,
    #[offset(3088)]
    pub m_vInitialVelocity: Vector2,
    #[offset(3100)]
    pub m_iDeflected: i32,
    #[offset(3112)]
    pub m_hLauncher: i32,
}

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct TFWeaponBaseMelee;

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct TFWeaponBaseGun;

#[tf2_struct(baselcass = BaseGrenade)]
pub struct TFWeaponBaseMerasmusGrenade;

#[tf2_struct(baselcass = BaseGrenade)]
pub struct TFWeaponBaseGrenadeProj {
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: Vector2,
    #[offset(3128)]
    pub m_vInitialVelocity: Vector2,
    #[offset(3144)]
    pub m_iDeflected: i32,
    #[offset(3148)]
    pub m_hDeflectOwner: i32,
    #[offset(3156)]
    pub m_bCritical: bool,
}

#[tf2_struct(baselcass = BaseCombatWeapon)]
pub struct TFWeaponBase {
    #[offset(3956)]
    pub m_iReloadMode: i32,
    #[offset(3992)]
    pub m_bLowered: bool,
    #[offset(4156)]
    pub m_bResetParity: bool,
    #[offset(4180)]
    pub m_bReloadedThroughAnimEvent: bool,
    #[offset(4184)]
    pub m_flEnergy: f32,
    #[offset(4188)]
    pub m_iConsecutiveShots: i32,
    #[offset(4192)]
    pub m_bDisguiseWeapon: bool,
    #[offset(4200)]
    pub m_hExtraWearable: i32,
    #[offset(4204)]
    pub m_hExtraWearableViewModel: i32,
    #[offset(4212)]
    pub m_bBeingRepurposedForTaunt: bool,
    #[offset(4216)]
    pub m_nKillComboClass: i32,
    #[offset(4220)]
    pub m_nKillComboCount: i32,
    #[offset(4224)]
    pub m_flInspectAnimEndTime: f32,
    #[offset(4228)]
    pub m_nInspectStage: i32,
}

impl TFWeaponBase {
    pub type LocalActiveTFWeaponData = LocalTFWeaponData;
    pub type NonLocalTFWeaponData = TFWeaponDataNonLocal;
}

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWearableRobotArm;

#[tf2_struct(baselcass = TFWeaponWrench)]
pub struct TFWeaponRobotArm {
    #[offset(4264)]
    pub m_hRobotArm: i32,
}

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponWrench;

#[tf2_struct(baselcass = TFProjectile_Throwable)]
pub struct TFProjectile_ThrowableBreadMonster;

#[tf2_struct(baselcass = TFProjectile_Throwable)]
pub struct TFProjectile_ThrowableBrick;

#[tf2_struct(baselcass = TFProjectile_Throwable)]
pub struct TFProjectile_ThrowableRepel;

#[tf2_struct(baselcass = TFProjectile_Jar)]
pub struct TFProjectile_Throwable;

#[tf2_struct(baselcass = TFWeaponJar)]
pub struct TFWeaponThrowable {
    #[offset(4264)]
    pub m_flChargeBeginTime: f32,
}

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponSyringeGun;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponKatana {
    #[offset(4268)]
    pub m_bIsBloody: bool,
}

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponSword;

#[tf2_struct(baselcass = BaseEntity)]
pub struct SniperDot {
    #[offset(2016)]
    pub m_flChargeStartTime: f32,
}

#[tf2_struct(baselcass = TFSniperRifle)]
pub struct TFSniperRifleClassic {
    #[offset(4276)]
    pub m_bCharging: bool,
}

#[tf2_struct(baselcass = TFSniperRifle)]
pub struct TFSniperRifleDecap;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct TFSniperRifle;

impl TFSniperRifle {
    pub type SniperRifleLocalData = SniperRifleLocalData;
}

#[tf2_struct(baselcass = TFSMG)]
pub struct WeaponChargedSMG {
    #[offset(4248)]
    pub m_flMinicritCharge: f32,
}

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct TFSMG;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponSlap {
    #[offset(4262)]
    pub m_bFirstHit: bool,
    #[offset(4264)]
    pub m_nNumKills: i32,
}

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponShovel;

#[tf2_struct(baselcass = TFShotgun)]
pub struct TFShotgunBuildingRescue;

#[tf2_struct(baselcass = TFScatterGun)]
pub struct TFPEPBrawlerBlaster;

#[tf2_struct(baselcass = TFScatterGun)]
pub struct TFSodaPopper;

#[tf2_struct(baselcass = TFShotgun)]
pub struct TFShotgun_Revenge;

#[tf2_struct(baselcass = TFShotgun)]
pub struct TFScatterGun;

#[tf2_struct(baselcass = TFShotgun)]
pub struct TFShotgun_Pyro;

#[tf2_struct(baselcass = TFShotgun)]
pub struct TFShotgun_HWG;

#[tf2_struct(baselcass = TFShotgun)]
pub struct TFShotgun_Soldier;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct TFShotgun;

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct Crossbow {
    #[offset(4256)]
    pub m_flRegenerateDuration: f32,
    #[offset(4260)]
    pub m_flLastUsedTimestamp: f32,
}

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct WeaponRocketLauncher_Mortar;

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct WeaponRocketLauncher_AirStrike;

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct WeaponRocketLauncher_DirectHit;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponRocketLauncher;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponRevolver;

#[tf2_struct(baselcass = WeaponRaygun)]
pub struct WeaponDRGPomson;

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct WeaponRaygun {
    #[offset(4261)]
    pub m_bUseNewProjectileCode: bool,
}

#[tf2_struct(baselcass = WeaponPistol_Scout)]
pub struct WeaponPistol_ScoutSecondary;

#[tf2_struct(baselcass = WeaponPistol_Scout)]
pub struct WeaponPistol_ScoutPrimary;

#[tf2_struct(baselcass = WeaponPistol)]
pub struct WeaponPistol_Scout;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponPistol;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponPipebombLauncher;

impl WeaponPipebombLauncher {
    pub type PipebombLauncherLocalData = PipebombLauncherLocalData;
}

#[tf2_struct(baselcass = TFWeaponPDA)]
pub struct TFWeaponPDA_Spy;

#[tf2_struct(baselcass = TFWeaponPDA)]
pub struct TFWeaponPDA_Engineer_Destroy;

#[tf2_struct(baselcass = TFWeaponPDA)]
pub struct TFWeaponPDA_Engineer_Build;

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWeaponPDAExpansion_Teleporter;

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWeaponPDAExpansion_Dispenser;

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct TFWeaponPDA;

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct ParticleCannon {
    #[offset(4256)]
    pub m_flChargeBeginTime: f32,
    #[offset(4260)]
    pub m_iChargeEffect: i32,
}

#[tf2_struct(baselcass = TFParachute)]
pub struct TFParachute_Secondary;

#[tf2_struct(baselcass = TFParachute)]
pub struct TFParachute_Primary;

#[tf2_struct(baselcass = TFWeaponBuffItem)]
pub struct TFParachute;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponMinigun {
    #[offset(4248)]
    pub m_iWeaponState: i32,
    #[offset(4252)]
    pub m_bCritShot: bool,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TFMedigunShield;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponMedigun {
    #[offset(4248)]
    pub m_hHealingTarget: i32,
    #[offset(4252)]
    pub m_hLastHealingTarget: i32,
    #[offset(4257)]
    pub m_bHealing: bool,
    #[offset(4258)]
    pub m_bAttacking: bool,
    #[offset(4280)]
    pub m_bHolstered: bool,
    #[offset(4281)]
    pub m_bChargeRelease: bool,
    #[offset(4288)]
    pub m_nChargeResistType: i32,
}

impl WeaponMedigun {
    pub type NonLocalTFWeaponMedigunData = TFWeaponMedigunDataNonLocal;
    pub type LocalTFWeaponMedigunData = LocalTFWeaponMedigunData;
}

#[tf2_struct(baselcass = WeaponLunchBox)]
pub struct TFLunchBox_Drink;

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct WeaponLunchBox {
    #[offset(4252)]
    pub m_bBroken: bool,
}

#[tf2_struct(baselcass = SniperDot)]
pub struct LaserDot;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct TFLaserPointer;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponKnife {
    #[offset(4276)]
    pub m_bReadyToBackstab: bool,
    #[offset(4277)]
    pub m_bKnifeExists: bool,
    #[offset(4280)]
    pub m_flKnifeRegenerateDuration: f32,
    #[offset(4284)]
    pub m_flKnifeMeltTimestamp: f32,
}

#[tf2_struct(baselcass = TFPointManager)]
pub struct TFGasManager;

#[tf2_struct(baselcass = TFProjectile_Jar)]
pub struct TFProjectile_JarGas;

#[tf2_struct(baselcass = TFWeaponJar)]
pub struct TFWeaponJarGas;

#[tf2_struct(baselcass = TFProjectile_Jar)]
pub struct TFProjectile_Cleaver;

#[tf2_struct(baselcass = TFProjectile_Jar)]
pub struct TFProjectile_JarMilk;

#[tf2_struct(baselcass = TFProjectile_Pipebomb)]
pub struct TFProjectile_Jar;

#[tf2_struct(baselcass = TFWeaponJar)]
pub struct TFWeaponCleaver;

#[tf2_struct(baselcass = TFWeaponJar)]
pub struct TFWeaponJarMilk;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct TFWeaponJar;

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct TFWeaponInvis;

#[tf2_struct(baselcass = WeaponGrenadeLauncher)]
pub struct TFCannon;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponGrenadeLauncher {
    #[offset(4260)]
    pub m_flDetonateTime: f32,
    #[offset(4264)]
    pub m_iCurrentTube: i32,
    #[offset(4268)]
    pub m_iGoalTube: i32,
}

#[tf2_struct(baselcass = TFWeaponBaseGrenadeProj)]
pub struct TFProjectile_Pipebomb {
    #[offset(3140)]
    pub m_hLauncher: i32,
    #[offset(3157)]
    pub m_bTouched: bool,
    #[offset(3160)]
    pub m_iType: i32,
    #[offset(3180)]
    pub m_bDefensiveBomb: bool,
}

#[tf2_struct(baselcass = WeaponRocketLauncher)]
pub struct GrapplingHook {
    #[offset(4272)]
    pub m_hProjectile: i32,
}

#[tf2_struct(baselcass = TFBaseRocket)]
pub struct TFFlameRocket;

#[tf2_struct(baselcass = TFWeaponBaseGun)]
pub struct WeaponFlameThrower {
    #[offset(4248)]
    pub m_iWeaponState: i32,
    #[offset(4252)]
    pub m_bCritFire: bool,
    #[offset(4253)]
    pub m_bHitTarget: bool,
    #[offset(4264)]
    pub m_flChargeBeginTime: f32,
}

impl WeaponFlameThrower {
    pub type LocalFlameThrowerData = LocalFlameThrower;
}

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponFists;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponFireAxe;

#[tf2_struct(baselcass = WeaponFlameThrower)]
pub struct WeaponFlameBall {
    #[offset(4968)]
    pub m_flRechargeScale: f32,
}

#[tf2_struct(baselcass = WeaponPipebombLauncher)]
pub struct WeaponCompoundBow {
    #[offset(4320)]
    pub m_bNoFire: bool,
    #[offset(4321)]
    pub m_bArrowAlight: bool,
}

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponClub;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponBuffItem;

#[tf2_struct(baselcass = TFWeaponBreakableMelee)]
pub struct TFWeaponStickBomb {
    #[offset(4264)]
    pub m_iDetonated: i32,
}

#[tf2_struct(baselcass = TFWeaponBreakableMelee)]
pub struct TFWeaponBreakableSign;

#[tf2_struct(baselcass = TFWeaponBreakableMelee)]
pub struct TFWeaponBottle;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponBreakableMelee {
    #[offset(4262)]
    pub m_bBroken: bool,
}

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponBonesaw;

#[tf2_struct(baselcass = TFProjectile_StunBall)]
pub struct TFProjectileBall_Ornament;

#[tf2_struct(baselcass = TFProjectile_Pipebomb)]
pub struct TFProjectile_StunBall;

#[tf2_struct(baselcass = TFWeaponBat_Wood)]
pub struct TFWeaponBat_Giftwrap;

#[tf2_struct(baselcass = TFWeaponBat)]
pub struct TFWeaponBat_Wood;

#[tf2_struct(baselcass = TFWeaponBat)]
pub struct TFWeaponBat_Fish;

#[tf2_struct(baselcass = TFWeaponBaseMelee)]
pub struct TFWeaponBat;

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TFDroppedWeapon {
    #[offset(3096)]
    pub m_Item: ScriptCreatedItem,
    #[offset(3432)]
    pub m_flChargeLevel: f32,
}

#[tf2_struct(baselcass = TFWeaponBuilder)]
pub struct TFWeaponSapper {
    #[offset(4312)]
    pub m_flChargeBeginTime: f32,
}

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct TFWeaponBuilder {
    #[offset(4248)]
    pub m_iBuildState: i32,
    #[offset(4256)]
    pub m_iObjectMode: i32,
    #[offset(4296)]
    pub m_flWheatleyTalkingUntil: f32,
}

impl TFWeaponBuilder {
    pub type BuilderLocalData = BuilderLocalData;
}

#[tf2_struct(baselcass = TFBaseRocket)]
pub struct TFProjectile_Rocket {
    #[offset(3116)]
    pub m_bCritical: bool,
}

#[tf2_struct(baselcass = TFBaseRocket)]
pub struct TFProjectile_Flare {
    #[offset(3116)]
    pub m_bCritical: bool,
}

#[tf2_struct(baselcass = TFBaseProjectile)]
pub struct TFProjectile_EnergyRing;

#[tf2_struct(baselcass = TFBaseRocket)]
pub struct TFProjectile_EnergyBall {
    #[offset(3128)]
    pub m_bChargedShot: bool,
    #[offset(3132)]
    pub m_vColor1: Vector2,
    #[offset(3144)]
    pub m_vColor2: Vector2,
}

#[tf2_struct(baselcass = TFProjectile_Arrow)]
pub struct TFProjectile_GrapplingHook;

#[tf2_struct(baselcass = TFProjectile_Arrow)]
pub struct TFProjectile_HealingBolt;

#[tf2_struct(baselcass = TFBaseRocket)]
pub struct TFProjectile_Arrow {
    #[offset(3125)]
    pub m_bArrowAlight: bool,
    #[offset(3126)]
    pub m_bCritical: bool,
    #[offset(3144)]
    pub m_iProjectileType: i32,
}

#[tf2_struct(baselcass = TFRobotDestructionLogic)]
pub struct TFPlayerDestructionLogic {
    #[offset(2292)]
    pub m_hRedTeamLeader: i32,
    #[offset(2296)]
    pub m_hBlueTeamLeader: i32,
    #[offset(2300)]
    pub m_bUsingCountdownImage: bool,
    #[offset(2301)]
    pub m_iszCountdownImage: [i8; 260],
}

#[tf2_struct(baselcass = ObjectDispenser)]
pub struct PlayerDestructionDispenser;

#[tf2_struct(baselcass = ObjectDispenser)]
pub struct RobotDispenser;

#[tf2_struct(baselcass = BaseCombatCharacter)]
pub struct TFRobotDestruction_Robot {
    #[offset(4588)]
    pub m_iHealth: i32,
    #[offset(4592)]
    pub m_iMaxHealth: i32,
    #[offset(4724)]
    pub m_eType: i32,
}

#[tf2_struct()]
pub struct TFRobotDestructionLogic {
    #[offset(1972)]
    pub m_nMaxPoints: i32,
    #[offset(1976)]
    pub m_flFinaleLength: f32,
    #[offset(1980)]
    pub m_flBlueFinaleEndTime: f32,
    #[offset(1984)]
    pub m_flRedFinaleEndTime: f32,
    #[offset(1988)]
    pub m_nBlueScore: i32,
    #[offset(1992)]
    pub m_nRedScore: i32,
    #[offset(1996)]
    pub m_nBlueTargetPoints: i32,
    #[offset(2000)]
    pub m_nRedTargetPoints: i32,
    #[offset(2004)]
    pub m_flBlueTeamRespawnScale: f32,
    #[offset(2008)]
    pub m_flRedTeamRespawnScale: f32,
    #[offset(2012)]
    pub m_szResFile: [i8; 260],
    #[offset(2272)]
    pub m_eWinningMethod: [i32; 4],
    #[offset(2288)]
    pub m_flCountdownEndTime: f32,
}

#[tf2_struct()]
pub struct TFRobotDestruction_RobotGroup {
    #[offset(1976)]
    pub m_iTeamNum: i32,
    #[offset(1980)]
    pub m_pszHudIcon: [i8; 260],
    #[offset(2240)]
    pub m_nGroupNumber: i32,
    #[offset(2244)]
    pub m_nState: i32,
    #[offset(2248)]
    pub m_flRespawnStartTime: f32,
    #[offset(2252)]
    pub m_flRespawnEndTime: f32,
    #[offset(2256)]
    pub m_flLastAttackedTime: f32,
}

#[tf2_struct()]
pub struct TFRobotDestructionRobotSpawn;

#[tf2_struct()]
pub struct TFMinigameLogic {
    #[offset(1968)]
    pub m_hActiveMinigame: i32,
}

#[tf2_struct(baselcass = TFHalloweenMinigame)]
pub struct TFHalloweenMinigame_FallingPlatforms;

#[tf2_struct(baselcass = TFMinigame)]
pub struct TFHalloweenMinigame;

#[tf2_struct()]
pub struct TFMinigame {
    #[offset(1976)]
    pub m_pszHudResFile: [i8; 260],
    #[offset(2236)]
    pub m_nMaxScoreForMiniGame: i32,
    #[offset(2240)]
    pub m_nMinigameTeamScore: [i32; 4],
    #[offset(2256)]
    pub m_eScoringType: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TFPumpkinBomb;

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TFGenericBomb;

#[tf2_struct(baselcass = BaseViewModel)]
pub struct TFViewModel;

#[tf2_struct(baselcass = TFProjectile_Rocket)]
pub struct TFProjectile_BallOfFire {
    #[offset(3152)]
    pub m_vecSpawnOrigin: Vector2,
    #[offset(3164)]
    pub m_vecInitialVelocity: Vector2,
}

#[tf2_struct(baselcass = BaseProjectile)]
pub struct TFBaseProjectile {
    #[offset(3096)]
    pub m_vInitialVelocity: Vector2,
    #[offset(3112)]
    pub m_hLauncher: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct TFPointManager {
    #[offset(2128)]
    pub m_nRandomSeed: i32,
    #[offset(2132)]
    pub m_nSpawnTime: [i32; 30],
    #[offset(2252)]
    pub m_unNextPointIndex: i32,
}

#[tf2_struct(baselcass = TFWearable)]
pub struct TFPowerupBottle {
    #[offset(3790)]
    pub m_bActive: bool,
    #[offset(3791)]
    pub m_usNumCharges: i32,
}

#[tf2_struct(baselcass = BaseObject)]
pub struct BaseObjectUpgrade;

#[tf2_struct(baselcass = DynamicProp)]
pub struct TFItem;

#[tf2_struct(baselcass = BaseEntity)]
pub struct HalloweenSoulPack {
    #[offset(1968)]
    pub m_hTarget: i32,
    #[offset(1976)]
    pub m_vecPreCurvePos: Vector2,
    #[offset(1988)]
    pub m_vecStartCurvePos: Vector2,
    #[offset(2000)]
    pub m_flDuration: f32,
}

#[tf2_struct(baselcass = BaseCombatCharacter)]
pub struct TFTauntProp;

#[tf2_struct()]
pub struct MonsterResource {
    #[offset(1968)]
    pub m_iBossHealthPercentageByte: i32,
    #[offset(1972)]
    pub m_iBossStunPercentageByte: i32,
    #[offset(1976)]
    pub m_iSkillShotCompleteCount: i32,
    #[offset(1980)]
    pub m_fSkillShotComboEndTime: f32,
    #[offset(1984)]
    pub m_iBossState: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TFReviveMarker {
    #[offset(3084)]
    pub m_hOwner: i32,
    #[offset(3096)]
    pub m_nRevives: i32,
    #[offset(3100)]
    pub m_iHealth: i32,
    #[offset(3104)]
    pub m_iMaxHealth: i32,
}

#[tf2_struct(baselcass = CHalloweenPickup)]
pub struct CHalloweenGiftPickup {
    #[offset(3112)]
    pub m_hTargetPlayer: i32,
}

#[tf2_struct(baselcass = CHalloweenPickup)]
pub struct CBonusDuckPickup {
    #[offset(3128)]
    pub m_bSpecial: bool,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct CHalloweenPickup;

#[tf2_struct(baselcass = BaseAnimating)]
pub struct CBonusPack;

#[tf2_struct()]
pub struct BonusRoundLogic {
    //probably invalid
    #[offset(0)]
    pub m_aBonusPlayerRoll: [i32; 101],
    #[offset(2048)]
    pub m_hBonusWinner: i32,
    #[offset(2064)]
    pub m_Item: ScriptCreatedItem,
}

#[tf2_struct(baselcass = TeamplayRoundBasedRulesProxy)]
pub struct TFGameRulesProxy;

impl TFGameRulesProxy {
    pub type tf_gamerules_data = TFGameRules;
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TETFParticleEffect {
    #[offset(32)]
    pub m_vecOrigin: [f32; 3],
    #[offset(44)]
    pub m_vecStart: [f32; 3],
    #[offset(56)]
    pub m_vecAngles: Vector2,
    #[offset(68)]
    pub m_iParticleSystemIndex: i32,
    #[offset(76)]
    pub m_iAttachType: i32,
    #[offset(80)]
    pub m_iAttachmentPointIndex: i32,
    #[offset(84)]
    pub m_bResetParticles: bool,
    #[offset(85)]
    pub m_bCustomColors: bool,
    #[offset(88)]
    pub m_CustomColors_m_vecColor1: Vector2,
    #[offset(100)]
    pub m_CustomColors_m_vecColor2: Vector2,
    #[offset(112)]
    pub m_bControlPoint1: bool,
    #[offset(116)]
    pub m_ControlPoint1_m_eParticleAttachment: i32,
    #[offset(120)]
    pub m_ControlPoint1_m_vecOffset: [f32; 3],
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TETFExplosion {
    #[offset(32)]
    pub m_vecOrigin: [f32; 3],
    #[offset(44)]
    pub m_vecNormal: Vector2,
    #[offset(56)]
    pub m_iWeaponID: i32,
    #[offset(64)]
    pub m_nDefID: i32,
    #[offset(68)]
    pub m_nSound: i32,
    #[offset(72)]
    pub m_iCustomParticleIndex: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TETFBlood {
    #[offset(32)]
    pub m_vecOrigin: [f32; 3],
    #[offset(44)]
    pub m_vecNormal: Vector2,
}

#[tf2_struct(baselcass = TFPointManager)]
pub struct TFFlameManager {
    #[offset(2368)]
    pub m_hWeapon: i32,
    #[offset(2372)]
    pub m_hAttacker: i32,
    #[offset(2376)]
    pub m_flSpreadDegree: f32,
    #[offset(2380)]
    pub m_flRedirectedFlameSizeMult: f32,
    #[offset(2384)]
    pub m_flFlameStartSizeMult: f32,
    #[offset(2388)]
    pub m_flFlameEndSizeMult: f32,
    #[offset(2392)]
    pub m_flFlameIgnorePlayerVelocity: f32,
    #[offset(2396)]
    pub m_flFlameReflectionAdditionalLifeTime: f32,
    #[offset(2400)]
    pub m_flFlameReflectionDamageReduction: f32,
    #[offset(2404)]
    pub m_iMaxFlameReflectionCount: i32,
    #[offset(2408)]
    pub m_nShouldReflect: i32,
    #[offset(2412)]
    pub m_flFlameSpeed: f32,
    #[offset(2416)]
    pub m_flFlameLifeTime: f32,
    #[offset(2420)]
    pub m_flRandomLifeTimeOffset: f32,
    #[offset(2424)]
    pub m_flFlameGravity: f32,
    #[offset(2428)]
    pub m_flFlameDrag: f32,
    #[offset(2432)]
    pub m_flFlameUp: f32,
    #[offset(2436)]
    pub m_bIsFiring: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct CaptureFlagReturnIcon;

#[tf2_struct(baselcass = TFItem)]
pub struct CaptureFlag {
    #[offset(3136)]
    pub m_bDisabled: bool,
    #[offset(3137)]
    pub m_bVisibleWhenDisabled: bool,
    #[offset(3140)]
    pub m_nType: i32,
    #[offset(3144)]
    pub m_nFlagStatus: i32,
    #[offset(3148)]
    pub m_flResetTime: f32,
    #[offset(3152)]
    pub m_flMaxResetTime: f32,
    #[offset(3156)]
    pub m_flNeutralTime: f32,
    #[offset(3160)]
    pub m_hPrevOwner: i32,
    #[offset(3164)]
    pub m_nPointValue: i32,
    #[offset(3168)]
    pub m_flAutoCapTime: f32,
    #[offset(3172)]
    pub m_bGlowEnabled: bool,
    #[offset(3173)]
    pub m_szModel: [i8; 260],
    #[offset(3433)]
    pub m_szHudIcon: [i8; 260],
    #[offset(3693)]
    pub m_szPaperEffect: [i8; 260],
    #[offset(3953)]
    pub m_szTrailEffect: [i8; 260],
    #[offset(4216)]
    pub m_nUseTrailEffect: i32,
    #[offset(4252)]
    pub m_flTimeToSetPoisonous: f32,
}

#[tf2_struct(baselcass = Team)]
pub struct TFTeam {
    #[offset(2088)]
    pub m_nFlagCaptures: i32,
    #[offset(2092)]
    pub m_iRole: i32,
    #[offset(2096)]
    pub m_hLeader: i32,
}

impl TFTeam {
    pub type team_object_array = [i32; 606];
}

#[tf2_struct(baselcass = PlayerResource)]
pub struct TFPlayerResource {
    #[offset(6096)]
    pub m_iTotalScore: [i32; 102],
    #[offset(6504)]
    pub m_iMaxHealth: [i32; 102],
    #[offset(6912)]
    pub m_iMaxBuffedHealth: [i32; 102],
    #[offset(7320)]
    pub m_iPlayerClass: [i32; 102],
    #[offset(7728)]
    pub m_bArenaSpectator: [bool; 102],
    #[offset(7832)]
    pub m_iActiveDominations: [i32; 102],
    #[offset(8240)]
    pub m_flNextRespawnTime: [f32; 102],
    #[offset(8648)]
    pub m_iChargeLevel: [i32; 102],
    #[offset(9056)]
    pub m_iDamage: [i32; 102],
    #[offset(9464)]
    pub m_iDamageAssist: [i32; 102],
    #[offset(9872)]
    pub m_iDamageBoss: [i32; 102],
    #[offset(10280)]
    pub m_iHealing: [i32; 102],
    #[offset(10688)]
    pub m_iHealingAssist: [i32; 102],
    #[offset(11096)]
    pub m_iDamageBlocked: [i32; 102],
    #[offset(11504)]
    pub m_iCurrencyCollected: [i32; 102],
    #[offset(11912)]
    pub m_iBonusPoints: [i32; 102],
    #[offset(12320)]
    pub m_iPlayerLevel: [i32; 102],
    #[offset(12728)]
    pub m_iStreaks: [i32; 408],
    #[offset(14360)]
    pub m_iUpgradeRefundCredits: [i32; 102],
    #[offset(14768)]
    pub m_iBuybackCredits: [i32; 102],
    #[offset(15176)]
    pub m_iPartyLeaderBlueTeamIndex: i32,
    #[offset(15180)]
    pub m_iPartyLeaderRedTeamIndex: i32,
    #[offset(15184)]
    pub m_iEventTeamStatus: i32,
    #[offset(15188)]
    pub m_iPlayerClassWhenKilled: [i32; 102],
    #[offset(15596)]
    pub m_iConnectionState: [i32; 102],
    #[offset(16004)]
    pub m_flConnectTime: [f32; 102],
}

#[tf2_struct(baselcass = BasePlayer)]
pub struct TFPlayer {
    #[offset(536)]
    pub m_nWaterLevel: i32,
    #[offset(4565)]
    pub m_bGlowEnabled: bool,
    #[offset(6768)]
    pub m_iKartState: i32,
    #[offset(6776)]
    pub m_AttributeManager: AttributeManager,
    #[offset(6920)]
    pub m_hRagdoll: i32,
    #[offset(7064)]
    pub m_PlayerClass: TFPlayerClassShared,
    #[offset(7776)]
    pub m_flKartNextAvailableBoost: f32,
    #[offset(7780)]
    pub m_iKartHealth: i32,
    #[offset(7800)]
    pub m_Shared: TFPlayerShared,
    #[offset(9120)]
    pub m_hItem: i32,
    #[offset(9168)]
    pub m_flLastDamageTime: f32,
    #[offset(9172)]
    pub m_bInPowerPlay: bool,
    #[offset(9176)]
    pub m_iSpawnCounter: i32,
    #[offset(9180)]
    pub m_bArenaSpectator: bool,
    #[offset(9181)]
    pub m_bIsMiniBoss: bool,
    #[offset(9182)]
    pub m_bIsABot: bool,
    #[offset(9184)]
    pub m_nBotSkill: i32,
    #[offset(9192)]
    pub m_bSaveMeParity: bool,
    #[offset(9208)]
    pub m_bAllowMoveDuringTaunt: bool,
    #[offset(9228)]
    pub m_bIsReadyToHighFive: bool,
    #[offset(9232)]
    pub m_hHighFivePartner: i32,
    #[offset(9236)]
    pub m_nForceTauntCam: i32,
    #[offset(9240)]
    pub m_flTauntYaw: f32,
    #[offset(9248)]
    pub m_nActiveTauntSlot: i32,
    #[offset(9256)]
    pub m_iTauntItemDefIndex: i32,
    #[offset(9260)]
    pub m_flCurrentTauntMoveSpeed: f32,
    #[offset(9264)]
    pub m_flVehicleReverseTime: f32,
    #[offset(14808)]
    pub m_flHeadScale: f32,
    #[offset(14812)]
    pub m_flTorsoScale: f32,
    #[offset(14816)]
    pub m_flHandScale: f32,
    #[offset(14848)]
    pub m_bUseBossHealthBar: bool,
    #[offset(14849)]
    pub m_bUsingVRHeadset: bool,
    #[offset(14850)]
    pub m_bForcedSkin: bool,
    #[offset(14852)]
    pub m_nForcedSkin: i32,
    #[offset(14868)]
    pub m_hGrapplingHookTarget: i32,
    #[offset(14872)]
    pub m_hSecondaryLastWeapon: i32,
    #[offset(14876)]
    pub m_bUsingActionSlot: bool,
    #[offset(14880)]
    pub m_iCampaignMedals: i32,
    #[offset(14884)]
    pub m_flInspectTime: f32,
    #[offset(14888)]
    pub m_flHelpmeButtonPressTime: f32,
    #[offset(14892)]
    pub m_bViewingCYOAPDA: bool,
    #[offset(14893)]
    pub m_bRegenerating: bool,
    #[offset(14968)]
    pub m_iPlayerSkinOverride: i32,
}

impl TFPlayer {
    pub type tfnonlocaldata = TFNonLocalPlayerExclusive;
    pub type tflocaldata = TFLocalPlayerExclusive;
    pub type TFSendHealersDataTable = TFSendHealersDataTable;
}

#[tf2_struct()]
pub struct TFRagdoll {
    //probably invalid
    #[offset(0)]
    pub m_hRagWearables: [i32; 8],
    #[offset(2064)]
    pub m_vecForce: Vector2,
    #[offset(2076)]
    pub m_nForceBone: i32,
    #[offset(4232)]
    pub m_vecRagdollVelocity: Vector2,
    #[offset(4244)]
    pub m_vecRagdollOrigin: Vector2,
    #[offset(4256)]
    pub m_hPlayer: i32,
    #[offset(4265)]
    pub m_bGib: bool,
    #[offset(4266)]
    pub m_bBurning: bool,
    #[offset(4267)]
    pub m_bElectrocuted: bool,
    #[offset(4270)]
    pub m_bFeignDeath: bool,
    #[offset(4271)]
    pub m_bWasDisguised: bool,
    #[offset(4272)]
    pub m_bCloaked: bool,
    #[offset(4273)]
    pub m_bBecomeAsh: bool,
    #[offset(4276)]
    pub m_iDamageCustom: i32,
    #[offset(4280)]
    pub m_bGoldRagdoll: bool,
    #[offset(4281)]
    pub m_bIceRagdoll: bool,
    #[offset(4320)]
    pub m_iTeam: i32,
    #[offset(4324)]
    pub m_iClass: i32,
    #[offset(4334)]
    pub m_bOnGround: bool,
    #[offset(4396)]
    pub m_bCritOnHardHit: bool,
    #[offset(4400)]
    pub m_flHeadScale: f32,
    #[offset(4404)]
    pub m_flTorsoScale: f32,
    #[offset(4408)]
    pub m_flHandScale: f32,
}

#[tf2_struct()]
pub struct TEPlayerAnimEvent {
    #[offset(32)]
    pub m_hPlayer: i32,
    #[offset(36)]
    pub m_iEvent: i32,
    #[offset(40)]
    pub m_nData: i32,
}

#[tf2_struct(baselcass = BaseTeamObjectiveResource)]
pub struct TFObjectiveResource {
    #[offset(7296)]
    pub m_nMannVsMachineMaxWaveCount: i32,
    #[offset(7300)]
    pub m_nMannVsMachineWaveCount: i32,
    #[offset(7304)]
    pub m_nMannVsMachineWaveEnemyCount: i32,
    #[offset(7308)]
    pub m_nMvMWorldMoney: i32,
    #[offset(7312)]
    pub m_flMannVsMachineNextWaveTime: f32,
    #[offset(7316)]
    pub m_bMannVsMachineBetweenWaves: bool,
    #[offset(7320)]
    pub m_nFlagCarrierUpgradeLevel: i32,
    #[offset(7324)]
    pub m_flMvMBaseBombUpgradeTime: f32,
    #[offset(7328)]
    pub m_flMvMNextBombUpgradeTime: f32,
    #[offset(7332)]
    pub m_nMvMEventPopfileType: i32,
    #[offset(7336)]
    pub m_nMannVsMachineWaveClassCounts: [i32; 12],
    #[offset(7384)]
    pub m_nMannVsMachineWaveClassCounts2: [i32; 12],
    #[offset(7432)]
    pub m_iszMannVsMachineWaveClassNames: [[[i8; 64]; 1]; 12],
    #[offset(8200)]
    pub m_iszMannVsMachineWaveClassNames2: [[[i8; 64]; 1]; 12],
    #[offset(8968)]
    pub m_iChallengeIndex: i32,
    #[offset(8972)]
    pub m_iszMvMPopfileName: [i8; 260],
    #[offset(9232)]
    pub m_nMannVsMachineWaveClassFlags: [i32; 12],
    #[offset(9280)]
    pub m_nMannVsMachineWaveClassFlags2: [i32; 12],
    #[offset(9328)]
    pub m_bMannVsMachineWaveClassActive: [bool; 12],
    #[offset(9340)]
    pub m_bMannVsMachineWaveClassActive2: [bool; 12],
}

#[tf2_struct()]
pub struct TEFireBullets {
    #[offset(32)]
    pub m_iPlayer: i32,
    #[offset(36)]
    pub m_vecOrigin: Vector2,
    #[offset(48)]
    pub m_vecAngles: [f32; 2],
    #[offset(60)]
    pub m_iWeaponID: i32,
    #[offset(64)]
    pub m_iMode: i32,
    #[offset(68)]
    pub m_iSeed: i32,
    #[offset(72)]
    pub m_flSpread: f32,
    #[offset(76)]
    pub m_bCritical: bool,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct TFBuffBanner;

#[tf2_struct(baselcass = BaseAnimating)]
pub struct AmmoPack {
    #[offset(1108)]
    pub m_angRotation: [f32; 3],
    #[offset(3096)]
    pub m_vecInitialVelocity: Vector2,
}

#[tf2_struct(baselcass = BaseObject)]
pub struct ObjectTeleporter {
    #[offset(5108)]
    pub m_iState: i32,
    #[offset(5116)]
    pub m_flRechargeTime: f32,
    #[offset(5120)]
    pub m_flCurrentRechargeDuration: f32,
    #[offset(5124)]
    pub m_iTimesUsed: i32,
    #[offset(5128)]
    pub m_flYawToExit: f32,
    #[offset(5132)]
    pub m_bMatchBuilding: bool,
}

#[tf2_struct(baselcass = BaseObject)]
pub struct ObjectSentrygun {
    #[offset(5108)]
    pub m_iState: i32,
    #[offset(5112)]
    pub m_iAmmoShells: i32,
    #[offset(5120)]
    pub m_iAmmoRockets: i32,
    #[offset(5144)]
    pub m_bPlayerControlled: bool,
    #[offset(5148)]
    pub m_nShieldLevel: i32,
    #[offset(5256)]
    pub m_hEnemy: i32,
    #[offset(5260)]
    pub m_hAutoAimTarget: i32,
}

impl ObjectSentrygun {
    pub type SentrygunLocalData = SentrygunLocalData;
}

#[tf2_struct(baselcass = TFProjectile_Rocket)]
pub struct TFProjectile_SentryRocket;

#[tf2_struct(baselcass = BaseObjectUpgrade)]
pub struct ObjectSapper;

#[tf2_struct(baselcass = ObjectDispenser)]
pub struct ObjectCartDispenser;

#[tf2_struct(baselcass = BaseObject)]
pub struct ObjectDispenser {
    #[offset(5148)]
    pub m_iState: i32,
    #[offset(5152)]
    pub m_iAmmoMetal: i32,
    #[offset(5156)]
    pub m_iMiniBombCounter: i32,
}

impl ObjectDispenser {
    pub type healing_array = [i32; 101];
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct CaptureZone {
    #[offset(1976)]
    pub m_bDisabled: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncRespawnRoomVisualizer;

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncRespawnRoom;

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncForceField;

#[tf2_struct(baselcass = BaseAnimating)]
pub struct CurrencyPack {
    #[offset(3097)]
    pub m_bDistributed: bool,
}

#[tf2_struct(baselcass = BaseCombatCharacter)]
pub struct BaseObject {
    #[offset(4916)]
    pub m_iUpgradeLevel: i32,
    #[offset(4924)]
    pub m_iUpgradeMetal: i32,
    #[offset(4928)]
    pub m_iHighestUpgradeLevel: i32,
    #[offset(4932)]
    pub m_iUpgradeMetalRequired: i32,
    #[offset(4964)]
    pub m_fObjectFlags: i32,
    #[offset(4980)]
    pub m_hBuilder: i32,
    #[offset(4992)]
    pub m_bHasSapper: bool,
    #[offset(4996)]
    pub m_iObjectType: i32,
    #[offset(5000)]
    pub m_iHealth: i32,
    #[offset(5004)]
    pub m_iMaxHealth: i32,
    #[offset(5009)]
    pub m_bBuilding: bool,
    #[offset(5011)]
    pub m_bPlacing: bool,
    #[offset(5012)]
    pub m_bDisabled: bool,
    #[offset(5014)]
    pub m_bCarried: bool,
    #[offset(5015)]
    pub m_bCarryDeploy: bool,
    #[offset(5017)]
    pub m_bMiniBuilding: bool,
    #[offset(5018)]
    pub m_bDisposableBuilding: bool,
    #[offset(5020)]
    pub m_flPercentageConstructed: f32,
    #[offset(5024)]
    pub m_hBuiltOnEntity: i32,
    #[offset(5028)]
    pub m_iObjectMode: i32,
    #[offset(5032)]
    pub m_bPlasmaDisable: bool,
    #[offset(5036)]
    pub m_vecBuildMaxs: Vector2,
    #[offset(5048)]
    pub m_vecBuildMins: Vector2,
    #[offset(5060)]
    pub m_iDesiredBuildRotations: i32,
    #[offset(5072)]
    pub m_bServerOverridePlacement: bool,
    #[offset(5104)]
    pub m_bWasMapPlaced: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct TFGlow {
    #[offset(1976)]
    pub m_iMode: i32,
    #[offset(1980)]
    pub m_glowColor: i32,
    #[offset(1984)]
    pub m_bDisabled: bool,
    #[offset(1988)]
    pub m_hTarget: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct TFPasstimeLogic {
    #[offset(2832)]
    pub m_bPlayerIsPackMember: [bool; 102],
    #[offset(2936)]
    pub m_hBall: i32,
    #[offset(2940)]
    pub m_trackPoints: [[Vector2; 1]; 16],
    #[offset(3132)]
    pub m_iNumSections: i32,
    #[offset(3136)]
    pub m_iCurrentSection: i32,
    #[offset(3140)]
    pub m_flMaxPassRange: f32,
    #[offset(3144)]
    pub m_iBallPower: i32,
    #[offset(3148)]
    pub m_flPackSpeed: f32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct PasstimeBall {
    #[offset(3088)]
    pub m_iCollisionCount: i32,
    #[offset(3092)]
    pub m_hHomingTarget: i32,
    #[offset(3096)]
    pub m_hCarrier: i32,
    #[offset(3100)]
    pub m_hPrevCarrier: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncPasstimeGoal {
    #[offset(1976)]
    pub m_bTriggerDisabled: bool,
    #[offset(1980)]
    pub m_iGoalType: i32,
}

#[tf2_struct(baselcass = TFWeaponBase)]
pub struct PasstimeGun {
    #[offset(4304)]
    pub m_eThrowState: i32,
    #[offset(4308)]
    pub m_fChargeBeginTime: f32,
}

#[tf2_struct(baselcass = TFWearable)]
pub struct TFWearableVM;

#[tf2_struct(baselcass = WearableItem)]
pub struct TFWearable {
    #[offset(3744)]
    pub m_bDisguiseWearable: bool,
    #[offset(3748)]
    pub m_hWeaponAssociatedWith: i32,
}

#[tf2_struct()]
pub struct TeamRoundTimer {
    #[offset(1965)]
    pub m_bTimerPaused: bool,
    #[offset(1968)]
    pub m_flTimeRemaining: f32,
    #[offset(1972)]
    pub m_flTimerEndTime: f32,
    #[offset(1976)]
    pub m_bIsDisabled: bool,
    #[offset(1977)]
    pub m_bShowInHUD: bool,
    #[offset(1980)]
    pub m_nTimerLength: i32,
    #[offset(1984)]
    pub m_nTimerInitialLength: i32,
    #[offset(1988)]
    pub m_nTimerMaxLength: i32,
    #[offset(1992)]
    pub m_bAutoCountdown: bool,
    #[offset(1996)]
    pub m_nSetupTimeLength: i32,
    #[offset(2000)]
    pub m_nState: i32,
    #[offset(2004)]
    pub m_bStartPaused: bool,
    #[offset(2005)]
    pub m_bShowTimeRemaining: bool,
    #[offset(2006)]
    pub m_bInCaptureWatchState: bool,
    #[offset(2008)]
    pub m_flTotalTime: f32,
    #[offset(2012)]
    pub m_bStopWatchTimer: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct TeamTrainWatcher {
    #[offset(1968)]
    pub m_flTotalProgress: f32,
    #[offset(1976)]
    pub m_iTrainSpeedLevel: i32,
    #[offset(1984)]
    pub m_flRecedeTime: f32,
    #[offset(1992)]
    pub m_nNumCappers: i32,
    #[offset(2000)]
    pub m_hGlowEnt: i32,
}

#[tf2_struct()]
pub struct BaseTeamObjectiveResource {
    #[offset(1968)]
    pub m_iTimerToShowInHUD: i32,
    #[offset(1972)]
    pub m_iStopWatchTimer: i32,
    #[offset(1976)]
    pub m_iNumControlPoints: i32,
    #[offset(1984)]
    pub m_bPlayingMiniRounds: bool,
    #[offset(1985)]
    pub m_bControlPointsReset: bool,
    #[offset(1988)]
    pub m_iUpdateCapHudParity: i32,
    #[offset(1996)]
    pub m_vCPPositions: [[Vector2; 1]; 8],
    #[offset(2092)]
    pub m_bCPIsVisible: [bool; 8],
    #[offset(2100)]
    pub m_flLazyCapPerc: [f32; 8],
    #[offset(2164)]
    pub m_iTeamIcons: [i32; 64],
    #[offset(2420)]
    pub m_iTeamOverlays: [i32; 64],
    #[offset(2676)]
    pub m_iTeamReqCappers: [i32; 64],
    #[offset(2932)]
    pub m_flTeamCapTime: [f32; 64],
    #[offset(3188)]
    pub m_iPreviousPoints: [i32; 192],
    #[offset(3956)]
    pub m_bTeamCanCap: [bool; 64],
    #[offset(4020)]
    pub m_iTeamBaseIcons: [i32; 32],
    #[offset(4148)]
    pub m_iBaseControlPoints: [i32; 32],
    #[offset(4276)]
    pub m_bInMiniRound: [bool; 8],
    #[offset(4284)]
    pub m_iWarnOnCap: [i32; 8],
    #[offset(4316)]
    pub m_iszWarnSound: [[[i8; 255]; 1]; 8],
    #[offset(6356)]
    pub m_flPathDistance: [f32; 8],
    #[offset(6388)]
    pub m_iCPGroup: [i32; 8],
    #[offset(6420)]
    pub m_bCPLocked: [bool; 8],
    #[offset(6428)]
    pub m_flUnlockTimes: [f32; 8],
    #[offset(6492)]
    pub m_flCPTimerTimes: [f32; 8],
    #[offset(6556)]
    pub m_iNumTeamMembers: [i32; 64],
    #[offset(6812)]
    pub m_iCappingTeam: [i32; 8],
    #[offset(6844)]
    pub m_iTeamInZone: [i32; 8],
    #[offset(6876)]
    pub m_bBlocked: [bool; 8],
    #[offset(6884)]
    pub m_iOwner: [i32; 8],
    #[offset(6916)]
    pub m_bCPCapRateScalesWithPlayers: [bool; 8],
    #[offset(7028)]
    pub m_pszCapLayoutInHUD: [i8; 32],
    #[offset(7068)]
    pub m_flCustomPositionX: f32,
    #[offset(7072)]
    pub m_flCustomPositionY: f32,
    #[offset(7076)]
    pub m_nNumNodeHillData: [i32; 4],
    #[offset(7092)]
    pub m_flNodeHillData: [f32; 40],
    #[offset(7272)]
    pub m_bTrackAlarm: [bool; 4],
    #[offset(7276)]
    pub m_bHillIsDownhill: [bool; 20],
}

#[tf2_struct(baselcass = WearableItem)]
pub struct TFWearableItem;

#[tf2_struct(baselcass = EconEntity)]
pub struct WearableItem;

#[tf2_struct(baselcass = EconEntity)]
pub struct BaseAttributableItem {
    #[offset(3120)]
    pub m_AttributeManager: AttributeContainer,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct EconEntity {
    #[offset(3120)]
    pub m_AttributeManager: AttributeContainer,
    #[offset(3600)]
    pub m_bValidatedAttachedEntity: bool,
}

#[tf2_struct()]
pub struct TestTraceline {
    #[offset(128)]
    pub m_clrRender: i32,
    #[offset(556)]
    pub moveparent: i32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: [f32; 3],
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEWorldDecal {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_nIndex: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TESpriteSpray {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecDirection: Vector2,
    #[offset(56)]
    pub m_nModelIndex: i32,
    #[offset(60)]
    pub m_nSpeed: i32,
    #[offset(64)]
    pub m_fNoise: f32,
    #[offset(68)]
    pub m_nCount: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TESprite {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_nModelIndex: i32,
    #[offset(48)]
    pub m_fScale: f32,
    #[offset(52)]
    pub m_nBrightness: i32,
}

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TESparks {
    #[offset(44)]
    pub m_nMagnitude: i32,
    #[offset(48)]
    pub m_nTrailLength: i32,
    #[offset(52)]
    pub m_vecDir: Vector2,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TESmoke {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_nModelIndex: i32,
    #[offset(48)]
    pub m_fScale: f32,
    #[offset(52)]
    pub m_nFrameRate: i32,
}

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TEShowLine {
    #[offset(44)]
    pub m_vecEnd: Vector2,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEProjectedDecal {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_angRotation: Vector2,
    #[offset(56)]
    pub m_flDistance: f32,
    #[offset(60)]
    pub m_nIndex: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEPlayerDecal {
    #[offset(32)]
    pub m_nPlayer: i32,
    #[offset(36)]
    pub m_vecOrigin: Vector2,
    #[offset(48)]
    pub m_nEntity: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEPhysicsProp {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_angRotation: [f32; 3],
    #[offset(56)]
    pub m_vecVelocity: Vector2,
    #[offset(68)]
    pub m_nModelIndex: i32,
    #[offset(72)]
    pub m_nSkin: i32,
    #[offset(76)]
    pub m_nFlags: i32,
    #[offset(80)]
    pub m_nEffects: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEParticleSystem {
    #[offset(32)]
    pub m_vecOrigin: [f32; 3],
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEMuzzleFlash {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecAngles: Vector2,
    #[offset(56)]
    pub m_flScale: f32,
    #[offset(60)]
    pub m_nType: i32,
}

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TELargeFunnel {
    #[offset(44)]
    pub m_nModelIndex: i32,
    #[offset(48)]
    pub m_nReversed: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEKillPlayerAttachments {
    #[offset(32)]
    pub m_nPlayer: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEImpact {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecNormal: Vector2,
    #[offset(56)]
    pub m_iType: i32,
    #[offset(60)]
    pub m_ucFlags: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEGlowSprite {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_nModelIndex: i32,
    #[offset(48)]
    pub m_fScale: f32,
    #[offset(52)]
    pub m_fLife: f32,
    #[offset(56)]
    pub m_nBrightness: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEShatterSurface {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecAngles: Vector2,
    #[offset(56)]
    pub m_vecForce: Vector2,
    #[offset(68)]
    pub m_vecForcePos: Vector2,
    #[offset(80)]
    pub m_flWidth: f32,
    #[offset(84)]
    pub m_flHeight: f32,
    #[offset(88)]
    pub m_flShardSize: f32,
    #[offset(104)]
    pub m_nSurfaceType: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEFootprintDecal {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecDirection: Vector2,
    #[offset(68)]
    pub m_nEntity: i32,
    #[offset(72)]
    pub m_nIndex: i32,
    #[offset(76)]
    pub m_chMaterialType: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEFizz {
    #[offset(32)]
    pub m_nEntity: i32,
    #[offset(36)]
    pub m_nModelIndex: i32,
    #[offset(40)]
    pub m_nDensity: i32,
    #[offset(44)]
    pub m_nCurrent: i32,
}

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TEExplosion {
    #[offset(44)]
    pub m_nModelIndex: i32,
    #[offset(48)]
    pub m_fScale: f32,
    #[offset(52)]
    pub m_nFrameRate: i32,
    #[offset(56)]
    pub m_nFlags: i32,
    #[offset(60)]
    pub m_vecNormal: Vector2,
    #[offset(72)]
    pub m_chMaterialType: i32,
    #[offset(76)]
    pub m_nRadius: i32,
    #[offset(80)]
    pub m_nMagnitude: i32,
}

#[tf2_struct()]
pub struct TEEnergySplash {
    #[offset(32)]
    pub m_vecPos: Vector2,
    #[offset(44)]
    pub m_vecDir: Vector2,
    #[offset(56)]
    pub m_bExplosive: bool,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEEffectDispatch {
    #[offset(32)]
    pub m_EffectData: EffectData,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEDynamicLight {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_fRadius: f32,
    #[offset(48)]
    pub r: i32,
    #[offset(52)]
    pub g: i32,
    #[offset(56)]
    pub b: i32,
    #[offset(60)]
    pub exponent: i32,
    #[offset(64)]
    pub m_fTime: f32,
    #[offset(68)]
    pub m_fDecay: f32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEDecal {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecStart: Vector2,
    #[offset(56)]
    pub m_nEntity: i32,
    #[offset(60)]
    pub m_nHitbox: i32,
    #[offset(64)]
    pub m_nIndex: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEClientProjectile {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecVelocity: Vector2,
    #[offset(56)]
    pub m_nModelIndex: i32,
    #[offset(60)]
    pub m_nLifeTime: i32,
    #[offset(64)]
    pub m_hOwner: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEBubbleTrail {
    #[offset(32)]
    pub m_vecMins: Vector2,
    #[offset(44)]
    pub m_vecMaxs: Vector2,
    #[offset(56)]
    pub m_flWaterZ: f32,
    #[offset(60)]
    pub m_nModelIndex: i32,
    #[offset(64)]
    pub m_nCount: i32,
    #[offset(68)]
    pub m_fSpeed: f32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEBubbles {
    #[offset(32)]
    pub m_vecMins: Vector2,
    #[offset(44)]
    pub m_vecMaxs: Vector2,
    #[offset(56)]
    pub m_fHeight: f32,
    #[offset(60)]
    pub m_nModelIndex: i32,
    #[offset(64)]
    pub m_nCount: i32,
    #[offset(68)]
    pub m_fSpeed: f32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEBSPDecal {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_nEntity: i32,
    #[offset(48)]
    pub m_nIndex: i32,
}

#[tf2_struct(baselcass = BaseTempEntity)]
pub struct TEBreakModel {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_angRotation: [f32; 3],
    #[offset(56)]
    pub m_vecSize: Vector2,
    #[offset(68)]
    pub m_vecVelocity: Vector2,
    #[offset(80)]
    pub m_nRandomization: i32,
    #[offset(84)]
    pub m_nModelIndex: i32,
    #[offset(88)]
    pub m_nCount: i32,
    #[offset(92)]
    pub m_fTime: f32,
    #[offset(96)]
    pub m_nFlags: i32,
}

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TEBloodStream {
    #[offset(44)]
    pub m_vecDirection: Vector2,
    #[offset(56)]
    pub r: i32,
    #[offset(60)]
    pub g: i32,
    #[offset(64)]
    pub b: i32,
    #[offset(68)]
    pub a: i32,
    #[offset(72)]
    pub m_nAmount: i32,
}

#[tf2_struct()]
pub struct TEBloodSprite {
    #[offset(32)]
    pub m_vecOrigin: Vector2,
    #[offset(44)]
    pub m_vecDirection: Vector2,
    #[offset(56)]
    pub r: i32,
    #[offset(60)]
    pub g: i32,
    #[offset(64)]
    pub b: i32,
    #[offset(68)]
    pub a: i32,
    #[offset(72)]
    pub m_nDropModel: i32,
    #[offset(76)]
    pub m_nSprayModel: i32,
    #[offset(80)]
    pub m_nSize: i32,
}

#[tf2_struct()]
pub struct TEBeamSpline {
    #[offset(32)]
    pub m_vecPoints: [[Vector2; 1]; 16],
    #[offset(224)]
    pub m_nPoints: i32,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamRingPoint {
    #[offset(92)]
    pub m_vecCenter: Vector2,
    #[offset(104)]
    pub m_flStartRadius: f32,
    #[offset(108)]
    pub m_flEndRadius: f32,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamRing {
    #[offset(92)]
    pub m_nStartEntity: i32,
    #[offset(96)]
    pub m_nEndEntity: i32,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamPoints {
    #[offset(92)]
    pub m_vecStartPoint: Vector2,
    #[offset(104)]
    pub m_vecEndPoint: Vector2,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamLaser {
    #[offset(92)]
    pub m_nStartEntity: i32,
    #[offset(96)]
    pub m_nEndEntity: i32,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamFollow {
    #[offset(92)]
    pub m_iEntIndex: i32,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamEnts {
    #[offset(92)]
    pub m_nStartEntity: i32,
    #[offset(96)]
    pub m_nEndEntity: i32,
}

#[tf2_struct(baselcass = BaseBeam)]
pub struct TEBeamEntPoint {
    #[offset(92)]
    pub m_nStartEntity: i32,
    #[offset(96)]
    pub m_nEndEntity: i32,
    #[offset(100)]
    pub m_vecStartPoint: Vector2,
    #[offset(112)]
    pub m_vecEndPoint: Vector2,
}

#[tf2_struct()]
pub struct BaseBeam {
    #[offset(32)]
    pub m_nModelIndex: i32,
    #[offset(36)]
    pub m_nHaloIndex: i32,
    #[offset(40)]
    pub m_nStartFrame: i32,
    #[offset(44)]
    pub m_nFrameRate: i32,
    #[offset(48)]
    pub m_fLife: f32,
    #[offset(52)]
    pub m_fWidth: f32,
    #[offset(56)]
    pub m_fEndWidth: f32,
    #[offset(60)]
    pub m_nFadeLength: i32,
    #[offset(64)]
    pub m_fAmplitude: f32,
    #[offset(68)]
    pub r: i32,
    #[offset(72)]
    pub g: i32,
    #[offset(76)]
    pub b: i32,
    #[offset(80)]
    pub a: i32,
    #[offset(84)]
    pub m_nSpeed: i32,
    #[offset(88)]
    pub m_nFlags: i32,
}

#[tf2_struct(baselcass = TEMetalSparks)]
pub struct TEArmorRicochet;

#[tf2_struct()]
pub struct TEMetalSparks {
    #[offset(32)]
    pub m_vecPos: Vector2,
    #[offset(44)]
    pub m_vecDir: Vector2,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct SteamJet {
    #[offset(2280)]
    pub m_SpreadSpeed: f32,
    #[offset(2284)]
    pub m_Speed: f32,
    #[offset(2288)]
    pub m_StartSize: f32,
    #[offset(2292)]
    pub m_EndSize: f32,
    #[offset(2296)]
    pub m_Rate: f32,
    #[offset(2300)]
    pub m_JetLength: f32,
    #[offset(2304)]
    pub m_bEmit: bool,
    #[offset(2308)]
    pub m_nType: i32,
    #[offset(2312)]
    pub m_bFaceLeft: bool,
    #[offset(2316)]
    pub m_spawnflags: i32,
    #[offset(2320)]
    pub m_flRollSpeed: f32,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct SmokeStack {
    #[offset(2368)]
    pub m_SpreadSpeed: f32,
    #[offset(2372)]
    pub m_Speed: f32,
    #[offset(2376)]
    pub m_StartSize: f32,
    #[offset(2380)]
    pub m_EndSize: f32,
    #[offset(2384)]
    pub m_Rate: f32,
    #[offset(2388)]
    pub m_JetLength: f32,
    #[offset(2392)]
    pub m_bEmit: bool,
    #[offset(2396)]
    pub m_flBaseSpread: f32,
    #[offset(2400)]
    pub m_AmbientLight_m_vPos: Vector2,
    #[offset(2412)]
    pub m_AmbientLight_m_vColor: Vector2,
    #[offset(2424)]
    pub m_AmbientLight_m_flIntensity: f32,
    #[offset(2428)]
    pub m_DirLight_m_vPos: Vector2,
    #[offset(2440)]
    pub m_DirLight_m_vColor: Vector2,
    #[offset(2452)]
    pub m_DirLight_m_flIntensity: f32,
    #[offset(2468)]
    pub m_vWind: Vector2,
    #[offset(2480)]
    pub m_flTwist: f32,
    #[offset(2484)]
    pub m_iMaterialModel: i32,
    #[offset(2552)]
    pub m_flRollSpeed: f32,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct DustTrail {
    #[offset(2280)]
    pub m_SpawnRate: f32,
    #[offset(2284)]
    pub m_Color: Vector2,
    #[offset(2296)]
    pub m_Opacity: f32,
    #[offset(2300)]
    pub m_ParticleLifetime: f32,
    #[offset(2308)]
    pub m_StopEmitTime: f32,
    #[offset(2312)]
    pub m_MinSpeed: f32,
    #[offset(2316)]
    pub m_MaxSpeed: f32,
    #[offset(2320)]
    pub m_MinDirectedSpeed: f32,
    #[offset(2324)]
    pub m_MaxDirectedSpeed: f32,
    #[offset(2328)]
    pub m_StartSize: f32,
    #[offset(2332)]
    pub m_EndSize: f32,
    #[offset(2336)]
    pub m_SpawnRadius: f32,
    #[offset(2352)]
    pub m_bEmit: bool,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct FireTrail {
    #[offset(2280)]
    pub m_nAttachment: i32,
    #[offset(2284)]
    pub m_flLifetime: f32,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct SporeTrail {
    #[offset(2272)]
    pub m_vecEndColor: Vector2,
    #[offset(2284)]
    pub m_flSpawnRate: f32,
    #[offset(2288)]
    pub m_flParticleLifetime: f32,
    #[offset(2292)]
    pub m_flStartSize: f32,
    #[offset(2296)]
    pub m_flEndSize: f32,
    #[offset(2300)]
    pub m_flSpawnRadius: f32,
    #[offset(2316)]
    pub m_bEmit: bool,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct SporeExplosion {
    #[offset(2280)]
    pub m_flSpawnRate: f32,
    #[offset(2284)]
    pub m_flParticleLifetime: f32,
    #[offset(2288)]
    pub m_flStartSize: f32,
    #[offset(2292)]
    pub m_flEndSize: f32,
    #[offset(2296)]
    pub m_flSpawnRadius: f32,
    #[offset(2304)]
    pub m_bEmit: bool,
    #[offset(2305)]
    pub m_bDontRemove: bool,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct RocketTrail {
    #[offset(2280)]
    pub m_SpawnRate: f32,
    #[offset(2284)]
    pub m_StartColor: Vector2,
    #[offset(2296)]
    pub m_EndColor: Vector2,
    #[offset(2308)]
    pub m_Opacity: f32,
    #[offset(2312)]
    pub m_ParticleLifetime: f32,
    #[offset(2316)]
    pub m_StopEmitTime: f32,
    #[offset(2320)]
    pub m_MinSpeed: f32,
    #[offset(2324)]
    pub m_MaxSpeed: f32,
    #[offset(2328)]
    pub m_StartSize: f32,
    #[offset(2332)]
    pub m_EndSize: f32,
    #[offset(2336)]
    pub m_SpawnRadius: f32,
    #[offset(2352)]
    pub m_bEmit: bool,
    #[offset(2353)]
    pub m_bDamaged: bool,
    #[offset(2356)]
    pub m_nAttachment: i32,
    #[offset(2372)]
    pub m_flFlareScale: f32,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct SmokeTrail {
    #[offset(2280)]
    pub m_SpawnRate: f32,
    #[offset(2284)]
    pub m_StartColor: Vector2,
    #[offset(2296)]
    pub m_EndColor: Vector2,
    #[offset(2308)]
    pub m_Opacity: f32,
    #[offset(2312)]
    pub m_ParticleLifetime: f32,
    #[offset(2316)]
    pub m_StopEmitTime: f32,
    #[offset(2320)]
    pub m_MinSpeed: f32,
    #[offset(2324)]
    pub m_MaxSpeed: f32,
    #[offset(2328)]
    pub m_MinDirectedSpeed: f32,
    #[offset(2332)]
    pub m_MaxDirectedSpeed: f32,
    #[offset(2336)]
    pub m_StartSize: f32,
    #[offset(2340)]
    pub m_EndSize: f32,
    #[offset(2344)]
    pub m_SpawnRadius: f32,
    #[offset(2360)]
    pub m_bEmit: bool,
    #[offset(2364)]
    pub m_nAttachment: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct PropVehicleDriveable {
    #[offset(3096)]
    pub m_hPlayer: i32,
    #[offset(3100)]
    pub m_nSpeed: i32,
    #[offset(3104)]
    pub m_nRPM: i32,
    #[offset(3108)]
    pub m_flThrottle: f32,
    #[offset(3112)]
    pub m_nBoostTimeLeft: i32,
    #[offset(3116)]
    pub m_nHasBoost: i32,
    #[offset(3120)]
    pub m_nScannerDisabledWeapons: i32,
    #[offset(3124)]
    pub m_nScannerDisabledVehicle: i32,
    #[offset(3156)]
    pub m_bEnterAnimOn: bool,
    #[offset(3157)]
    pub m_bExitAnimOn: bool,
    #[offset(3164)]
    pub m_vecGunCrosshair: Vector2,
    #[offset(3256)]
    pub m_vecEyeExitEndpoint: Vector2,
    #[offset(3268)]
    pub m_bHasGun: bool,
    #[offset(3269)]
    pub m_bUnableToFire: bool,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct ParticleSmokeGrenade {
    #[offset(2280)]
    pub m_CurrentStage: i32,
    #[offset(2296)]
    pub m_flSpawnTime: f32,
    #[offset(2300)]
    pub m_FadeStartTime: f32,
    #[offset(2304)]
    pub m_FadeEndTime: f32,
}

#[tf2_struct()]
pub struct ParticleFire {
    #[offset(2296)]
    pub m_vOrigin: Vector2,
    #[offset(2308)]
    pub m_vDirection: Vector2,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct MovieExplosion;

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TEGaussExplosion {
    #[offset(44)]
    pub m_nType: i32,
    #[offset(48)]
    pub m_vecDirection: Vector2,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct QuadraticBeam {
    #[offset(1968)]
    pub m_targetPosition: Vector2,
    #[offset(1980)]
    pub m_controlPosition: Vector2,
    #[offset(1992)]
    pub m_scrollRate: f32,
    #[offset(1996)]
    pub m_flWidth: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct Embers {
    #[offset(1968)]
    pub m_nDensity: i32,
    #[offset(1972)]
    pub m_nLifetime: i32,
    #[offset(1976)]
    pub m_nSpeed: i32,
    #[offset(1980)]
    pub m_bEmit: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EnvWind {
    #[offset(1968)]
    pub m_EnvWindShared: EnvWindShared,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct Precipitation {
    #[offset(2012)]
    pub m_nPrecipType: i32,
}

#[tf2_struct()]
pub struct BaseTempEntity;

#[tf2_struct(baselcass = BaseEntity)]
pub struct VoteController {
    #[offset(1980)]
    pub m_iActiveIssueIndex: i32,
    #[offset(1984)]
    pub m_iOnlyTeamToVote: i32,
    #[offset(1988)]
    pub m_nVoteOptionCount: [i32; 5],
    #[offset(2012)]
    pub m_nPotentialVotes: i32,
    #[offset(2018)]
    pub m_bIsYesNoVote: bool,
    #[offset(2020)]
    pub m_nVoteIdx: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct HandleTest {
    #[offset(1968)]
    pub m_Handle: i32,
    #[offset(1972)]
    pub m_bSendHandle: bool,
}

#[tf2_struct(baselcass = GameRulesProxy)]
pub struct TeamplayRoundBasedRulesProxy;

impl TeamplayRoundBasedRulesProxy {
    pub type teamplayroundbased_gamerules_data = TeamplayRoundBasedRules;
}

#[tf2_struct(baselcass = Sprite)]
pub struct SpriteTrail {
    #[offset(8264)]
    pub m_flLifeTime: f32,
    #[offset(8268)]
    pub m_flStartWidth: f32,
    #[offset(8272)]
    pub m_flEndWidth: f32,
    #[offset(8276)]
    pub m_flStartWidthVariance: f32,
    #[offset(8280)]
    pub m_flTextureRes: f32,
    #[offset(8284)]
    pub m_flMinFadeLength: f32,
    #[offset(8288)]
    pub m_vecSkyboxOrigin: Vector2,
    #[offset(8300)]
    pub m_flSkyboxScale: f32,
}

#[tf2_struct(baselcass = Sprite)]
pub struct SpriteOriented;

#[tf2_struct(baselcass = BaseEntity)]
pub struct Sprite {
    #[offset(1988)]
    pub m_hAttachedToEntity: i32,
    #[offset(1992)]
    pub m_nAttachment: i32,
    #[offset(1996)]
    pub m_flSpriteFramerate: f32,
    #[offset(2000)]
    pub m_flFrame: f32,
    #[offset(2008)]
    pub m_nBrightness: i32,
    #[offset(2012)]
    pub m_flBrightnessTime: f32,
    #[offset(2016)]
    pub m_flSpriteScale: f32,
    #[offset(2020)]
    pub m_flScaleTime: f32,
    #[offset(2024)]
    pub m_bWorldSpaceScale: bool,
    #[offset(2028)]
    pub m_flGlowProxySize: f32,
    #[offset(2032)]
    pub m_flHDRColorScale: f32,
}

#[tf2_struct(baselcass = Ragdoll)]
pub struct Ragdoll_Attached {
    #[offset(3944)]
    pub m_attachmentPointBoneSpace: Vector2,
    #[offset(3968)]
    pub m_attachmentPointRagdollSpace: Vector2,
    #[offset(3980)]
    pub m_ragdollAttachedObjectIndex: i32,
    #[offset(3984)]
    pub m_boneIndexAttached: bool,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct Ragdoll {
    #[offset(3084)]
    pub m_ragPos: [[Vector2; 1]; 24],
    #[offset(3372)]
    pub m_ragAngles: [[Vector2; 1]; 24],
    #[offset(3924)]
    pub m_hUnragdoll: i32,
    #[offset(3928)]
    pub m_flBlendWeight: f32,
    #[offset(3936)]
    pub m_nOverlaySequence: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct PoseController {
    #[offset(1988)]
    pub m_bPoseValueParity: bool,
    #[offset(1992)]
    pub m_fPoseValue: f32,
    #[offset(1996)]
    pub m_fInterpolationTime: f32,
    #[offset(2000)]
    pub m_bInterpolationWrap: bool,
    #[offset(2004)]
    pub m_fCycleFrequency: f32,
    #[offset(2008)]
    pub m_nFModType: i32,
    #[offset(2012)]
    pub m_fFModTimeOffset: f32,
    #[offset(2016)]
    pub m_fFModRate: f32,
    #[offset(2020)]
    pub m_fFModAmplitude: f32,
}

#[tf2_struct()]
pub struct GameRulesProxy;

#[tf2_struct(baselcass = BaseEntity)]
pub struct InfoLadderDismount;

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncLadder {
    #[offset(1968)]
    pub m_vecLadderDir: Vector2,
    #[offset(2016)]
    pub m_vecPlayerMountPositionTop: Vector2,
    #[offset(2028)]
    pub m_vecPlayerMountPositionBottom: Vector2,
    #[offset(2041)]
    pub m_bFakeLadder: bool,
}

#[tf2_struct()]
pub struct DetailController {
    #[offset(1968)]
    pub m_flFadeStartDist: f32,
    #[offset(1972)]
    pub m_flFadeEndDist: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct World {
    #[offset(1968)]
    pub m_flWaveHeight: f32,
    #[offset(1972)]
    pub m_WorldMins: Vector2,
    #[offset(1984)]
    pub m_WorldMaxs: Vector2,
    #[offset(1996)]
    pub m_bStartDark: bool,
    #[offset(2000)]
    pub m_flMaxOccludeeArea: f32,
    #[offset(2004)]
    pub m_flMinOccluderArea: f32,
    #[offset(2008)]
    pub m_flMinPropScreenSpaceWidth: f32,
    #[offset(2012)]
    pub m_flMaxPropScreenSpaceWidth: f32,
    #[offset(2016)]
    pub m_bColdWorld: bool,
    #[offset(2017)]
    pub m_iszDetailSpriteMaterial: [i8; 256],
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct WaterLODControl {
    #[offset(1968)]
    pub m_flCheapWaterStartDistance: f32,
    #[offset(1972)]
    pub m_flCheapWaterEndDistance: f32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct WaterBullet;

#[tf2_struct(baselcass = BaseEntity)]
pub struct VGuiScreen {
    #[offset(1976)]
    pub m_flWidth: f32,
    #[offset(1980)]
    pub m_flHeight: f32,
    #[offset(1984)]
    pub m_nPanelName: i32,
    #[offset(2012)]
    pub m_nAttachmentIndex: i32,
    #[offset(2016)]
    pub m_nOverlayMaterial: i32,
    #[offset(2020)]
    pub m_fScreenFlags: i32,
    #[offset(2128)]
    pub m_hPlayerOwner: i32,
}

#[tf2_struct(baselcass = PropVehicleDriveable)]
pub struct PropJeep {
    #[offset(3472)]
    pub m_bHeadlightIsOn: bool,
}

#[tf2_struct(baselcass = DynamicProp)]
pub struct PropVehicleChoreoGeneric {
    #[offset(3120)]
    pub m_hPlayer: i32,
    #[offset(3128)]
    pub m_bEnterAnimOn: bool,
    #[offset(3129)]
    pub m_bExitAnimOn: bool,
    #[offset(3132)]
    pub m_vecEyeExitEndpoint: Vector2,
    #[offset(3288)]
    pub m_vehicleView_bClampEyeAngles: i32,
    #[offset(3292)]
    pub m_vehicleView_flPitchCurveZero: f32,
    #[offset(3296)]
    pub m_vehicleView_flPitchCurveLinear: f32,
    #[offset(3300)]
    pub m_vehicleView_flRollCurveZero: f32,
    #[offset(3304)]
    pub m_vehicleView_flRollCurveLinear: f32,
    #[offset(3308)]
    pub m_vehicleView_flFOV: f32,
    #[offset(3312)]
    pub m_vehicleView_flYawMin: f32,
    #[offset(3316)]
    pub m_vehicleView_flYawMax: f32,
    #[offset(3320)]
    pub m_vehicleView_flPitchMin: f32,
    #[offset(3324)]
    pub m_vehicleView_flPitchMax: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct ProxyToggle;

impl ProxyToggle {
    pub type blah = ProxyToggle_ProxiedData;
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct Tesla {
    #[offset(2016)]
    pub m_SoundName: [i8; 64],
    #[offset(2080)]
    pub m_iszSpriteName: [i8; 256],
}

#[tf2_struct()]
pub struct Team {
    #[offset(2000)]
    pub m_szTeamname: [i8; 32],
    #[offset(2032)]
    pub m_iScore: i32,
    #[offset(2036)]
    pub m_iRoundsWon: i32,
    #[offset(2052)]
    pub m_iTeamNum: i32,
}

impl Team {
    pub type player_array = [i32; 101];
}

#[tf2_struct()]
pub struct Sun {
    #[offset(128)]
    pub m_clrRender: i32,
    #[offset(2384)]
    pub m_clrOverlay: i32,
    #[offset(2388)]
    pub m_nSize: i32,
    #[offset(2392)]
    pub m_nOverlaySize: i32,
    #[offset(2396)]
    pub m_vDirection: Vector2,
    #[offset(2408)]
    pub m_bOn: bool,
    #[offset(2412)]
    pub m_nMaterial: i32,
    #[offset(2416)]
    pub m_nOverlayMaterial: i32,
}

impl Sun {
    pub type HDRColorScale = f32;
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct ParticlePerformanceMonitor {
    #[offset(1965)]
    pub m_bDisplayPerf: bool,
    #[offset(1966)]
    pub m_bMeasurePerf: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct SpotlightEnd {
    #[offset(1968)]
    pub m_flLightScale: f32,
    #[offset(1972)]
    pub m_Radius: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct SlideshowDisplay {
    #[offset(1965)]
    pub m_bEnabled: bool,
    #[offset(1966)]
    pub m_szDisplayText: [i8; 128],
    #[offset(2094)]
    pub m_szSlideshowDirectory: [i8; 128],
    #[offset(2280)]
    pub m_fMinSlideTime: f32,
    #[offset(2284)]
    pub m_fMaxSlideTime: f32,
    #[offset(2292)]
    pub m_iCycleType: i32,
    #[offset(2296)]
    pub m_bNoListRepeats: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct ShadowControl {
    #[offset(1968)]
    pub m_shadowDirection: Vector2,
    #[offset(1980)]
    pub m_shadowColor: i32,
    #[offset(1984)]
    pub m_flShadowMaxDist: f32,
    #[offset(1988)]
    pub m_bDisableShadows: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct SceneEntity {
    //probably invalid
    #[offset(0)]
    pub m_hActorList: [i32; 16],
    #[offset(1976)]
    pub m_bIsPlayingBack: bool,
    #[offset(1977)]
    pub m_bPaused: bool,
    #[offset(1978)]
    pub m_bMultiplayer: bool,
    #[offset(1984)]
    pub m_flForceClientTime: f32,
    #[offset(1988)]
    pub m_nSceneStringIndex: i32,
}

#[tf2_struct()]
pub struct RopeKeyframe {
    #[offset(534)]
    pub m_iParentAttachment: i32,
    #[offset(556)]
    pub moveparent: i32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(2020)]
    pub m_flScrollSpeed: f32,
    #[offset(2024)]
    pub m_RopeFlags: i32,
    #[offset(2028)]
    pub m_iRopeMaterialModelIndex: i32,
    #[offset(2776)]
    pub m_nSegments: i32,
    #[offset(2780)]
    pub m_hStartPoint: i32,
    #[offset(2784)]
    pub m_hEndPoint: i32,
    #[offset(2792)]
    pub m_Subdiv: i32,
    #[offset(2796)]
    pub m_RopeLength: i32,
    #[offset(2800)]
    pub m_Slack: i32,
    #[offset(2804)]
    pub m_TextureScale: f32,
    #[offset(2808)]
    pub m_fLockedPoints: i32,
    #[offset(2812)]
    pub m_Width: f32,
    #[offset(2960)]
    pub m_bConstrainBetweenEndpoints: bool,
}

#[tf2_struct()]
pub struct RagdollManager {
    #[offset(1968)]
    pub m_iCurrentMaxRagdollCount: i32,
}

#[tf2_struct(baselcass = PhysicsProp)]
pub struct PhysicsPropMultiplayer {
    #[offset(3096)]
    pub m_iPhysicsMode: i32,
    #[offset(3100)]
    pub m_fMass: f32,
    #[offset(3104)]
    pub m_collisionMins: Vector2,
    #[offset(3116)]
    pub m_collisionMaxs: Vector2,
}

#[tf2_struct(baselcass = PhysBox)]
pub struct PhysBoxMultiplayer {
    #[offset(1984)]
    pub m_iPhysicsMode: i32,
    #[offset(1988)]
    pub m_fMass: f32,
}

#[tf2_struct(baselcass = DynamicProp)]
pub struct BasePropDoor;

#[tf2_struct(baselcass = BreakableProp)]
pub struct DynamicProp {
    #[offset(3081)]
    pub m_bUseHitboxesForRenderBox: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct PointWorldText {
    #[offset(1992)]
    pub m_szText: [i8; 260],
    #[offset(2252)]
    pub m_flTextSize: f32,
    #[offset(2256)]
    pub m_flTextSpacingX: f32,
    #[offset(2260)]
    pub m_flTextSpacingY: f32,
    #[offset(2264)]
    pub m_colTextColor: i32,
    #[offset(2268)]
    pub m_nOrientation: i32,
    #[offset(2276)]
    pub m_nFont: i32,
    #[offset(2280)]
    pub m_bRainbow: bool,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct PointCommentaryNode {
    #[offset(3081)]
    pub m_bActive: bool,
    #[offset(3084)]
    pub m_flStartTime: f32,
    #[offset(3088)]
    pub m_iszCommentaryFile: [i8; 260],
    #[offset(3348)]
    pub m_iszCommentaryFileNoHDR: [i8; 260],
    #[offset(3608)]
    pub m_iszSpeakers: [i8; 256],
    #[offset(3864)]
    pub m_iNodeNumber: i32,
    #[offset(3868)]
    pub m_iNodeNumberMax: i32,
    #[offset(3880)]
    pub m_hViewPosition: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct PointCamera {
    #[offset(1968)]
    pub m_FOV: f32,
    #[offset(1972)]
    pub m_Resolution: f32,
    #[offset(1976)]
    pub m_bFogEnable: bool,
    #[offset(1977)]
    pub m_FogColor: i32,
    #[offset(1984)]
    pub m_flFogStart: f32,
    #[offset(1988)]
    pub m_flFogEnd: f32,
    #[offset(1992)]
    pub m_flFogMaxDensity: f32,
    #[offset(1996)]
    pub m_bActive: bool,
    #[offset(1997)]
    pub m_bUseScreenAspectRatio: bool,
}

#[tf2_struct()]
pub struct PlayerResource {
    #[offset(2792)]
    pub m_iPing: [i32; 102],
    #[offset(3200)]
    pub m_iScore: [i32; 102],
    #[offset(3608)]
    pub m_iDeaths: [i32; 102],
    #[offset(4016)]
    pub m_bConnected: [bool; 102],
    #[offset(4120)]
    pub m_iTeam: [i32; 102],
    #[offset(4528)]
    pub m_bAlive: [bool; 102],
    #[offset(4632)]
    pub m_iHealth: [i32; 102],
    #[offset(5168)]
    pub m_iAccountID: [i32; 102],
    #[offset(5576)]
    pub m_bValid: [bool; 102],
    #[offset(5680)]
    pub m_iUserID: [i32; 102],
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct Plasma {
    #[offset(1968)]
    pub m_flStartScale: f32,
    #[offset(1972)]
    pub m_flScale: f32,
    #[offset(1976)]
    pub m_flScaleTime: f32,
    #[offset(1980)]
    pub m_nFlags: i32,
    #[offset(1984)]
    pub m_nPlasmaModelIndex: i32,
    #[offset(1988)]
    pub m_nPlasmaModelIndex2: i32,
    #[offset(1992)]
    pub m_nGlowModelIndex: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct PhysMagnet;

#[tf2_struct(baselcass = BreakableProp)]
pub struct PhysicsProp {
    #[offset(3081)]
    pub m_bAwake: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct PhysBox {
    #[offset(1968)]
    pub m_mass: f32,
}

#[tf2_struct()]
pub struct ParticleSystem {
    #[offset(534)]
    pub m_iParentAttachment: i32,
    #[offset(556)]
    pub moveparent: i32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: Vector2,
    #[offset(1876)]
    pub m_hOwnerEntity: i32,
    #[offset(1968)]
    pub m_iEffectIndex: i32,
    #[offset(1972)]
    pub m_bActive: bool,
    #[offset(1976)]
    pub m_flStartTime: f32,
    #[offset(1980)]
    pub m_hControlPointEnts: [i32; 63],
    #[offset(2295)]
    pub m_bWeatherEffect: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct MaterialModifyControl {
    #[offset(1965)]
    pub m_szMaterialName: [i8; 255],
    #[offset(2220)]
    pub m_szMaterialVar: [i8; 255],
    #[offset(2475)]
    pub m_szMaterialVarValue: [i8; 255],
    #[offset(2748)]
    pub m_iFrameStart: i32,
    #[offset(2752)]
    pub m_iFrameEnd: i32,
    #[offset(2756)]
    pub m_bWrap: bool,
    #[offset(2760)]
    pub m_flFramerate: f32,
    #[offset(2764)]
    pub m_bNewAnimCommandsSemaphore: bool,
    #[offset(2768)]
    pub m_flFloatLerpStartValue: f32,
    #[offset(2772)]
    pub m_flFloatLerpEndValue: f32,
    #[offset(2776)]
    pub m_flFloatLerpTransitionTime: f32,
    #[offset(2780)]
    pub m_bFloatLerpWrap: bool,
    #[offset(2788)]
    pub m_nModifyMode: i32,
}

#[tf2_struct()]
pub struct LightGlow {
    #[offset(128)]
    pub m_clrRender: i32,
    #[offset(556)]
    pub moveparent: i32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: Vector2,
    #[offset(1968)]
    pub m_nHorizontalSize: i32,
    #[offset(1972)]
    pub m_nVerticalSize: i32,
    #[offset(1976)]
    pub m_nMinDist: i32,
    #[offset(1980)]
    pub m_nMaxDist: i32,
    #[offset(1984)]
    pub m_nOuterMaxDist: i32,
    #[offset(1988)]
    pub m_spawnflags: i32,
    #[offset(2240)]
    pub m_flGlowProxySize: f32,
}

impl LightGlow {
    pub type HDRColorScale = f32;
}

#[tf2_struct()]
pub struct InfoOverlayAccessor {
    #[offset(1864)]
    pub m_iTextureFrameIndex: i32,
    #[offset(1968)]
    pub m_iOverlayID: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncTrackTrain;

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct FuncSmokeVolume {
    #[offset(576)]
    pub m_Collision: CollisionProperty,
    #[offset(2280)]
    pub m_Color1: i32,
    #[offset(2284)]
    pub m_Color2: i32,
    #[offset(2288)]
    pub m_MaterialName: [i8; 255],
    #[offset(2544)]
    pub m_ParticleDrawWidth: f32,
    #[offset(2548)]
    pub m_ParticleSpacingDistance: f32,
    #[offset(2552)]
    pub m_DensityRampSpeed: f32,
    #[offset(2556)]
    pub m_RotationSpeed: f32,
    #[offset(2560)]
    pub m_MovementSpeed: f32,
    #[offset(2564)]
    pub m_Density: f32,
    #[offset(2568)]
    pub m_spawnflags: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncRotating {
    #[offset(152)]
    pub m_flSimulationTime: i32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: [f32; 3],
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncReflectiveGlass;

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncOccluder {
    #[offset(1968)]
    pub m_nOccluderIndex: i32,
    #[offset(1972)]
    pub m_bActive: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct Func_LOD {
    #[offset(1968)]
    pub m_fDisappearDist: f32,
}

#[tf2_struct(baselcass = TEParticleSystem)]
pub struct TEDust {
    #[offset(44)]
    pub m_flSize: f32,
    #[offset(48)]
    pub m_flSpeed: f32,
    #[offset(52)]
    pub m_vecDirection: Vector2,
}

#[tf2_struct()]
pub struct Func_Dust {
    #[offset(188)]
    pub m_nModelIndex: i32,
    #[offset(576)]
    pub m_Collision: CollisionProperty,
    #[offset(1965)]
    pub m_Color: i32,
    #[offset(1972)]
    pub m_SpawnRate: i32,
    #[offset(1976)]
    pub m_flSizeMin: f32,
    #[offset(1980)]
    pub m_flSizeMax: f32,
    #[offset(1984)]
    pub m_SpeedMax: i32,
    #[offset(1988)]
    pub m_LifetimeMin: i32,
    #[offset(1992)]
    pub m_LifetimeMax: i32,
    #[offset(1996)]
    pub m_DistMax: i32,
    #[offset(2000)]
    pub m_FallSpeed: f32,
    #[offset(2004)]
    pub m_DustFlags: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncConveyor {
    #[offset(1968)]
    pub m_flConveyorSpeed: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct BreakableSurface {
    #[offset(1976)]
    pub m_nNumWide: i32,
    #[offset(1980)]
    pub m_nNumHigh: i32,
    #[offset(1984)]
    pub m_flPanelWidth: f32,
    #[offset(1988)]
    pub m_flPanelHeight: f32,
    #[offset(1992)]
    pub m_vNormal: Vector2,
    #[offset(2004)]
    pub m_vCorner: Vector2,
    #[offset(2016)]
    pub m_bIsBroken: bool,
    #[offset(2020)]
    pub m_nSurfaceType: i32,
    #[offset(2072)]
    pub m_RawPanelBitVec: [i32; 256],
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncAreaPortalWindow {
    #[offset(1968)]
    pub m_flFadeStartDist: f32,
    #[offset(1972)]
    pub m_flFadeDist: f32,
    #[offset(1976)]
    pub m_flTranslucencyLimit: f32,
    #[offset(1980)]
    pub m_iBackgroundModelIndex: i32,
}

#[tf2_struct()]
pub struct CFish {
    #[offset(188)]
    pub m_nModelIndex: i32,
    #[offset(209)]
    pub m_lifeState: bool,
    #[offset(3160)]
    pub m_x: f32,
    #[offset(3164)]
    pub m_y: f32,
    #[offset(3168)]
    pub m_z: f32,
    #[offset(3176)]
    pub m_angle: f32,
    #[offset(3184)]
    pub m_poolOrigin: Vector2,
    #[offset(3196)]
    pub m_waterLevel: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EntityFlame {
    #[offset(1976)]
    pub m_hEntAttached: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FireSmoke {
    #[offset(1968)]
    pub m_flStartScale: f32,
    #[offset(1972)]
    pub m_flScale: f32,
    #[offset(1976)]
    pub m_flScaleTime: f32,
    #[offset(1980)]
    pub m_nFlags: i32,
    #[offset(1984)]
    pub m_nFlameModelIndex: i32,
    #[offset(1988)]
    pub m_nFlameFromAboveModelIndex: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EnvTonemapController {
    #[offset(1965)]
    pub m_bUseCustomAutoExposureMin: bool,
    #[offset(1966)]
    pub m_bUseCustomAutoExposureMax: bool,
    #[offset(1967)]
    pub m_bUseCustomBloomScale: bool,
    #[offset(1968)]
    pub m_flCustomAutoExposureMin: f32,
    #[offset(1972)]
    pub m_flCustomAutoExposureMax: f32,
    #[offset(1976)]
    pub m_flCustomBloomScale: f32,
    #[offset(1980)]
    pub m_flCustomBloomScaleMinimum: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EnvScreenEffect {
    #[offset(1968)]
    pub m_flDuration: f32,
    #[offset(1972)]
    pub m_nType: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EnvScreenOverlay {
    #[offset(1965)]
    pub m_iszOverlayNames: [[[i8; 255]; 1]; 10],
    #[offset(4516)]
    pub m_flOverlayTimes: [[f32; 1]; 10],
    #[offset(4556)]
    pub m_flStartTime: f32,
    #[offset(4560)]
    pub m_iDesiredOverlay: i32,
    #[offset(4564)]
    pub m_bIsActive: bool,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EnvProjectedTexture {
    #[offset(1968)]
    pub m_hTargetEntity: i32,
    #[offset(1972)]
    pub m_bState: bool,
    #[offset(1976)]
    pub m_flLightFOV: f32,
    #[offset(1980)]
    pub m_bEnableShadows: bool,
    #[offset(1981)]
    pub m_bLightOnlyTarget: bool,
    #[offset(1982)]
    pub m_bLightWorld: bool,
    #[offset(1983)]
    pub m_bCameraSpace: bool,
    #[offset(1984)]
    pub m_LinearFloatLightColor: Vector2,
    #[offset(1996)]
    pub m_flAmbient: f32,
    #[offset(2000)]
    pub m_flNearZ: f32,
    #[offset(2004)]
    pub m_flFarZ: f32,
    #[offset(2008)]
    pub m_SpotlightTextureName: [i8; 260],
    #[offset(2268)]
    pub m_nSpotlightTextureFrame: i32,
    #[offset(2272)]
    pub m_nShadowQuality: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct EnvParticleScript {
    #[offset(3392)]
    pub m_flSequenceScale: f32,
}

#[tf2_struct()]
pub struct FogController {
    #[offset(1976)]
    pub m_fog_dirPrimary: Vector2,
    #[offset(1988)]
    pub m_fog_colorPrimary: i32,
    #[offset(1992)]
    pub m_fog_colorSecondary: i32,
    #[offset(1996)]
    pub m_fog_colorPrimaryLerpTo: i32,
    #[offset(2000)]
    pub m_fog_colorSecondaryLerpTo: i32,
    #[offset(2004)]
    pub m_fog_start: f32,
    #[offset(2008)]
    pub m_fog_end: f32,
    #[offset(2012)]
    pub m_fog_farz: f32,
    #[offset(2016)]
    pub m_fog_maxdensity: f32,
    #[offset(2020)]
    pub m_fog_startLerpTo: f32,
    #[offset(2024)]
    pub m_fog_endLerpTo: f32,
    #[offset(2028)]
    pub m_fog_lerptime: f32,
    #[offset(2032)]
    pub m_fog_duration: f32,
    #[offset(2036)]
    pub m_fog_enable: bool,
    #[offset(2037)]
    pub m_fog_blend: bool,
}

#[tf2_struct(baselcass = BaseParticleEntity)]
pub struct EntityParticleTrail {
    #[offset(2272)]
    pub m_iMaterialName: i32,
    #[offset(2280)]
    pub m_Info: EntityParticleTrailInfo,
    #[offset(2312)]
    pub m_hConstraintEntity: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct EntityDissolve {
    #[offset(1976)]
    pub m_flStartTime: f32,
    #[offset(1980)]
    pub m_flFadeOutStart: f32,
    #[offset(1984)]
    pub m_flFadeOutLength: f32,
    #[offset(1988)]
    pub m_flFadeOutModelStart: f32,
    #[offset(1992)]
    pub m_flFadeOutModelLength: f32,
    #[offset(1996)]
    pub m_flFadeInStart: f32,
    #[offset(2000)]
    pub m_flFadeInLength: f32,
    #[offset(2004)]
    pub m_nDissolveType: i32,
    #[offset(2024)]
    pub m_vDissolverOrigin: Vector2,
    #[offset(2036)]
    pub m_nMagnitude: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct DynamicLight {
    #[offset(1965)]
    pub m_Flags: bool,
    #[offset(1968)]
    pub m_Radius: f32,
    #[offset(1972)]
    pub m_Exponent: i32,
    #[offset(1976)]
    pub m_InnerAngle: f32,
    #[offset(1980)]
    pub m_OuterAngle: f32,
    #[offset(1984)]
    pub m_SpotRadius: f32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct ColorCorrectionVolume {
    #[offset(1968)]
    pub m_Weight: f32,
    #[offset(1972)]
    pub m_lookupFilename: [i8; 260],
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct ColorCorrection {
    #[offset(1968)]
    pub m_vecOrigin: Vector2,
    #[offset(1980)]
    pub m_minFalloff: f32,
    #[offset(1984)]
    pub m_maxFalloff: f32,
    #[offset(1988)]
    pub m_flCurWeight: f32,
    #[offset(1992)]
    pub m_netLookupFilename: [i8; 260],
    #[offset(2252)]
    pub m_bEnabled: bool,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct BreakableProp;

#[tf2_struct(baselcass = BaseCombatCharacter)]
pub struct BasePlayer {
    //probably invalid
    #[offset(0)]
    pub m_hMyWearables: [i32; 8],
    #[offset(209)]
    pub m_lifeState: bool,
    #[offset(212)]
    pub m_iHealth: i32,
    #[offset(1120)]
    pub m_fFlags: i32,
    #[offset(5520)]
    pub m_AttributeList: AttributeList,
    #[offset(5568)]
    pub pl: PlayerState,
    #[offset(5592)]
    pub m_iFOV: i32,
    #[offset(5596)]
    pub m_iFOVStart: i32,
    #[offset(5600)]
    pub m_flFOVTime: f32,
    #[offset(5604)]
    pub m_iDefaultFOV: i32,
    #[offset(5608)]
    pub m_hZoomOwner: i32,
    #[offset(5700)]
    pub m_iObserverMode: i32,
    #[offset(5704)]
    pub m_hObserverTarget: i32,
    #[offset(5748)]
    pub m_hVehicle: i32,
    #[offset(5756)]
    pub m_hUseEntity: i32,
    #[offset(5760)]
    pub m_flMaxspeed: f32,
    #[offset(5764)]
    pub m_iBonusProgress: i32,
    #[offset(5768)]
    pub m_iBonusChallenge: i32,
    #[offset(5976)]
    pub m_hViewModel: [[i32; 1]; 2],
    #[offset(6320)]
    pub m_szLastPlaceName: [i8; 18],
}

impl BasePlayer {
    pub type localdata = LocalPlayerExclusive;
}

#[tf2_struct(baselcass = BaseAnimatingOverlay)]
pub struct BaseFlex {
    #[offset(3224)]
    pub m_viewtarget: Vector2,
    #[offset(3320)]
    pub m_flexWeight: [f32; 96],
    #[offset(3784)]
    pub m_blinktoggle: bool,
}

#[tf2_struct()]
pub struct BaseEntity {
    #[offset(124)]
    pub m_nRenderFX: i32,
    #[offset(128)]
    pub m_clrRender: i32,
    #[offset(152)]
    pub m_flSimulationTime: i32,
    #[offset(164)]
    pub m_ubInterpolationFrame: i32,
    #[offset(168)]
    pub m_fEffects: i32,
    #[offset(172)]
    pub m_nRenderMode: i32,
    #[offset(188)]
    pub m_nModelIndex: i32,
    #[offset(192)]
    pub m_nModelIndexOverrides: [i32; 4],
    #[offset(220)]
    pub m_iTeamNum: i32,
    #[offset(534)]
    pub m_iParentAttachment: i32,
    #[offset(556)]
    pub moveparent: i32,
    #[offset(576)]
    pub m_Collision: CollisionProperty,
    #[offset(784)]
    pub m_flElasticity: f32,
    #[offset(788)]
    pub m_flShadowCastDistance: f32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1108)]
    pub m_angRotation: Vector2,
    #[offset(1124)]
    pub m_CollisionGroup: i32,
    #[offset(1861)]
    pub m_bSimulatedEveryTick: bool,
    #[offset(1862)]
    pub m_bAnimatedEveryTick: bool,
    #[offset(1863)]
    pub m_bAlternateSorting: bool,
    #[offset(1864)]
    pub m_iTextureFrameIndex: i32,
    #[offset(1876)]
    pub m_hOwnerEntity: i32,
    #[offset(1880)]
    pub m_hEffectEntity: i32,
}

impl BaseEntity {
    pub type movecollide = i32;
    pub type predictable_id = PredictableId;
    pub type AnimTimeMustBeFirst = AnimTimeMustBeFirst;
    pub type movetype = i32;
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct BaseDoor {
    #[offset(1968)]
    pub m_flWaveHeight: f32,
}

#[tf2_struct(baselcass = BaseFlex)]
pub struct BaseCombatCharacter {
    #[offset(4368)]
    pub m_hMyWeapons: [i32; 48],
    #[offset(4560)]
    pub m_hActiveWeapon: i32,
    #[offset(4565)]
    pub m_bGlowEnabled: bool,
}

impl BaseCombatCharacter {
    pub type bcc_localdata = BCCLocalPlayerExclusive;
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct BaseAnimatingOverlay;

impl BaseAnimatingOverlay {
    pub type overlay_vars = OverlayVars;
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct BoneFollower {
    #[offset(1968)]
    pub m_modelIndex: i32,
    #[offset(1972)]
    pub m_solidIndex: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct BaseAnimating {
    #[offset(1984)]
    pub m_nSkin: i32,
    #[offset(1988)]
    pub m_nBody: i32,
    #[offset(1992)]
    pub m_nHitboxSet: i32,
    #[offset(2044)]
    pub m_flPlaybackRate: f32,
    #[offset(2064)]
    pub m_vecForce: Vector2,
    #[offset(2076)]
    pub m_nForceBone: i32,
    #[offset(2148)]
    pub m_bClientSideFrameReset: bool,
    #[offset(2216)]
    pub m_fadeMinDist: f32,
    #[offset(2220)]
    pub m_fadeMaxDist: f32,
    #[offset(2224)]
    pub m_flFadeScale: f32,
    #[offset(2324)]
    pub m_flModelScale: f32,
    #[offset(2328)]
    pub m_flPoseParameter: [f32; 24],
    #[offset(2656)]
    pub m_flEncodedController: [f32; 4],
    #[offset(2768)]
    pub m_bClientSideAnimation: bool,
    #[offset(2772)]
    pub m_nNewSequenceParity: i32,
    #[offset(2776)]
    pub m_nResetEventsParity: i32,
    #[offset(2816)]
    pub m_nSequence: i32,
    #[offset(3024)]
    pub m_hLightingOrigin: i32,
    #[offset(3028)]
    pub m_hLightingOriginRelative: i32,
    #[offset(3032)]
    pub m_nMuzzleFlashParity: i32,
}

impl BaseAnimating {
    pub type serveranimdata = ServerAnimationData;
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct InfoLightingRelative {
    #[offset(1968)]
    pub m_hLightingLandmark: i32,
}

#[tf2_struct(baselcass = BaseCombatCharacter)]
pub struct AI_BaseNPC {
    #[offset(209)]
    pub m_lifeState: bool,
    #[offset(4576)]
    pub m_flTimePingEffect: f32,
    #[offset(4580)]
    pub m_iDeathPose: i32,
    #[offset(4584)]
    pub m_iDeathFrame: i32,
    #[offset(4588)]
    pub m_iSpeedModRadius: i32,
    #[offset(4592)]
    pub m_iSpeedModSpeed: i32,
    #[offset(4596)]
    pub m_bPerformAvoidance: bool,
    #[offset(4597)]
    pub m_bIsMoving: bool,
    #[offset(4598)]
    pub m_bFadeCorpse: bool,
    #[offset(4599)]
    pub m_bSpeedModActive: bool,
    #[offset(4600)]
    pub m_bImportanRagdoll: bool,
}

#[tf2_struct()]
pub struct Beam {
    #[offset(124)]
    pub m_nRenderFX: i32,
    #[offset(128)]
    pub m_clrRender: i32,
    #[offset(172)]
    pub m_nRenderMode: i32,
    #[offset(188)]
    pub m_nModelIndex: i32,
    #[offset(556)]
    pub moveparent: i32,
    #[offset(1096)]
    pub m_vecOrigin: Vector2,
    #[offset(1968)]
    pub m_flFrameRate: f32,
    #[offset(1972)]
    pub m_flHDRColorScale: f32,
    #[offset(1984)]
    pub m_nNumBeamEnts: i32,
    #[offset(1992)]
    pub m_nHaloIndex: i32,
    #[offset(1996)]
    pub m_nBeamType: i32,
    #[offset(2000)]
    pub m_nBeamFlags: i32,
    #[offset(2004)]
    pub m_hAttachEntity: [i32; 10],
    #[offset(2044)]
    pub m_nAttachIndex: [i32; 10],
    #[offset(2084)]
    pub m_fWidth: f32,
    #[offset(2088)]
    pub m_fEndWidth: f32,
    #[offset(2092)]
    pub m_fFadeLength: f32,
    #[offset(2096)]
    pub m_fHaloScale: f32,
    #[offset(2100)]
    pub m_fAmplitude: f32,
    #[offset(2104)]
    pub m_fStartFrame: f32,
    #[offset(2108)]
    pub m_fSpeed: f32,
    #[offset(2112)]
    pub m_nMinDXLevel: i32,
    #[offset(2116)]
    pub m_flFrame: f32,
    #[offset(2120)]
    pub m_vecEndPos: Vector2,
}

impl Beam {
    pub type beampredictable_id = BeamPredictableId;
}

#[tf2_struct()]
pub struct BaseViewModel {
    #[offset(168)]
    pub m_fEffects: i32,
    #[offset(188)]
    pub m_nModelIndex: i32,
    #[offset(1984)]
    pub m_nSkin: i32,
    #[offset(1988)]
    pub m_nBody: i32,
    #[offset(2044)]
    pub m_flPlaybackRate: f32,
    #[offset(2328)]
    pub m_flPoseParameter: [[f32; 1]; 24],
    #[offset(2772)]
    pub m_nNewSequenceParity: i32,
    #[offset(2776)]
    pub m_nResetEventsParity: i32,
    #[offset(2816)]
    pub m_nSequence: i32,
    #[offset(3032)]
    pub m_nMuzzleFlashParity: i32,
    #[offset(3108)]
    pub m_nViewModelIndex: i32,
    #[offset(3112)]
    pub m_hOwner: i32,
    #[offset(3124)]
    pub m_nAnimationParity: i32,
    #[offset(3148)]
    pub m_hWeapon: i32,
}

#[tf2_struct(baselcass = BaseAnimating)]
pub struct BaseProjectile {
    #[offset(3084)]
    pub m_hOriginalLauncher: i32,
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct BaseParticleEntity;

#[tf2_struct(baselcass = BaseProjectile)]
pub struct BaseGrenade {
    #[offset(360)]
    pub m_vecVelocity: Vector2,
    #[offset(1120)]
    pub m_fFlags: i32,
    #[offset(3089)]
    pub m_bIsLive: bool,
    #[offset(3092)]
    pub m_DmgRadius: f32,
    #[offset(3108)]
    pub m_flDamage: f32,
    #[offset(3120)]
    pub m_hThrower: i32,
}

#[tf2_struct(baselcass = EconEntity)]
pub struct BaseCombatWeapon {
    #[offset(3712)]
    pub m_hOwner: i32,
    #[offset(3748)]
    pub m_iViewModelIndex: i32,
    #[offset(3752)]
    pub m_iWorldModelIndex: i32,
    #[offset(3784)]
    pub m_iState: i32,
}

impl BaseCombatWeapon {
    pub type LocalActiveWeaponData = LocalActiveWeaponData;
    pub type LocalWeaponData = LocalWeaponData;
}

#[tf2_struct(baselcass = BaseEntity)]
pub struct FuncMonitor;

#[tf2_struct()]
pub struct CMannVsMachineWaveStats {
    #[offset(8)]
    pub nCreditsDropped: i32,
    #[offset(12)]
    pub nCreditsAcquired: i32,
    #[offset(16)]
    pub nCreditsBonus: i32,
    #[offset(20)]
    pub nPlayerDeaths: i32,
    #[offset(24)]
    pub nBuyBacks: i32,
}

#[tf2_struct()]
pub struct LocalTFWeaponData {
    #[offset(3948)]
    pub m_flEffectBarRegenTime: f32,
    #[offset(3960)]
    pub m_flReloadPriorNextFire: f32,
    #[offset(4008)]
    pub m_flLastCritCheckTime: f32,
    #[offset(4196)]
    pub m_flLastFireTime: f32,
    #[offset(4208)]
    pub m_flObservedCritChance: f32,
}

#[tf2_struct()]
pub struct TFWeaponDataNonLocal;

#[tf2_struct()]
pub struct SniperRifleLocalData {
    #[offset(4252)]
    pub m_flChargedDamage: f32,
}

#[tf2_struct()]
pub struct PipebombLauncherLocalData {
    #[offset(4256)]
    pub m_iPipebombCount: i32,
    #[offset(4312)]
    pub m_flChargeBeginTime: f32,
}

#[tf2_struct()]
pub struct TFWeaponMedigunDataNonLocal {
    #[offset(4284)]
    pub m_flChargeLevel: f32,
}

#[tf2_struct()]
pub struct LocalTFWeaponMedigunData {
    #[offset(4284)]
    pub m_flChargeLevel: f32,
}

#[tf2_struct()]
pub struct LocalFlameThrower {
    #[offset(4256)]
    pub m_iActiveFlames: i32,
    #[offset(4260)]
    pub m_iDamagingFlames: i32,
    #[offset(4272)]
    pub m_hFlameManager: i32,
    #[offset(4276)]
    pub m_bHasHalloweenSpell: bool,
}

#[tf2_struct()]
pub struct ScriptCreatedItem {
    #[offset(68)]
    pub m_iItemDefinitionIndex: i32,
    #[offset(72)]
    pub m_iEntityQuality: i32,
    #[offset(76)]
    pub m_iEntityLevel: i32,
    #[offset(88)]
    pub m_iItemIDHigh: i32,
    #[offset(92)]
    pub m_iItemIDLow: i32,
    #[offset(96)]
    pub m_iAccountID: i32,
    #[offset(200)]
    pub m_iTeamNumber: i32,
    #[offset(204)]
    pub m_bInitialized: bool,
    #[offset(232)]
    pub m_AttributeList: AttributeList,
    #[offset(280)]
    pub m_NetworkedDynamicAttributesForDemos: AttributeList,
    #[offset(328)]
    pub m_bOnlyIterateItemViewAttributes: bool,
}

#[tf2_struct()]
pub struct AttributeList;

impl AttributeList {
    pub type m_Attributes = _ST_m_Attributes_20;
}

#[tf2_struct()]
pub struct _ST_m_Attributes_20;

impl _ST_m_Attributes_20 {
    pub type i019 = ScriptCreatedAttribute;
    pub type i001 = ScriptCreatedAttribute;
    pub type i018 = ScriptCreatedAttribute;
    pub type i014 = ScriptCreatedAttribute;
    pub type i003 = ScriptCreatedAttribute;
    pub type i006 = ScriptCreatedAttribute;
    pub type i011 = ScriptCreatedAttribute;
    pub type i016 = ScriptCreatedAttribute;
    pub type i017 = ScriptCreatedAttribute;
    pub type i010 = ScriptCreatedAttribute;
    pub type i002 = ScriptCreatedAttribute;
    pub type i009 = ScriptCreatedAttribute;
    pub type i004 = ScriptCreatedAttribute;
    pub type i013 = ScriptCreatedAttribute;
    pub type i007 = ScriptCreatedAttribute;
    pub type i015 = ScriptCreatedAttribute;
    pub type i008 = ScriptCreatedAttribute;
    pub type i000 = ScriptCreatedAttribute;
    pub type i005 = ScriptCreatedAttribute;
    pub type i012 = ScriptCreatedAttribute;
    pub type lengthproxy = _LPT_m_Attributes_20;
}

#[tf2_struct()]
pub struct ScriptCreatedAttribute {
    #[offset(8)]
    pub m_iAttributeDefinitionIndex: i32,
    #[offset(12)]
    pub m_flValue: f32,
    #[offset(16)]
    pub m_nRefundableCurrency: i32,
}

#[tf2_struct()]
pub struct _LPT_m_Attributes_20 {
    //probably invalid
    #[offset(0)]
    pub lengthprop20: i32,
}

#[tf2_struct()]
pub struct BuilderLocalData {
    #[offset(4252)]
    pub m_iObjectType: i32,
    #[offset(4288)]
    pub m_hObjectBeingBuilt: i32,
    #[offset(4300)]
    pub m_aBuildableObjectTypes: [i32; 4],
}

#[tf2_struct()]
pub struct TFGameRules {
    #[offset(612)]
    pub m_nGameType: i32,
    #[offset(616)]
    pub m_nStopWatchState: i32,
    #[offset(620)]
    pub m_pszTeamGoalStringRed: [i8; 256],
    #[offset(876)]
    pub m_pszTeamGoalStringBlue: [i8; 256],
    #[offset(1132)]
    pub m_flCapturePointEnableTime: f32,
    #[offset(1140)]
    pub m_nHudType: i32,
    #[offset(1144)]
    pub m_bIsInTraining: bool,
    #[offset(1145)]
    pub m_bAllowTrainingAchievements: bool,
    #[offset(1146)]
    pub m_bIsWaitingForTrainingContinue: bool,
    #[offset(1147)]
    pub m_bIsTrainingHUDVisible: bool,
    #[offset(1148)]
    pub m_bIsInItemTestingMode: bool,
    #[offset(1164)]
    pub m_hBonusLogic: i32,
    #[offset(1168)]
    pub m_bPlayingKoth: bool,
    #[offset(1169)]
    pub m_bPowerupMode: bool,
    #[offset(1170)]
    pub m_bPlayingRobotDestructionMode: bool,
    #[offset(1171)]
    pub m_bPlayingMedieval: bool,
    #[offset(1172)]
    pub m_bPlayingHybrid_CTF_CP: bool,
    #[offset(1173)]
    pub m_bPlayingSpecialDeliveryMode: bool,
    #[offset(1174)]
    pub m_bPlayingMannVsMachine: bool,
    #[offset(1175)]
    pub m_bMannVsMachineAlarmStatus: bool,
    #[offset(1176)]
    pub m_bHaveMinPlayersToEnableReady: bool,
    #[offset(1177)]
    pub m_bBountyModeEnabled: bool,
    #[offset(1178)]
    pub m_bCompetitiveMode: bool,
    #[offset(1184)]
    pub m_nMatchGroupType: i32,
    #[offset(1188)]
    pub m_bMatchEnded: bool,
    #[offset(1189)]
    pub m_bHelltowerPlayersInHell: bool,
    #[offset(1190)]
    pub m_bIsUsingSpells: bool,
    #[offset(1191)]
    pub m_bTruceActive: bool,
    #[offset(1192)]
    pub m_bTeamsSwitched: bool,
    #[offset(1193)]
    pub m_bRopesHolidayLightsAllowed: bool,
    #[offset(1196)]
    pub m_hRedKothTimer: i32,
    #[offset(1200)]
    pub m_hBlueKothTimer: i32,
    #[offset(1204)]
    pub m_nMapHolidayType: i32,
    #[offset(1468)]
    pub m_bShowMatchSummary: bool,
    #[offset(1469)]
    pub m_bMapHasMatchSummaryStage: bool,
    #[offset(1470)]
    pub m_bPlayersAreOnMatchSummaryStage: bool,
    #[offset(1471)]
    pub m_bStopWatchWinner: bool,
    #[offset(1472)]
    pub m_ePlayerWantsRematch: [i32; 102],
    #[offset(1880)]
    pub m_eRematchState: i32,
    #[offset(1884)]
    pub m_nNextMapVoteOptions: [i32; 3],
    #[offset(2260)]
    pub m_nBossHealth: i32,
    #[offset(2264)]
    pub m_nMaxBossHealth: i32,
    #[offset(2268)]
    pub m_fBossNormalizedTravelDistance: i32,
    #[offset(2272)]
    pub m_itHandle: i32,
    #[offset(2276)]
    pub m_hBirthdayPlayer: i32,
    #[offset(2280)]
    pub m_nHalloweenEffect: i32,
    #[offset(2284)]
    pub m_fHalloweenEffectStartTime: f32,
    #[offset(2288)]
    pub m_fHalloweenEffectDuration: f32,
    #[offset(2292)]
    pub m_halloweenScenario: i32,
    #[offset(2296)]
    pub m_nForceUpgrades: i32,
    #[offset(2300)]
    pub m_nForceEscortPushLogic: i32,
}

#[tf2_struct()]
pub struct AttributeManager {
    #[offset(80)]
    pub m_iReapplyProvisionParity: i32,
    #[offset(84)]
    pub m_hOuter: i32,
    #[offset(92)]
    pub m_ProviderType: i32,
}

#[tf2_struct()]
pub struct TFPlayerClassShared {
    #[offset(8)]
    pub m_iClass: i32,
    #[offset(12)]
    pub m_iszClassIcon: [i8; 260],
    #[offset(272)]
    pub m_iszCustomModel: [i8; 260],
    #[offset(532)]
    pub m_vecCustomModelOffset: Vector2,
    #[offset(544)]
    pub m_angCustomModelRotation: Vector2,
    #[offset(556)]
    pub m_bCustomModelRotates: bool,
    #[offset(557)]
    pub m_bCustomModelRotationSet: bool,
    #[offset(558)]
    pub m_bCustomModelVisibleToSelf: bool,
    #[offset(559)]
    pub m_bUseClassAnimations: bool,
    #[offset(560)]
    pub m_iClassModelParity: i32,
}

#[tf2_struct()]
pub struct TFPlayerShared {
    #[offset(232)]
    pub m_nPlayerState: i32,
    #[offset(236)]
    pub m_nPlayerCond: i32,
    #[offset(240)]
    pub m_nPlayerCondEx: i32,
    #[offset(244)]
    pub m_nPlayerCondEx2: i32,
    #[offset(248)]
    pub m_nPlayerCondEx3: i32,
    #[offset(252)]
    pub m_nPlayerCondEx4: i32,
    #[offset(256)]
    pub m_ConditionList: TFPlayerConditionListExclusive,
    #[offset(304)]
    pub m_nDisguiseTeam: i32,
    #[offset(308)]
    pub m_nDisguiseClass: i32,
    #[offset(312)]
    pub m_nDisguiseSkinOverride: i32,
    #[offset(316)]
    pub m_nMaskClass: i32,
    #[offset(320)]
    pub m_hDisguiseTarget: i32,
    #[offset(324)]
    pub m_iDisguiseHealth: i32,
    #[offset(336)]
    pub m_hDisguiseWeapon: i32,
    #[offset(340)]
    pub m_nTeamTeleporterUsed: i32,
    #[offset(376)]
    pub m_flInvisChangeCompleteTime: f32,
    #[offset(432)]
    pub m_nNumHealers: i32,
    #[offset(436)]
    pub m_nHalloweenBombHeadStage: i32,
    #[offset(452)]
    pub m_bKingRuneBuffActive: bool,
    #[offset(456)]
    pub m_iTauntIndex: i32,
    #[offset(460)]
    pub m_iTauntConcept: i32,
    #[offset(464)]
    pub m_unTauntSourceItemID_Low: i32,
    #[offset(468)]
    pub m_unTauntSourceItemID_High: i32,
    #[offset(540)]
    pub m_bFeignDeathReady: bool,
    #[offset(544)]
    pub m_iDesiredPlayerClass: i32,
    #[offset(552)]
    pub m_flCloakMeter: f32,
    #[offset(560)]
    pub m_flEnergyDrinkMeter: f32,
    #[offset(564)]
    pub m_flHypeMeter: f32,
    #[offset(568)]
    pub m_flChargeMeter: f32,
    #[offset(608)]
    pub m_bJumping: bool,
    #[offset(612)]
    pub m_iAirDash: i32,
    #[offset(616)]
    pub m_nAirDucked: i32,
    #[offset(620)]
    pub m_flDuckTimer: f32,
    #[offset(632)]
    pub m_flRuneCharge: f32,
    #[offset(724)]
    pub m_iCritMult: i32,
    #[offset(932)]
    pub m_flMovementStunTime: f32,
    #[offset(936)]
    pub m_iMovementStunAmount: i32,
    #[offset(940)]
    pub m_iMovementStunParity: i32,
    #[offset(944)]
    pub m_hStunner: i32,
    #[offset(948)]
    pub m_iStunFlags: i32,
    #[offset(952)]
    pub m_iStunIndex: i32,
    #[offset(956)]
    pub m_hCarriedObject: i32,
    #[offset(960)]
    pub m_bCarryingObject: bool,
    #[offset(968)]
    pub m_nArenaNumChanges: i32,
    #[offset(972)]
    pub m_iWeaponKnockbackID: i32,
    #[offset(976)]
    pub m_bLoadoutUnavailable: bool,
    #[offset(980)]
    pub m_iItemFindBonus: i32,
    #[offset(984)]
    pub m_bShieldEquipped: bool,
    #[offset(985)]
    pub m_bParachuteEquipped: bool,
    #[offset(988)]
    pub m_iDecapitations: i32,
    #[offset(996)]
    pub m_nStreaks: [i32; 4],
    #[offset(1024)]
    pub m_iRevengeCrits: i32,
    #[offset(1028)]
    pub m_iNextMeleeCrit: i32,
    #[offset(1040)]
    pub m_iDisguiseBody: i32,
    #[offset(1044)]
    pub m_iSpawnRoomTouchCount: i32,
    #[offset(1048)]
    pub m_flNextNoiseMakerTime: f32,
    #[offset(1084)]
    pub m_iKillCountSinceLastDeploy: i32,
    #[offset(1088)]
    pub m_flFirstPrimaryAttack: f32,
    #[offset(1092)]
    pub m_flSpyTranqBuffDuration: f32,
    #[offset(1188)]
    pub m_bArenaFirstBloodBoost: bool,
    #[offset(1208)]
    pub m_bHasPasstimeBall: bool,
    #[offset(1209)]
    pub m_bIsTargetedForPasstimePass: bool,
    #[offset(1212)]
    pub m_hPasstimePassTarget: i32,
    #[offset(1216)]
    pub m_askForBallTime: f32,
    #[offset(1220)]
    pub m_flHolsterAnimTime: f32,
    #[offset(1224)]
    pub m_hSwitchTo: i32,
}

impl TFPlayerShared {
    pub type tfsharedlocaldata = TFPlayerSharedLocal;
    pub type m_ConditionData = _ST_m_ConditionData_131;
}

#[tf2_struct()]
pub struct TFPlayerConditionListExclusive {
    #[offset(40)]
    pub _condition_bits: i32,
}

#[tf2_struct()]
pub struct TFPlayerSharedLocal {
    #[offset(56)]
    pub m_ScoreData: TFPlayerScoringDataExclusive,
    #[offset(144)]
    pub m_RoundScoreData: TFPlayerScoringDataExclusive,
    #[offset(328)]
    pub m_nDesiredDisguiseClass: i32,
    #[offset(332)]
    pub m_nDesiredDisguiseTeam: i32,
    #[offset(454)]
    pub m_bLastDisguisedAsOwnTeam: bool,
    #[offset(556)]
    pub m_bInUpgradeZone: bool,
    #[offset(572)]
    pub m_flRageMeter: f32,
    #[offset(576)]
    pub m_bRageDraining: bool,
    #[offset(580)]
    pub m_flNextRageEarnTime: f32,
    #[offset(624)]
    pub m_flStealthNoAttackExpire: f32,
    #[offset(628)]
    pub m_flStealthNextChangeTime: f32,
    #[offset(636)]
    pub m_flItemChargeMeter: [f32; 11],
    #[offset(728)]
    pub m_bPlayerDominated: [bool; 102],
    #[offset(830)]
    pub m_bPlayerDominatingMe: [bool; 102],
}

#[tf2_struct()]
pub struct TFPlayerScoringDataExclusive {
    #[offset(8)]
    pub m_iCaptures: i32,
    #[offset(12)]
    pub m_iDefenses: i32,
    #[offset(16)]
    pub m_iKills: i32,
    #[offset(20)]
    pub m_iDeaths: i32,
    #[offset(24)]
    pub m_iSuicides: i32,
    #[offset(28)]
    pub m_iDominations: i32,
    #[offset(32)]
    pub m_iRevenge: i32,
    #[offset(36)]
    pub m_iBuildingsBuilt: i32,
    #[offset(40)]
    pub m_iBuildingsDestroyed: i32,
    #[offset(44)]
    pub m_iHeadshots: i32,
    #[offset(48)]
    pub m_iBackstabs: i32,
    #[offset(52)]
    pub m_iHealPoints: i32,
    #[offset(56)]
    pub m_iInvulns: i32,
    #[offset(60)]
    pub m_iTeleports: i32,
    #[offset(64)]
    pub m_iDamageDone: i32,
    #[offset(68)]
    pub m_iCrits: i32,
    #[offset(72)]
    pub m_iResupplyPoints: i32,
    #[offset(76)]
    pub m_iKillAssists: i32,
    #[offset(80)]
    pub m_iBonusPoints: i32,
    #[offset(84)]
    pub m_iPoints: i32,
}

#[tf2_struct()]
pub struct _ST_m_ConditionData_131;

impl _ST_m_ConditionData_131 {
    pub type i039 = TFPlayerConditionSource;
    pub type i082 = TFPlayerConditionSource;
    pub type i071 = TFPlayerConditionSource;
    pub type i065 = TFPlayerConditionSource;
    pub type i106 = TFPlayerConditionSource;
    pub type i121 = TFPlayerConditionSource;
    pub type i012 = TFPlayerConditionSource;
    pub type i105 = TFPlayerConditionSource;
    pub type i113 = TFPlayerConditionSource;
    pub type i102 = TFPlayerConditionSource;
    pub type i072 = TFPlayerConditionSource;
    pub type i114 = TFPlayerConditionSource;
    pub type i024 = TFPlayerConditionSource;
    pub type i043 = TFPlayerConditionSource;
    pub type i019 = TFPlayerConditionSource;
    pub type i033 = TFPlayerConditionSource;
    pub type i078 = TFPlayerConditionSource;
    pub type i112 = TFPlayerConditionSource;
    pub type i007 = TFPlayerConditionSource;
    pub type i025 = TFPlayerConditionSource;
    pub type i077 = TFPlayerConditionSource;
    pub type i035 = TFPlayerConditionSource;
    pub type i093 = TFPlayerConditionSource;
    pub type i130 = TFPlayerConditionSource;
    pub type i066 = TFPlayerConditionSource;
    pub type i027 = TFPlayerConditionSource;
    pub type i122 = TFPlayerConditionSource;
    pub type i118 = TFPlayerConditionSource;
    pub type i126 = TFPlayerConditionSource;
    pub type i044 = TFPlayerConditionSource;
    pub type i048 = TFPlayerConditionSource;
    pub type i041 = TFPlayerConditionSource;
    pub type i073 = TFPlayerConditionSource;
    pub type i032 = TFPlayerConditionSource;
    pub type i083 = TFPlayerConditionSource;
    pub type i104 = TFPlayerConditionSource;
    pub type i115 = TFPlayerConditionSource;
    pub type i006 = TFPlayerConditionSource;
    pub type i068 = TFPlayerConditionSource;
    pub type i080 = TFPlayerConditionSource;
    pub type i014 = TFPlayerConditionSource;
    pub type i119 = TFPlayerConditionSource;
    pub type i037 = TFPlayerConditionSource;
    pub type i051 = TFPlayerConditionSource;
    pub type i088 = TFPlayerConditionSource;
    pub type i000 = TFPlayerConditionSource;
    pub type i129 = TFPlayerConditionSource;
    pub type i120 = TFPlayerConditionSource;
    pub type i031 = TFPlayerConditionSource;
    pub type i098 = TFPlayerConditionSource;
    pub type i023 = TFPlayerConditionSource;
    pub type i053 = TFPlayerConditionSource;
    pub type i094 = TFPlayerConditionSource;
    pub type i038 = TFPlayerConditionSource;
    pub type i100 = TFPlayerConditionSource;
    pub type i111 = TFPlayerConditionSource;
    pub type i058 = TFPlayerConditionSource;
    pub type i067 = TFPlayerConditionSource;
    pub type i050 = TFPlayerConditionSource;
    pub type i046 = TFPlayerConditionSource;
    pub type i076 = TFPlayerConditionSource;
    pub type i090 = TFPlayerConditionSource;
    pub type i055 = TFPlayerConditionSource;
    pub type i107 = TFPlayerConditionSource;
    pub type i123 = TFPlayerConditionSource;
    pub type i029 = TFPlayerConditionSource;
    pub type i116 = TFPlayerConditionSource;
    pub type i034 = TFPlayerConditionSource;
    pub type i001 = TFPlayerConditionSource;
    pub type i081 = TFPlayerConditionSource;
    pub type i017 = TFPlayerConditionSource;
    pub type i047 = TFPlayerConditionSource;
    pub type i009 = TFPlayerConditionSource;
    pub type i052 = TFPlayerConditionSource;
    pub type i008 = TFPlayerConditionSource;
    pub type i117 = TFPlayerConditionSource;
    pub type i064 = TFPlayerConditionSource;
    pub type i016 = TFPlayerConditionSource;
    pub type i054 = TFPlayerConditionSource;
    pub type i097 = TFPlayerConditionSource;
    pub type i109 = TFPlayerConditionSource;
    pub type i069 = TFPlayerConditionSource;
    pub type i074 = TFPlayerConditionSource;
    pub type i092 = TFPlayerConditionSource;
    pub type i030 = TFPlayerConditionSource;
    pub type i103 = TFPlayerConditionSource;
    pub type i086 = TFPlayerConditionSource;
    pub type i004 = TFPlayerConditionSource;
    pub type i110 = TFPlayerConditionSource;
    pub type i011 = TFPlayerConditionSource;
    pub type i013 = TFPlayerConditionSource;
    pub type i061 = TFPlayerConditionSource;
    pub type i003 = TFPlayerConditionSource;
    pub type i062 = TFPlayerConditionSource;
    pub type i085 = TFPlayerConditionSource;
    pub type i005 = TFPlayerConditionSource;
    pub type i108 = TFPlayerConditionSource;
    pub type i096 = TFPlayerConditionSource;
    pub type i018 = TFPlayerConditionSource;
    pub type i040 = TFPlayerConditionSource;
    pub type i087 = TFPlayerConditionSource;
    pub type i028 = TFPlayerConditionSource;
    pub type i091 = TFPlayerConditionSource;
    pub type i128 = TFPlayerConditionSource;
    pub type i059 = TFPlayerConditionSource;
    pub type i084 = TFPlayerConditionSource;
    pub type i010 = TFPlayerConditionSource;
    pub type i049 = TFPlayerConditionSource;
    pub type i070 = TFPlayerConditionSource;
    pub type i089 = TFPlayerConditionSource;
    pub type i079 = TFPlayerConditionSource;
    pub type i101 = TFPlayerConditionSource;
    pub type i075 = TFPlayerConditionSource;
    pub type lengthproxy = _LPT_m_ConditionData_131;
    pub type i042 = TFPlayerConditionSource;
    pub type i036 = TFPlayerConditionSource;
    pub type i002 = TFPlayerConditionSource;
    pub type i099 = TFPlayerConditionSource;
    pub type i124 = TFPlayerConditionSource;
    pub type i020 = TFPlayerConditionSource;
    pub type i022 = TFPlayerConditionSource;
    pub type i125 = TFPlayerConditionSource;
    pub type i127 = TFPlayerConditionSource;
    pub type i026 = TFPlayerConditionSource;
    pub type i056 = TFPlayerConditionSource;
    pub type i060 = TFPlayerConditionSource;
    pub type i095 = TFPlayerConditionSource;
    pub type i045 = TFPlayerConditionSource;
    pub type i057 = TFPlayerConditionSource;
    pub type i015 = TFPlayerConditionSource;
    pub type i063 = TFPlayerConditionSource;
    pub type i021 = TFPlayerConditionSource;
}

#[tf2_struct()]
pub struct TFPlayerConditionSource {
    #[offset(16)]
    pub m_pProvider: i32,
}

#[tf2_struct()]
pub struct _LPT_m_ConditionData_131 {
    //probably invalid
    #[offset(0)]
    pub lengthprop131: i32,
}

#[tf2_struct()]
pub struct TFNonLocalPlayerExclusive {
    #[offset(1096)]
    pub m_vecOrigin: Vector3,
    #[offset(9196)]
    pub m_angEyeAngles: [f32; 2],
}

#[tf2_struct()]
pub struct TFLocalPlayerExclusive {
    //probably invalid
    #[offset(0)]
    pub player_object_array: [i32; 6],
    #[offset(1096)]
    pub m_vecOrigin: Vector3,
    #[offset(9128)]
    pub m_hCoach: i32,
    #[offset(9132)]
    pub m_hStudent: i32,
    #[offset(9194)]
    pub m_bIsCoaching: bool,
    #[offset(9196)]
    pub m_angEyeAngles: [f32; 2],
    #[offset(14756)]
    pub m_nCurrency: i32,
    #[offset(14764)]
    pub m_nExperienceLevel: i32,
    #[offset(14768)]
    pub m_nExperienceLevelProgress: i32,
    #[offset(14776)]
    pub m_bMatchSafeToLeave: bool,
}

#[tf2_struct()]
pub struct TFSendHealersDataTable {
    #[offset(14778)]
    pub m_nActiveWpnClip: i32,
}

#[tf2_struct()]
pub struct SentrygunLocalData {
    #[offset(5124)]
    pub m_iKills: i32,
    #[offset(5128)]
    pub m_iAssists: i32,
}

#[tf2_struct()]
pub struct AttributeContainer {
    #[offset(80)]
    pub m_iReapplyProvisionParity: i32,
    #[offset(84)]
    pub m_hOuter: i32,
    #[offset(92)]
    pub m_ProviderType: i32,
    #[offset(144)]
    pub m_Item: ScriptCreatedItem,
}

#[tf2_struct()]
pub struct EffectData {
    //probably invalid
    #[offset(0)]
    pub m_vOrigin: [f32; 3],
    #[offset(12)]
    pub m_vStart: [f32; 3],
    #[offset(24)]
    pub m_vNormal: Vector2,
    #[offset(36)]
    pub m_vAngles: Vector2,
    #[offset(48)]
    pub m_fFlags: i32,
    #[offset(56)]
    pub m_flScale: f32,
    #[offset(60)]
    pub m_flMagnitude: f32,
    #[offset(64)]
    pub m_flRadius: f32,
    #[offset(68)]
    pub m_nAttachmentIndex: i32,
    #[offset(72)]
    pub m_nSurfaceProp: i32,
    #[offset(76)]
    pub m_nMaterial: i32,
    #[offset(80)]
    pub m_nDamageType: i32,
    #[offset(84)]
    pub m_nHitBox: i32,
    #[offset(88)]
    pub m_nColor: bool,
    #[offset(89)]
    pub m_bCustomColors: bool,
    #[offset(92)]
    pub m_CustomColors_m_vecColor1: Vector2,
    #[offset(104)]
    pub m_CustomColors_m_vecColor2: Vector2,
    #[offset(116)]
    pub m_bControlPoint1: bool,
    #[offset(120)]
    pub m_ControlPoint1_m_eParticleAttachment: i32,
    #[offset(124)]
    pub m_ControlPoint1_m_vecOffset: [f32; 3],
    #[offset(136)]
    pub m_iEffectName: i32,
}

#[tf2_struct()]
pub struct EnvWindShared {
    #[offset(8)]
    pub m_flStartTime: f32,
    #[offset(12)]
    pub m_iWindSeed: i32,
    #[offset(16)]
    pub m_iMinWind: i32,
    #[offset(20)]
    pub m_iMaxWind: i32,
    #[offset(28)]
    pub m_iMinGust: i32,
    #[offset(32)]
    pub m_iMaxGust: i32,
    #[offset(36)]
    pub m_flMinGustDelay: f32,
    #[offset(40)]
    pub m_flMaxGustDelay: f32,
    #[offset(44)]
    pub m_flGustDuration: f32,
    #[offset(48)]
    pub m_iGustDirChange: i32,
    #[offset(112)]
    pub m_iInitialWindDir: i32,
    #[offset(116)]
    pub m_flInitialWindSpeed: f32,
}

#[tf2_struct()]
pub struct TeamplayRoundBasedRules {
    #[offset(108)]
    pub m_iRoundState: i32,
    #[offset(112)]
    pub m_bInOvertime: bool,
    #[offset(113)]
    pub m_bInSetup: bool,
    #[offset(114)]
    pub m_bSwitchedTeamsThisRound: bool,
    #[offset(116)]
    pub m_iWinningTeam: i32,
    #[offset(124)]
    pub m_bInWaitingForPlayers: bool,
    #[offset(125)]
    pub m_bAwaitingReadyRestart: bool,
    #[offset(128)]
    pub m_flRestartRoundTime: f32,
    #[offset(132)]
    pub m_flMapResetTime: f32,
    #[offset(136)]
    pub m_flNextRespawnWave: [f32; 32],
    #[offset(264)]
    pub m_bTeamReady: [bool; 32],
    #[offset(296)]
    pub m_bStopWatch: bool,
    #[offset(297)]
    pub m_bMultipleTrains: bool,
    #[offset(298)]
    pub m_bPlayerReady: [bool; 102],
    #[offset(400)]
    pub m_bCheatsEnabledDuringLevel: bool,
    #[offset(404)]
    pub m_nRoundsPlayed: i32,
    #[offset(408)]
    pub m_flCountdownTime: f32,
    #[offset(412)]
    pub m_flStateTransitionTime: f32,
    #[offset(416)]
    pub m_TeamRespawnWaveTimes: [f32; 32],
}

#[tf2_struct()]
pub struct ProxyToggle_ProxiedData {
    #[offset(1968)]
    pub m_WithProxy: i32,
}

#[tf2_struct()]
pub struct CollisionProperty {
    #[offset(16)]
    pub m_vecMinsPreScaled: Vector2,
    #[offset(28)]
    pub m_vecMaxsPreScaled: Vector2,
    #[offset(40)]
    pub m_vecMins: Vector2,
    #[offset(52)]
    pub m_vecMaxs: Vector2,
    #[offset(68)]
    pub m_usSolidFlags: i32,
    #[offset(72)]
    pub m_nSurroundType: bool,
    #[offset(73)]
    pub m_nSolidType: bool,
    #[offset(74)]
    pub m_triggerBloat: bool,
    #[offset(75)]
    pub m_bUniformTriggerBloat: bool,
    #[offset(76)]
    pub m_vecSpecifiedSurroundingMinsPreScaled: Vector2,
    #[offset(88)]
    pub m_vecSpecifiedSurroundingMaxsPreScaled: Vector2,
    #[offset(100)]
    pub m_vecSpecifiedSurroundingMins: Vector2,
    #[offset(112)]
    pub m_vecSpecifiedSurroundingMaxs: Vector2,
}

#[tf2_struct()]
pub struct EntityParticleTrailInfo {
    #[offset(16)]
    pub m_flLifetime: f32,
    #[offset(20)]
    pub m_flStartSize: f32,
    #[offset(24)]
    pub m_flEndSize: f32,
}

#[tf2_struct()]
pub struct PlayerState {
    #[offset(8)]
    pub deadflag: i32,
}

#[tf2_struct()]
pub struct LocalPlayerExclusive {
    #[offset(180)]
    pub m_nNextThinkTick: i32,
    #[offset(324)]
    pub m_vecViewOffset: [f32; 3],
    #[offset(360)]
    pub m_vecVelocity: [f32; 3],
    #[offset(496)]
    pub m_vecBaseVelocity: Vector2,
    #[offset(536)]
    pub m_nWaterLevel: i32,
    #[offset(796)]
    pub m_hGroundEntity: i32,
    #[offset(804)]
    pub m_flFriction: f32,
    #[offset(4240)]
    pub m_iAmmo: [i32; 32],
    #[offset(4672)]
    pub m_Local: Local,
    #[offset(5612)]
    pub m_fOnTarget: i32,
    #[offset(5672)]
    pub m_hConstraintEntity: i32,
    #[offset(5676)]
    pub m_vecConstraintCenter: Vector2,
    #[offset(5688)]
    pub m_flConstraintRadius: f32,
    #[offset(5692)]
    pub m_flConstraintWidth: f32,
    #[offset(5696)]
    pub m_flConstraintSpeedFactor: f32,
    #[offset(5736)]
    pub m_flDeathTime: f32,
    #[offset(5912)]
    pub m_nTickBase: i32,
    #[offset(5936)]
    pub m_hLastWeapon: i32,
    #[offset(6288)]
    pub m_flLaggedMovementValue: f32,
}

#[tf2_struct()]
pub struct Local {
    #[offset(8)]
    pub m_chAreaBits: [i32; 1],
    #[offset(40)]
    pub m_chAreaPortalBits: [i32; 1],
    #[offset(64)]
    pub m_iHideHUD: i32,
    #[offset(68)]
    pub m_flFOVRate: f32,
    #[offset(72)]
    pub m_bDucked: bool,
    #[offset(73)]
    pub m_bDucking: bool,
    #[offset(74)]
    pub m_bInDuckJump: bool,
    #[offset(76)]
    pub m_flDucktime: f32,
    #[offset(80)]
    pub m_flDuckJumpTime: f32,
    #[offset(84)]
    pub m_flJumpTime: f32,
    #[offset(92)]
    pub m_flFallVelocity: f32,
    #[offset(116)]
    pub m_vecPunchAngle: Vector2,
    #[offset(208)]
    pub m_vecPunchAngleVel: Vector2,
    #[offset(304)]
    pub m_bDrawViewmodel: bool,
    #[offset(305)]
    pub m_bWearingSuit: bool,
    #[offset(306)]
    pub m_bPoisoned: bool,
    #[offset(307)]
    pub m_bForceLocalPlayerDraw: bool,
    #[offset(312)]
    pub m_flStepSize: f32,
    #[offset(316)]
    pub m_bAllowAutoMovement: bool,
    #[offset(328)]
    pub m_skybox3d_scale: i32,
    #[offset(332)]
    pub m_skybox3d_origin: Vector2,
    #[offset(344)]
    pub m_skybox3d_area: i32,
    #[offset(360)]
    pub m_skybox3d_fog_dirPrimary: Vector2,
    #[offset(372)]
    pub m_skybox3d_fog_colorPrimary: i32,
    #[offset(376)]
    pub m_skybox3d_fog_colorSecondary: i32,
    #[offset(388)]
    pub m_skybox3d_fog_start: f32,
    #[offset(392)]
    pub m_skybox3d_fog_end: f32,
    #[offset(400)]
    pub m_skybox3d_fog_maxdensity: f32,
    #[offset(420)]
    pub m_skybox3d_fog_enable: bool,
    #[offset(421)]
    pub m_skybox3d_fog_blend: bool,
    #[offset(432)]
    pub m_PlayerFog_m_hCtrl: i32,
    #[offset(472)]
    pub m_audio_localSound: [Vector2; 8],
    #[offset(568)]
    pub m_audio_soundscapeIndex: i32,
    #[offset(572)]
    pub m_audio_localBits: i32,
    #[offset(576)]
    pub m_audio_entIndex: i32,
    #[offset(585)]
    pub m_szScriptOverlayMaterial: [i8; 260],
}

#[tf2_struct()]
pub struct PredictableId {
    #[offset(224)]
    pub m_PredictableID: i32,
    #[offset(1860)]
    pub m_bIsPlayerSimulated: bool,
}

#[tf2_struct()]
pub struct AnimTimeMustBeFirst {
    #[offset(144)]
    pub m_flAnimTime: i32,
}

#[tf2_struct()]
pub struct BCCLocalPlayerExclusive {
    #[offset(4232)]
    pub m_flNextAttack: f32,
}

#[tf2_struct()]
pub struct OverlayVars;

impl OverlayVars {
    pub type m_AnimOverlay = _ST_m_AnimOverlay_15;
}

#[tf2_struct()]
pub struct _ST_m_AnimOverlay_15;

impl _ST_m_AnimOverlay_15 {
    pub type i007 = Animationlayer;
    pub type i005 = Animationlayer;
    pub type lengthproxy = _LPT_m_AnimOverlay_15;
    pub type i013 = Animationlayer;
    pub type i010 = Animationlayer;
    pub type i004 = Animationlayer;
    pub type i014 = Animationlayer;
    pub type i008 = Animationlayer;
    pub type i009 = Animationlayer;
    pub type i003 = Animationlayer;
    pub type i002 = Animationlayer;
    pub type i012 = Animationlayer;
    pub type i006 = Animationlayer;
    pub type i001 = Animationlayer;
    pub type i011 = Animationlayer;
    pub type i000 = Animationlayer;
}

#[tf2_struct()]
pub struct Animationlayer {
    //probably invalid
    #[offset(0)]
    pub m_nSequence: i32,
    #[offset(4)]
    pub m_flPrevCycle: f32,
    #[offset(8)]
    pub m_flWeight: f32,
    #[offset(12)]
    pub m_nOrder: i32,
    #[offset(20)]
    pub m_flCycle: f32,
}

#[tf2_struct()]
pub struct _LPT_m_AnimOverlay_15 {
    //probably invalid
    #[offset(0)]
    pub lengthprop15: i32,
}

#[tf2_struct()]
pub struct ServerAnimationData {
    #[offset(2824)]
    pub m_flCycle: f32,
}

#[tf2_struct()]
pub struct BeamPredictableId {
    #[offset(224)]
    pub m_PredictableID: i32,
    #[offset(1860)]
    pub m_bIsPlayerSimulated: bool,
}

#[tf2_struct()]
pub struct LocalActiveWeaponData {
    #[offset(180)]
    pub m_nNextThinkTick: i32,
    #[offset(3732)]
    pub m_flNextPrimaryAttack: f32,
    #[offset(3736)]
    pub m_flNextSecondaryAttack: f32,
    #[offset(3740)]
    pub m_flTimeWeaponIdle: f32,
}

#[tf2_struct()]
pub struct LocalWeaponData {
    #[offset(3728)]
    pub m_nViewModelIndex: i32,
    #[offset(3800)]
    pub m_iPrimaryAmmoType: i32,
    #[offset(3804)]
    pub m_iSecondaryAmmoType: i32,
    #[offset(3808)]
    pub m_iClip1: i32,
    #[offset(3812)]
    pub m_iClip2: i32,
    #[offset(3856)]
    pub m_bFlipViewModel: bool,
    #[offset(3892)]
    pub m_nCustomViewmodelModelIndex: i32,
}

