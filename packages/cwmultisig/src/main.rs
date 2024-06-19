use clap::{arg, Parser, Subcommand,Command};
use cosmwasm_std::{to_json_binary, to_json_string, Addr, CosmosMsg};
use cw3_flex_multisig::msg::ExecuteMsg;
use cw20_base::msg::MigrateMsg;
use cw4::Member;

#[derive(Parser,Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
     method: Commands,
     #[clap(short, long)]
     propose:bool

}

#[derive(Subcommand,Debug)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long)]
        list: bool,
    },
     /// update admin address of a contract
    UpdateAdmin {
        /// new admin address
        #[clap(short, long)]
        admin:String,
        /// contract address
        #[clap(short, long)]
        contract:String,
    },
    /// update members of multisig contract
    UpdateMembers {
        #[clap(short, long)]
        add:Option<Vec<String>>,
        #[clap(short, long)]
        remove:Option<Vec<String>>,
        #[clap(short, long)]
        contract:String
    },
    UpdateContract {
        #[clap(short, long)]
        contract:String,
        #[clap(short, long)]
        wasm_code_id:u64


    }
}

fn main() {
    let args=Cli::parse();
    println!("{:?}",&args);
    println!("Hello, world!");

  let res=  match args.method {
        Commands::Test { list } => "".to_string(),
        Commands::UpdateAdmin { admin, contract } => {
             let msg= CosmosMsg::Wasm(cosmwasm_std::WasmMsg::UpdateAdmin { contract_addr: contract, admin: admin });
             let proposal= ExecuteMsg::Propose {
                title: "Update Contract Admin".to_owned(),
                description: "Update Contract Admin".to_owned(),
                
                msgs: vec![msg],
                latest: None,
                
            };
            to_json_string(&proposal).unwrap()

        },
        Commands::UpdateMembers { add, remove ,contract}=>{
            let inner=cw4_group::msg::ExecuteMsg::UpdateMembers { remove:remove.unwrap_or_default(), add:add.unwrap_or_default().into_iter().map(|m|{
                Member{
                    addr:m,
                    weight:1,
                }
            }).collect() };
            let msg:CosmosMsg<cosmwasm_std::Empty> =CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute { 
                contract_addr: contract, 
                msg: to_json_binary(&inner).unwrap(), 
                funds: vec![],
            });

            let proposal= ExecuteMsg::Propose {
                title: "Update Group Members".to_owned(),
                description: "Update Group Members".to_owned(),
                
                msgs: vec![msg],
                latest: None,
                
            };
            to_json_string(&proposal).unwrap()
        }
    Commands::UpdateContract { contract ,wasm_code_id} => {

        let migrate_msg= MigrateMsg{};
        let migrate: CosmosMsg<cosmwasm_std::Empty>= CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Migrate { 
            contract_addr:contract, 
            new_code_id: wasm_code_id, 
            msg: to_json_binary(&migrate_msg).unwrap()
        });
        let proposal= ExecuteMsg::Propose {
            title: "Upgrade Contracts".to_owned(),
            description: "Upgrade Contract".to_owned(),
            
            msgs: vec![migrate],
            latest: None,
            
        };



        to_json_string(&proposal).unwrap()
    },
    };
    println!("{:?}",&res);
}
