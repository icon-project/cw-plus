use clap::{Parser, Subcommand};
use cosmwasm_std::{to_json_binary, to_json_string, CosmosMsg};
use cw20_base::msg::MigrateMsg;
use cw3::Vote;
use cw3_flex_multisig::msg::ExecuteMsg;
use cw4::Member;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    method: Commands,
    #[clap(short, long)]
    propose: bool,
}

pub struct MigrateMsgCore {
    clear_store:bool,
}

pub enum MigrateMsgs{
    MigrateMsg(MigrateMsg),
    MigrateMsgCore(MigrateMsgCore)
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// update admin address of a contract
    UpdateAdmin {
        /// new admin address
        #[clap(short, long)]
        admin: String,
        /// contract address
        #[clap(short, long)]
        contract: String,
    },
    /// update members of multisig contract
    UpdateMembers {
        #[clap(short, long)]
        add: String,
        #[clap(short, long)]
        remove: String,
        #[clap(short, long)]
        members_contract: String,
        #[clap(short, long)]
        threshold: u64,
        #[clap(short = 's', long)]
        multisig_contract: String,
    },
    UpdateContract {
        #[clap(short, long)]
        contract: String,
        #[clap(short, long)]
        wasm_code_id: u64,
        #[clap(short = 'cn', long)]
        contract_name:String

    },
    Vote {
        #[clap(short, long)]
        proposal_id: u64,
        #[clap(short, long)]
        vote: String,
    },
    InitMultisig {
        #[clap(short, long)]
        group_contract: String,
        #[clap(short, long)]
        threshold: u64,
    },
    InitMembers {
        #[clap(short, long)]
        members: String,
        #[clap(short, long)]
        admin: String,
    },
}

fn main() {
    let args = Cli::parse();
    // println!("{:?}",&args);

    let res = match args.method {
        Commands::UpdateAdmin { admin, contract } => {
            let msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::UpdateAdmin {
                contract_addr: contract,
                admin,
            });
            let proposal = ExecuteMsg::Propose {
                title: "UpdateContractAdmin".to_owned(),
                description: "UpdateContractAdmin".to_owned(),

                msgs: vec![msg],
                latest: None,
            };
            to_json_string(&proposal).unwrap()
        }
        Commands::UpdateMembers {
            add,
            remove,
            members_contract,
            multisig_contract,
            threshold,
        } => {
            let mut remove_list: Vec<String> = vec![];
            let mut add_list: Vec<Member> = vec![];
            if remove != *"none" {
                remove_list = remove.split(',').map(|s| s.to_string()).collect();
            }
            if add != *"none" {
                add_list = add
                    .split(',')
                    .map(|m| Member {
                        addr: m.to_string(),
                        weight: 1,
                    })
                    .collect::<Vec<Member>>();
            }
            let inner = cw4_group::msg::ExecuteMsg::UpdateMembers {
                add: add_list,
                remove: remove_list,
            };
            let update_member: CosmosMsg<cosmwasm_std::Empty> =
                CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: members_contract,
                    msg: to_json_binary(&inner).unwrap(),
                    funds: vec![],
                });
            let inner_2 = cw3_flex_multisig::msg::ExecuteMsg::UpdateThreshold { threshold };

            let update_threshold = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: multisig_contract,
                msg: to_json_binary(&inner_2).unwrap(),
                funds: vec![],
            });

            let proposal = ExecuteMsg::Propose {
                title: "UpdateGroupMembers".to_owned(),
                description: "UpdateGroupMembers".to_owned(),

                msgs: vec![update_member, update_threshold],
                latest: None,
            };
            to_json_string(&proposal).unwrap()
        }
        Commands::UpdateContract {
            contract,
            wasm_code_id,
        } => {
            let migrate_msg:MigrateMsgs = if contract_name=="ibc-core".toString() {
                MigrateMsgs::MigrateMsgCore(MigrateMsgCore{clear_store:false})
            }else{
                MigrateMsgs::MigrateMsg(MigrateMsg {})
            }
            let migrate: CosmosMsg<cosmwasm_std::Empty> =
                CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Migrate {
                    contract_addr: contract,
                    new_code_id: wasm_code_id,
                    msg: to_json_binary(&migrate_msg.0).unwrap(),
                });
            let proposal = ExecuteMsg::Propose {
                title: "UpgradeContracts".to_owned(),
                description: "UpgradeContract".to_owned(),

                msgs: vec![migrate],
                latest: None,
            };

            to_json_string(&proposal).unwrap()
        }
        Commands::Vote { proposal_id, vote } => {
            let vote = match vote.to_lowercase().as_str() {
                "yes" => Vote::Yes,
                "no" => Vote::No,
                _ => Vote::Abstain,
            };

            let execute_vote = ExecuteMsg::Vote { proposal_id, vote };
            to_json_string(&execute_vote).unwrap()
        }
        Commands::InitMultisig {
            group_contract,
            threshold,
        } => {
            let msg = cw3_flex_multisig::msg::InstantiateMsg {
                group_addr: group_contract,
                threshold: cw_utils::Threshold::AbsoluteCount { weight: threshold },
                max_voting_period: cw_utils::Duration::Time(2592000),
                executor: None,
                proposal_deposit: None,
            };
            to_json_string(&msg).unwrap()
        }
        Commands::InitMembers { members, admin } => {
            let msg = cw4_group::msg::InstantiateMsg {
                admin: Some(admin),
                members: members
                    .split(',')
                    .map(|m| Member {
                        addr: m.to_string(),
                        weight: 1,
                    })
                    .collect(),
            };

            to_json_string(&msg).unwrap()
        }
    };
    println!("{:?}", &res);
}
