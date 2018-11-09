#[macro_use] extern crate log;

extern crate db;
//use db ;
use db::util ;

fn main()  {
	util::logger_init ("/tmp/db.log")	;
	let pool = db::db::db_pool(None) 		;
	let db_conn = db::db::db_conn(&pool)		;
	
	let sql= "select row_to_json(a, true) json from usr a limit 3" ;
	let result = db::db::runsql_conn(&db_conn, &sql  , &[], 2) ;
	info!("query 1 {:?}", result);
	//let sql= "select row_to_json(a, true) json from usr a where balance > $1 limit 1" ;
	let sql= "select row_to_json(a, true) json from usr a where last_name = $1 and trips_posted >= $2 limit 1" ;
	
	let result=db::db::runsql_one_row(&db_conn, &sql, &[&"Lin".to_string(), &0] );
	info!("query 2 {:?}", result);
}
