#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr, BlockInfo, Coin, Empty, HexBinary, Uint128};
    use cw721::OwnerOfResponse;
    use cw_multi_test::{BankSudo, Executor, SudoMsg};
    use nois::NoisCallback;
    use raffles::{
        error::ContractError,
        msg::{ExecuteMsg, InstantiateMsg, RaffleResponse},
        state::{RaffleOptionsMsg, RaffleState, NOIS_AMOUNT},
    };
    use sg721::CollectionInfo;
    use sg_std::NATIVE_DENOM;
    use utils::state::{AssetInfo, Sg721Token};
    use vending_factory::msg::VendingMinterCreateMsg;

    use crate::common_setup::{
        contract_boxes::contract_raffles,
        helpers::assert_error,
        setup_minter::common::constants::{
            CREATION_FEE_AMNT, NOIS_PROXY_ADDR, RAFFLE_NAME, SG721_CONTRACT, VENDING_MINTER,
        },
    };
    use sg721_base::QueryMsg as Sg721QueryMsg;

    const OWNER_ADDR: &str = "fee";

    mod init {
        use crate::common_setup::{helpers::setup_block_time, setup_raffle::proper_instantiate};

        use super::*;
        #[test]
        fn can_init() {
            // create testing app
            let (mut app, raffle_addr, factory_addr) = proper_instantiate();
            let query_config: raffles::msg::ConfigResponse = app
                .wrap()
                .query_wasm_smart(raffle_addr.clone(), &raffles::msg::QueryMsg::Config {})
                .unwrap();
            assert_eq!(query_config.owner, Addr::unchecked("fee"));

            let current_time = app.block_info().time.clone();
            let current_block = app.block_info().height.clone();
            let chainid = app.block_info().chain_id.clone();

            // fund test account
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: OWNER_ADDR.to_string(),
                    amount: vec![coin(200000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: "wallet-1".to_string(),
                    amount: vec![coin(100000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: "wallet-2".to_string(),
                    amount: vec![coin(100000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: "wallet-3".to_string(),
                    amount: vec![coin(100000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: "wallet-4".to_string(),
                    amount: vec![coin(100000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: "wallet-5".to_string(),
                    amount: vec![coin(100000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();
            // fund raffle contract for nois_proxy fee
            app.sudo(SudoMsg::Bank({
                BankSudo::Mint {
                    to_address: raffle_addr.clone().to_string(),
                    amount: vec![coin(100000000000u128, "ustars".to_string())],
                }
            }))
            .unwrap();

            let raffle_code_id = app.store_code(contract_raffles());

            // create nft minter
            let _create_nft_minter = app.execute_contract(
                Addr::unchecked(OWNER_ADDR),
                factory_addr.clone(),
                &vending_factory::msg::ExecuteMsg::CreateMinter {
                    0: VendingMinterCreateMsg {
                        init_msg: vending_factory::msg::VendingMinterInitMsgExtension {
                            base_token_uri: "ipfs://aldkfjads".to_string(),
                            payment_address: Some(OWNER_ADDR.to_string()),
                            start_time: current_time.clone(),
                            num_tokens: 100,
                            mint_price: coin(Uint128::new(100000u128).u128(), NATIVE_DENOM),
                            per_address_limit: 3,
                            whitelist: None,
                        },
                        collection_params: sg2::msg::CollectionParams {
                            code_id: 4,
                            name: "Collection Name".to_string(),
                            symbol: "COL".to_string(),
                            info: CollectionInfo {
                                creator: "creator".to_string(),
                                description: String::from("Atlanauts"),
                                image: "https://example.com/image.png".to_string(),
                                external_link: Some(
                                    "https://example.com/external.html".to_string(),
                                ),
                                start_trading_time: None,
                                explicit_content: Some(false),
                                royalty_info: None,
                            },
                        },
                    },
                },
                &[Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(100000u128),
                }],
            );
            // println!("{:#?}", create_nft_minter);

            // VENDING_MINTER is minter
            let _mint_nft_tokens = app
                .execute_contract(
                    Addr::unchecked(OWNER_ADDR),
                    Addr::unchecked(VENDING_MINTER),
                    &vending_minter::msg::ExecuteMsg::Mint {},
                    &[Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(100000u128),
                    }],
                )
                .unwrap();
            // println!("{:#?}", _mint_nft_tokens);

            // token id 41
            let _grant_approval = app
                .execute_contract(
                    Addr::unchecked(OWNER_ADDR),
                    Addr::unchecked(SG721_CONTRACT),
                    &sg721_base::msg::ExecuteMsg::<Empty, Empty>::Approve {
                        spender: raffle_addr.clone().to_string(),
                        token_id: "41".to_string(),
                        expires: None,
                    },
                    &[],
                )
                .unwrap();
            // println!("{:#?}", _grant_approval);

            // create nois_proxy

            // let good_nois_proxy

            // create a raffle
            let _good_create_raffle = app
                .execute_contract(
                    Addr::unchecked(OWNER_ADDR),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::CreateRaffle {
                        owner: Some(OWNER_ADDR.to_string()),
                        assets: vec![AssetInfo::Sg721Token(Sg721Token {
                            address: SG721_CONTRACT.to_string(),
                            token_id: "41".to_string(),
                        })],
                        raffle_options: RaffleOptionsMsg {
                            raffle_start_timestamp: Some(current_time.clone()),
                            raffle_duration: None,
                            raffle_timeout: None,
                            comment: None,
                            max_participant_number: None,
                            max_ticket_per_address: None,
                            raffle_preview: None,
                        },
                        raffle_ticket_price: AssetInfo::Coin(Coin {
                            denom: "ustars".to_string(),
                            amount: Uint128::new(4u128),
                        }),
                    },
                    &[coin(4, "ustars")],
                )
                .unwrap();
            // println!("{:#?}", _good_create_raffle);

            // confirm owner is set
            let res: raffles::msg::RaffleResponse = app
                .wrap()
                .query_wasm_smart(
                    raffle_addr.clone(),
                    &raffles::msg::QueryMsg::RaffleInfo { raffle_id: 0 },
                )
                .unwrap();
            assert_eq!(res.raffle_info.unwrap().owner, "fee");

            // error if no creation fee provided when creating raffle
            let create_raffle_no_creation_fee_error = app
                .execute_contract(
                    Addr::unchecked(OWNER_ADDR),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::CreateRaffle {
                        owner: Some(OWNER_ADDR.to_string()),
                        assets: vec![AssetInfo::Sg721Token(Sg721Token {
                            address: SG721_CONTRACT.to_string(),
                            token_id: "41".to_string(),
                        })],
                        raffle_options: RaffleOptionsMsg {
                            raffle_start_timestamp: None,
                            raffle_duration: Some(30),
                            raffle_timeout: None,
                            comment: None,
                            max_participant_number: None,
                            max_ticket_per_address: None,
                            raffle_preview: None,
                        },
                        raffle_ticket_price: AssetInfo::Coin(Coin {
                            denom: "ustars".to_string(),
                            amount: Uint128::new(100u128),
                        }),
                    },
                    &[],
                )
                .unwrap_err();
            // println!("{:#?}", create_raffle_no_creation_fee_error);

            assert_error(
                Err(create_raffle_no_creation_fee_error),
                ContractError::InvalidRaffleFee {}.to_string(),
            );

            //  error if unauthorized to cancel a raffle
            let invalid_cancel_raffle = app
                .execute_contract(
                    Addr::unchecked("not-owner"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::CancelRaffle { raffle_id: 0 },
                    &[],
                )
                .unwrap_err();
            // println!("{:#?}", invalid_cancel_raffle);
            assert_error(
                Err(invalid_cancel_raffle),
                ContractError::Unauthorized {}.to_string(),
            );

            // err if no nois_proxy address is provided
            let invalid_proxy = app
                .instantiate_contract(
                    raffle_code_id,
                    Addr::unchecked(OWNER_ADDR),
                    &InstantiateMsg {
                        name: RAFFLE_NAME.to_string(),
                        nois_proxy_addr: "".to_string(),
                        nois_proxy_coin: coin(NOIS_AMOUNT.into(), NATIVE_DENOM.to_string()),
                        owner: Some(OWNER_ADDR.to_string()),
                        fee_addr: Some(OWNER_ADDR.to_owned()),
                        minimum_raffle_duration: None,
                        minimum_raffle_timeout: None,
                        max_participant_number: None,
                        raffle_fee: None,
                        creation_coins: vec![
                            coin(CREATION_FEE_AMNT, NATIVE_DENOM.to_string()),
                            coin(CREATION_FEE_AMNT, "usstars".to_string()),
                        ]
                        .into(),
                    },
                    &[],
                    "raffle",
                    None,
                )
                .unwrap_err();
            // println!("{:#?}", invalid_proxy);
            assert_error(
                Err(invalid_proxy),
                ContractError::InvalidProxyAddress {}.to_string(),
            );

            // errors if no funds are sent
            let invalid_raffle_purchase = app
                .execute_contract(
                    Addr::unchecked(OWNER_ADDR),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::BuyTicket {
                        raffle_id: 1,
                        ticket_count: 1,
                        sent_assets: AssetInfo::Coin(Coin {
                            denom: "ustars".to_string(),
                            amount: Uint128::new(69u128),
                        }),
                    },
                    &[],
                )
                .unwrap_err();
            // println!("{:#?}", invalid_raffle_purchase);
            assert_error(
                Err(invalid_raffle_purchase),
                ContractError::AssetMismatch {}.to_string(),
            );

            // errors if unauthorized to lock contract
            let invalid_toggle_lock = app
                .execute_contract(
                    Addr::unchecked("not-owner"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::ToggleLock { lock: true },
                    &[],
                )
                .unwrap_err();
            // println!("{:#?}", invalid_toggle_lock);
            assert_error(
                Err(invalid_toggle_lock),
                ContractError::Unauthorized {}.to_string(),
            );

            // invalid modify raffle
            let invalid_modify_raffle = app
                .execute_contract(
                    Addr::unchecked("not-admin"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::ModifyRaffle {
                        raffle_id: 0,
                        raffle_ticket_price: None,
                        raffle_options: RaffleOptionsMsg {
                            raffle_start_timestamp: None,
                            raffle_duration: None,
                            raffle_timeout: None,
                            comment: Some("rust is dooope".to_string()),
                            max_participant_number: None,
                            max_ticket_per_address: None,
                            raffle_preview: None,
                        },
                    },
                    &[],
                )
                .unwrap_err();
            // println!("{:#?}", invalid_modify_raffle);
            assert_error(
                Err(invalid_modify_raffle),
                ContractError::Unauthorized {}.to_string(),
            );

            // buy tickets
            let _ticket_purchase1 = app
                .execute_contract(
                    Addr::unchecked("wallet-1"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::BuyTicket {
                        raffle_id: 0,
                        ticket_count: 16,
                        sent_assets: AssetInfo::Coin(Coin {
                            denom: NATIVE_DENOM.to_string(),
                            amount: Uint128::new(64u128),
                        }),
                    },
                    &[Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(64u128),
                    }],
                )
                .unwrap();
            // println!("{:#?}", _ticket_purchase1);
            let _ticket_purchase2 = app
                .execute_contract(
                    Addr::unchecked("wallet-2"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::BuyTicket {
                        raffle_id: 0,
                        ticket_count: 16,
                        sent_assets: AssetInfo::Coin(Coin {
                            denom: NATIVE_DENOM.to_string(),
                            amount: Uint128::new(64u128),
                        }),
                    },
                    &[Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(64u128),
                    }],
                )
                .unwrap();
            // println!("{:#?}", _ticket_purchase2);
            let _ticket_purchase3 = app
                .execute_contract(
                    Addr::unchecked("wallet-3"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::BuyTicket {
                        raffle_id: 0,
                        ticket_count: 16,
                        sent_assets: AssetInfo::Coin(Coin {
                            denom: NATIVE_DENOM.to_string(),
                            amount: Uint128::new(64u128),
                        }),
                    },
                    &[Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(64u128),
                    }],
                )
                .unwrap();
            // println!("{:#?}", _ticket_purchase3);
            let _ticket_purchase4 = app
                .execute_contract(
                    Addr::unchecked("wallet-4"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::BuyTicket {
                        raffle_id: 0,
                        ticket_count: 16,
                        sent_assets: AssetInfo::Coin(Coin {
                            denom: NATIVE_DENOM.to_string(),
                            amount: Uint128::new(64u128),
                        }),
                    },
                    &[Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(64u128),
                    }],
                )
                .unwrap();
            // println!("{:#?}", _ticket_purchase4);
            let _ticket_purchase5 = app
                .execute_contract(
                    Addr::unchecked("wallet-5"),
                    raffle_addr.clone(),
                    &raffles::msg::ExecuteMsg::BuyTicket {
                        raffle_id: 0,
                        ticket_count: 16,
                        sent_assets: AssetInfo::Coin(Coin {
                            denom: NATIVE_DENOM.to_string(),
                            amount: Uint128::new(64u128),
                        }),
                    },
                    &[Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(64u128),
                    }],
                )
                .unwrap();
            // println!("{:#?}", _ticket_purchase5);

            let res: u32 = app
                .wrap()
                .query_wasm_smart(
                    raffle_addr.clone(),
                    &raffles::msg::QueryMsg::TicketCount {
                        owner: Addr::unchecked("wallet-1").to_string(),
                        raffle_id: 0,
                    },
                )
                .unwrap();
            assert_eq!(res, 16);

            // move forward in time
            setup_block_time(
                &mut app,
                current_time.clone().plus_seconds(130).nanos(),
                Some(current_block.clone() + 100),
                &chainid.clone(),
            );

            // try to claim ticket before randomness is requested
            let claim_but_no_randomness_yet = app
                .execute_contract(
                    Addr::unchecked("wallet-1".to_string()),
                    raffle_addr.clone(),
                    &ExecuteMsg::DetermineWinner { raffle_id: 0 },
                    &[],
                )
                .unwrap_err();

            // println!("{:#?}", claim_but_no_randomness_yet);
            assert_error(
                Err(claim_but_no_randomness_yet),
                ContractError::WrongStateForClaim {
                    status: RaffleState::Closed,
                }
                .to_string(),
            );

            // ensure only nois_proxy provides randomness
            let bad_recieve_randomness = app
                .execute_contract(
                    Addr::unchecked("wallet-1"),
                    raffle_addr.clone(),
                    &ExecuteMsg::NoisReceive {
                        callback: NoisCallback {
                            job_id: "raffle-0".to_string(),
                            published: current_time.clone(),
                            randomness: HexBinary::from_hex(
                                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa115",
                            )
                            .unwrap(),
                        },
                    },
                    &[],
                )
                .unwrap_err();
            // println!("{:#?}", bad_recieve_randomness);
            assert_error(
                Err(bad_recieve_randomness),
                ContractError::UnauthorizedReceive.to_string(),
            );

            // simulates the response from nois_proxy
            let _good_receive_randomness = app
                .execute_contract(
                    Addr::unchecked(NOIS_PROXY_ADDR),
                    raffle_addr.clone(),
                    &ExecuteMsg::NoisReceive {
                        callback: NoisCallback {
                            job_id: "raffle-0".to_string(),
                            published: current_time.clone(),
                            randomness: HexBinary::from_hex(
                                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa115",
                            )
                            .unwrap(),
                        },
                    },
                    &[],
                )
                .unwrap();

            let res: RaffleResponse = app
                .wrap()
                .query_wasm_smart(
                    raffle_addr.clone(),
                    &raffles::msg::QueryMsg::RaffleInfo { raffle_id: 0 },
                )
                .unwrap();
            println!("{:#?}", res);

            // determine the raffle winner, send tokens to winner
            let _claim_ticket = app
                .execute_contract(
                    Addr::unchecked("wallet-1".to_string()),
                    raffle_addr.clone(),
                    &ExecuteMsg::DetermineWinner { raffle_id: 0 },
                    &[],
                )
                .unwrap();
            // println!("{:#?}", _claim_ticket);
            let res: RaffleResponse = app
                .wrap()
                .query_wasm_smart(
                    raffle_addr.clone(),
                    &raffles::msg::QueryMsg::RaffleInfo { raffle_id: 0 },
                )
                .unwrap();
            assert_eq!(res.raffle_state, RaffleState::Claimed);

            // confirm owner of nft is now raffle winner
            let res: OwnerOfResponse = app
                .wrap()
                .query_wasm_smart(
                    SG721_CONTRACT.to_string(),
                    &Sg721QueryMsg::OwnerOf {
                        token_id: "41".to_string(),
                        include_expired: None,
                    },
                )
                .unwrap();
            assert_eq!(res.owner, "wallet-2".to_string())
        }
    }
}
