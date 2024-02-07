use bigdecimal::ToPrimitive;
use diesel::ExpressionMethods;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use strum::IntoEnumIterator;

use crate::config::get_config;
use crate::config::NetworkName;

use crate::constants::VRF_REQUESTS;
use crate::error::MonitoringError;
use crate::models::VrfEntry;
// use crate::schema::mainnet_vrf_requests::dsl as mainnet_dsl;
use crate::schema::vrf_requests::dsl as testnet_dsl;
use crate::types::VrfStatus;

pub async fn process_vrf_data(
    pool: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
) -> Result<(), MonitoringError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| MonitoringError::Connection("Failed to get connection".to_string()))?;

    let config = get_config(None).await;

    let result: Result<VrfEntry, _> = match config.network().name {
        NetworkName::Testnet => testnet_dsl::vrf_requests..filter(testnet_dsl::_cursor::).first(&mut conn).await,
        NetworkName::Mainnet => {
            todo!("Implement mainnet vrf processing")
        }
    };

    match result {
        Ok(vrf_entry) => {
            log::info!(
                "Processing VRF data for request_id: {}",
                vrf_entry.request_id
            );
            let network_env = &config.network_str();

            let enum_status = vrf_entry
                .status
                .to_usize()
                .ok_or(MonitoringError::Conversion(
                    "Failed to convert status to enum".to_string(),
                ))?;
            let status = VrfStatus::iter()
                .nth(enum_status)
                .ok_or(MonitoringError::Vrf("Invalid status".to_string()))?;

            let vrf_request_label = VRF_REQUESTS.with_label_values(&[
                network_env,
                &status.to_string(),
                &vrf_entry.requestor_address,
            ]);

            let num_requests 

            vrf_request_label.set(num_requests);
        }
        Err(e) => {
            log::error!("Error processing VRF data: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
