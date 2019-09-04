//! Shared definitions for the Cranelift intermediate language.

pub mod entities;
pub mod formats;
pub mod immediates;
pub mod instructions;
pub mod legalize;
pub mod settings;
pub mod types;

use crate::cdsl::formats::FormatRegistry;
use crate::cdsl::instructions::{AllInstructions, InstructionGroup};
use crate::cdsl::operands::OperandKind;
use crate::cdsl::settings::SettingGroup;
use crate::cdsl::xform::TransformGroups;

use crate::shared::immediates::Immediates;

pub struct Definitions {
    pub settings: SettingGroup,
    pub all_instructions: AllInstructions,
    pub instructions: InstructionGroup,
    pub imm: Immediates,
    pub format_registry: FormatRegistry,
    pub transform_groups: TransformGroups,
}

pub struct OperandKinds(Vec<OperandKind>);

impl OperandKinds {
    pub fn by_name(&self, name: &'static str) -> &OperandKind {
        self.0
            .iter()
            .find(|op| op.name == name)
            .expect(&format!("unknown Operand name: {}", name))
    }
}

impl From<Vec<OperandKind>> for OperandKinds {
    fn from(kinds: Vec<OperandKind>) -> Self {
        OperandKinds(kinds)
    }
}

pub fn define() -> Definitions {
    let mut all_instructions = AllInstructions::new();

    let immediates = Immediates::new();
    let entities = OperandKinds(entities::define());
    let format_registry = formats::define(&immediates, &entities);
    let instructions = instructions::define(
        &mut all_instructions,
        &format_registry,
        &immediates,
        &entities,
    );
    let transform_groups = legalize::define(&instructions, &immediates);

    Definitions {
        settings: settings::define(),
        all_instructions,
        instructions,
        imm: immediates,
        format_registry,
        transform_groups,
    }
}