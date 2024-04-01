use unc_primitives::views::AllMinersView;
use crate::client_actor::ClientActor;
use crate::view_client::ViewClientActor;
use unc_network::types::{
    NetworkInfo, ReasonForBan, StateResponseInfo,
};
use unc_o11y::WithSpanContextExt;
use unc_primitives::block::{Approval, Block, BlockHeader};
use unc_primitives::challenge::Challenge;
use unc_primitives::chunk_validation::{ChunkEndorsement, ChunkStateWitness};
use unc_primitives::errors::InvalidTxError;
use unc_primitives::hash::CryptoHash;
use unc_primitives::network::{AnnounceAccount, PeerId};
use unc_primitives::transaction::SignedTransaction;
use unc_primitives::types::{AccountId, BlockHeight, EpochId, Power, ShardId};
use unc_primitives::views::FinalExecutionOutcomeView;
use unc_primitives::views::validator_power_view::ValidatorPowerView;

/// Transaction status query
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<Box<FinalExecutionOutcomeView>>")]
pub(crate) struct TxStatusRequest {
    pub tx_hash: CryptoHash,
    pub signer_account_id: AccountId,
}

/// Transaction status response
#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct TxStatusResponse(pub Box<FinalExecutionOutcomeView>);

/// Request a provider.
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<AccountId>")]
pub(crate) struct ProviderRequest(pub EpochId, pub BlockHeight);

/// Provider response.
#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct ProviderResponse {
    pub provider: AccountId,
}

/// Request all miners.
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<AllMinersView>")]
pub(crate) struct AllMinersRequest(pub CryptoHash);

/// All Miners response.
#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct AllMinersResponse {
    pub total_power: Power,
    pub miners: Vec<ValidatorPowerView>,
}

/// Request a block.
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<Box<Block>>")]
pub(crate) struct BlockRequest(pub CryptoHash);

/// Block response.
#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct BlockResponse {
    pub block: Block,
    pub peer_id: PeerId,
    pub was_requested: bool,
}

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct BlockApproval(pub Approval, pub PeerId);

/// Request headers.
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<Vec<BlockHeader>>")]
pub(crate) struct BlockHeadersRequest(pub Vec<CryptoHash>);

/// Headers response.
#[derive(actix::Message, Debug)]
#[rtype(result = "Result<(),ReasonForBan>")]
pub(crate) struct BlockHeadersResponse(pub Vec<BlockHeader>, pub PeerId);

/// State request header.
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<StateResponse>")]
pub struct StateRequestHeader {
    pub shard_id: ShardId,
    pub sync_hash: CryptoHash,
}

/// State request part.
#[derive(actix::Message, Debug)]
#[rtype(result = "Option<StateResponse>")]
pub struct StateRequestPart {
    pub shard_id: ShardId,
    pub sync_hash: CryptoHash,
    pub part_id: u64,
}

/// Response to state request.
#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct StateResponse(pub Box<StateResponseInfo>);

/// Account announcements that needs to be validated before being processed.
/// They are paired with last epoch id known to this announcement, in order to accept only
/// newer announcements.
#[derive(actix::Message, Debug)]
#[rtype(result = "Result<Vec<AnnounceAccount>,ReasonForBan>")]
pub(crate) struct AnnounceAccountRequest(pub Vec<(AnnounceAccount, Option<EpochId>)>);

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct SetNetworkInfo(pub NetworkInfo);

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct RecvChallenge(pub Challenge);


#[derive(actix::Message, Debug)]
#[rtype(result = "ProcessTxResponse")]
pub struct ProcessTxRequest {
    pub transaction: SignedTransaction,
    pub is_forwarded: bool,
    pub check_only: bool,
}

#[derive(actix::MessageResponse, Debug, PartialEq, Eq)]
pub enum ProcessTxResponse {
    /// No response.
    NoResponse,
    /// Valid transaction inserted into mempool as response to Transaction.
    ValidTx,
    /// Invalid transaction inserted into mempool as response to Transaction.
    InvalidTx(InvalidTxError),
    /// The request is routed to other shards
    RequestRouted,
    /// The node being queried does not track the shard needed and therefore cannot provide userful
    /// response.
    DoesNotTrackShard,
}

#[derive(actix::Message, Debug, PartialEq, Eq)]
#[rtype(result = "()")]
pub struct ChunkStateWitnessMessage(pub ChunkStateWitness);

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct ChunkEndorsementMessage(pub ChunkEndorsement);

pub struct Adapter {
    /// Address of the client actor.
    client_addr: actix::Addr<ClientActor>,
    /// Address of the view client actor.
    view_client_addr: actix::Addr<ViewClientActor>,
}

impl Adapter {
    pub fn new(
        client_addr: actix::Addr<ClientActor>,
        view_client_addr: actix::Addr<ViewClientActor>,
    ) -> Self {
        Self { client_addr, view_client_addr }
    }
}

#[async_trait::async_trait]
impl unc_network::client::Client for Adapter {
    async fn tx_status_request(
        &self,
        account_id: AccountId,
        tx_hash: CryptoHash,
    ) -> Option<Box<FinalExecutionOutcomeView>> {
        match self
            .view_client_addr
            .send(TxStatusRequest { tx_hash, signer_account_id: account_id }.with_span_context())
            .await
        {
            Ok(res) => res,
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                None
            }
        }
    }

    async fn tx_status_response(&self, tx_result: FinalExecutionOutcomeView) {
        match self
            .view_client_addr
            .send(TxStatusResponse(Box::new(tx_result.clone())).with_span_context())
            .await
        {
            Ok(()) => {}
            Err(err) => {
                tracing::error!("mailbox error: {err}");
            }
        }
    }

    async fn state_request_header(
        &self,
        shard_id: ShardId,
        sync_hash: CryptoHash,
    ) -> Result<Option<StateResponseInfo>, ReasonForBan> {
        match self
            .view_client_addr
            .send(StateRequestHeader { shard_id, sync_hash }.with_span_context())
            .await
        {
            Ok(Some(StateResponse(resp))) => Ok(Some(*resp)),
            Ok(None) => Ok(None),
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                Ok(None)
            }
        }
    }

    async fn state_request_part(
        &self,
        shard_id: ShardId,
        sync_hash: CryptoHash,
        part_id: u64,
    ) -> Result<Option<StateResponseInfo>, ReasonForBan> {
        match self
            .view_client_addr
            .send(StateRequestPart { shard_id, sync_hash, part_id }.with_span_context())
            .await
        {
            Ok(Some(StateResponse(resp))) => Ok(Some(*resp)),
            Ok(None) => Ok(None),
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                Ok(None)
            }
        }
    }

    async fn state_response(&self, info: StateResponseInfo) {
        match self.client_addr.send(StateResponse(Box::new(info)).with_span_context()).await {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }

    async fn block_approval(&self, approval: Approval, peer_id: PeerId) {
        match self.client_addr.send(BlockApproval(approval, peer_id).with_span_context()).await {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }

    async fn transaction(&self, transaction: SignedTransaction, is_forwarded: bool) {
        match self
            .client_addr
            .send(
                ProcessTxRequest { transaction, is_forwarded, check_only: false }
                    .with_span_context(),
            )
            .await
        {
            Ok(ProcessTxResponse::InvalidTx(err)) => {
                tracing::warn!(target: "network", ?err, "Received invalid tx");
                // TODO: count as malicious behavior?
            }
            Ok(_) => {}
            Err(err) => {
                tracing::error!("mailbox error: {err}");
            }
        }
    }

    async fn block_request(&self, hash: CryptoHash) -> Option<Box<Block>> {
        match self.view_client_addr.send(BlockRequest(hash).with_span_context()).await {
            Ok(res) => res,
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                None
            }
        }
    }

    async fn block_headers_request(&self, hashes: Vec<CryptoHash>) -> Option<Vec<BlockHeader>> {
        match self.view_client_addr.send(BlockHeadersRequest(hashes).with_span_context()).await {
            Ok(headers) => headers,
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                None
            }
        }
    }

    async fn block(&self, block: Block, peer_id: PeerId, was_requested: bool) {
        match self
            .client_addr
            .send(BlockResponse { block, peer_id, was_requested }.with_span_context())
            .await
        {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }

    async fn block_headers(
        &self,
        headers: Vec<BlockHeader>,
        peer_id: PeerId,
    ) -> Result<(), ReasonForBan> {
        match self
            .client_addr
            .send(BlockHeadersResponse(headers, peer_id).with_span_context())
            .await
        {
            Ok(res) => res,
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                Ok(())
            }
        }
    }

    async fn challenge(&self, challenge: Challenge) {
        match self.client_addr.send(RecvChallenge(challenge).with_span_context()).await {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }

    async fn network_info(&self, info: NetworkInfo) {
        match self.client_addr.send(SetNetworkInfo(info).with_span_context()).await {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }

    async fn announce_account(
        &self,
        accounts: Vec<(AnnounceAccount, Option<EpochId>)>,
    ) -> Result<Vec<AnnounceAccount>, ReasonForBan> {
        match self.view_client_addr.send(AnnounceAccountRequest(accounts).with_span_context()).await
        {
            Ok(res) => res,
            Err(err) => {
                tracing::error!("mailbox error: {err}");
                Ok(vec![])
            }
        }
    }

    async fn chunk_state_witness(&self, witness: ChunkStateWitness) {
        match self.client_addr.send(ChunkStateWitnessMessage(witness).with_span_context()).await {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }

    async fn chunk_endorsement(&self, endorsement: ChunkEndorsement) {
        match self.client_addr.send(ChunkEndorsementMessage(endorsement).with_span_context()).await
        {
            Ok(()) => {}
            Err(err) => tracing::error!("mailbox error: {err}"),
        }
    }
}
