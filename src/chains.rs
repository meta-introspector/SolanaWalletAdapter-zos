#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Chains {
    MainNet,
    DevNet,
    TestNet,
    LocalNet,
}
