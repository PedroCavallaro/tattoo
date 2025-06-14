use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaginationDTO<T> {
    pub data: T,
    pub page: i64,
    pub limit: i64,
}

pub fn get_limit_offset(page: i32, limit: i32) -> (i32, i32) {
    if page == 0 {
        return (1, limit);
    }

    let offset = (page - 1) * limit;
    let limit = (page + 1) * limit;

    (limit, offset)
}
