trait Character {
    type WeaponType: Weapon;
    type ShieldType: Shield;

    fn create_weapon() -> Self::WeaponType;
    fn create_shield() -> Self::ShieldType;
}

trait Weapon {
    fn attack(&self);
}

trait Shield {
    fn block(&self);
}

struct Sword;
struct Staff;
struct BoneShield;
struct GoldShield;
pub struct Warrior;
pub struct Mage;

impl Shield for BoneShield {
    fn block(&self) {
        println!("Bone shild blocked")
    }
}
impl Shield for GoldShield {
    fn block(&self) {
        println!("Gold shild blocked")
    }
}
impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack")
    }
}

impl Weapon for Staff {
    fn attack(&self) {
        println!("Staff attack")
    }
}

impl Character for Warrior {
    type WeaponType = Sword;
    type ShieldType = GoldShield;
    fn create_weapon() -> Self::WeaponType {
        Sword
    }
    fn create_shield() -> Self::ShieldType {
        GoldShield
    }
}

impl Character for Mage {
    type WeaponType = Staff;
    type ShieldType = BoneShield;
    fn create_weapon() -> Self::WeaponType {
        Staff
    }
    fn create_shield() -> Self::ShieldType {
        BoneShield
    }
}

pub fn block<C: Character>() {
    let shield = C::create_shield();
    shield.block();
}

pub fn attack<C: Character>() {
    let weapon = C::create_weapon();
    weapon.attack();
}
