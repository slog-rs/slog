#[deprecated(note = "Renamed to Value")]
/// Compatibility name to ease the pain of moving
pub type Serialize = Value;

#[deprecated(note = "Renamed to PushFnValue")]
/// Compatibility name to ease the pain of moving
pub type PushLazy<T> = PushFnValue<T>;

#[deprecated(note = "Renamed to PushFnSerializer")]
/// Compatibility name to ease the pain of moving
pub type ValueSerializer<'a> = PushFnSerializer<'a>;
