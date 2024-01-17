use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryInfo {
    pub queryId: u64,
    pub queryString: String,
    queryStatus: String,
    queryPlan: String,
    queryMetaData: String
}

//
#[derive(Serialize, Deserialize)]
struct QuerySubPlan {
    pub operator: String,
    #[serde(rename = "querySubPlanId")]
    pub query_sub_plan_id: i64,
}

#[derive(Serialize, Deserialize)]
struct ScheduledQuerySuplanList {
    #[serde(rename = "queryId")]
    pub query_id: i64,
    #[serde(rename = "querySubPlans")]
    pub query_sub_plans: Vec<QuerySubPlan>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionNode {
    #[serde(rename = "ScheduledQueries")]
    pub scheduled_queries: Vec<ScheduledQuerySuplanList>,
    #[serde(rename = "executionNodeId")]
    pub execution_node_id: u64,
    #[serde(rename = "topologyNodeId")]
    pub topology_node_id: u64,
    #[serde(rename = "topologyNodeIpAddress")]
    pub topology_node_ip_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionPlan {
    #[serde(rename = "executionNodes")]
    pub execution_nodes: Vec<ExecutionNode>,
}