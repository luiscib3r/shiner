use flutter_rust_bridge::frb;

pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

impl Pagination {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            limit: 10,
            offset: 0,
        }
    }

    #[frb(sync)]
    pub fn set_limit(&self, limit: i64) -> Pagination {
        Pagination {
            limit,
            offset: self.offset,
        }
    }

    #[frb(sync)]
    pub fn set_offset(&self, offset: i64) -> Pagination {
        Pagination {
            limit: self.limit,
            offset,
        }
    }
}
