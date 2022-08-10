use cli_table::{Cell, CellStruct, Color, Style, Table, TableStruct};
use pi_hole_api::api_types::*;

pub trait ToTableRows {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>>;
}

impl ToTableRows for AllQueries {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.data
            .into_iter()
            .flat_map(|query| query.to_table_rows(host))
            .collect()
    }
}

impl ToTableRows for CacheInfo {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.cache_size.cell(),
            self.cache_live_freed.cell(),
            self.cache_inserted.cell(),
        ]]
    }
}

impl ToTableRows for ClientName {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![host.cell(), self.name.cell(), self.ip.cell()]]
    }
}

impl ToTableRows for CustomCNAMERecord {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.domain.cell(),
            self.target_domain.cell(),
        ]]
    }
}

impl ToTableRows for CustomDNSRecord {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.domain.cell(),
            self.ip_address.cell(),
        ]]
    }
}

impl ToTableRows for CustomListDomainDetails {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.id.cell(),
            self.domain_type.cell(),
            self.domain.cell(),
            self.enabled.cell(),
            self.date_added.cell(),
            self.date_modified.cell(),
            self.comment.cell(),
            format!("{:?}", self.groups).cell(),
        ]]
    }
}

impl ToTableRows for ForwardDestinations {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.forward_destinations
            .into_iter()
            .map(|(ip, percentage)| vec![host.cell(), ip.cell(), percentage.cell()])
            .collect()
    }
}

impl ToTableRows for ListModificationResponse {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.success.cell(),
            self.message.unwrap_or("None".to_string()).cell(),
        ]]
    }
}

impl ToTableRows for Network {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.network
            .into_iter()
            .flat_map(|network_client| network_client.to_table_rows(host))
            .collect()
    }
}

impl ToTableRows for NetworkClient {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.id.cell(),
            format!("{:?}", self.ip).cell(),
            self.hwaddr.cell(),
            self.interface.cell(),
            format!("{:?}", self.name).cell(),
            self.first_seen.cell(),
            self.last_query.cell(),
            self.num_queries.cell(),
            self.mac_vendor.cell(),
        ]]
    }
}

impl ToTableRows for OverTimeData {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        let domains_over_time = self
            .domains_over_time
            .into_iter()
            .map(|(time, count)| vec![host.cell(), "all".cell(), time.cell(), count.cell()]);
        let ads_over_time = self.ads_over_time.into_iter().map(|(time, count)| {
            vec![
                host.cell(),
                "ads".cell().foreground_color(Some(Color::Red)),
                time.cell(),
                count.cell(),
            ]
        });

        domains_over_time.chain(ads_over_time).collect()
    }
}

impl ToTableRows for Query {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.timestring.cell(),
            format!("{:?}", self.query_type).cell(),
            self.domain.cell(),
            self.client.cell(),
            format!("{:?}", self.status).cell(),
            format!("{:?}", self.dnssec_status).cell(),
            format!("{:?}", self.reply_type).cell(),
            format!("{:?}", self.response_time).cell(),
            self.cname_domain.cell(),
            self.regex_id.cell(),
            self.upstream_destination.cell(),
            self.ede.cell(),
        ]]
    }
}

impl ToTableRows for QueryTypes {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.querytypes
            .into_iter()
            .map(|(query_type, percentage)| vec![host.cell(), query_type.cell(), percentage.cell()])
            .collect()
    }
}

impl ToTableRows for Status {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            string_status_to_colored_cell(&self.status),
        ]]
    }
}

impl ToTableRows for Summary {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.domains_being_blocked.cell(),
            self.dns_queries_today.cell(),
            self.ads_blocked_today.cell(),
            self.ads_percentage_today.cell(),
            self.unique_domains.cell(),
            self.queries_forwarded.cell(),
            self.queries_cached.cell(),
            self.clients_ever_seen.cell(),
            self.unique_clients.cell(),
            self.dns_queries_all_types.cell(),
            self.reply_nodata.cell(),
            self.reply_nxdomain.cell(),
            self.reply_cname.cell(),
            self.reply_ip.cell(),
            self.privacy_level.cell(),
            string_status_to_colored_cell(&self.status),
        ]]
    }
}

impl ToTableRows for SummaryRaw {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.domains_being_blocked.cell(),
            self.dns_queries_today.cell(),
            self.ads_blocked_today.cell(),
            self.ads_percentage_today.cell(),
            self.unique_domains.cell(),
            self.queries_forwarded.cell(),
            self.queries_cached.cell(),
            self.clients_ever_seen.cell(),
            self.unique_clients.cell(),
            self.dns_queries_all_types.cell(),
            self.reply_nodata.cell(),
            self.reply_nxdomain.cell(),
            self.reply_cname.cell(),
            self.reply_ip.cell(),
            self.privacy_level.cell(),
            string_status_to_colored_cell(&self.status),
        ]]
    }
}

impl ToTableRows for TopClients {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.top_sources
            .into_iter()
            .map(|(ip, count)| vec![host.cell(), ip.cell(), count.cell()])
            .collect()
    }
}

impl ToTableRows for TopClientsBlocked {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.top_sources_blocked
            .into_iter()
            .map(|(ip, count)| vec![host.cell(), ip.cell(), count.cell()])
            .collect()
    }
}

impl ToTableRows for TopItems {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        let top_queries = self.top_queries.into_iter().map(|(domain, count)| {
            vec![
                host.cell(),
                "ok".cell().foreground_color(Some(Color::Green)),
                domain.cell(),
                count.cell(),
            ]
        });
        let top_ads = self.top_ads.into_iter().map(|(domain, count)| {
            vec![
                host.cell(),
                "ad".cell().foreground_color(Some(Color::Red)),
                domain.cell(),
                count.cell(),
            ]
        });

        top_queries.chain(top_ads).collect()
    }
}

impl ToTableRows for Version {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![host.cell(), self.version.cell()]]
    }
}

impl ToTableRows for Versions {
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        vec![vec![
            host.cell(),
            self.core_update.cell(),
            self.web_update.cell(),
            self.ftl_update.cell(),
            self.core_current.cell(),
            self.web_current.cell(),
            self.ftl_current.cell(),
            self.core_latest.cell(),
            self.web_latest.cell(),
            self.ftl_latest.cell(),
            self.core_branch.cell(),
            self.web_branch.cell(),
            self.ftl_branch.cell(),
        ]]
    }
}

impl<T> ToTableRows for Vec<T>
where
    T: ToTableRows,
{
    fn to_table_rows(self, host: &str) -> Vec<Vec<CellStruct>> {
        self.into_iter()
            .flat_map(|item| item.to_table_rows(host))
            .collect()
    }
}

fn string_status_to_colored_cell(status: &str) -> CellStruct {
    let color = if status == "enabled" {
        Color::Green
    } else {
        Color::Red
    };
    status.cell().foreground_color(Some(color))
}

pub trait ToTableTitle {
    fn to_table_title() -> Vec<CellStruct>;
}

impl ToTableTitle for AllQueries {
    fn to_table_title() -> Vec<CellStruct> {
        Query::to_table_title()
    }
}

impl ToTableTitle for CacheInfo {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "cache_size".cell(),
            "cache_live_freed".cell(),
            "cache_inserted".cell(),
        ]
    }
}

impl ToTableTitle for ClientName {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "name".cell(), "ip".cell()]
    }
}

impl ToTableTitle for CustomCNAMERecord {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "domain".cell(), "target_domain".cell()]
    }
}

impl ToTableTitle for CustomDNSRecord {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "domain".cell(), "ip_address".cell()]
    }
}

impl ToTableTitle for CustomListDomainDetails {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "id".cell(),
            "domain_type".cell(),
            "domain".cell(),
            "enabled".cell(),
            "date_added".cell(),
            "date_modified".cell(),
            "comment".cell(),
            "groups".cell(),
        ]
    }
}

impl ToTableTitle for ForwardDestinations {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "Name/IP".cell(), "Percentage".cell()]
    }
}

impl ToTableTitle for ListModificationResponse {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "success".cell(), "message".cell()]
    }
}

impl ToTableTitle for Network {
    fn to_table_title() -> Vec<CellStruct> {
        NetworkClient::to_table_title()
    }
}

impl ToTableTitle for NetworkClient {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "id".cell(),
            "ip".cell(),
            "hwaddr".cell(),
            "interface".cell(),
            "name".cell(),
            "first_seen".cell(),
            "last_query".cell(),
            "num_queries".cell(),
            "mac_vendor".cell(),
        ]
    }
}

impl ToTableTitle for OverTimeData {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "type".cell(), "time".cell(), "count".cell()]
    }
}

impl ToTableTitle for Query {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "timestring".cell(),
            "query_type".cell(),
            "domain".cell(),
            "client".cell(),
            "status".cell(),
            "dnssec_status".cell(),
            "reply_type".cell(),
            "response_time".cell(),
            "cname_domain".cell(),
            "regex_id".cell(),
            "upstream_destination".cell(),
            "ede".cell(),
        ]
    }
}

impl ToTableTitle for QueryTypes {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "type".cell(), "percentage".cell()]
    }
}

impl ToTableTitle for Status {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "status".cell()]
    }
}

impl ToTableTitle for Summary {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "domains_being_blocked".cell(),
            "dns_queries_today".cell(),
            "ads_blocked_today".cell(),
            "ads_percentage_today".cell(),
            "unique_domains".cell(),
            "queries_forwarded".cell(),
            "queries_cached".cell(),
            "clients_ever_seen".cell(),
            "unique_clients".cell(),
            "dns_queries_all_types".cell(),
            "reply_nodata".cell(),
            "reply_nxdomain".cell(),
            "reply_cname".cell(),
            "reply_ip".cell(),
            "privacy_level".cell(),
            "status".cell(),
        ]
    }
}

impl ToTableTitle for SummaryRaw {
    fn to_table_title() -> Vec<CellStruct> {
        Summary::to_table_title()
    }
}

impl ToTableTitle for TopClients {
    fn to_table_title() -> Vec<CellStruct> {
        vec!["Host".cell(), "Hostname/IP".cell(), "count".cell()]
    }
}

impl ToTableTitle for TopClientsBlocked {
    fn to_table_title() -> Vec<CellStruct> {
        TopClients::to_table_title()
    }
}

impl ToTableTitle for TopItems {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "type".cell(),
            "domain".cell(),
            "count".cell(),
        ]
    }
}

impl ToTableTitle for Versions {
    fn to_table_title() -> Vec<CellStruct> {
        vec![
            "Host".cell(),
            "core_update".cell(),
            "web_update".cell(),
            "ftl_update".cell(),
            "core_current".cell(),
            "web_current".cell(),
            "ftl_current".cell(),
            "core_latest".cell(),
            "web_latest".cell(),
            "ftl_latest".cell(),
            "core_branch".cell(),
            "web_branch".cell(),
            "ftl_branch".cell(),
        ]
    }
}

impl<T> ToTableTitle for Vec<T>
where
    T: ToTableTitle,
{
    fn to_table_title() -> Vec<CellStruct> {
        T::to_table_title()
    }
}

pub trait ToTable {
    fn to_table(self) -> TableStruct;
}

impl<T> ToTable for Vec<(String, T)>
where
    T: ToTableRows + ToTableTitle,
{
    fn to_table(self) -> TableStruct {
        let rows: Vec<_> = self
            .into_iter()
            .flat_map(|(host, response_data)| response_data.to_table_rows(&host))
            .collect();
        rows.table().title(T::to_table_title())
    }
}
