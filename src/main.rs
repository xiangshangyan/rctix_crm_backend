mod config;

use actix_web::{App, HttpRequest, HttpServer, post, Responder, web};
use rbatis::{crud, impl_select, RBatis};
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::decimal::Decimal;
use rbdc_mysql::driver::MysqlDriver;
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello").to(index))
            .service(register)
            .service(web::resource("/selectByIds").to(select_by_ids))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/*无参请求*/
async fn index(req: HttpRequest) -> String {
    println!("REQ:{req:?}");

    // fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let mut rb = RBatis::new();
    /// connect to database
    // sqlite
    rb.init(MysqlDriver {}, &*config::get_conn_string()).unwrap();

    // crud!(Account{});
    impl_select!(Account{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});

    let data = Account::select_by_id(&mut rb, "1014041506545352708".to_string()).await;
    let test = data.unwrap();
    serde_json::to_string(&test).expect("open failed")
}

async fn select_by_ids(req: HttpRequest) -> String {
    println!("REQ:{req:?}");

    // fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let mut rb = RBatis::new();
    /// connect to database
    // sqlite
    rb.init(MysqlDriver {}, &*config::get_conn_string()).unwrap();
    let mut ids = Vec::new();
    ids.push("1014041506545352708");
    ids.push("1014041506545352711");
    ids.push("1014041506545352712");

    crud!(Account{});
    let data = Account::select_in_column(&mut rb, "id", &ids).await;
    match data {
        Ok(results) => {
            for x in &results {
                println!("{:?}", x.balance_amount);
                let amount = &x.balance_amount;
                match &amount {
                    Some(a) => { println!("the amount is {}", a) }
                    None => {}
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    // let test = data.unwrap();

    // serde_json::to_string(&test).expect("open failed")
    "string".to_string()
}


#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

#[post("/register")]
async fn register(form: web::Form<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: Option<u64>,
    // 用户id
    pub user_id: Option<u64>,
    // 币种id
    pub coin_id: Option<u64>,
    // 账号状态：1，正常；2，冻结；
    pub status: Option<u8>,
    // 可用金额
    pub balance_amount: Option<Decimal>,
    // 冻结金额
    pub freeze_amount: Option<Decimal>,
    // 累计充值金额
    pub recharge_amount: Option<Decimal>,
    // 累计提现金额
    pub withdrawals_amount: Option<Decimal>,
    // 净值
    pub net_value: Option<Decimal>,
    // 占用保证金
    pub lock_margin: Option<Decimal>,
    // 持仓盈亏/浮动盈亏
    pub float_profit: Option<Decimal>,
    // 总盈亏
    pub total_profit: Option<Decimal>,
    // 充值地址
    pub rec_addr: Option<String>,
    // 版本号
    pub version: Option<u32>,
    pub last_update_time: Option<DateTime>,
    pub created: Option<DateTime>,
}




