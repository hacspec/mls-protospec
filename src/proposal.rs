use crate::key_package::KeyPackage;

pub enum ProposalType {
    Invalid = 0,
    Add = 1,
    Update = 2,
    Remove = 3,
}

pub struct Proposal {
    proposal_type: ProposalType,
    msg: Box<dyn ProposalTrait>,
}

pub trait ProposalTrait {}

struct Add {
    key_package: KeyPackage,
}

impl ProposalTrait for Add {}

struct Update {
    key_package: KeyPackage,
}

impl ProposalTrait for Update {}

struct Remove {
    removed: u32,
}

impl ProposalTrait for Remove {}
