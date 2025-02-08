#[cfg(test)]
mod test {
    use crate::database::database;
    use crate::task::task::Task;

    #[test]
    fn test() {
        let conn = database::connect_to_db().unwrap();
        let _ = database::create_table(&conn);
        let a = vec![1313,32];
        a.push(5);
        // let a = Task::new();
        // let b = Task::new();
        // let _ = database::insert_task(&conn, &a);
        // let _ = database::insert_task(&conn, &b);
        let tmp = database::fetch_task(&conn).unwrap();
        tmp.iter().for_each(|val|database::delete_task(&conn, val.id).unwrap())
    }
}
