//! GAP commands and types needed for those commands.

extern crate byteorder;

pub use crate::host::{AdvertisingFilterPolicy, AdvertisingType, OwnAddressType};
use crate::types::extended_advertisement::{
    AdvSet, AdvertisingEvent, AdvertisingOperation, AdvertisingPhy, ExtendedAdvertisingInterval,
};
pub use crate::types::{ConnectionInterval, ExpectedConnectionLength, ScanWindow};
use crate::{
    host::{Channels, PeerAddrType, ScanFilterPolicy, ScanType},
    types::extended_advertisement::AdvertisingMode,
};
use crate::{AdvertisingHandle, ConnectionHandle, Controller};
pub use crate::{BdAddr, BdAddrType};
use byteorder::{ByteOrder, LittleEndian};
use core::time::Duration;

/// GAP-specific commands.
pub trait GapCommands {
    /// Set the device in non-discoverable mode. This command will disable the LL advertising and
    /// put the device in standby state.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetNonDiscoverable) event
    /// is generated.
    async fn gap_set_nondiscoverable(&mut self);

    /// Set the device in limited discoverable mode.
    ///
    /// Limited discoverability is defined in in GAP specification volume 3, section 9.2.3. The
    /// device will be discoverable for maximum period of TGAP (lim_adv_timeout) = 180 seconds (from
    /// errata). The advertising can be disabled at any time by issuing a
    /// [`set_nondiscoverable`](GapCommands::gap_set_nondiscoverable) command.
    ///
    /// # Errors
    ///
    /// - [`BadAdvertisingType`](Error::BadAdvertisingType) if
    ///   [`advertising_type`](DiscoverableParameters::advertising_type) is one of the disallowed
    ///   types:
    ///   [ConnectableDirectedHighDutyCycle](crate::host::AdvertisingType::ConnectableDirectedHighDutyCycle)
    ///   or
    ///   [ConnectableDirectedLowDutyCycle](crate::host::AdvertisingType::ConnectableDirectedLowDutyCycle).
    /// - [`BadAdvertisingInterval`](Error::BadAdvertisingInterval) if
    ///   [`advertising_interval`](DiscoverableParameters::advertising_interval) is inverted.
    ///   That is, if the min is greater than the max.
    /// - [`BadConnectionInterval`](Error::BadConnectionInterval) if
    ///   [`conn_interval`](DiscoverableParameters::conn_interval) is inverted. That is, both the
    ///   min and max are provided, and the min is greater than the max.
    ///
    /// # Generated evenst
    ///
    /// When the controller receives the command, it will generate a [command status](crate::event::Event::CommandStatus)
    /// event. The controller starts the advertising after this and when advertising timeout happens
    /// (i.e. limited discovery period has elapsed), the controller generates an
    /// [GAP Limited Discoverable Complete](crate::vendor::event::VendorEvent::GapLimitedDiscoverableTimeout) event.

    async fn set_limited_discoverable(
        &mut self,
        params: &DiscoverableParameters<'_, '_>,
    ) -> Result<(), Error>;

    /// Set the device in discoverable mode.
    ///
    /// Limited discoverability is defined in in GAP specification volume 3, section 9.2.4. The
    /// device will be discoverable for maximum period of TGAP (lim_adv_timeout) = 180 seconds (from
    /// errata). The advertising can be disabled at any time by issuing a
    /// [`set_nondiscoverable`](GapCommands::set_nondiscoverable) command.
    ///
    /// # Errors
    ///
    /// - [`BadAdvertisingType`](Error::BadAdvertisingType) if
    ///   [`advertising_type`](DiscoverableParameters::advertising_type) is one of the disallowed
    ///   types:
    ///   [ConnectableDirectedHighDutyCycle](crate::host::AdvertisingType::ConnectableDirectedHighDutyCycle)
    ///   or
    ///   [ConnectableDirectedLowDutyCycle](crate::host::AdvertisingType::ConnectableDirectedLowDutyCycle).
    /// - [`BadAdvertisingInterval`](Error::BadAdvertisingInterval) if
    ///   [`advertising_interval`](DiscoverableParameters::advertising_interval) is inverted.
    ///   That is, if the min is greater than the max.
    /// - [`BadConnectionInterval`](Error::BadConnectionInterval) if
    ///   [`conn_interval`](DiscoverableParameters::conn_interval) is inverted. That is, both the
    ///   min and max are provided, and the min is greater than the max.
    ///
    /// # Generated evenst
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetDiscoverable) event is
    /// generated.
    async fn set_discoverable(
        &mut self,
        params: &DiscoverableParameters<'_, '_>,
    ) -> Result<(), Error>;

    /// Set the device in direct connectable mode.
    ///
    /// Direct connectable mode is defined in GAP specification Volume 3,
    /// Section 9.3.3). Device uses direct connectable mode to advertise using either High Duty
    /// cycle advertisement events or Low Duty cycle advertisement events and the address as
    /// what is specified in the Own Address Type parameter. The Advertising Type parameter in
    /// the command specifies the type of the advertising used.
    ///
    /// When the `ms` feature is _not_ enabled, the device will be in directed connectable mode only
    /// for 1.28 seconds. If no connection is established within this duration, the device enters
    /// non discoverable mode and advertising will have to be again enabled explicitly.
    ///
    /// When the `ms` feature _is_ enabled, the advertising interval is explicitly provided in the
    /// [parameters][DirectConnectableParameters].
    ///
    /// # Errors
    ///
    /// - [`BadAdvertisingType`](Error::BadAdvertisingType) if
    ///   [`advertising_type`](DiscoverableParameters::advertising_type) is one of the disallowed
    ///   types:
    ///   [ConnectableUndirected](crate::host::AdvertisingType::ConnectableUndirected),
    ///   [ScannableUndirected](crate::host::AdvertisingType::ScannableUndirected), or
    ///   [NonConnectableUndirected](crate::host::AdvertisingType::NonConnectableUndirected),
    /// - (`ms` feature only) [`BadAdvertisingInterval`](Error::BadAdvertisingInterval) if
    ///   [`advertising_interval`](DiscoverableParameters::advertising_interval) is
    ///   out of range (20 ms to 10.24 s) or inverted (the min is greater than the max).
    ///
    /// # Generated evenst
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetDirectConnectable) event
    /// is generated.
    async fn set_direct_connectable(
        &mut self,
        params: &DirectConnectableParameters,
    ) -> Result<(), Error>;

    /// Set the IO capabilities of the device.
    ///
    /// This command has to be given only when the device is not in a connected state.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetIoCapability) event is
    /// generated.
    async fn set_io_capability(&mut self, capability: IoCapability);

    /// Set the authentication requirements for the device.
    ///
    /// This command has to be given only when the device is not in a connected state.
    ///
    /// # Errors
    ///
    /// - [BadEncryptionKeySizeRange](Error::BadEncryptionKeySizeRange) if the
    ///   [`encryption_key_size_range`](AuthenticationRequirements::encryption_key_size_range) min
    ///   is greater than the max.
    /// - [BadFixedPin](Error::BadFixedPin) if the
    ///   [`fixed_pin`](AuthenticationRequirements::fixed_pin) is [Fixed](Pin::Fixed) with a value
    ///   greater than 999999.
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// - A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetAuthenticationRequirement) event
    ///   is generated.
    /// - If [`fixed_pin`](AuthenticationRequirements::fixed_pin) is [Request](Pin::Requested), then
    ///   a [GAP Pass Key](crate::vendor::event::VendorEvent::GapPassKeyRequest) event is generated.
    async fn set_authentication_requirement(
        &mut self,
        requirements: &AuthenticationRequirements,
    ) -> Result<(), Error>;

    /// Set the authorization requirements of the device.
    ///
    /// This command has to be given when connected to a device if authorization is required to
    /// access services which require authorization.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// - A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetAuthorizationRequirement) event
    ///   is generated.
    /// - If authorization is required, then a [GAP Authorization Request](crate::vendor::event::VendorEvent::GapAuthorizationRequest)
    /// event is generated.
    async fn set_authorization_requirement(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        authorization_required: bool,
    );

    /// This command should be send by the host in response to the
    /// [GAP Pass Key Request](crate::vendor::event::VendorEvent::GapPassKeyRequest) event.
    ///
    /// `pin` contains the pass key which will be used during the pairing process.
    ///
    /// # Errors
    ///
    /// - [BadFixedPin](Error::BadFixedPin) if the pin is greater than 999999.
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// - A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapPassKeyResponse) event is
    ///   generated.
    /// - When the pairing process completes, it will generate a
    ///   [PairingComplete](crate::vendor::event::VendorEvent::GapPairingComplete) event.
    async fn pass_key_response(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        pin: u32,
    ) -> Result<(), Error>;

    /// This command should be send by the host in response to the
    /// [GAP Authorization Request](crate::vendor::event::VendorEvent::GapAuthorizationRequest) event.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapAuthorizationResponse)
    /// event is generated.
    async fn authorization_response(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        authorization: Authorization,
    );

    /// Register the GAP service with the GATT.
    ///
    /// The device name characteristic and appearance characteristic are added by default and the
    /// handles of these characteristics are returned in the
    /// [event data](crate::vendor::event::command::GapInit).
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapInit) event is generated.
    async fn init(&mut self, role: Role, privacy_enabled: bool, dev_name_characteristic_len: u8);

    /// Register the GAP service with the GATT.
    ///
    /// This function exists to prevent name conflicts with other Commands traits' init methods.
    async fn init_gap(
        &mut self,
        role: Role,
        privacy_enabled: bool,
        dev_name_characteristic_len: u8,
    ) {
        self.init(role, privacy_enabled, dev_name_characteristic_len)
            .await
    }

    /// Put the device into non-connectable mode.
    ///
    /// This mode does not support connection. The privacy setting done in the
    /// [`init`](GapCommands::init) command plays a role in deciding the valid
    /// parameters for this command. If privacy was not enabled, `address_type` may be
    /// [Public](AddressType::Public) or [Random](AddressType::Random).  If privacy was
    /// enabled, `address_type` may be [ResolvablePrivate](AddressType::ResolvablePrivate) or
    /// [NonResolvablePrivate](AddressType::NonResolvablePrivate).
    ///
    /// # Errors
    ///
    /// - [BadAdvertisingType](Error::BadAdvertisingType) if the advertising type is not one
    ///   of the supported modes. It must be
    ///   [ScannableUndirected](AdvertisingType::ScannableUndirected) or
    ///   [NonConnectableUndirected](AdvertisingType::NonConnectableUndirected).
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapInit) event is generated.
    async fn set_nonconnectable(
        &mut self,
        advertising_type: AdvertisingType,
        address_type: AddressType,
    ) -> Result<(), Error>;

    /// Put the device into undirected connectable mode.
    ///
    /// The privacy setting done in the [`init`](GapCommands::init) command plays a role
    /// in deciding the valid parameters for this command.
    ///
    /// # Errors
    ///
    /// - [BadAdvertisingFilterPolicy](Error::BadAdvertisingFilterPolicy) if the filter is
    ///   not one of the supported modes. It must be
    ///   [AllowConnectionAndScan](AdvertisingFilterPolicy::AllowConnectionAndScan) or
    ///   [WhiteListConnectionAllowScan](AdvertisingFilterPolicy::WhiteListConnectionAllowScan).
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetUndirectedConnectable)
    /// event is generated.
    async fn set_undirected_connectable(
        &mut self,
        params: &UndirectedConnectableParameters,
    ) -> Result<(), Error>;

    /// This command has to be issued to notify the central device of the security requirements of
    /// the peripheral.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command status](crate::event::Event::CommandStatus) event will be generated when a valid
    /// command is received. On completion of the command, i.e. when the security request is
    /// successfully transmitted to the master, a
    /// [GAP Peripheral Security Initiated](crate::vendor::event::VendorEvent::GapPeripheralSecurityInitiated)
    /// vendor-specific event will be generated.
    async fn peripheral_security_request(&mut self, conn_handle: &ConnectionHandle);

    /// This command can be used to update the advertising data for a particular AD type. If the AD
    /// type specified does not exist, then it is added to the advertising data. If the overall
    /// advertising data length is more than 31 octets after the update, then the command is
    /// rejected and the old data is retained.
    ///
    /// # Errors
    ///
    /// - [BadAdvertisingDataLength](Error::BadAdvertisingDataLength) if the provided data is longer
    ///   than 31 bytes.
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapUpdateAdvertisingData)
    /// event is generated.
    async fn update_advertising_data(&mut self, data: &[u8]) -> Result<(), Error>;

    /// This command can be used to delete the specified AD type from the advertisement data if
    /// present.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapDeleteAdType) event is
    /// generated.
    async fn delete_ad_type(&mut self, ad_type: AdvertisingDataType);

    /// This command can be used to get the current security settings of the device.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapGetSecurityLevel) event is
    /// generated.
    async fn get_security_level(&mut self, conn_handle: &ConnectionHandle);

    /// Allows masking events from the GAP.
    ///
    /// The default configuration is all the events masked.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapSetEventMask) event is
    /// generated.
    async fn set_event_mask(&mut self, flags: EventFlags);

    /// Allows masking events from the GAP.
    ///
    /// This function exists to prevent name conflicts with other Commands traits' set_event_mask
    /// methods.
    async fn set_gap_event_mask(&mut self, flags: EventFlags) {
        self.set_event_mask(flags).await
    }

    /// Configure the controller's white list with devices that are present in the security
    /// database.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapConfigureWhiteList) event
    /// is generated.
    async fn configure_white_list(&mut self);

    /// Command the controller to terminate the connection.
    ///
    /// # Errors
    ///
    /// - [BadTerminationReason](Error::BadTerminationReason) if provided termination reason is
    ///   invalid. Valid reasons are the same as HCI [disconnect](crate::host::HostHci::disconnect):
    ///   [`AuthFailure`](crate::Status::AuthFailure),
    ///   [`RemoteTerminationByUser`](crate::Status::RemoteTerminationByUser),
    ///   [`RemoteTerminationLowResources`](crate::Status::RemoteTerminationLowResources),
    ///   [`RemoteTerminationPowerOff`](crate::Status::RemoteTerminationPowerOff),
    ///   [`UnsupportedRemoteFeature`](crate::Status::UnsupportedRemoteFeature),
    ///   [`PairingWithUnitKeyNotSupported`](crate::Status::PairingWithUnitKeyNotSupported), or
    ///   [`UnacceptableConnectionParameters`](crate::Status::UnacceptableConnectionParameters).
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command status](crate::event::Event::CommandStatus) event when
    /// the command is received and a [Disconnection Complete](crate::event::Event::DisconnectionComplete)
    /// event will be generated when the link is
    /// disconnected.
    async fn terminate(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        reason: crate::Status,
    ) -> Result<(), Error>;

    /// Clear the bonding table. All the devices in the bonding table are removed.
    ///
    /// See also [remove_bonded_device](GapCommands::remove_bonded_device) to remove only one device.
    ///
    /// # Note
    /// As a fallback mode, in case the bonding table is full, the BLE stack automatically clears the bonding
    /// table just before putting into it information about a new bonded device.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapClearSecurityDatabase)
    /// event is generated.
    async fn clear_security_database(&mut self);

    /// This command should be given by the application when it receives the
    /// [GAP Bond Lost](crate::vendor::event::VendorEvent::GapBondLost) event if it wants the re-bonding to happen
    /// successfully. If this command is not given on receiving the event, the bonding procedure
    /// will timeout.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [Command Complete](crate::vendor::event::command::VendorReturnParameters::GapAllowRebond) event is
    /// generated. Even if the command is given when it is not valid, success will be returned but
    /// internally it will have no effect.
    async fn allow_rebond(&mut self, conn_handle: crate::ConnectionHandle);

    /// Start the limited discovery procedure.
    ///
    /// The controller is commanded to start active scanning.  When this procedure is started, only
    /// the devices in limited discoverable mode are returned to the upper layers.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command status](crate::event::Event::CommandStatus) event is generated as soon as the
    /// command is given.
    ///
    /// If [Success](crate::Status::Success) is returned in the command status, the procedure is
    /// terminated when either the upper layers issue a command to terminate the procedure by
    /// issuing the command [`terminate_procedure`](GapCommands::terminate_gap_procedure) with the
    /// procedure code set to [LimitedDiscovery](crate::vendor::event::GapProcedure::LimitedDiscovery) or a
    /// [timeout](crate::vendor::event::VendorEvent::GapLimitedDiscoverableTimeout) happens. When the
    /// procedure is terminated due to any of the above reasons, a
    /// [ProcedureComplete](crate::vendor::event::VendorEvent::GapProcedureComplete) event is returned with
    /// the procedure code set to [LimitedDiscovery](crate::vendor::event::GapProcedure::LimitedDiscovery).
    ///
    /// The device found when the procedure is ongoing is returned to the upper layers through the
    /// [LeAdvertisingReport](crate::event::Event::LeAdvertisingReport) event.
    async fn start_limited_discovery_procedure(&mut self, params: &DiscoveryProcedureParameters);

    /// Start the general discovery procedure. The controller is commanded to start active scanning.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command status](crate::event::Event::CommandStatus) event is generated as soon as the
    /// command is given.
    ///
    /// If [Success](crate::Status::Success) is returned in the command status, the procedure is
    /// terminated when either the upper layers issue a command to terminate the procedure by
    /// issuing the command [`terminate_procedure`](GapCommands::terminate_gap_procedure) with the
    /// procedure code set to [GeneralDiscovery](crate::vendor::event::GapProcedure::GeneralDiscovery) or a
    /// timeout happens. When the procedure is terminated due to any of the above reasons, a
    /// [ProcedureComplete](crate::vendor::event::VendorEvent::GapProcedureComplete) event is returned with
    /// the procedure code set to [GeneralDiscovery](crate::vendor::event::GapProcedure::GeneralDiscovery).
    ///
    /// The device found when the procedure is ongoing is returned to the upper layers through the
    /// [LeAdvertisingReport](crate::event::Event::LeAdvertisingReport) event.
    async fn start_general_discovery_procedure(&mut self, params: &DiscoveryProcedureParameters);

    /// Start the auto connection establishment procedure.
    ///
    /// The devices specified are added to the white list of the controller and a
    /// [`le_create_connection`](crate::host::crate::le_create_connection) call will be made to the
    /// controller by GAP with the [initiator filter policy](crate::host::ConnectionParameters::initiator_filter_policy) set to
    /// [WhiteList](crate::host::ConnectionFilterPolicy::WhiteList), to "use whitelist to determine
    /// which advertiser to connect to". When a command is issued to terminate the procedure by
    /// upper layer, a [`le_create_connection_cancel`](crate::host::crate::le_create_connection_cancel)
    /// call will be made to the controller by GAP.
    ///
    /// # Errors
    ///
    /// - If the [`white_list`](AutoConnectionEstablishmentParameters::white_list) is too long
    ///   (such that the serialized command would not fit in 255 bytes), a
    ///   [WhiteListTooLong](Error::WhiteListTooLong) is returned. The list cannot have more than 33
    ///   elements.
    async fn start_auto_connection_establishment_procedure(
        &mut self,
        params: &AutoConnectionEstablishmentParameters<'_>,
    ) -> Result<(), Error>;

    /// Start a general connection establishment procedure.
    ///
    /// The host [enables scanning](crate::host::crate::le_set_scan_enable) in the controller with the
    /// scanner [filter policy](crate::host::ScanParameters::filter_policy) set to
    /// [AcceptAll](crate::host::ScanFilterPolicy::AcceptAll), to "accept all advertising packets" and
    /// from the scanning results, all the devices are sent to the upper layer using the event
    /// [LE Advertising Report](crate::event::Event::LeAdvertisingReport). The upper layer then has to
    /// select one of the devices to which it wants to connect by issuing the command
    /// [`create_connection`](GapCommands::create_connection). If privacy is enabled,
    /// then either a private resolvable address or a non-resolvable address, based on the address
    /// type specified in the command is set as the scanner address but the GAP create connection
    /// always uses a private resolvable address if the general connection establishment procedure
    /// is active.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    async fn start_general_connection_establishment_procedure(
        &mut self,
        params: &GeneralConnectionEstablishmentParameters,
    );

    /// Start a selective connection establishment procedure.
    ///
    /// The GAP adds the specified device addresses into white list and
    /// [enables scanning](crate::host::HostHci::le_set_scan_enable) in the controller with the scanner
    /// [filter policy](crate::host::ScanParameters::filter_policy) set to
    /// [WhiteList](crate::host::ScanFilterPolicy::WhiteList), to "accept packets only from devices in
    /// whitelist". All the devices found are sent to the upper layer by the event
    /// [LE Advertising Report](crate::event::Event::LeAdvertisingReport). The upper layer then has to select one of
    /// the devices to which it wants to connect by issuing the command
    /// [`create_connection`](GapCommands::create_connection).
    ///
    /// # Errors
    ///
    /// - If the [`white_list`](SelectiveConnectionEstablishmentParameters::white_list) is too
    ///   long (such that the serialized command would not fit in 255 bytes), a
    ///   [WhiteListTooLong](Error::WhiteListTooLong) is returned. The list cannot have more than 35
    ///   elements.
    async fn start_selective_connection_establishment_procedure(
        &mut self,
        params: &SelectiveConnectionEstablishmentParameters<'_>,
    ) -> Result<(), Error>;

    /// Start the direct connection establishment procedure.
    ///
    /// A [LE Create Connection](crate::host::crate::le_create_connection) call will be made to the
    /// controller by GAP with the initiator [filter policy](crate::host::ConnectionParameters::initiator_filter_policy) set to
    /// [UseAddress](crate::host::ConnectionFilterPolicy::UseAddress) to "ignore whitelist and process
    /// connectable advertising packets only for the specified device". The procedure can be
    /// terminated explicitly by the upper layer by issuing the command
    /// [`terminate_procedure`](GapCommands::terminate_gap_procedure). When a command is
    /// issued to terminate the procedure by upper layer, a
    /// [`le_create_connection_cancel`](crate::host::HostHci::le_create_connection_cancel) call will be
    /// made to the controller by GAP.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command status](crate::event::Event::CommandStatus) event is generated as soon as the
    /// command is given. If [Success](crate::Status::Success) is returned, on termination of the
    /// procedure, a [LE Connection Complete](crate::event::LeConnectionComplete) event is
    /// returned. The procedure can be explicitly terminated by the upper layer by issuing the
    /// command [`terminate_procedure`](GapCommands::terminate_gap_procedure) with the procedure_code set
    /// to
    /// [DirectConnectionEstablishment](crate::vendor::event::GapProcedure::DirectConnectionEstablishment).
    async fn create_connection(&mut self, params: &ConnectionParameters);

    /// The GAP procedure(s) specified is terminated.
    ///
    /// # Errors
    ///
    /// - [NoProcedure](Error::NoProcedure) if the bitfield is empty.
    /// - Underlying communication errors
    ///
    /// # Generated events
    ///
    /// A [command complete](crate::vendor::event::command::VendorReturnParameters::GapTerminateProcedure) event
    /// is generated for this command. If the command was successfully processed, the status field
    /// will be [Success](crate::Status::Success) and a
    /// [ProcedureCompleted](crate::vendor::event::VendorEvent::GapProcedureComplete) event is returned
    /// with the procedure code set to the corresponding procedure.
    async fn terminate_gap_procedure(&mut self, procedure: Procedure) -> Result<(), Error>;

    /// Start the connection update procedure.
    ///
    /// A [`le_connection_update`](crate::host::HostHci::le_connection_update) call is be made to the
    /// controller by GAP.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command status](crate::event::Event::CommandStatus) event is generated as soon as the
    /// command is given. If [Success](crate::Status::Success) is returned, on completion of
    /// connection update, a
    /// [LeConnectionUpdateComplete](crate::event::Event::LeConnectionUpdateComplete) event is
    /// returned to the upper layer.
    async fn start_connection_update(&mut self, params: &ConnectionUpdateParameters);

    /// Send the SM pairing request to start a pairing process. The authentication requirements and
    /// I/O capabilities should be set before issuing this command using the
    /// [`set_io_capability`](GapCommands::set_io_capability) and
    /// [`set_authentication_requirement`](GapCommands::set_authentication_requirement)
    /// commands.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command status](crate::event::Event::CommandStatus) event is generated when the command is
    /// received. If [Success](crate::Status::Success) is returned in the command status event, a
    /// [Pairing Complete](crate::vendor::event::VendorEvent::GapPairingComplete) event is returned after
    /// the pairing process is completed.
    async fn send_pairing_request(&mut self, params: &PairingRequest);

    /// This command tries to resolve the address provided with the IRKs present in its database.
    ///
    /// If the address is resolved successfully with any one of the IRKs present in the database, it
    /// returns success and also the corresponding public/static random address stored with the IRK
    /// in the database.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command complete](crate::vendor::event::command::VendorReturnParameters::GapResolvePrivateAddress)
    /// event is generated. If [Success](crate::Status::Success) is returned as the status, then the
    /// address is also returned in the event.
    async fn resolve_private_address(&mut self, addr: crate::BdAddr);

    /// This command puts the device into broadcast mode.
    ///
    /// # Errors
    ///
    /// - [BadAdvertisingType](Error::BadAdvertisingType) if the advertising type is not
    ///   [ScannableUndirected](crate::types::AdvertisingType::ScannableUndirected) or
    ///   [NonConnectableUndirected](crate::types::AdvertisingType::NonConnectableUndirected).
    /// - [BadAdvertisingDataLength](Error::BadAdvertisingDataLength) if the advertising data is
    ///   longer than 31 bytes.
    /// - [WhiteListTooLong](Error::WhiteListTooLong) if the length of the white list would put the
    ///   packet length over 255 bytes. The exact number of addresses that can be in the white list
    ///   can range from 35 to 31, depending on the length of the advertising data.
    /// - Underlying communication errors.
    ///
    /// # Generated events
    ///
    /// A [command complete](crate::vendor::event::command::VendorReturnParameters::GapSetBroadcastMode) event is
    /// returned where the status indicates whether the command was successful.
    async fn set_broadcast_mode(&mut self, params: &BroadcastModeParameters) -> Result<(), Error>;

    /// Starts an Observation procedure, when the device is in Observer Role.
    ///
    /// The host enables scanning in the controller. The advertising reports are sent to the upper
    /// layer using standard LE Advertising Report Event. See Bluetooth Core v4.1, Vol. 2, part E,
    /// Ch. 7.7.65.2, LE Advertising Report Event.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command complete](crate::vendor::event::command::VendorReturnParameters::GapStartObservationProcedure)
    /// event is generated.
    async fn start_observation_procedure(&mut self, params: &ObservationProcedureParameters);

    /// This command gets the list of the devices which are bonded. It returns the number of
    /// addresses and the corresponding address types and values.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command complete](crate::vendor::event::command::VendorReturnParameters::GapGetBondedDevices) event is
    /// generated.
    async fn get_bonded_devices(&mut self);

    /// The command finds whether the device, whose address is specified in the command, is
    /// bonded. If the device is using a resolvable private address and it has been bonded, then the
    /// command will return [Success](crate::Status::Success).
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// A [command complete](crate::vendor::event::command::VendorReturnParameters::GapIsDeviceBonded) event is
    /// generated.
    async fn is_device_bonded(&mut self, addr: crate::host::PeerAddrType);

    /// This command allows the user to validate/confirm or not the numeric comparison value showed through
    /// the [`NumericComparisonValueEvent`]
    async fn numeric_comparison_value_confirm_yes_no(
        &mut self,
        params: &NumericComparisonValueConfirmYesNoParameters,
    );

    /// This command permits to signal to the Stack the input type detected during Passkey input.
    async fn passkey_input(&mut self, conn_handle: ConnectionHandle, input_type: InputType);

    /// This command is sent by the user to get (i.e. to extract from the Stack) the OOB
    /// data generated by the Stack itself.
    async fn get_oob_data(&mut self, oob_data_type: OobDataType);

    /// This command is sent (by the User) to input the OOB data arrived via OOB
    /// communication.
    async fn set_oob_data(&mut self, params: &SetOobDataParameters);

    /// This  command is used to add devices to the list of address translations
    /// used to resolve Resolvable Private Addresses in the Controller.
    async fn add_devices_to_resolving_list(
        &mut self,
        whitelist_identities: &[PeerAddrType],
        clear_resolving_list: bool,
    );

    /// This command is used to remove a specified device from bonding table
    async fn remove_bonded_device(&mut self, address: BdAddrType);

    /// This  command is used to add specific device addresses to the white and/or resolving list.
    async fn add_devices_to_list(&mut self, list_entries: &[BdAddrType], mode: AddDeviceToListMode);

    /// This command starts an advertising beacon. It allows additional advertising
    /// packets to be transmitted independently of the packets transmitted with GAP
    /// advertising commands such as ACI_GAP_SET_DISCOVERABLE or
    /// ACI_GAP_SET_LIMITED_DISCOVERABLE.
    async fn additional_beacon_start(
        &mut self,
        params: &AdditonalBeaconStartParameters,
    ) -> Result<(), Error>;

    /// This command stops the advertising beacon started with
    /// ACI_GAP_ADDITIONAL_BEACON_START.
    async fn additional_beacon_stop(&mut self);

    /// This command sets the data transmitted by the advertising beacon started
    /// with ACI_GAP_ADDITIONAL_BEACON_START. If the advertising beacon is already
    /// started, the new data is used in subsequent beacon advertising events.
    async fn additonal_beacon_set_data(&mut self, advertising_data: &[u8]);

    /// This command is used to set the extended advertising configuration for one
    /// advertising set.
    ///
    /// This command, in association with
    /// [adv_set_scan_response_data](GapCommands::adv_set_scan_response_data),
    /// [adv_set_advertising_data](GapCommands::adv_set_advertising_data) and
    /// [adv_set_enable](GapCommands::adv_set_enable), enables to start extended
    /// advertising.
    ///
    /// These commands must be used in replacement of
    /// [set_discoverable](GapCommands::set_discoverable),
    /// [set_limited_discoverable](GapCommands::set_limited_discoverable),
    /// [set_direct_connectable](GapCommands::set_direct_connectable),
    /// [set_nonconnectable](GapCommands::set_nonconnectable),
    /// [set_undirected_connectable](GapCommands::set_undirected_connectable) and
    /// [set_broadcast_mode](GapCommands::set_broadcast_mode) that only support
    /// legacy advertising.
    async fn adv_set_config(&mut self, params: &AdvSetConfig);

    /// This command is used to request the Controller to enable or disbale one
    /// or more extended advertising sets.
    async fn adv_set_enable<'a>(&mut self, params: &AdvSetEnable<'a>);

    /// This command is used to set the data used in extended advertising PDUs
    /// that have a data field
    async fn adv_set_advertising_data(&mut self, params: &AdvSetAdvertisingData);

    /// This command is used to provide scan response data used during extended
    /// advertising
    async fn adv_set_scan_response_data(&mut self, params: &AdvSetAdvertisingData);

    /// This command is used to remove an advertising set from the Controller.
    async fn adv_remove_set(&mut self, handle: AdvertisingHandle);

    /// This command is used to remove all exisiting advertising sets from
    /// the Controller.
    async fn adv_clear_sets(&mut self);

    /// This command is used to set the random device address of an advertising
    /// set configured to use specific random address.
    async fn adv_set_random_address(&mut self, handle: AdvertisingHandle, addr: BdAddr);
}

impl<T: Controller> GapCommands for T {
    async fn gap_set_nondiscoverable(&mut self) {
        self.controller_write(crate::vendor::opcode::GAP_SET_NONDISCOVERABLE, &[])
            .await
    }

    impl_validate_variable_length_params!(
        set_limited_discoverable<'a, 'b>,
        DiscoverableParameters<'a, 'b>,
        crate::vendor::opcode::GAP_SET_LIMITED_DISCOVERABLE
    );

    impl_validate_variable_length_params!(
        set_discoverable<'a, 'b>,
        DiscoverableParameters<'a, 'b>,
        crate::vendor::opcode::GAP_SET_DISCOVERABLE
    );

    impl_validate_params!(
        set_direct_connectable,
        DirectConnectableParameters,
        crate::vendor::opcode::GAP_SET_DIRECT_CONNECTABLE
    );

    async fn set_io_capability(&mut self, capability: IoCapability) {
        self.controller_write(
            crate::vendor::opcode::GAP_SET_IO_CAPABILITY,
            &[capability as u8],
        )
        .await
    }

    impl_validate_params!(
        set_authentication_requirement,
        AuthenticationRequirements,
        crate::vendor::opcode::GAP_SET_AUTHENTICATION_REQUIREMENT
    );

    async fn set_authorization_requirement(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        authorization_required: bool,
    ) {
        let mut bytes = [0; 3];
        LittleEndian::write_u16(&mut bytes[0..2], conn_handle.0);
        bytes[2] = authorization_required as u8;

        self.controller_write(
            crate::vendor::opcode::GAP_SET_AUTHORIZATION_REQUIREMENT,
            &bytes,
        )
        .await
    }

    async fn pass_key_response(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        pin: u32,
    ) -> Result<(), Error> {
        if pin > 999_999 {
            return Err(Error::BadFixedPin(pin));
        }

        let mut bytes = [0; 6];
        LittleEndian::write_u16(&mut bytes[0..2], conn_handle.0);
        LittleEndian::write_u32(&mut bytes[2..6], pin);

        self.controller_write(crate::vendor::opcode::GAP_PASS_KEY_RESPONSE, &bytes)
            .await;

        Ok(())
    }

    async fn authorization_response(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        authorization: Authorization,
    ) {
        let mut bytes = [0; 3];
        LittleEndian::write_u16(&mut bytes[0..2], conn_handle.0);
        bytes[2] = authorization as u8;

        self.controller_write(crate::vendor::opcode::GAP_AUTHORIZATION_RESPONSE, &bytes)
            .await
    }

    async fn init(&mut self, role: Role, privacy_enabled: bool, dev_name_characteristic_len: u8) {
        let mut bytes = [0; 3];
        bytes[0] = role.bits();
        bytes[1] = privacy_enabled as u8;
        bytes[2] = dev_name_characteristic_len;

        self.controller_write(crate::vendor::opcode::GAP_INIT, &bytes)
            .await;
    }

    async fn set_nonconnectable(
        &mut self,
        advertising_type: AdvertisingType,
        address_type: AddressType,
    ) -> Result<(), Error> {
        match advertising_type {
            AdvertisingType::ScannableUndirected | AdvertisingType::NonConnectableUndirected => (),
            _ => {
                return Err(Error::BadAdvertisingType(advertising_type));
            }
        }

        self.controller_write(
            crate::vendor::opcode::GAP_SET_NONCONNECTABLE,
            &[advertising_type as u8, address_type as u8],
        )
        .await;

        Ok(())
    }

    impl_validate_params!(
        set_undirected_connectable,
        UndirectedConnectableParameters,
        crate::vendor::opcode::GAP_SET_UNDIRECTED_CONNECTABLE
    );

    async fn peripheral_security_request(&mut self, conn_handle: &ConnectionHandle) {
        let mut bytes = [0; 2];

        LittleEndian::write_u16(&mut bytes[0..2], conn_handle.0);

        self.controller_write(
            crate::vendor::opcode::GAP_PERIPHERAL_SECURITY_REQUEST,
            &bytes,
        )
        .await
    }

    async fn update_advertising_data(&mut self, data: &[u8]) -> Result<(), Error> {
        const MAX_LENGTH: usize = 31;
        if data.len() > MAX_LENGTH {
            return Err(Error::BadAdvertisingDataLength(data.len()));
        }

        let mut bytes = [0; 1 + MAX_LENGTH];
        bytes[0] = data.len() as u8;
        bytes[1..=data.len()].copy_from_slice(data);

        self.controller_write(
            crate::vendor::opcode::GAP_UPDATE_ADVERTISING_DATA,
            &bytes[0..=data.len()],
        )
        .await;

        Ok(())
    }

    async fn delete_ad_type(&mut self, ad_type: AdvertisingDataType) {
        self.controller_write(crate::vendor::opcode::GAP_DELETE_AD_TYPE, &[ad_type as u8])
            .await
    }

    async fn get_security_level(&mut self, conn_handle: &ConnectionHandle) {
        let mut bytes = [0; 2];

        LittleEndian::write_u16(&mut bytes, conn_handle.0);

        self.controller_write(crate::vendor::opcode::GAP_GET_SECURITY_LEVEL, &bytes)
            .await
    }

    async fn set_event_mask(&mut self, flags: EventFlags) {
        let mut bytes = [0; 2];
        LittleEndian::write_u16(&mut bytes, flags.bits());

        self.controller_write(crate::vendor::opcode::GAP_SET_EVENT_MASK, &bytes)
            .await
    }

    async fn configure_white_list(&mut self) {
        self.controller_write(crate::vendor::opcode::GAP_CONFIGURE_WHITE_LIST, &[])
            .await
    }

    async fn terminate(
        &mut self,
        conn_handle: crate::ConnectionHandle,
        reason: crate::Status,
    ) -> Result<(), Error> {
        match reason {
            crate::Status::AuthFailure
            | crate::Status::RemoteTerminationByUser
            | crate::Status::RemoteTerminationLowResources
            | crate::Status::RemoteTerminationPowerOff
            | crate::Status::UnsupportedRemoteFeature
            | crate::Status::PairingWithUnitKeyNotSupported
            | crate::Status::UnacceptableConnectionParameters => (),
            _ => return Err(Error::BadTerminationReason(reason)),
        }

        let mut bytes = [0; 3];
        LittleEndian::write_u16(&mut bytes[0..2], conn_handle.0);
        bytes[2] = reason.into();

        self.controller_write(crate::vendor::opcode::GAP_TERMINATE, &bytes)
            .await;
        Ok(())
    }

    async fn clear_security_database(&mut self) {
        self.controller_write(crate::vendor::opcode::GAP_CLEAR_SECURITY_DATABASE, &[])
            .await
    }

    async fn allow_rebond(&mut self, conn_handle: crate::ConnectionHandle) {
        let mut bytes = [0; 2];
        LittleEndian::write_u16(&mut bytes, conn_handle.0);
        self.controller_write(crate::vendor::opcode::GAP_ALLOW_REBOND, &bytes)
            .await
    }

    impl_params!(
        start_limited_discovery_procedure,
        DiscoveryProcedureParameters,
        crate::vendor::opcode::GAP_START_LIMITED_DISCOVERY_PROCEDURE
    );

    impl_params!(
        start_general_discovery_procedure,
        DiscoveryProcedureParameters,
        crate::vendor::opcode::GAP_START_GENERAL_DISCOVERY_PROCEDURE
    );

    impl_validate_variable_length_params!(
        start_auto_connection_establishment_procedure<'a>,
        AutoConnectionEstablishmentParameters<'a>,
        crate::vendor::opcode::GAP_START_AUTO_CONNECTION_ESTABLISHMENT
    );

    impl_params!(
        start_general_connection_establishment_procedure,
        GeneralConnectionEstablishmentParameters,
        crate::vendor::opcode::GAP_START_GENERAL_CONNECTION_ESTABLISHMENT
    );

    impl_validate_variable_length_params!(
        start_selective_connection_establishment_procedure<'a>,
        SelectiveConnectionEstablishmentParameters<'a>,
        crate::vendor::opcode::GAP_START_SELECTIVE_CONNECTION_ESTABLISHMENT
    );
    impl_params!(
        create_connection,
        ConnectionParameters,
        crate::vendor::opcode::GAP_CREATE_CONNECTION
    );

    async fn terminate_gap_procedure(&mut self, procedure: Procedure) -> Result<(), Error> {
        if procedure.is_empty() {
            return Err(Error::NoProcedure);
        }

        self.controller_write(
            crate::vendor::opcode::GAP_TERMINATE_PROCEDURE,
            &[procedure.bits()],
        )
        .await;

        Ok(())
    }

    impl_params!(
        start_connection_update,
        ConnectionUpdateParameters,
        crate::vendor::opcode::GAP_START_CONNECTION_UPDATE
    );

    impl_params!(
        send_pairing_request,
        PairingRequest,
        crate::vendor::opcode::GAP_SEND_PAIRING_REQUEST
    );

    async fn resolve_private_address(&mut self, addr: crate::BdAddr) {
        self.controller_write(crate::vendor::opcode::GAP_RESOLVE_PRIVATE_ADDRESS, &addr.0)
            .await
    }

    impl_validate_variable_length_params!(
        set_broadcast_mode<'a, 'b>,
        BroadcastModeParameters<'a, 'b>,
        crate::vendor::opcode::GAP_SET_BROADCAST_MODE
    );

    impl_params!(
        start_observation_procedure,
        ObservationProcedureParameters,
        crate::vendor::opcode::GAP_START_OBSERVATION_PROCEDURE
    );

    async fn get_bonded_devices(&mut self) {
        self.controller_write(crate::vendor::opcode::GAP_GET_BONDED_DEVICES, &[])
            .await
    }

    async fn is_device_bonded(&mut self, addr: crate::host::PeerAddrType) {
        let mut bytes = [0; 7];
        addr.copy_into_slice(&mut bytes);

        self.controller_write(crate::vendor::opcode::GAP_IS_DEVICE_BONDED, &bytes)
            .await
    }

    impl_params!(
        numeric_comparison_value_confirm_yes_no,
        NumericComparisonValueConfirmYesNoParameters,
        crate::vendor::opcode::GAP_NUMERIC_COMPARISON_VALUE_YES_NO
    );

    async fn passkey_input(&mut self, conn_handle: ConnectionHandle, input_type: InputType) {
        let mut bytes = [0; 3];

        LittleEndian::write_u16(&mut bytes[..2], conn_handle.0);
        bytes[2] = input_type as u8;

        self.controller_write(crate::vendor::opcode::GAP_PASSKEY_INPUT, &bytes)
            .await
    }

    async fn get_oob_data(&mut self, oob_data_type: OobDataType) {
        self.controller_write(
            crate::vendor::opcode::GAP_GET_OOB_DATA,
            &[oob_data_type as u8],
        )
        .await
    }

    impl_params!(
        set_oob_data,
        SetOobDataParameters,
        crate::vendor::opcode::GAP_SET_OOB_DATA
    );

    async fn add_devices_to_resolving_list(
        &mut self,
        whitelist_identities: &[PeerAddrType],
        clear_resolving_list: bool,
    ) {
        let mut bytes = [0; 254];

        bytes[0] = whitelist_identities.len() as u8;

        let mut index = 1;
        for id in whitelist_identities {
            id.copy_into_slice(&mut bytes[index..index + 7]);
            index += 7;
        }
        bytes[index] = clear_resolving_list as u8;

        self.controller_write(
            crate::vendor::opcode::GAP_ADD_DEVICES_TO_RESOLVING_LIST,
            &bytes[..(index + 1)],
        )
        .await;
    }

    async fn remove_bonded_device(&mut self, address: BdAddrType) {
        let mut bytes = [0; 7];

        address.copy_into_slice(&mut bytes);
        self.controller_write(crate::vendor::opcode::GAP_REMOVE_BONDED_DEVICE, &bytes)
            .await;
    }

    async fn add_devices_to_list(
        &mut self,
        list_entries: &[BdAddrType],
        mode: AddDeviceToListMode,
    ) {
        let mut bytes = [0; 254];

        bytes[0] = list_entries.len() as u8;

        let mut index = 0;
        for entry in list_entries {
            entry.copy_into_slice(&mut bytes[index..index + 7]);
            index += 7;
        }
        bytes[index] = mode as u8;

        self.controller_write(
            crate::vendor::opcode::GAP_ADD_DEVICES_TO_LIST,
            &bytes[..(index + 1)],
        )
        .await;
    }

    impl_validate_params!(
        additional_beacon_start,
        AdditonalBeaconStartParameters,
        crate::vendor::opcode::GAP_ADDITIONAL_BEACON_START
    );

    async fn additional_beacon_stop(&mut self) {
        self.controller_write(crate::vendor::opcode::GAP_ADDITIONAL_BEACON_STOP, &[])
            .await;
    }

    async fn additonal_beacon_set_data(&mut self, advertising_data: &[u8]) {
        self.controller_write(
            crate::vendor::opcode::GAP_ADDITIONAL_BEACON_SET_DATA,
            advertising_data,
        )
        .await;
    }

    impl_params!(
        adv_set_config,
        AdvSetConfig,
        crate::vendor::opcode::GAP_ADV_SET_CONFIGURATION
    );

    impl_variable_length_params!(
        adv_set_enable<'a>,
        AdvSetEnable<'a>,
        crate::vendor::opcode::GAP_ADV_SET_ENABLE
    );

    impl_variable_length_params!(
        adv_set_advertising_data<'a>,
        AdvSetAdvertisingData<'a>,
        crate::vendor::opcode::GAP_ADV_SET_ADV_DATA
    );

    impl_variable_length_params!(
        adv_set_scan_response_data<'a>,
        AdvSetAdvertisingData<'a>,
        crate::vendor::opcode::GAP_ADV_SET_SCAN_RESPONSE_DATA
    );

    async fn adv_remove_set(&mut self, handle: AdvertisingHandle) {
        self.controller_write(crate::vendor::opcode::GAP_ADV_REMOVE_SET, &[handle.0])
            .await;
    }

    async fn adv_clear_sets(&mut self) {
        self.controller_write(crate::vendor::opcode::GAP_ADV_CLEAR_SETS, &[])
            .await;
    }

    async fn adv_set_random_address(&mut self, handle: AdvertisingHandle, addr: BdAddr) {
        let mut payload = [0; 7];
        payload[0] = handle.0;
        payload[1..].copy_from_slice(&addr.0);
        self.controller_write(crate::vendor::opcode::GAP_ADV_SET_RANDOM_ADDRESS, &payload)
            .await;
    }
}

/// Potential errors from parameter validation.
///
/// Before some commands are sent to the controller, the parameters are validated. This type
/// enumerates the potential validation errors. Must be specialized on the types of communication
/// errors.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// For the [GAP Set Limited Discoverable](GapCommands::set_limited_discoverable) and
    /// [GAP Set Discoverable](GapCommands::set_discoverable) commands, the connection
    /// interval is inverted (the min is greater than the max).  Return the provided min as the
    /// first element, max as the second.
    BadConnectionInterval(Duration, Duration),

    /// For the [GAP Set Limited Discoverable](GapCommands::set_limited_discoverable) and
    /// [GAP Set Broadcast Mode](GapCommands::set_broadcast_mode) commands, the advertising
    /// type is disallowed.  Returns the invalid advertising type.
    BadAdvertisingType(crate::types::AdvertisingType),

    /// For the [GAP Set Limited Discoverable](GapCommands::set_limited_discoverable)
    /// command, the advertising interval is inverted (that is, the max is less than the
    /// min). Includes the provided range.
    BadAdvertisingInterval(Duration, Duration),

    /// For the [GAP Set Authentication Requirement](GapCommands::set_authentication_requirement)
    /// command, the encryption key size range is inverted (the max is less than the min). Includes the provided range.
    BadEncryptionKeySizeRange(u8, u8),

    /// For the [GAP Set Authentication Requirement](GapCommands::set_authentication_requirement)
    /// command, the address type must be either Public or Random
    BadAddressType(AddressType),

    BadPowerAmplifierLevel(u8),

    /// For the [GAP Set Authentication Requirement](GapCommands::set_authentication_requirement) and
    /// [GAP Pass Key Response](GapCommands::pass_key_response) commands, the provided fixed pin is out of
    /// range (must be less than or equal to 999999).  Includes the provided PIN.
    BadFixedPin(u32),

    /// For the [GAP Set Undirected Connectable](GapCommands::set_undirected_connectable) command, the
    /// advertising filter policy is not one of the allowed values. Only
    /// [AllowConnectionAndScan](crate::host::AdvertisingFilterPolicy::AllowConnectionAndScan) and
    /// [WhiteListConnectionAndScan](crate::host::AdvertisingFilterPolicy::WhiteListConnectionAndScan) are
    /// allowed.
    BadAdvertisingFilterPolicy(crate::host::AdvertisingFilterPolicy),

    /// For the [GAP Update Advertising Data](GapCommands::update_advertising_data) and
    /// [GAP Set Broadcast Mode](GapCommands::set_broadcast_mode) commands, the advertising data
    /// is too long. It must be 31 bytes or less. The length of the provided data is returned.
    BadAdvertisingDataLength(usize),

    /// For the [GAP Terminate](GapCommands::terminate) command, the termination reason was
    /// not one of the allowed reason. The reason is returned.
    BadTerminationReason(crate::Status),

    /// For the [GAP Start Auto Connection Establishment](GapCommands::start_auto_connection_establishment_procedure) or
    /// [GAP Start Selective Connection Establishment](GapCommands::start_selective_connection_establishment_procedure) commands, the
    /// provided [white list](AutoConnectionEstablishmentParameters::white_list) has more than 33
    /// or 35 entries, respectively, which would cause the command to be longer than 255 bytes.
    ///
    /// For the [GAP Set Broadcast Mode](GapCommands::set_broadcast_mode), the provided
    /// [white list](BroadcastModeParameters::white_list) the maximum number of entries ranges
    /// from 31 to 35, depending on the length of the advertising data.
    WhiteListTooLong,

    /// For the [GAP Terminate Procedure](GapCommands::terminate_gap_procedure) command, the
    /// provided bitfield had no bits set.
    NoProcedure,
}

fn to_conn_interval_value(d: Duration) -> u16 {
    // Connection interval value: T = N * 1.25 ms
    // We have T, we need to return N.
    // N = T / 1.25 ms
    //   = 4 * T / 5 ms
    let millis = (d.as_secs() * 1000) as u32 + d.subsec_millis();
    (4 * millis / 5) as u16
}

fn to_connection_length_value(d: Duration) -> u16 {
    // Connection interval value: T = N * 0.625 ms
    // We have T, we need to return N.
    // N = T / 0.625 ms
    //   = T / 625 us
    // 1600 = 1_000_000 / 625
    (1600 * d.as_secs() as u32 + (d.subsec_micros() / 625)) as u16
}

/// Parameters for the
/// [`set_limited_discoverable`](GapCommands::set_limited_discoverable) and
/// [`set_discoverable`](GapCommands::set_discoverable) commands.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DiscoverableParameters<'a, 'b> {
    /// Advertising method for the device.
    ///
    /// Must be
    /// [ConnectableUndirected](crate::host::AdvertisingType::ConnectableUndirected),
    /// [ScannableUndirected](crate::host::AdvertisingType::ScannableUndirected), or
    /// [NonConnectableUndirected](crate::host::AdvertisingType::NonConnectableUndirected).
    pub advertising_type: AdvertisingType,

    /// Range of advertising for non-directed advertising.
    ///
    /// If not provided, the GAP will use default values (1.28 seconds).
    ///
    /// Range for both limits: 20 ms to 10.24 seconds.  The second value must be greater than or
    /// equal to the first.
    pub advertising_interval: Option<(Duration, Duration)>,

    /// Address type for this device.
    pub address_type: OwnAddressType,

    /// Filter policy for this device.
    pub filter_policy: AdvertisingFilterPolicy,

    /// Name of the device.
    pub local_name: Option<LocalName<'a>>,

    /// Service UUID list as defined in the Bluetooth spec, v4.1, Vol 3, Part C, Section 11.
    ///
    /// Must be 31 bytes or fewer.
    pub advertising_data: &'b [u8],

    /// Expected length of the connection to the peripheral.
    pub conn_interval: (Option<Duration>, Option<Duration>),
}

impl<'a, 'b> DiscoverableParameters<'a, 'b> {
    // 14 fixed-size parameters, one parameter of up to 31 bytes, and one of up to 248 bytes.
    const MAX_LENGTH: usize = 14 + 31 + 248;

    fn validate(&self) -> Result<(), Error> {
        match self.advertising_type {
            AdvertisingType::ConnectableUndirected
            | AdvertisingType::ScannableUndirected
            | AdvertisingType::NonConnectableUndirected => (),
            _ => return Err(Error::BadAdvertisingType(self.advertising_type)),
        }

        if let Some(interval) = self.advertising_interval {
            if interval.0 > interval.1 {
                return Err(Error::BadAdvertisingInterval(interval.0, interval.1));
            }
        }

        if let (Some(min), Some(max)) = self.conn_interval {
            if min > max {
                return Err(Error::BadConnectionInterval(min, max));
            }
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) -> usize {
        const NO_SPECIFIC_CONN_INTERVAL: u16 = 0x0000;

        let len = self.required_len();
        assert!(len <= bytes.len());

        let no_duration = Duration::from_secs(0);
        let no_interval: (Duration, Duration) = (no_duration, no_duration);

        bytes[0] = self.advertising_type as u8;
        LittleEndian::write_u16(
            &mut bytes[1..],
            to_connection_length_value(self.advertising_interval.unwrap_or(no_interval).0),
        );
        LittleEndian::write_u16(
            &mut bytes[3..],
            to_connection_length_value(self.advertising_interval.unwrap_or(no_interval).1),
        );
        bytes[5] = self.address_type as u8;
        bytes[6] = self.filter_policy as u8;
        let advertising_data_len_index = match self.local_name {
            None => {
                bytes[7] = 0;
                7
            }
            Some(LocalName::Shortened(name)) => {
                const AD_TYPE_SHORTENED_LOCAL_NAME: u8 = 0x08;
                bytes[7] = 1 + name.len() as u8;
                bytes[8] = AD_TYPE_SHORTENED_LOCAL_NAME;
                bytes[9..9 + name.len()].copy_from_slice(name);
                9 + name.len()
            }
            Some(LocalName::Complete(name)) => {
                const AD_TYPE_COMPLETE_LOCAL_NAME: u8 = 0x09;
                bytes[7] = 1 + name.len() as u8;
                bytes[8] = AD_TYPE_COMPLETE_LOCAL_NAME;
                bytes[9..9 + name.len()].copy_from_slice(name);
                9 + name.len()
            }
        };
        bytes[advertising_data_len_index] = self.advertising_data.len() as u8;
        bytes[(advertising_data_len_index + 1)
            ..(advertising_data_len_index + 1 + self.advertising_data.len())]
            .copy_from_slice(self.advertising_data);
        let conn_interval_index = advertising_data_len_index + 1 + self.advertising_data.len();
        LittleEndian::write_u16(
            &mut bytes[conn_interval_index..],
            if self.conn_interval.0.is_some() {
                to_conn_interval_value(self.conn_interval.0.unwrap())
            } else {
                NO_SPECIFIC_CONN_INTERVAL
            },
        );
        LittleEndian::write_u16(
            &mut bytes[(conn_interval_index + 2)..],
            if self.conn_interval.1.is_some() {
                to_conn_interval_value(self.conn_interval.1.unwrap())
            } else {
                NO_SPECIFIC_CONN_INTERVAL
            },
        );

        len
    }

    fn required_len(&self) -> usize {
        let fixed_len = 13;

        fixed_len + self.name_len() + self.advertising_data.len()
    }

    fn name_len(&self) -> usize {
        // The serialized name includes one byte indicating the type of name. That byte is not
        // included if the name is empty.
        match self.local_name {
            Some(LocalName::Shortened(bytes)) | Some(LocalName::Complete(bytes)) => 1 + bytes.len(),
            None => 0,
        }
    }
}

/// Allowed types for the local name.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LocalName<'a> {
    /// The shortened local name.
    Shortened(&'a [u8]),

    /// The complete local name.
    Complete(&'a [u8]),
}

/// Parameters for the
/// [`set_undirected_connectable`](GapCommands::set_undirected_connectable) command.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UndirectedConnectableParameters {
    /// Range of advertising interval for advertising.
    ///
    /// Range for both limits: 20 ms to 10.24 seconds.  The second value must be greater than or
    /// equal to the first.
    pub advertising_interval: (Duration, Duration),

    /// Address type of this device.
    pub own_address_type: OwnAddressType,

    /// filter policy for this device
    pub filter_policy: AdvertisingFilterPolicy,
}

impl UndirectedConnectableParameters {
    const LENGTH: usize = 6;

    fn validate(&self) -> Result<(), Error> {
        const MIN_DURATION: Duration = Duration::from_millis(20);
        const MAX_DURATION: Duration = Duration::from_millis(10240);

        match self.filter_policy {
            AdvertisingFilterPolicy::AllowConnectionAndScan
            | AdvertisingFilterPolicy::WhiteListConnectionAndScan => {}
            _ => return Err(Error::BadAdvertisingFilterPolicy(self.filter_policy)),
        }

        if self.advertising_interval.0 < MIN_DURATION
            || self.advertising_interval.1 > MAX_DURATION
            || self.advertising_interval.0 > self.advertising_interval.1
        {
            return Err(Error::BadAdvertisingInterval(
                self.advertising_interval.0,
                self.advertising_interval.1,
            ));
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), Self::LENGTH);

        LittleEndian::write_u16(
            &mut bytes[0..],
            to_connection_length_value(self.advertising_interval.0),
        );
        LittleEndian::write_u16(
            &mut bytes[2..],
            to_connection_length_value(self.advertising_interval.1),
        );

        bytes[4] = self.own_address_type as u8;
        bytes[5] = self.filter_policy as u8;
    }
}

/// Parameters for the
/// [`set_direct_connectable`](GapCommands::set_direct_connectable) command.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DirectConnectableParameters {
    /// Address type of this device.
    pub own_address_type: OwnAddressType,

    /// Advertising method for the device.
    ///
    /// Must be
    /// [ConnectableDirectedHighDutyCycle](crate::host::AdvertisingType::ConnectableDirectedHighDutyCycle),
    /// or
    /// [ConnectableDirectedLowDutyCycle](crate::host::AdvertisingType::ConnectableDirectedLowDutyCycle).
    pub advertising_type: AdvertisingType,

    /// Initiator's Bluetooth address.
    pub initiator_address: BdAddrType,

    /// Range of advertising interval for advertising.
    ///
    /// Range for both limits: 20 ms to 10.24 seconds.  The second value must be greater than or
    /// equal to the first.
    pub advertising_interval: (Duration, Duration),
}

impl DirectConnectableParameters {
    const LENGTH: usize = 13;

    fn validate(&self) -> Result<(), Error> {
        const MIN_DURATION: Duration = Duration::from_millis(20);
        const MAX_DURATION: Duration = Duration::from_millis(10240);

        match self.advertising_type {
            AdvertisingType::ConnectableDirectedHighDutyCycle
            | AdvertisingType::ConnectableDirectedLowDutyCycle => (),
            _ => return Err(Error::BadAdvertisingType(self.advertising_type)),
        }

        if self.advertising_interval.0 < MIN_DURATION
            || self.advertising_interval.1 > MAX_DURATION
            || self.advertising_interval.0 > self.advertising_interval.1
        {
            return Err(Error::BadAdvertisingInterval(
                self.advertising_interval.0,
                self.advertising_interval.1,
            ));
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), Self::LENGTH);

        bytes[0] = self.own_address_type as u8;

        bytes[1] = self.advertising_type as u8;
        self.initiator_address.copy_into_slice(&mut bytes[2..9]);
        LittleEndian::write_u16(
            &mut bytes[9..],
            to_connection_length_value(self.advertising_interval.0),
        );
        LittleEndian::write_u16(
            &mut bytes[11..],
            to_connection_length_value(self.advertising_interval.1),
        );
    }
}

/// I/O capabilities available for the [GAP Set I/O Capability](GapCommands::set_io_capability) command.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IoCapability {
    /// Display Only
    Display = 0x00,
    /// Display yes/no
    DisplayConfirm = 0x01,
    /// Keyboard Only
    Keyboard = 0x02,
    /// No Input, no output
    None = 0x03,
    /// Keyboard display
    KeyboardDisplay = 0x04,
}

/// Parameters for the [GAP Set Authentication Requirement](GapCommands::set_authentication_requirement) command.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AuthenticationRequirements {
    /// Is bonding required?
    pub bonding_required: bool,

    /// Is MITM (man-in-the-middle) protection required?
    pub mitm_protection_required: bool,

    /// is secure connection support required
    pub secure_connection_support: SecureConnectionSupport,

    /// is keypress notification support required
    pub keypress_notification_support: bool,

    /// Minimum and maximum size of the encryption key.
    pub encryption_key_size_range: (u8, u8),

    /// Pin to use during the pairing process.
    pub fixed_pin: Pin,

    /// identity address type.
    pub identity_address_type: AddressType,
}

impl AuthenticationRequirements {
    const LENGTH: usize = 12;

    fn validate(&self) -> Result<(), Error> {
        if self.encryption_key_size_range.0 > self.encryption_key_size_range.1 {
            return Err(Error::BadEncryptionKeySizeRange(
                self.encryption_key_size_range.0,
                self.encryption_key_size_range.1,
            ));
        }

        if let Pin::Fixed(pin) = self.fixed_pin {
            if pin > 999_999 {
                return Err(Error::BadFixedPin(pin));
            }
        }

        if self.identity_address_type != AddressType::Public
            && self.identity_address_type != AddressType::Random
        {
            return Err(Error::BadAddressType(self.identity_address_type));
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), Self::LENGTH);

        bytes[0] = self.bonding_required as u8;
        bytes[1] = self.mitm_protection_required as u8;
        bytes[2] = self.secure_connection_support as u8;
        bytes[3] = self.keypress_notification_support as u8;
        bytes[4] = self.encryption_key_size_range.0;
        bytes[5] = self.encryption_key_size_range.1;
        match self.fixed_pin {
            Pin::Requested => {
                bytes[6] = 1;
                bytes[7..11].copy_from_slice(&[0; 4]);
            }
            Pin::Fixed(pin) => {
                bytes[6] = 0;
                LittleEndian::write_u32(&mut bytes[7..11], pin);
            }
        }
        bytes[11] = self.identity_address_type as u8;
    }
}

/// Options for [`out_of_band_auth`](AuthenticationRequirements::out_of_band_auth).
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutOfBandAuthentication {
    /// Out Of Band authentication not enabled
    Disabled,
    /// Out Of Band authentication enabled; includes the OOB data.
    Enabled([u8; 16]),
}

/// Options for [`secure_connection_support`](AuthenticationRequirements)
#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecureConnectionSupport {
    NotSupported = 0x00,
    Optional = 0x01,
    Mandatory = 0x02,
}

/// Options for [`fixed_pin`](AuthenticationRequirements).
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pin {
    /// Do not use fixed pin during the pairing process.  In this case, GAP will generate a
    /// [GAP Pass Key Request](crate::vendor::event::VendorEvent::GapPassKeyRequest) event to the host.
    Requested,

    /// Use a fixed pin during pairing. The provided value is used as the PIN, and must be 999999 or
    /// less.
    Fixed(u32),
}

/// Options for the [GAP Authorization Response](GapCommands::authorization_response).
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Authorization {
    /// Accept the connection.
    Authorized = 0x01,
    /// Reject the connection.
    Rejected = 0x02,
}

#[cfg(not(feature = "defmt"))]
bitflags::bitflags! {
    /// Roles for a [GAP service](GapCommands::init).
    pub struct Role: u8 {
        /// Peripheral
        const PERIPHERAL = 0x01;
        /// Broadcaster
        const BROADCASTER = 0x02;
        /// Central Device
        const CENTRAL = 0x04;
        /// Observer
        const OBSERVER = 0x08;
    }
}

#[cfg(feature = "defmt")]
defmt::bitflags! {
    /// Roles for a [GAP service](GapCommands::init).
    pub struct Role: u8 {
        /// Peripheral
        const PERIPHERAL = 0x01;
        /// Broadcaster
        const BROADCASTER = 0x02;
        /// Central Device
        const CENTRAL = 0x04;
        /// Observer
        const OBSERVER = 0x08;
    }
}

/// Indicates the type of address being used in the advertising packets, for the
/// [`set_nonconnectable`](GapCommands::set_nonconnectable).
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddressType {
    /// Public device address.
    Public = 0x00,
    /// Static random device address.
    Random = 0x01,
    /// Controller generates Resolvable Private Address.
    ResolvablePrivate = 0x02,
    /// Controller generates Resolvable Private Address. based on the local IRK from resolving
    /// list.
    NonResolvablePrivate = 0x03,
}

/// Available types of advertising data.
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdvertisingDataType {
    /// Flags
    Flags = 0x01,
    /// 16-bit service UUID
    Uuid16 = 0x02,
    /// Complete list of 16-bit service UUIDs
    UuidCompleteList16 = 0x03,
    /// 32-bit service UUID
    Uuid32 = 0x04,
    /// Complete list of 32-bit service UUIDs
    UuidCompleteList32 = 0x05,
    /// 128-bit service UUID
    Uuid128 = 0x06,
    /// Complete list of 128-bit service UUIDs.
    UuidCompleteList128 = 0x07,
    /// Shortened local name
    ShortenedLocalName = 0x08,
    /// Complete local name
    CompleteLocalName = 0x09,
    /// Transmitter power level
    TxPowerLevel = 0x0A,
    /// Serurity Manager TK Value
    SecurityManagerTkValue = 0x10,
    /// Serurity Manager out-of-band flags
    SecurityManagerOutOfBandFlags = 0x11,
    /// Connection interval
    PeripheralConnectionInterval = 0x12,
    /// Service solicitation list, 16-bit UUIDs
    SolicitUuidList16 = 0x14,
    /// Service solicitation list, 32-bit UUIDs
    SolicitUuidList32 = 0x15,
    /// Service data
    ServiceData = 0x16,
    /// Manufacturer-specific data
    ManufacturerSpecificData = 0xFF,
}

#[cfg(not(feature = "defmt"))]
bitflags::bitflags! {
    /// Event types for [GAP Set Event Mask](GapCommands::set_event_mask).
    #[derive(Debug, Clone, Copy)]
    pub struct EventFlags: u16 {
        /// [Limited Discoverable](::event::VendorEvent::GapLimitedDiscoverableTimeout)
        const LIMITED_DISCOVERABLE_TIMEOUT = 0x0001;
        /// [Pairing Complete](::event::VendorEvent::GapPairingComplete)
        const PAIRING_COMPLETE = 0x0002;
        /// [Pass Key Request](::event::VendorEvent::GapPassKeyRequest)
        const PASS_KEY_REQUEST = 0x0004;
        /// [Authorization Request](::event::VendorEvent::GapAuthorizationRequest)
        const AUTHORIZATION_REQUEST = 0x0008;
        /// [Peripheral Security Initiated](::event::VendorEvent::GapPeripheralSecurityInitiated).
        const PERIPHERAL_SECURITY_INITIATED = 0x0010;
        /// [Bond Lost](::event::VendorEvent::GapBondLost)
        const BOND_LOST = 0x0020;
    }
}

#[cfg(feature = "defmt")]
defmt::bitflags! {
    /// Event types for [GAP Set Event Mask](GapCommands::set_event_mask).
    pub struct EventFlags: u16 {
        /// [Limited Discoverable](::event::VendorEvent::GapLimitedDiscoverableTimeout)
        const LIMITED_DISCOVERABLE_TIMEOUT = 0x0001;
        /// [Pairing Complete](::event::VendorEvent::GapPairingComplete)
        const PAIRING_COMPLETE = 0x0002;
        /// [Pass Key Request](::event::VendorEvent::GapPassKeyRequest)
        const PASS_KEY_REQUEST = 0x0004;
        /// [Authorization Request](::event::VendorEvent::GapAuthorizationRequest)
        const AUTHORIZATION_REQUEST = 0x0008;
        /// [Peripheral Security Initiated](::event::VendorEvent::GapPeripheralSecurityInitiated).
        const PERIPHERAL_SECURITY_INITIATED = 0x0010;
        /// [Bond Lost](::event::VendorEvent::GapBondLost)
        const BOND_LOST = 0x0020;
    }
}

/// Parameters for the [GAP Limited Discovery](GapCommands::start_limited_discovery_procedure) and
/// [GAP General Discovery](GapCommands::start_general_discovery_procedure) procedures.
pub struct DiscoveryProcedureParameters {
    /// Scanning window for the discovery procedure.
    pub scan_window: ScanWindow,

    /// Address type of this device.
    pub own_address_type: crate::host::OwnAddressType,

    /// If true, duplicate devices are filtered out.
    pub filter_duplicates: bool,
}

impl DiscoveryProcedureParameters {
    const LENGTH: usize = 6;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), Self::LENGTH);

        self.scan_window.copy_into_slice(&mut bytes[0..4]);
        bytes[4] = self.own_address_type as u8;
        bytes[5] = self.filter_duplicates as u8;
    }
}

/// Parameters for the [GAP Name Discovery](GapCommands::start_name_discovery_procedure)
/// procedure.
pub struct NameDiscoveryProcedureParameters {
    /// Scanning window for the discovery procedure.
    pub scan_window: ScanWindow,

    /// Address of the connected device
    pub peer_address: crate::host::PeerAddrType,

    /// Address type of this device.
    pub own_address_type: crate::host::OwnAddressType,

    /// Connection interval parameters.
    pub conn_interval: ConnectionInterval,

    /// Expected connection length
    pub expected_connection_length: ExpectedConnectionLength,
}

impl NameDiscoveryProcedureParameters {
    const LENGTH: usize = 24;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), Self::LENGTH);

        self.scan_window.copy_into_slice(&mut bytes[0..4]);
        self.peer_address.copy_into_slice(&mut bytes[4..11]);
        bytes[11] = self.own_address_type as u8;
        self.conn_interval.copy_into_slice(&mut bytes[12..20]);
        self.expected_connection_length
            .copy_into_slice(&mut bytes[20..24]);
    }
}

/// Parameters for the
/// [GAP Start Auto Connection Establishment](GapCommands::start_auto_connection_establishment_procedure) command.
pub struct AutoConnectionEstablishmentParameters<'a> {
    /// Scanning window for connection establishment.
    pub scan_window: ScanWindow,

    /// Address type of this device.
    pub own_address_type: crate::host::OwnAddressType,

    /// Connection interval parameters.
    pub conn_interval: ConnectionInterval,

    /// Expected connection length
    pub expected_connection_length: ExpectedConnectionLength,

    /// Addresses to white-list for automatic connection.
    pub white_list: &'a [crate::host::PeerAddrType],
}

impl<'a> AutoConnectionEstablishmentParameters<'a> {
    const MAX_LENGTH: usize = 249;

    fn validate(&self) -> Result<(), Error> {
        const MAX_WHITE_LIST_LENGTH: usize = 33;
        if self.white_list.len() > MAX_WHITE_LIST_LENGTH {
            return Err(Error::WhiteListTooLong);
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) -> usize {
        let len = self.len();
        assert!(bytes.len() >= len);

        self.scan_window.copy_into_slice(&mut bytes[0..4]);
        bytes[4] = self.own_address_type as u8;
        self.conn_interval.copy_into_slice(&mut bytes[5..13]);
        self.expected_connection_length
            .copy_into_slice(&mut bytes[13..17]);

        let index = 17;

        bytes[index] = self.white_list.len() as u8;
        let index = index + 1;
        for i in 0..self.white_list.len() {
            self.white_list[i].copy_into_slice(&mut bytes[(index + 7 * i)..(index + 7 * (i + 1))]);
        }

        len
    }

    fn len(&self) -> usize {
        let reconn_addr_len = 0;
        18 + reconn_addr_len + 7 * self.white_list.len()
    }
}

/// Parameters for the
/// [GAP Start General Connection Establishment](GapCommands::start_general_connection_establishment_procedure) command.
pub struct GeneralConnectionEstablishmentParameters {
    /// passive or active scanning. With passive scanning, no scan request PDUs are sent
    pub scan_type: ScanType,

    /// Scanning window for connection establishment.
    pub scan_window: ScanWindow,

    /// Address type of this device.
    pub own_address_type: crate::host::OwnAddressType,

    /// Scanning filter policy.
    ///
    /// # Note
    /// if privacy is enabled, filter policy can only assume values
    /// [Accept All](ScanFilterPolicy::AcceptAll) or
    /// [Addressed To This Device](ScanFilterPolicy::AddressedToThisDevice)
    pub filter_policy: ScanFilterPolicy,

    /// If true, only report unique devices.
    pub filter_duplicates: bool,
}

impl GeneralConnectionEstablishmentParameters {
    const LENGTH: usize = 8;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::LENGTH);

        bytes[0] = self.scan_type as u8;
        self.scan_window.copy_into_slice(&mut bytes[1..5]);
        bytes[5] = self.filter_policy as u8;
        bytes[6] = self.own_address_type as u8;
        bytes[7] = self.filter_duplicates as u8;
    }
}

/// Parameters for the
/// [GAP Start Selective Connection Establishment](GapCommands::start_selective_connection_establishment_procedure) command.
pub struct SelectiveConnectionEstablishmentParameters<'a> {
    /// Type of scanning
    pub scan_type: crate::host::ScanType,

    /// Scanning window for connection establishment.
    pub scan_window: ScanWindow,

    /// Address type of this device.
    pub own_address_type: crate::host::OwnAddressType,

    /// Scanning filter policy.
    ///
    /// # Note
    /// if privacy is enabled, filter policy can only assume values
    /// [Accept All](ScanFilterPolicy::AcceptAll) or
    /// [Whitelist Addressed to this Device](ScanFilterPolicy::WhiteListAddressedToThisDevice)
    pub filter_policy: ScanFilterPolicy,

    /// If true, only report unique devices.
    pub filter_duplicates: bool,

    /// Addresses to white-list for automatic connection.
    pub white_list: &'a [crate::host::PeerAddrType],
}

impl<'a> SelectiveConnectionEstablishmentParameters<'a> {
    const MAX_LENGTH: usize = 254;

    fn validate(&self) -> Result<(), Error> {
        const MAX_WHITE_LIST_LENGTH: usize = 35;
        if self.white_list.len() > MAX_WHITE_LIST_LENGTH {
            return Err(Error::WhiteListTooLong);
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) -> usize {
        let len = self.len();
        assert!(bytes.len() >= len);

        bytes[0] = self.scan_type as u8;
        self.scan_window.copy_into_slice(&mut bytes[1..5]);
        bytes[5] = self.own_address_type as u8;
        bytes[6] = self.filter_policy as u8;
        bytes[7] = self.filter_duplicates as u8;
        bytes[8] = self.white_list.len() as u8;
        for i in 0..self.white_list.len() {
            self.white_list[i].copy_into_slice(&mut bytes[(9 + 7 * i)..(9 + 7 * (i + 1))]);
        }

        len
    }

    fn len(&self) -> usize {
        9 + 7 * self.white_list.len()
    }
}

/// The parameters for the [GAP Name Discovery](GapCommands::start_name_discovery_procedure)
/// and [GAP Create Connection](GapCommands::create_connection) commands are identical.
pub type ConnectionParameters = NameDiscoveryProcedureParameters;

#[cfg(not(feature = "defmt"))]
bitflags::bitflags! {
    /// Roles for a [GAP service](GapCommands::init).
    pub struct Procedure: u8 {
        /// [Limited Discovery](GapCommands::start_limited_discovery_procedure) procedure.
        const LIMITED_DISCOVERY = 0x01;
        /// [General Discovery](GapCommands::start_general_discovery_procedure) procedure.
        const GENERAL_DISCOVERY = 0x02;
        /// [Name Discovery](GapCommands::start_name_discovery_procedure) procedure.
        const NAME_DISCOVERY = 0x04;
        /// [Auto Connection Establishment](GapCommands::auto_connection_establishment).
        const AUTO_CONNECTION_ESTABLISHMENT = 0x08;
        /// [General Connection Establishment](GapCommands::general_connection_establishment).
        const GENERAL_CONNECTION_ESTABLISHMENT = 0x10;
        /// [Selective Connection Establishment](GapCommands::selective_connection_establishment).
        const SELECTIVE_CONNECTION_ESTABLISHMENT = 0x20;
        /// [Direct Connection Establishment](GapCommands::direct_connection_establishment).
        const DIRECT_CONNECTION_ESTABLISHMENT = 0x40;
        /// [Observation](GapCommands::start_observation_procedure) procedure.
        const OBSERVATION = 0x80;
    }
}

#[cfg(feature = "defmt")]
defmt::bitflags! {
    /// Roles for a [GAP service](GapCommands::init).
    pub struct Procedure: u8 {
        /// [Limited Discovery](GapCommands::start_limited_discovery_procedure) procedure.
        const LIMITED_DISCOVERY = 0x01;
        /// [General Discovery](GapCommands::start_general_discovery_procedure) procedure.
        const GENERAL_DISCOVERY = 0x02;
        /// [Name Discovery](GapCommands::start_name_discovery_procedure) procedure.
        const NAME_DISCOVERY = 0x04;
        /// [Auto Connection Establishment](GapCommands::auto_connection_establishment).
        const AUTO_CONNECTION_ESTABLISHMENT = 0x08;
        /// [General Connection Establishment](GapCommands::general_connection_establishment).
        const GENERAL_CONNECTION_ESTABLISHMENT = 0x10;
        /// [Selective Connection Establishment](GapCommands::selective_connection_establishment).
        const SELECTIVE_CONNECTION_ESTABLISHMENT = 0x20;
        /// [Direct Connection Establishment](GapCommands::direct_connection_establishment).
        const DIRECT_CONNECTION_ESTABLISHMENT = 0x40;
        /// [Observation](GapCommands::start_observation_procedure) procedure.
        const OBSERVATION = 0x80;
    }
}

/// Parameters for the [`start_connection_update`](GapCommands::start_connection_update)
/// command.
pub struct ConnectionUpdateParameters {
    /// Handle of the connection for which the update procedure has to be started.
    pub conn_handle: crate::ConnectionHandle,

    /// Updated connection interval for the connection.
    pub conn_interval: ConnectionInterval,

    /// Expected length of connection event needed for this connection.
    pub expected_connection_length: ExpectedConnectionLength,
}

impl ConnectionUpdateParameters {
    const LENGTH: usize = 14;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        LittleEndian::write_u16(&mut bytes[0..2], self.conn_handle.0);
        self.conn_interval.copy_into_slice(&mut bytes[2..10]);
        self.expected_connection_length
            .copy_into_slice(&mut bytes[10..14]);
    }
}

/// Parameters for the [`send_pairing_request`](GapCommands::send_pairing_request)
/// command.
pub struct PairingRequest {
    /// Handle of the connection for which the pairing request has to be sent.
    pub conn_handle: crate::ConnectionHandle,

    /// Whether pairing request has to be sent if the device is previously bonded or not. If false,
    /// the pairing request is sent only if the device has not previously bonded.
    pub force_rebond: bool,
}

impl PairingRequest {
    const LENGTH: usize = 2;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::LENGTH);

        LittleEndian::write_u16(&mut bytes[0..2], self.conn_handle.0);
    }
}

/// Parameters for the [GAP Set Broadcast Mode](GapCommands::set_broadcast_mode) command.
pub struct BroadcastModeParameters<'a, 'b> {
    /// Advertising type and interval.
    ///
    /// Only the [ScannableUndirected](crate::types::AdvertisingType::ScannableUndirected) and
    /// [NonConnectableUndirected](crate::types::AdvertisingType::NonConnectableUndirected).
    pub advertising_interval: crate::types::AdvertisingInterval,

    /// Type of this device's address.
    ///
    /// A privacy enabled device uses either a
    /// [resolvable private address](AddressType::ResolvablePrivate) or a
    /// [non-resolvable private](AddressType::NonResolvablePrivate) address.
    pub own_address_type: AddressType,

    /// Advertising data used by the device when advertising.
    ///
    /// Must be 31 bytes or fewer.
    pub advertising_data: &'a [u8],

    /// Addresses to add to the white list.
    ///
    /// Each address takes up 7 bytes (1 byte for the type, 6 for the address). The full length of
    /// this packet must not exceed 255 bytes. The white list must be less than a maximum of between
    /// 31 and 35 entries, depending on the length of
    /// [`advertising_data`](BroadcastModeParameters::advertising_data). Shorter advertising data
    /// allows more white list entries.
    pub white_list: &'b [crate::host::PeerAddrType],
}

impl<'a, 'b> BroadcastModeParameters<'a, 'b> {
    const MAX_LENGTH: usize = 255;

    fn validate(&self) -> Result<(), Error> {
        const MAX_ADVERTISING_DATA_LENGTH: usize = 31;

        match self.advertising_interval.advertising_type() {
            crate::types::AdvertisingType::ScannableUndirected
            | crate::types::AdvertisingType::NonConnectableUndirected => (),
            other => return Err(Error::BadAdvertisingType(other)),
        }

        if self.advertising_data.len() > MAX_ADVERTISING_DATA_LENGTH {
            return Err(Error::BadAdvertisingDataLength(self.advertising_data.len()));
        }

        if self.len() > Self::MAX_LENGTH {
            return Err(Error::WhiteListTooLong);
        }

        Ok(())
    }

    fn len(&self) -> usize {
        5 + // advertising_interval
            1 + // own_address_type
            1 + self.advertising_data.len() + // advertising_data
            1 + 7 * self.white_list.len() // white_list
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) -> usize {
        assert!(self.len() <= bytes.len());

        self.advertising_interval.copy_into_slice(&mut bytes[0..5]);
        bytes[5] = self.own_address_type as u8;
        bytes[6] = self.advertising_data.len() as u8;
        bytes[7..7 + self.advertising_data.len()].copy_from_slice(self.advertising_data);
        bytes[7 + self.advertising_data.len()] = self.white_list.len() as u8;

        let mut index = 8 + self.advertising_data.len();
        for addr in self.white_list.iter() {
            addr.copy_into_slice(&mut bytes[index..index + 7]);
            index += 7;
        }

        index
    }
}

/// Parameters for the [GAP Start Observation Procedure](GapCommands::start_observation_procedure)
/// command.
pub struct ObservationProcedureParameters {
    /// Scanning window.
    pub scan_window: crate::types::ScanWindow,

    /// Active or passive scanning
    pub scan_type: crate::host::ScanType,

    /// Address type of this device.
    pub own_address_type: AddressType,

    /// If true, do not report duplicate events in the
    /// [advertising report](crate::event::Event::LeAdvertisingReport).
    pub filter_duplicates: bool,

    /// Scanning filter policy
    pub filter_policy: ScanFilterPolicy,
}

impl ObservationProcedureParameters {
    const LENGTH: usize = 8;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::LENGTH);

        self.scan_window.copy_into_slice(&mut bytes[0..4]);
        bytes[4] = self.scan_type as u8;
        bytes[5] = self.own_address_type as u8;
        bytes[6] = self.filter_duplicates as u8;
        bytes[7] = self.filter_policy as u8;
    }
}

/// Parameters for [GAP Numeric Comparison Confirm Yes or No](crate::vendor::command::gap::GapCommands::numeric_comparison_value_confirm_yes_no)
pub struct NumericComparisonValueConfirmYesNoParameters {
    conn_handle: ConnectionHandle,
    confirm_yes_no: bool,
}

impl NumericComparisonValueConfirmYesNoParameters {
    const LENGTH: usize = 3;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::LENGTH);

        LittleEndian::write_u16(&mut bytes[0..2], self.conn_handle.0);
        bytes[2] = self.confirm_yes_no as u8;
    }
}

/// Parameter for [GAP Passkey Input](GapCommands::passkey_input)
pub enum InputType {
    EntryStarted = 0x00,
    DigitEntered = 0x01,
    DigitErased = 0x02,
    Cleared = 0x03,
    EntryCompleted = 0x04,
}

#[derive(Clone, Copy)]
pub enum OobDataType {
    /// TK (LP v.4.1)
    TK,
    /// Random (SC)
    Random,
    /// Confirm (SC)
    Confirm,
}

#[derive(Clone, Copy)]
pub enum OobDeviceType {
    Local = 0x00,
    Remote = 0x01,
}

/// Parameters for [GAP Set OOB Data](GapCommands::set_oob_data)
pub struct SetOobDataParameters {
    /// OOB Device type
    device_type: OobDeviceType,
    /// Identity address
    address: BdAddrType,
    /// OOB Data type
    oob_data_type: OobDataType,
    /// Pairing Data received through OOB from remote device
    oob_data: [u8; 16],
}

impl SetOobDataParameters {
    const LENGTH: usize = 26;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::LENGTH);

        bytes[0] = self.device_type as u8;
        self.address.copy_into_slice(&mut bytes[1..8]);
        bytes[9] = self.oob_data_type as u8;
        bytes[10..26].copy_from_slice(&self.oob_data)
    }
}

/// Parameter for [GAP Add Devices to List](GapCommands::add_devices_to_list)
pub enum AddDeviceToListMode {
    /// Append to the resolving list only
    AppendResoling = 0x00,
    /// clear and set the resolving list only
    ClearAndSetResolving = 0x01,
    /// append to the whitelist only
    AppendWhitelist = 0x02,
    /// clear and set the whitelist only
    ClearAndSetWhitelist = 0x03,
    /// apppend to both resolving and white lists
    AppendBoth = 0x04,
    /// clear and set both resolving and white lists
    ClearAndSetBoth = 0x05,
}

/// Parameters for [GAP Additional Beacon Start](GapCommands::additional_beacon_start)
pub struct AdditonalBeaconStartParameters {
    /// Advertising interval
    pub advertising_interval: (Duration, Duration),
    /// advertising channel map
    pub advertising_channel_map: Channels,
    /// Own address type
    pub own_address_type: BdAddrType,
    /// Power amplifier output level. Range: 0x00 .. 0x23
    pub pa_level: u8,
}

impl AdditonalBeaconStartParameters {
    const LENGTH: usize = 13;

    fn validate(&self) -> Result<(), Error> {
        const AMPLIFIER_MAX: u8 = 0x23;

        if self.pa_level > AMPLIFIER_MAX {
            return Err(Error::BadPowerAmplifierLevel(self.pa_level));
        }

        Ok(())
    }

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::LENGTH);

        LittleEndian::write_u16(
            &mut bytes[0..],
            to_connection_length_value(self.advertising_interval.0),
        );
        LittleEndian::write_u16(
            &mut bytes[2..],
            to_connection_length_value(self.advertising_interval.1),
        );
        bytes[4] = self.advertising_channel_map.bits();
        self.own_address_type.copy_into_slice(&mut bytes[5..12]);
        bytes[12] = self.pa_level;
    }
}

/// Params for the [adv_set_config](GapCommands::adv_set_config) command
pub struct AdvSetConfig {
    /// Bitmap of extended advertising modes
    pub adv_mode: AdvertisingMode,
    /// Used to identify an advertising set
    pub adv_handle: AdvertisingHandle,
    /// Type of advertising event
    pub adv_event_properties: AdvertisingEvent,
    /// Advertising interval
    pub adv_interval: ExtendedAdvertisingInterval,
    /// Advertising channel map
    pub primary_adv_channel_map: Channels,
    /// Own address type.
    ///
    /// If privacy is disabled, the address can be public or static random, otherwise,
    /// it can be a resolvable private address or a non-resolvabble private address.
    pub own_addr_type: OwnAddressType,
    /// Public device address, random device addressm public identity address, or random
    /// (static) identity address of the device to be connected.
    pub peer_addr: BdAddrType,
    /// Advertising filter policy
    pub adv_filter_policy: AdvertisingFilterPolicy,
    /// Advertising TX power. Units; dBm.
    ///
    /// Values;
    /// - -127 .. 20
    pub adv_tx_power: u8,
    /// Secondary advertising maximum skip.
    ///
    /// Values:
    /// - 0x00: `AUX_QDV_IND` shall be sent prior to the next advertising event
    /// - 0x01 .. 0xFF: Maximum advertising events to the Controller can skip
    /// before sending the `AUX_QDV_IND` packets on the secondary physical channel.
    pub secondary_adv_max_skip: u8,
    /// Secondary advertising PHY
    pub secondary_adv_phy: AdvertisingPhy,
    /// Value of advertising SID subfield in the ADI field of the PDU.
    ///
    /// Values:
    /// - 0x00 .. 0x0F
    pub adv_sid: u8,
    /// Scan request notifications
    pub scan_req_notification_enable: bool,
}

impl AdvSetConfig {
    const LENGTH: usize = 26;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), Self::LENGTH);

        bytes[0] = self.adv_mode.bits();
        bytes[1] = self.adv_handle.0;
        LittleEndian::write_u16(&mut bytes[2..], self.adv_event_properties.bits());
        self.adv_interval.copy_into_slice(&mut bytes[4..]);
        bytes[12] = self.primary_adv_channel_map.bits();
        bytes[13] = self.own_addr_type as u8;
        self.peer_addr.copy_into_slice(&mut bytes[14..21]);
        bytes[21] = self.adv_filter_policy as u8;
        bytes[22] = self.adv_tx_power;
        bytes[23] = self.secondary_adv_max_skip;
        bytes[24] = self.adv_sid;
        bytes[25] = self.scan_req_notification_enable as u8;
    }
}

/// Params for the [adv_set_enable](GapCommands::adv_set_enable) command
pub struct AdvSetEnable<'a> {
    /// Enable/Disable advertising
    pub enable: bool,
    /// Number of advertising sets.
    ///
    /// Values
    /// - 0x00: disable all advertising sets
    /// - 0x01 .. 0x3F: Number of advertising sets to enable or disable
    pub num_sets: u8,
    /// Advertising sets
    pub adv_set: &'a [AdvSet],
}

impl<'a> AdvSetEnable<'a> {
    const MAX_LENGTH: usize = 254;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::MAX_LENGTH);

        bytes[0] = self.enable as u8;
        bytes[1] = self.num_sets;
        for (idx, set) in self.adv_set.iter().enumerate() {
            set.copy_into_slice(&mut bytes[2 + (idx * 4)..]);
        }
    }
}

/// Params for the [adv_set_advertising_data](GapCommands::adv_set_advertising_data) command
pub struct AdvSetAdvertisingData<'a> {
    /// Used to identify an advertising set
    pub adv_handle: AdvertisingHandle,
    /// Advertising operation
    pub operation: AdvertisingOperation,
    /// Fragment preference. If set to `true`, the Controller may fragment all data, else
    /// the Controller should not fragment or should minimize fragmentation of data
    pub fragment: bool,
    /// Data formatted as defined in Bluetooth spec. v.5.4 [Vol 3, Part C, 11].
    pub data: &'a [u8],
}

impl<'a> AdvSetAdvertisingData<'a> {
    const MAX_LENGTH: usize = 255;

    fn copy_into_slice(&self, bytes: &mut [u8]) {
        assert!(bytes.len() >= Self::MAX_LENGTH);

        bytes[0] = self.adv_handle.0;
        bytes[1] = self.operation as u8;
        bytes[2] = (!self.fragment) as u8;
        let length = self.data.len();
        bytes[3] = length as u8;
        bytes[4..(4 + length)].copy_from_slice(self.data);
    }
}
