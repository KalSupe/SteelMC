use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_STOP_SOUND;
use steel_utils::Identifier;

#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_STOP_SOUND)]
pub struct CStopSound {
    pub flags: u8,

    #[write(as = Unprefixed(inner = VarInt))]
    pub source: Option<i32>,

    #[write(as = Unprefixed)]
    pub sound: Option<Identifier>,
}

impl CStopSound {
    pub fn new(source: Option<i32>, sound: Option<Identifier>) -> Self {
        let mut flags = 0;

        if source.is_some() {
            flags |= 1;
        }

        if sound.is_some() {
            flags |= 2;
        }

        Self {
            flags,
            source,
            sound,
        }
    }
}
