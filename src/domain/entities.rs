#[derive(PartialEq, Clone)]
pub struct EntryId(u32);

impl TryFrom<u32> for EntryId {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n < 3 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<EntryId> for u32 {
    fn from(n: EntryId) -> u32 {
        n.0
    }
}

pub struct EntryUsername(String);

impl TryFrom<String> for EntryUsername {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct EntryLevel(u32);

impl TryFrom<u32> for EntryLevel {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n < 3 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<EntryLevel> for u32 {
    fn from(n: EntryLevel) -> u32 {
        n.0
    }
}

#[derive(PartialEq, Clone)]
pub struct EntrySeconds(u64);

impl TryFrom<u64> for EntrySeconds {
    type Error = ();

    fn try_from(n: u64) -> Result<Self, Self::Error> {
        Ok(Self(n))
    }
}

impl From<EntrySeconds> for u64 {
    fn from(n: EntrySeconds) -> u64 {
        n.0
    }
}

pub struct Entry {
    pub id: EntryId,
    user: EntryUsername,
    level: EntryLevel,
    seconds: EntrySeconds,
}

impl Entry {
    pub fn new(id: EntryId, user: EntryUsername, level: EntryLevel, seconds: EntrySeconds) -> Self {
        Self {
            id,
            user,
            level,
            seconds,
        }
    }
}