use crate::{
    benln::{
        ben_ln_server::BenLn,
        AddPeerRequest, AddPeerResponse, CloseChannelRequest, CloseChannelResponse,
        CreateInvoiceRequest, CreateInvoiceResponse, GetNodeInfoRequest, GetNodeInfoResponse,
        GetOnchainSpendableRequest, GetOnchainSpendableResponse, GetPaymentRequest,
        GetPaymentResponse, GetTotalOnchainBalanceRequest, GetTotalOnchainBalanceResponse,
        ListChannelsRequest, ListChannelsResponse, NewAddressRequest, NewAddressResponse,
        OpenChannelRequest, OpenChannelResponse, RemovePeerRequest, RemovePeerResponse,
        SendPaymentRequest, SendPaymentResponse, SendPaymentWithAmountRequest,
        SendPaymentWithAmountResponse, SendSpontaneousPaymentRequest,
        SendSpontaneousPaymentResponse, SendToAddressRequest, SendToAddressResponse,
        SignMessageRequest, SignMessageResponse, StopRequest, StopResponse, SyncWalletRequest,
        SyncWalletResponse, ZeroAmountRequest, ZeroAmountResponse,
    },
    node::BenLnNode,
};
use ldk_node::{bitcoin::secp256k1::PublicKey, SocketAddress};
use std::str::FromStr;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl BenLn for BenLnNode {
    async fn get_node_info(
        &self,
        _request: Request<GetNodeInfoRequest>,
    ) -> Result<Response<GetNodeInfoResponse>, Status> {
        let node_id = self.node.node_id().to_string();
        let listening_address = match self.node.listening_address() {
            Some(addr) => addr.to_string(),
            None => "".to_string(),
        };
        let reply = GetNodeInfoResponse {
            node_id,
            listening_address,
        };
        Ok(Response::new(reply))
    }

    async fn new_address(
        &self,
        _request: Request<NewAddressRequest>,
    ) -> Result<Response<NewAddressResponse>, Status> {
        let address = self.node.new_onchain_address().unwrap().to_string();
        let reply = NewAddressResponse { address };
        Ok(Response::new(reply))
    }

    async fn sign_message(
        &self,
        request: Request<SignMessageRequest>,
    ) -> Result<Response<SignMessageResponse>, Status> {
        let msg_bytes = request.get_ref().msg.as_bytes();
        let signature = self.node.sign_message(msg_bytes).unwrap().to_string();
        let response = SignMessageResponse { signature };
        Ok(Response::new(response))
    }

    async fn stop(&self, _request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        self.node.stop().unwrap();
        Ok(Response::new(StopResponse {}))
    }

    async fn add_peer(
        &self,
        request: Request<AddPeerRequest>,
    ) -> Result<Response<AddPeerResponse>, Status> {
        self.node
            .connect(
                PublicKey::from_str(request.get_ref().pubkey.as_str()).unwrap(),
                SocketAddress::from_str(request.get_ref().uri.as_str()).unwrap(),
                request.get_ref().persist,
            )
            .unwrap();

        Ok(Response::new(AddPeerResponse {}))
    }

    async fn remove_peer(
        &self,
        _request: Request<RemovePeerRequest>,
    ) -> Result<Response<RemovePeerResponse>, Status> {
        todo!()
    }

    async fn open_channel(
        &self,
        _request: Request<OpenChannelRequest>,
    ) -> Result<Response<OpenChannelResponse>, Status> {
        todo!()
    }

    async fn close_channel(
        &self,
        _request: Request<CloseChannelRequest>,
    ) -> Result<Response<CloseChannelResponse>, Status> {
        todo!()
    }

    async fn create_invoice(
        &self,
        _request: Request<CreateInvoiceRequest>,
    ) -> Result<Response<CreateInvoiceResponse>, Status> {
        todo!()
    }

    async fn zero_amount_invoice(
        &self,
        _request: Request<ZeroAmountRequest>,
    ) -> Result<Response<ZeroAmountResponse>, Status> {
        todo!()
    }

    async fn send_payment(
        &self,
        _request: Request<SendPaymentRequest>,
    ) -> Result<Response<SendPaymentResponse>, Status> {
        todo!()
    }

    async fn send_payment_with_amount(
        &self,
        _request: Request<SendPaymentWithAmountRequest>,
    ) -> Result<Response<SendPaymentWithAmountResponse>, Status> {
        todo!()
    }

    async fn send_spontaneous_payment(
        &self,
        _request: Request<SendSpontaneousPaymentRequest>,
    ) -> Result<Response<SendSpontaneousPaymentResponse>, Status> {
        todo!()
    }

    async fn get_onchain_spendable(
        &self,
        _request: Request<GetOnchainSpendableRequest>,
    ) -> Result<Response<GetOnchainSpendableResponse>, Status> {
        todo!()
    }

    async fn get_total_onchain_balance(
        &self,
        _request: Request<GetTotalOnchainBalanceRequest>,
    ) -> Result<Response<GetTotalOnchainBalanceResponse>, Status> {
        todo!()
    }

    async fn send_to_address(
        &self,
        _request: Request<SendToAddressRequest>,
    ) -> Result<Response<SendToAddressResponse>, Status> {
        todo!()
    }

    async fn get_payment(
        &self,
        _request: Request<GetPaymentRequest>,
    ) -> Result<Response<GetPaymentResponse>, Status> {
        todo!()
    }

    async fn list_channels(
        &self,
        _request: Request<ListChannelsRequest>,
    ) -> Result<Response<ListChannelsResponse>, Status> {
        todo!()
    }

    async fn sync_wallet(
        &self,
        _request: Request<SyncWalletRequest>,
    ) -> Result<Response<SyncWalletResponse>, Status> {
        todo!()
    }
}
