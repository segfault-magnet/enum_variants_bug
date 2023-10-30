#[cfg(test)]
mod tests {
    use fuels::prelude::*;

    #[tokio::test]
    async fn the_issue() -> Result<()> {
        setup_program_test!(
            Abigen(Contract(name = "MyContract", project = "sway_project")),
            Wallets("wallet"),
            Deploy(
                name = "contract_instance",
                contract = "MyContract",
                wallet = "wallet"
            )
        );

        let ret = contract_instance
            .methods()
            .test_function(MyEnum::Variant254(10))
            .call()
            .await?
            .value;

        if let MyEnum::Variant254(val) = ret {
            assert_eq!(val, 11);
        } else {
            panic!("Didn't receive the correct variant, expected Variant254, got: {ret:?}");
        }

        Ok(())
    }
}
