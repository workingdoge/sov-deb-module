use anyhow::Result;
use sov_modules_api::GenesisState;

use crate::TAccountModule;

impl<S: sov_modules_api::Spec> TAccountModule<S> {
    pub(crate) fn init_module(
        &self,
        _config: &<Self as sov_modules_api::Module>::Config,
        _state: &mut impl GenesisState<S>,
    ) -> Result<()> {
        Ok(())
    }
}
