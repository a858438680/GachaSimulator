pub enum ItemLevel {
    Star3,
    Star4,
    Star5,
}

pub enum ItemType {
    Character,
    Weapon,
}

pub enum UpType {
    Up(u32),
    NonUp
}

pub enum WantState {
    Want(u32, u32),
    None
}

pub enum NormalGachaType {
    Character5Star,
    Weapon5Star,
    Character4Star,
    Weapon4Star,
    Other3Star,
}

pub enum CharacterGachaType {
    Up5Star,
    Up4Star(u32),
    Other5Star,
    Other4StarCharacter,
    Other4StarWeapon,
    Other3Star,
}

pub enum WeaponGachaType {
    Up5Star(u32),
    Up4Star(u32),
    Other5Star,
    Other4StarCharacter,
    Other4StarWeapon,
    Other3Star,
}
