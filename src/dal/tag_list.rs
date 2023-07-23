use crate::bo::tag::{Tag, TagGroup};
use crate::dal::query::query;
use crate::PgDatabase;
use rocket_db_pools::Connection;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Row};

pub async fn tag_list(
    client: &Connection<PgDatabase>,
    lang: String,
) -> Result<Vec<TagGroup>, Error> {
    let sql_query = "SELECT id_tag,label,id_group,group_label FROM f_tag_list($1);";
    let params: &[&(dyn ToSql + Sync)] = &[&lang];
    match query(client, sql_query, params).await {
        Ok(rows) => Ok(rows_to_tag_group(rows)),
        Err(err) => Err(err),
    }
}

fn rows_to_tag_group(rows: Vec<Row>) -> Vec<TagGroup> {
    let mut result: Vec<TagGroup> = vec![];
    for row in rows {
        let id_group: String = row.get("id_group");
        let tag = Tag {
            id: row.get("id_tag"),
            label: row.get("label"),
        };
        if !result.iter().any(|el| el.id.eq(&id_group)) {
            // generates group if not exist
            let group_label: String = row.get("group_label");
            result.push(TagGroup {
                id: id_group.clone(),
                label: group_label,
                tags: vec![tag],
            });
        } else {
            // push tag in corresponding group
            result
                .iter_mut()
                .find(|el| el.id.eq(&id_group))
                .unwrap()
                .tags
                .push(tag);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rows_to_tag_test() {
        let result = rows_to_tag_group(vec![]);
        let data_set: Vec<TagGroup> = vec![];
        assert!(result.iter().all(|item| data_set.contains(item)));
    }
}
