use integration_utils::context::Context;
use model::ContractNameInterfaceIntegration;

use crate::contract_name_interface::ContractName;

pub trait IntegrationContracts {
    fn contract_name(&self) -> ContractName;
}

impl IntegrationContracts for Context {
    fn contract_name(&self) -> ContractName {
        ContractName {
            contract: &self.contracts["contract_name"],
        }
    }
}

pub async fn prepare_contract() -> anyhow::Result<Context> {
    let context = Context::new(&["contract_name"]).await?;
    context.contract_name().init().await?;
    Ok(context)
}
