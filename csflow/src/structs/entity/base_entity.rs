use memflow::{types::Address, mem::MemoryView};
use crate::{CheatCtx, Error, cs2dumper, traits::{BaseEntity, MemoryClass}, structs::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct CBaseEntity(Address);

impl MemoryClass for CBaseEntity {
    fn ptr(&self) -> Address {
        self.0
    }

    fn new(ptr: Address) -> Self {
        Self(ptr)
    }
}

impl BaseEntity for CBaseEntity {
    fn from_index(ctx: &mut CheatCtx, entity_list: Address, index: i32) -> Result<Option<CBaseEntity>, Error> {
        let list_entry = ctx.process.read_addr64(entity_list + 8 * (index >> 9) + 16)?;
        if list_entry.is_null() && !list_entry.is_valid() {
            return Ok(None);
        }

        let player_ptr = ctx.process.read_addr64(list_entry + 120 * (index & 0x1FF))?;
        if player_ptr.is_null() && !player_ptr.is_valid() {
            return Ok(None);
        }

        Ok(Some(Self::new(player_ptr)))
    }

    fn pos(&self, ctx: &mut CheatCtx) -> Result<Vec3, Error> {
        let node = ctx.process.read_addr64(self.0 + cs2dumper::client::C_BaseEntity::m_pGameSceneNode)?;
        Ok(ctx.process.read(node + cs2dumper::client::CGameSceneNode::m_vecAbsOrigin)?)
    }

    fn class_name(&self, ctx: &mut CheatCtx) -> Result<String, Error> {
        let entity_identity_ptr = ctx.process.read_addr64(self.0 + cs2dumper::client::CEntityInstance::m_pEntity)?;
        let class_name_ptr = ctx.process.read_addr64(entity_identity_ptr + cs2dumper::client::CEntityIdentity::m_designerName)?;
        Ok(ctx.process.read_char_string_n(class_name_ptr, 32)?)
    }
}

impl CBaseEntity {
    pub fn to_player_controller(&self) -> super::CPlayerController {
        super::CPlayerController::new(self.0)
    }
}