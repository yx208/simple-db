use tabled::builder::Builder;
use sql_execution::ExecResponse;

pub fn display_response(res: ExecResponse) {
    match res {
        ExecResponse::Select(rows) => {
            let row = rows.get(0).expect("For now assuming we et data back");
            let columns: Vec<String> = row
                .columns()
                .iter()
                .map(|col| col.name.to_string())
                .collect();
            let mut builder = Builder::default();
            builder.set_columns(&columns);
            for row in rows.into_iter() {
                builder.add_record(columns.iter().map(|col| row.get(col)));
            }
            println!("{}", builder.build());
        },
        _ => println!("res")
    }
}

