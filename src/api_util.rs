use crate::api_type_wrappers::*;
use crate::table::{ToTableRows, ToTableTitle};
use cli_table::{Cell, CellStruct, Color, Style, Table, TableStruct};
use pi_hole_api::{
    api_types::*, errors::APIError, AuthenticatedPiHoleAPI, PiHoleAPIConfig,
    PiHoleAPIConfigWithKey, UnauthenticatedPiHoleAPI,
};
use serde::Serialize;

pub enum PiHoleConfigImplementation {
    Default(PiHoleAPIConfig),
    WithKey(PiHoleAPIConfigWithKey),
}

impl From<PiHoleAPIConfig> for PiHoleConfigImplementation {
    fn from(config: PiHoleAPIConfig) -> Self {
        PiHoleConfigImplementation::Default(config)
    }
}

impl From<PiHoleAPIConfigWithKey> for PiHoleConfigImplementation {
    fn from(config: PiHoleAPIConfigWithKey) -> Self {
        PiHoleConfigImplementation::WithKey(config)
    }
}

impl PiHoleConfigImplementation {
    pub fn new(host: String, api_key: Option<String>) -> Self {
        match api_key {
            Some(key) => PiHoleAPIConfigWithKey::new(host, key).into(),
            None => PiHoleAPIConfig::new(host).into(),
        }
    }

    pub fn get_unauthenticated_api(&self) -> &dyn UnauthenticatedPiHoleAPI {
        match self {
            Self::Default(config) => config,
            Self::WithKey(config) => config,
        }
    }

    pub fn get_authenticated_api(&self) -> Result<&dyn AuthenticatedPiHoleAPI, APIError> {
        match self {
            Self::Default(_) => Err(APIError::MissingAPIKey),
            Self::WithKey(config) => Ok(config),
        }
    }
}

/// Applies the function to each api which can provide an authenticated API
/// Unauthenticated APIs are replaced with `APIError::MissingAPIKey` errors
pub fn map_authenticated_apis<F, T>(
    apis: &[PiHoleConfigImplementation],
    func: F,
) -> Vec<Result<T, APIError>>
where
    F: Fn(&dyn AuthenticatedPiHoleAPI) -> Result<T, APIError>,
{
    apis.iter()
        .map(|api| api.get_authenticated_api().and_then(&func))
        .collect()
}

/// Applies the function to each api
pub fn map_unauthenticated_apis<F, T>(
    apis: &[PiHoleConfigImplementation],
    func: F,
) -> Vec<Result<T, APIError>>
where
    F: Fn(&dyn UnauthenticatedPiHoleAPI) -> Result<T, APIError>,
{
    apis.iter()
        .map(|api| api.get_unauthenticated_api())
        .map(&func)
        .collect()
}

#[derive(Debug, Serialize)]
pub enum APIResult {
    AllQueries(AllQueries),
    CacheInfo(CacheInfo),
    ClientName(ClientName),
    CustomCNAMERecord(CustomCNAMERecord),
    CustomDNSRecord(CustomDNSRecord),
    CustomListDomainDetails(CustomListDomainDetails),
    ForwardDestinations(ForwardDestinations),
    ListModificationResponse(ListModificationResponse),
    Network(Network),
    NetworkClient(NetworkClient),
    OverTimeData(OverTimeData),
    Query(Query),
    QueryTypes(QueryTypes),
    Status(Status),
    Summary(Summary),
    SummaryRaw(SummaryRaw),
    TopClients(TopClients),
    TopClientsBlocked(TopClientsBlocked),
    TopItems(TopItems),
    Version(Version),
    Versions(Versions),
    OverTimeDataClientsWrapper(OverTimeDataClientsWrapper),
    QueriesCountWrapper(QueriesCountWrapper),
    VersionWrapper(VersionWrapper),
    LogageWrapper(LogageWrapper),
}

impl ToTableRows for APIResult {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        match self {
            Self::AllQueries(data) => data.to_table_rows(host),
            Self::CacheInfo(data) => data.to_table_rows(host),
            Self::ClientName(data) => data.to_table_rows(host),
            Self::CustomCNAMERecord(data) => data.to_table_rows(host),
            Self::CustomDNSRecord(data) => data.to_table_rows(host),
            Self::CustomListDomainDetails(data) => data.to_table_rows(host),
            Self::ForwardDestinations(data) => data.to_table_rows(host),
            Self::ListModificationResponse(data) => data.to_table_rows(host),
            Self::Network(data) => data.to_table_rows(host),
            Self::NetworkClient(data) => data.to_table_rows(host),
            Self::OverTimeData(data) => data.to_table_rows(host),
            Self::Query(data) => data.to_table_rows(host),
            Self::QueryTypes(data) => data.to_table_rows(host),
            Self::Status(data) => data.to_table_rows(host),
            Self::Summary(data) => data.to_table_rows(host),
            Self::SummaryRaw(data) => data.to_table_rows(host),
            Self::TopClients(data) => data.to_table_rows(host),
            Self::TopClientsBlocked(data) => data.to_table_rows(host),
            Self::TopItems(data) => data.to_table_rows(host),
            Self::Version(data) => data.to_table_rows(host),
            Self::Versions(data) => data.to_table_rows(host),
            Self::OverTimeDataClientsWrapper(data) => data.to_table_rows(host),
            Self::QueriesCountWrapper(data) => data.to_table_rows(host),
            Self::VersionWrapper(data) => data.to_table_rows(host),
            Self::LogageWrapper(data) => data.to_table_rows(host),
        }
    }
}

impl ToTableTitle for APIResult {
    fn to_table_title() -> Vec<Vec<CellStruct>> {
        match self {
            Self::AllQueries => Self::AllQueries::to_table_title(),
            Self::CacheInfo => Self::CacheInfo::to_table_title(),
            Self::ClientName => Self::ClientName::to_table_title(),
            Self::CustomCNAMERecord => Self::CustomCNAMERecord::to_table_title(),
            Self::CustomDNSRecord => Self::CustomDNSRecord::to_table_title(),
            Self::CustomListDomainDetails => Self::CustomListDomainDetails::to_table_title(),
            Self::ForwardDestinations => Self::ForwardDestinations::to_table_title(),
            Self::ListModificationResponse => Self::ListModificationResponse::to_table_title(),
            Self::Network => Self::Network::to_table_title(),
            Self::NetworkClient => Self::NetworkClient::to_table_title(),
            Self::OverTimeData => Self::OverTimeData::to_table_title(),
            Self::Query => Self::Query::to_table_title(),
            Self::QueryTypes => Self::QueryTypes::to_table_title(),
            Self::Status => Self::Status::to_table_title(),
            Self::Summary => Self::Summary::to_table_title(),
            Self::SummaryRaw => Self::SummaryRaw::to_table_title(),
            Self::TopClients => Self::TopClients::to_table_title(),
            Self::TopClientsBlocked => Self::TopClientsBlocked::to_table_title(),
            Self::TopItems => Self::TopItems::to_table_title(),
            Self::Version => Self::Version::to_table_title(),
            Self::Versions => Self::Versions::to_table_title(),
            Self::OverTimeDataClientsWrapper => Self::OverTimeDataClientsWrapper::to_table_title(),
            Self::QueriesCountWrapper => Self::QueriesCountWrapper::to_table_title(),
            Self::VersionWrapper => Self::VersionWrapper::to_table_title(),
            Self::LogageWrapper => Self::LogageWrapper::to_table_title(),
        }
    }
}

impl From<AllQueries> for APIResult {
    fn from(data: AllQueries) -> Self {
        APIResult::AllQueries(data)
    }
}
impl From<CacheInfo> for APIResult {
    fn from(data: CacheInfo) -> Self {
        APIResult::CacheInfo(data)
    }
}
impl From<ClientName> for APIResult {
    fn from(data: ClientName) -> Self {
        APIResult::ClientName(data)
    }
}
impl From<CustomCNAMERecord> for APIResult {
    fn from(data: CustomCNAMERecord) -> Self {
        APIResult::CustomCNAMERecord(data)
    }
}
impl From<CustomDNSRecord> for APIResult {
    fn from(data: CustomDNSRecord) -> Self {
        APIResult::CustomDNSRecord(data)
    }
}
impl From<CustomListDomainDetails> for APIResult {
    fn from(data: CustomListDomainDetails) -> Self {
        APIResult::CustomListDomainDetails(data)
    }
}
impl From<ForwardDestinations> for APIResult {
    fn from(data: ForwardDestinations) -> Self {
        APIResult::ForwardDestinations(data)
    }
}
impl From<ListModificationResponse> for APIResult {
    fn from(data: ListModificationResponse) -> Self {
        APIResult::ListModificationResponse(data)
    }
}
impl From<Network> for APIResult {
    fn from(data: Network) -> Self {
        APIResult::Network(data)
    }
}
impl From<NetworkClient> for APIResult {
    fn from(data: NetworkClient) -> Self {
        APIResult::NetworkClient(data)
    }
}
impl From<OverTimeData> for APIResult {
    fn from(data: OverTimeData) -> Self {
        APIResult::OverTimeData(data)
    }
}
impl From<Query> for APIResult {
    fn from(data: Query) -> Self {
        APIResult::Query(data)
    }
}
impl From<QueryTypes> for APIResult {
    fn from(data: QueryTypes) -> Self {
        APIResult::QueryTypes(data)
    }
}
impl From<Status> for APIResult {
    fn from(data: Status) -> Self {
        APIResult::Status(data)
    }
}
impl From<Summary> for APIResult {
    fn from(data: Summary) -> Self {
        APIResult::Summary(data)
    }
}
impl From<SummaryRaw> for APIResult {
    fn from(data: SummaryRaw) -> Self {
        APIResult::SummaryRaw(data)
    }
}
impl From<TopClients> for APIResult {
    fn from(data: TopClients) -> Self {
        APIResult::TopClients(data)
    }
}
impl From<TopClientsBlocked> for APIResult {
    fn from(data: TopClientsBlocked) -> Self {
        APIResult::TopClientsBlocked(data)
    }
}
impl From<TopItems> for APIResult {
    fn from(data: TopItems) -> Self {
        APIResult::TopItems(data)
    }
}
impl From<Version> for APIResult {
    fn from(data: Version) -> Self {
        APIResult::Version(data)
    }
}
impl From<Versions> for APIResult {
    fn from(data: Versions) -> Self {
        APIResult::Versions(data)
    }
}
impl From<OverTimeDataClientsWrapper> for APIResult {
    fn from(data: OverTimeDataClientsWrapper) -> Self {
        APIResult::OverTimeDataClientsWrapper(data)
    }
}
impl From<QueriesCountWrapper> for APIResult {
    fn from(data: QueriesCountWrapper) -> Self {
        APIResult::QueriesCountWrapper(data)
    }
}
impl From<VersionWrapper> for APIResult {
    fn from(data: VersionWrapper) -> Self {
        APIResult::VersionWrapper(data)
    }
}
impl From<LogageWrapper> for APIResult {
    fn from(data: LogageWrapper) -> Self {
        APIResult::LogageWrapper(data)
    }
}

impl ToTableTitle for APIResult {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "count".cell()]
    }
}
