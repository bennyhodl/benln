#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoResponse {
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub listening_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAddressRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAddressResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageRequest {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageResponse {
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPeerRequest {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub persist: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPeerResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOnchainSpendableRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOnchainSpendableResponse {
    #[prost(string, tag = "1")]
    pub spendable_sats: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalOnchainBalanceRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalOnchainBalanceResponse {
    #[prost(uint64, tag = "1")]
    pub total_balance_sats: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendToAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub amount_sats: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendToAddressResponse {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SweepWalletRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SweepWalletResponse {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub counterparty_node_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub funding_txo: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub channel_value_sats: u64,
    #[prost(uint64, tag = "5")]
    pub unspendable_punishment_reserve: u64,
    #[prost(string, tag = "6")]
    pub user_channel_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub feerate_sat_per_1000_weight: u32,
    #[prost(uint64, tag = "8")]
    pub balance_msat: u64,
    #[prost(uint64, tag = "9")]
    pub outbound_capacity_msat: u64,
    #[prost(uint64, tag = "10")]
    pub inbound_capacity_msat: u64,
    #[prost(uint32, tag = "11")]
    pub confirmations_required: u32,
    #[prost(uint32, tag = "12")]
    pub confirmations: u32,
    #[prost(bool, tag = "13")]
    pub is_outbound: bool,
    #[prost(bool, tag = "14")]
    pub is_channel_ready: bool,
    #[prost(bool, tag = "15")]
    pub is_usable: bool,
    #[prost(bool, tag = "16")]
    pub is_public: bool,
    #[prost(uint32, tag = "17")]
    pub cltv_expiry_delta: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeersRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeersResponse {
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_persisted: bool,
    #[prost(bool, tag = "4")]
    pub is_connected: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePeerRequest {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePeerResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenChannelRequest {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub channel_amount_sats: u64,
    #[prost(string, tag = "4")]
    pub push_amount: ::prost::alloc::string::String,
    /// Fix me
    #[prost(bool, tag = "5")]
    pub channel_config: bool,
    #[prost(bool, tag = "6")]
    pub announce_channel: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenChannelResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncWalletRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncWalletResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseChannelRequest {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    /// fix me
    #[prost(bool, tag = "3")]
    pub channel_config: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseChannelResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentRequest {
    #[prost(string, tag = "1")]
    pub invoice: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentResponse {
    #[prost(string, tag = "2")]
    pub payment_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentWithAmountRequest {
    #[prost(string, tag = "1")]
    pub invoice: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub amount_msat: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentWithAmountResponse {
    #[prost(string, tag = "1")]
    pub payment_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendSpontaneousPaymentRequest {
    #[prost(uint64, tag = "1")]
    pub amount_msat: u64,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendSpontaneousPaymentResponse {
    #[prost(string, tag = "1")]
    pub payment_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInvoiceRequest {
    #[prost(uint64, tag = "1")]
    pub amount_msat: u64,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub expiry_secs: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInvoiceResponse {
    #[prost(string, tag = "1")]
    pub invoice: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZeroAmountRequest {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub expiry_secs: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZeroAmountResponse {
    #[prost(string, tag = "1")]
    pub invoice: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPaymentRequest {
    #[prost(string, tag = "1")]
    pub payment_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPaymentResponse {
    #[prost(string, tag = "1")]
    pub details: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ben_ln_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BenLnClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BenLnClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BenLnClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BenLnClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BenLnClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodeInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/GetNodeInfo");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "GetNodeInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn new_address(
            &mut self,
            request: impl tonic::IntoRequest<super::NewAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/NewAddress");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "NewAddress"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignMessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/SignMessage");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "SignMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/Stop");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "Stop"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddPeerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/AddPeer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "AddPeer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_onchain_spendable(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOnchainSpendableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOnchainSpendableResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/GetOnchainSpendable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("benln.BenLn", "GetOnchainSpendable"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_total_onchain_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTotalOnchainBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTotalOnchainBalanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/GetTotalOnchainBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("benln.BenLn", "GetTotalOnchainBalance"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_to_address(
            &mut self,
            request: impl tonic::IntoRequest<super::SendToAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendToAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/SendToAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "SendToAddress"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListChannelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/ListChannels");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "ListChannels"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_peers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPeersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPeersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/ListPeers");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "ListPeers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePeerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/RemovePeer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "RemovePeer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenChannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/OpenChannel");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "OpenChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseChannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/CloseChannel");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "CloseChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sync_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncWalletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SyncWalletResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/SyncWallet");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "SyncWallet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendPaymentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/SendPayment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "SendPayment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_payment_with_amount(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPaymentWithAmountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendPaymentWithAmountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/SendPaymentWithAmount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("benln.BenLn", "SendPaymentWithAmount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_spontaneous_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::SendSpontaneousPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendSpontaneousPaymentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/SendSpontaneousPayment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("benln.BenLn", "SendSpontaneousPayment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInvoiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/CreateInvoice",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "CreateInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn zero_amount_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::ZeroAmountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ZeroAmountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/benln.BenLn/ZeroAmountInvoice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("benln.BenLn", "ZeroAmountInvoice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPaymentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/benln.BenLn/GetPayment");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("benln.BenLn", "GetPayment"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ben_ln_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BenLnServer.
    #[async_trait]
    pub trait BenLn: Send + Sync + 'static {
        async fn get_node_info(
            &self,
            request: tonic::Request<super::GetNodeInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodeInfoResponse>,
            tonic::Status,
        >;
        async fn new_address(
            &self,
            request: tonic::Request<super::NewAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewAddressResponse>,
            tonic::Status,
        >;
        async fn sign_message(
            &self,
            request: tonic::Request<super::SignMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignMessageResponse>,
            tonic::Status,
        >;
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopResponse>, tonic::Status>;
        async fn add_peer(
            &self,
            request: tonic::Request<super::AddPeerRequest>,
        ) -> std::result::Result<tonic::Response<super::AddPeerResponse>, tonic::Status>;
        async fn get_onchain_spendable(
            &self,
            request: tonic::Request<super::GetOnchainSpendableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOnchainSpendableResponse>,
            tonic::Status,
        >;
        async fn get_total_onchain_balance(
            &self,
            request: tonic::Request<super::GetTotalOnchainBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTotalOnchainBalanceResponse>,
            tonic::Status,
        >;
        async fn send_to_address(
            &self,
            request: tonic::Request<super::SendToAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendToAddressResponse>,
            tonic::Status,
        >;
        async fn list_channels(
            &self,
            request: tonic::Request<super::ListChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListChannelsResponse>,
            tonic::Status,
        >;
        async fn list_peers(
            &self,
            request: tonic::Request<super::ListPeersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPeersResponse>,
            tonic::Status,
        >;
        async fn remove_peer(
            &self,
            request: tonic::Request<super::RemovePeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePeerResponse>,
            tonic::Status,
        >;
        async fn open_channel(
            &self,
            request: tonic::Request<super::OpenChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenChannelResponse>,
            tonic::Status,
        >;
        async fn close_channel(
            &self,
            request: tonic::Request<super::CloseChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseChannelResponse>,
            tonic::Status,
        >;
        async fn sync_wallet(
            &self,
            request: tonic::Request<super::SyncWalletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SyncWalletResponse>,
            tonic::Status,
        >;
        async fn send_payment(
            &self,
            request: tonic::Request<super::SendPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendPaymentResponse>,
            tonic::Status,
        >;
        async fn send_payment_with_amount(
            &self,
            request: tonic::Request<super::SendPaymentWithAmountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendPaymentWithAmountResponse>,
            tonic::Status,
        >;
        async fn send_spontaneous_payment(
            &self,
            request: tonic::Request<super::SendSpontaneousPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendSpontaneousPaymentResponse>,
            tonic::Status,
        >;
        async fn create_invoice(
            &self,
            request: tonic::Request<super::CreateInvoiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInvoiceResponse>,
            tonic::Status,
        >;
        async fn zero_amount_invoice(
            &self,
            request: tonic::Request<super::ZeroAmountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ZeroAmountResponse>,
            tonic::Status,
        >;
        async fn get_payment(
            &self,
            request: tonic::Request<super::GetPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPaymentResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BenLnServer<T: BenLn> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BenLn> BenLnServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BenLnServer<T>
    where
        T: BenLn,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/benln.BenLn/GetNodeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeInfoSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::GetNodeInfoRequest>
                    for GetNodeInfoSvc<T> {
                        type Response = super::GetNodeInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNodeInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::get_node_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/NewAddress" => {
                    #[allow(non_camel_case_types)]
                    struct NewAddressSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::NewAddressRequest>
                    for NewAddressSvc<T> {
                        type Response = super::NewAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::new_address(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/SignMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessageSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::SignMessageRequest>
                    for SignMessageSvc<T> {
                        type Response = super::SignMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::sign_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::StopRequest>
                    for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::stop(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/AddPeer" => {
                    #[allow(non_camel_case_types)]
                    struct AddPeerSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::AddPeerRequest>
                    for AddPeerSvc<T> {
                        type Response = super::AddPeerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddPeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::add_peer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/GetOnchainSpendable" => {
                    #[allow(non_camel_case_types)]
                    struct GetOnchainSpendableSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::GetOnchainSpendableRequest>
                    for GetOnchainSpendableSvc<T> {
                        type Response = super::GetOnchainSpendableResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOnchainSpendableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::get_onchain_spendable(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOnchainSpendableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/GetTotalOnchainBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetTotalOnchainBalanceSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::GetTotalOnchainBalanceRequest>
                    for GetTotalOnchainBalanceSvc<T> {
                        type Response = super::GetTotalOnchainBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTotalOnchainBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::get_total_onchain_balance(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTotalOnchainBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/SendToAddress" => {
                    #[allow(non_camel_case_types)]
                    struct SendToAddressSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::SendToAddressRequest>
                    for SendToAddressSvc<T> {
                        type Response = super::SendToAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendToAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::send_to_address(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendToAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/ListChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListChannelsSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::ListChannelsRequest>
                    for ListChannelsSvc<T> {
                        type Response = super::ListChannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListChannelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::list_channels(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/ListPeers" => {
                    #[allow(non_camel_case_types)]
                    struct ListPeersSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::ListPeersRequest>
                    for ListPeersSvc<T> {
                        type Response = super::ListPeersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPeersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::list_peers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPeersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/RemovePeer" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePeerSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::RemovePeerRequest>
                    for RemovePeerSvc<T> {
                        type Response = super::RemovePeerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemovePeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::remove_peer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemovePeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/OpenChannel" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannelSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::OpenChannelRequest>
                    for OpenChannelSvc<T> {
                        type Response = super::OpenChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenChannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::open_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/CloseChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CloseChannelSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::CloseChannelRequest>
                    for CloseChannelSvc<T> {
                        type Response = super::CloseChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseChannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::close_channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/SyncWallet" => {
                    #[allow(non_camel_case_types)]
                    struct SyncWalletSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::SyncWalletRequest>
                    for SyncWalletSvc<T> {
                        type Response = super::SyncWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyncWalletRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::sync_wallet(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SyncWalletSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/SendPayment" => {
                    #[allow(non_camel_case_types)]
                    struct SendPaymentSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::SendPaymentRequest>
                    for SendPaymentSvc<T> {
                        type Response = super::SendPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::send_payment(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPaymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/SendPaymentWithAmount" => {
                    #[allow(non_camel_case_types)]
                    struct SendPaymentWithAmountSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::SendPaymentWithAmountRequest>
                    for SendPaymentWithAmountSvc<T> {
                        type Response = super::SendPaymentWithAmountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendPaymentWithAmountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::send_payment_with_amount(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPaymentWithAmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/SendSpontaneousPayment" => {
                    #[allow(non_camel_case_types)]
                    struct SendSpontaneousPaymentSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::SendSpontaneousPaymentRequest>
                    for SendSpontaneousPaymentSvc<T> {
                        type Response = super::SendSpontaneousPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendSpontaneousPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::send_spontaneous_payment(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendSpontaneousPaymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/CreateInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInvoiceSvc<T: BenLn>(pub Arc<T>);
                    impl<
                        T: BenLn,
                    > tonic::server::UnaryService<super::CreateInvoiceRequest>
                    for CreateInvoiceSvc<T> {
                        type Response = super::CreateInvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInvoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::create_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/ZeroAmountInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct ZeroAmountInvoiceSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::ZeroAmountRequest>
                    for ZeroAmountInvoiceSvc<T> {
                        type Response = super::ZeroAmountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ZeroAmountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::zero_amount_invoice(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZeroAmountInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/benln.BenLn/GetPayment" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaymentSvc<T: BenLn>(pub Arc<T>);
                    impl<T: BenLn> tonic::server::UnaryService<super::GetPaymentRequest>
                    for GetPaymentSvc<T> {
                        type Response = super::GetPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BenLn>::get_payment(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPaymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BenLn> Clone for BenLnServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: BenLn> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BenLn> tonic::server::NamedService for BenLnServer<T> {
        const NAME: &'static str = "benln.BenLn";
    }
}
