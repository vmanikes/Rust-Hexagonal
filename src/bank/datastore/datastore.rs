pub trait Datastore {
    fn deposit();
}

//
// fn initialize_connection() -> Result<Client, Error> {
//
//     let mut client = match initialize_connection() {
//         Ok(client) => client,
//         Err(e) => {
//             error!("unable to initialize client: {}", e);
//             process::exit(1);
//         }
//     };
//
//     let mut db_config = Config::new();
//     db_config.user("postgres");
//     db_config.password("root");
//     db_config.host("localhost");
//
//     return db_config.connect(NoTls);
// }