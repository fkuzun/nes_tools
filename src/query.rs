use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryInfo {
    pub queryId: u64,
    pub queryString: String,
    queryStatus: String,
    queryPlan: String,
    queryMetaData: String
}