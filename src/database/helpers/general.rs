mod general {
    pub fn get_select_rows(query : &'static str) -> Vec<Row> {
        let mut rows : Vec<Row> = Vec::new();
        match get_connection() {
            Ok(mut conn) => {
                if let Ok(mut result) = conn.query_iter(query) {
                    while let Some(result_set) = result.next_set() {
                        if let Ok(set) = result_set {
                            for r in set {
                                if let Ok(row) = r {
                                    rows.push(row);
                                }
                            }
                        }
                    }
                }
            },
            Err(_) => {}
        }
        rows
    }
}
