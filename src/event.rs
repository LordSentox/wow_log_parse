use crate::unit::Unit;
use chrono::NaiveDateTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    DamageShield,
    DamageShieldMissed,
    EnchantApplied,
    EnvironmentalDamage,
    PartyKill,
    RangeDamage,
    SpellAuraApplied,
    SpellAuraAppliedDose,
    SpellAuraRefresh,
    SpellAuraRemoved,
    SpellCastFailed,
    SpellCastStart,
    SpellCastSuccess,
    SpellCreate,
    SpellDamage,
    SpellDispel,
    SpellEnergise,
    SpellHeal,
    SpellInterrupt,
    SpellMissed,
    SpellPeriodicDamage,
    SpellPeriodicDrain,
    SpellPeriodicEnergise,
    SpellPeriodicHeal,
    SpellPeriodicMissed,
    SpellStolen,
    SpellSummon,
    SpellResurrect,
    SwingDamage,
    SwingMissed,
    UnitDied
}

#[derive(Clone, Debug)]
pub struct Event {
    /// The time this event occured, depending on the client time used for
    /// logging. Therefore it can only be trusted to be correct *relative*
    /// to other events of the same log file.
    time: NaiveDateTime,
    /// The type of this event
    typ: EventType,
    /// The unit that is the cause or source of this event or `None` if no such
    /// unit exists, for instance with `EventType::EnvironmentalDamage`.
    source: Option<Unit>,
    /// The unit that is the target or receiver of this event or `None` if no
    /// such unit exists, for instance with `EventType::SpellCastSuccess`
    target: Option<Unit>,
    /// Many events have an amount of for instance damage or healing. In that
    /// case, this will be set. For events like `EventType::UnitDied` this
    /// will be set to `None`.
    amount: Option<u64>
}

#[derive(Clone, Debug)]
pub enum ParseErrorType {
    WrongHeadLength,
    WrongTimeFormat,
    InvalidArg,
    UnknownEventType(String)
}

#[derive(Clone, Debug)]
pub struct ParseError {
    typ: ParseErrorType,
    col: usize
}

impl EventType {
    pub fn from_str<S: AsRef<str>>(s: S, col: usize) -> Result<EventType, ParseError> {
        match s.as_ref() {
            "DAMAGE_SHIELD" => Ok(EventType::DamageShield),
            "DAMAGE_SHIELD_MISSED" => Ok(EventType::DamageShieldMissed),
            "ENCHANT_APPLIED" => Ok(EventType::EnchantApplied),
            "ENVIRONMENTAL_DAMAGE" => Ok(EventType::EnvironmentalDamage),
            "PARTY_KILL" => Ok(EventType::PartyKill),
            "RANGE_DAMAGE" => Ok(EventType::RangeDamage),
            "SPELL_AURA_APPLIED" => Ok(EventType::SpellAuraApplied),
            "SPELL_AURA_APPLIED_DOSE" => Ok(EventType::SpellAuraAppliedDose),
            "SPELL_AURA_REFRESH" => Ok(EventType::SpellAuraRefresh),
            "SPELL_AURA_REMOVED" => Ok(EventType::SpellAuraRemoved),
            "SPELL_CAST_FAILED" => Ok(EventType::SpellCastFailed),
            "SPELL_CAST_START" => Ok(EventType::SpellCastStart),
            "SPELL_CAST_SUCCESS" => Ok(EventType::SpellCastSuccess),
            "SPELL_CREATE" => Ok(EventType::SpellCreate),
            "SPELL_DAMAGE" => Ok(EventType::SpellDamage),
            "SPELL_DISPEL" => Ok(EventType::SpellDispel),
            "SPELL_ENERGIZE" => Ok(EventType::SpellEnergise),
            "SPELL_HEAL" => Ok(EventType::SpellHeal),
            "SPELL_INTERRUPT" => Ok(EventType::SpellInterrupt),
            "SPELL_MISSED" => Ok(EventType::SpellMissed),
            "SPELL_PERIODIC_DAMAGE" => Ok(EventType::SpellPeriodicDamage),
            "SPELL_PERIODIC_DRAIN" => Ok(EventType::SpellPeriodicDrain),
            "SPELL_PERIODIC_ENERGIZE" => Ok(EventType::SpellPeriodicEnergise),
            "SPELL_PERIODIC_HEAL" => Ok(EventType::SpellPeriodicHeal),
            "SPELL_PERIODIC_MISSED" => Ok(EventType::SpellPeriodicMissed),
            "SPELL_STOLEN" => Ok(EventType::SpellStolen),
            "SPELL_SUMMON" => Ok(EventType::SpellSummon),
            "SPELL_RESURRECT" => Ok(EventType::SpellResurrect),
            "SWING_DAMAGE" => Ok(EventType::SwingDamage),
            "SWING_MISSED" => Ok(EventType::SwingMissed),
            "UNIT_DIED" => Ok(EventType::UnitDied),
            other => Err(ParseError::new(
                ParseErrorType::UnknownEventType(other.to_string()),
                col
            ))
        }
    }

    /// Returns true, if the event is hostile, like a damaging event or a curse
    /// event. Environmental damage does not count, since the environment has no
    /// will.
    pub fn is_hostile(&self) -> bool {
        match self {
            EventType::DamageShield
            | EventType::DamageShieldMissed
            | EventType::RangeDamage
            | EventType::SpellDamage
            | EventType::SpellInterrupt
            | EventType::SpellMissed
            | EventType::SpellPeriodicDamage
            | EventType::SpellPeriodicMissed
            | EventType::SpellStolen
            | EventType::SwingDamage
            | EventType::SwingMissed => true,
            _ => false
        }
    }

    /// Returns true, if the event type is a damaging event.
    pub fn damaging(&self) -> bool {
        match self {
            EventType::DamageShield
            | EventType::EnvironmentalDamage
            | EventType::RangeDamage
            | EventType::SpellDamage
            | EventType::SpellPeriodicDamage
            | EventType::SwingDamage => true,
            _ => false
        }
    }

    pub fn healing(&self) -> bool {
        match self {
            EventType::SpellHeal | EventType::SpellPeriodicHeal => true,
            _ => false
        }
    }
}

impl ParseError {
    pub fn new(typ: ParseErrorType, col: usize) -> ParseError { ParseError { typ, col } }

    pub fn typ(&self) -> ParseErrorType { self.typ.clone() }

    pub fn col(&self) -> usize { self.col }
}

impl Event {
    /// Try to parse the event struct from an event string and return it.
    /// Returns None if the string is not properly formatted.
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Event, ParseError> {
        // Cut the later parts containing the advanced event information first,
        // because we have to cut by spaces afterwards, which would cut spell
        // names such as "Healing Stream Totem IX" into multiple parts
        let parts: Vec<&str> = s.as_ref().split(',').collect();

        // Take the first element, which should contain the timestamp and the EventType.
        let head: Vec<&str> = parts[0].split_whitespace().collect();
        // Check if the Head is properly formatted.
        if head.len() != 3 {
            error!(
                "Event head has incorrect length. Should be three, but found {}",
                head.len()
            );
            return Err(ParseError::new(
                ParseErrorType::WrongHeadLength,
                head.len() - 1
            ));
        }

        // Read the time from the stamp. Have to use Naive, because the Timezone is not
        // provided.
        let time = match NaiveDateTime::parse_from_str(
            &format!("{} {} 2019", head[0], head[1]),
            "%m/%d %H:%M:%S%.3f %Y"
        ) {
            Ok(time) => time,
            Err(err) => {
                error!("Error while parsing the time: {}", err);
                return Err(ParseError::new(ParseErrorType::WrongTimeFormat, 0));
            }
        };

        // Last item in the head is the event type.
        let typ = EventType::from_str(head[2], head[0].len() + head[1].len())?;

        // Read the source that this event was done by, or no cause, in case the
        // event was part of the environment.
        let source = Unit::from_raw(parts[1], parts[2]);

        // Read the target this event is affecting. None is not an option here.
        let target = Unit::from_raw(parts[4], parts[5]);

        let amount = if parts.len() > 10 {
            Some(
                parts[10]
                    .parse()
                    .expect("Could not parse amount. Unexpected non-integer at expected position.")
            )
        }
        else {
            None
        };

        // Create the event from the parsed data
        Ok(Event {
            time,
            typ,
            source,
            target,
            amount
        })
    }

    pub fn time(&self) -> NaiveDateTime { self.time }

    pub fn typ(&self) -> EventType { self.typ }

    pub fn is_hostile(&self) -> bool {
        if !self.typ.is_hostile() {
            return false;
        }

        // No two units of the same team may be involved in a hostile event
        // XXX: Assumes, Players are friendly and npcs are hostile.
        if let (Some(src), Some(tgt)) = (&self.source, &self.target) {
            if src.hostile() == tgt.hostile() {
                warn!("Detected hostile event on same side: {:?}", &self);
            }
        }

        true
    }

    pub fn source(&self) -> Option<Unit> { self.source.clone() }

    pub fn target(&self) -> Option<Unit> { self.target.clone() }

    pub fn amount(&self) -> Option<u64> { self.amount.clone() }
}
