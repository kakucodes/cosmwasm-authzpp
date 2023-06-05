use cosmwasm_std::{Coin, Decimal};

use crate::helpers::{partition_coins_by_percentage, sum_coins};

// unit tests for the sum_coins helper function
#[test]
fn sum_coins_test() {
    let xs = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 100u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 100u128.into(),
        },
    ];
    let ys = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 100u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 100u128.into(),
        },
    ];
    let expected = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 200u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 200u128.into(),
        },
    ];
    assert_eq!(sum_coins(xs, ys), expected);

    let xs = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 100u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 100u128.into(),
        },
    ];
    let ys = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 100u128.into(),
        },
        Coin {
            denom: "ubtc".to_string(),
            amount: 100u128.into(),
        },
    ];
    let expected = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 200u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 100u128.into(),
        },
        Coin {
            denom: "ubtc".to_string(),
            amount: 100u128.into(),
        },
    ];
    assert_eq!(sum_coins(xs, ys), expected);
}

#[test]
fn partition_coins() {
    let coins = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 100u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 200u128.into(),
        },
    ];
    let (coins_to_send, coins_to_remain) =
        partition_coins_by_percentage(Decimal::percent(25), coins);
    let expected_to_send = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 25u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 50u128.into(),
        },
    ];
    let expected_to_remain = vec![
        Coin {
            denom: "ujuno".to_string(),
            amount: 75u128.into(),
        },
        Coin {
            denom: "uosmo".to_string(),
            amount: 150u128.into(),
        },
    ];
    assert_eq!(coins_to_send, expected_to_send);
    assert_eq!(coins_to_remain, expected_to_remain);
}

// #[test]
// fn generate_neta_staking_msg() {
//     let delegator_addr = Addr::unchecked("test1");
//     let sim_response = SimulationResponse {
//         referral_amount: 0u128.into(),
//         return_amount: 100u128.into(),
//         spread_amount: 0u128.into(),
//         commission_amount: 0u128.into(),
//     };

//     let expected_msgs: Vec<CosmosProtoMsg> = vec![
//         CosmosProtoMsg::ExecuteContract(MsgExecuteContract {
//             contract: JUNO_NETA_PAIR_ADDR.to_string(),
//             sender: "test1".to_string(),
//             msg: to_binary(&wyndex::pair::ExecuteMsg::Swap {
//                 offer_asset: wyndex::asset::Asset {
//                     info: wyndex::asset::AssetInfo::Native("ujuno".to_string()),
//                     amount: 1000u128.into(),
//                 },
//                 ask_asset_info: Some(AssetInfo::Token(NETA_CW20_ADDR.to_string())),
//                 max_spread: None,
//                 belief_price: None,
//                 to: None,
//                 referral_address: None,
//                 referral_commission: None,
//             })
//             .expect("failed to encode swap msg")
//             .to_vec(),
//             funds: vec![Coin {
//                 amount: 1000u128.to_string(),
//                 denom: "ujuno".to_string(),
//             }],
//         }),
//         CosmosProtoMsg::ExecuteContract(MsgExecuteContract {
//             contract: NETA_CW20_ADDR.to_string(),
//             sender: "test1".to_string(),
//             msg: to_binary(&cw20::Cw20ExecuteMsg::Send {
//                 contract: NETA_STAKING_ADDR.to_string(),
//                 amount: 100u128.into(),
//                 msg: to_binary(&cw20_stake::msg::ReceiveMsg::Stake {})
//                     .expect("failed to encode cw20 send msg"),
//             })
//             .expect("failed to encode cw20 send msg")
//             .to_vec(),
//             funds: vec![],
//         }),
//     ];

//     assert_eq!(
//         neta_staking_msgs(
//             delegator_addr,
//             1000u128.into(),
//             "ujuno".to_string(),
//             sim_response
//         )
//         .unwrap(),
//         expected_msgs
//     );
// }
