#include <string.h>
namespace LDK {
class Event {
private:
	LDKEvent self;
public:
	Event(const Event&) = delete;
	~Event() { Event_free(self); }
	Event(Event&& o) : self(o.self) { o.self.inner = NULL; }
	Event(LDKEvent&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKEvent() { LDKEvent res = self; self.inner = NULL; return res; }
	LDKEvent* operator &() { return &self; }
};
class MessageSendEvent {
private:
	LDKMessageSendEvent self;
public:
	MessageSendEvent(const MessageSendEvent&) = delete;
	~MessageSendEvent() { MessageSendEvent_free(self); }
	MessageSendEvent(MessageSendEvent&& o) : self(o.self) { o.self.inner = NULL; }
	MessageSendEvent(LDKMessageSendEvent&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKMessageSendEvent() { LDKMessageSendEvent res = self; self.inner = NULL; return res; }
	LDKMessageSendEvent* operator &() { return &self; }
};
typedef LDKMessageSendEventsProvider MessageSendEventsProvider;
typedef LDKEventsProvider EventsProvider;
class APIError {
private:
	LDKAPIError self;
public:
	APIError(const APIError&) = delete;
	~APIError() { APIError_free(self); }
	APIError(APIError&& o) : self(o.self) { o.self.inner = NULL; }
	APIError(LDKAPIError&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKAPIError() { LDKAPIError res = self; self.inner = NULL; return res; }
	LDKAPIError* operator &() { return &self; }
};
typedef LDKLevel Level;
typedef LDKLogger Logger;
class UserConfig {
private:
	LDKUserConfig self;
public:
	UserConfig(const UserConfig&) = delete;
	~UserConfig() { UserConfig_free(self); }
	UserConfig(UserConfig&& o) : self(o.self) { o.self.inner = NULL; }
	UserConfig(LDKUserConfig&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUserConfig() { LDKUserConfig res = self; self.inner = NULL; return res; }
	LDKUserConfig* operator &() { return &self; }
};
class ChannelHandshakeConfig {
private:
	LDKChannelHandshakeConfig self;
public:
	ChannelHandshakeConfig(const ChannelHandshakeConfig&) = delete;
	~ChannelHandshakeConfig() { ChannelHandshakeConfig_free(self); }
	ChannelHandshakeConfig(ChannelHandshakeConfig&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelHandshakeConfig(LDKChannelHandshakeConfig&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelHandshakeConfig() { LDKChannelHandshakeConfig res = self; self.inner = NULL; return res; }
	LDKChannelHandshakeConfig* operator &() { return &self; }
};
class ChannelHandshakeLimits {
private:
	LDKChannelHandshakeLimits self;
public:
	ChannelHandshakeLimits(const ChannelHandshakeLimits&) = delete;
	~ChannelHandshakeLimits() { ChannelHandshakeLimits_free(self); }
	ChannelHandshakeLimits(ChannelHandshakeLimits&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelHandshakeLimits(LDKChannelHandshakeLimits&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelHandshakeLimits() { LDKChannelHandshakeLimits res = self; self.inner = NULL; return res; }
	LDKChannelHandshakeLimits* operator &() { return &self; }
};
class ChannelConfig {
private:
	LDKChannelConfig self;
public:
	ChannelConfig(const ChannelConfig&) = delete;
	~ChannelConfig() { ChannelConfig_free(self); }
	ChannelConfig(ChannelConfig&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelConfig(LDKChannelConfig&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelConfig() { LDKChannelConfig res = self; self.inner = NULL; return res; }
	LDKChannelConfig* operator &() { return &self; }
};
typedef LDKChainError ChainError;
typedef LDKChainWatchInterface ChainWatchInterface;
typedef LDKBroadcasterInterface BroadcasterInterface;
typedef LDKChainListener ChainListener;
typedef LDKConfirmationTarget ConfirmationTarget;
typedef LDKFeeEstimator FeeEstimator;
class ChainWatchedUtil {
private:
	LDKChainWatchedUtil self;
public:
	ChainWatchedUtil(const ChainWatchedUtil&) = delete;
	~ChainWatchedUtil() { ChainWatchedUtil_free(self); }
	ChainWatchedUtil(ChainWatchedUtil&& o) : self(o.self) { o.self.inner = NULL; }
	ChainWatchedUtil(LDKChainWatchedUtil&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChainWatchedUtil() { LDKChainWatchedUtil res = self; self.inner = NULL; return res; }
	LDKChainWatchedUtil* operator &() { return &self; }
};
class BlockNotifier {
private:
	LDKBlockNotifier self;
public:
	BlockNotifier(const BlockNotifier&) = delete;
	~BlockNotifier() { BlockNotifier_free(self); }
	BlockNotifier(BlockNotifier&& o) : self(o.self) { o.self.inner = NULL; }
	BlockNotifier(LDKBlockNotifier&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKBlockNotifier() { LDKBlockNotifier res = self; self.inner = NULL; return res; }
	LDKBlockNotifier* operator &() { return &self; }
};
class ChainWatchInterfaceUtil {
private:
	LDKChainWatchInterfaceUtil self;
public:
	ChainWatchInterfaceUtil(const ChainWatchInterfaceUtil&) = delete;
	~ChainWatchInterfaceUtil() { ChainWatchInterfaceUtil_free(self); }
	ChainWatchInterfaceUtil(ChainWatchInterfaceUtil&& o) : self(o.self) { o.self.inner = NULL; }
	ChainWatchInterfaceUtil(LDKChainWatchInterfaceUtil&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChainWatchInterfaceUtil() { LDKChainWatchInterfaceUtil res = self; self.inner = NULL; return res; }
	LDKChainWatchInterfaceUtil* operator &() { return &self; }
};
class OutPoint {
private:
	LDKOutPoint self;
public:
	OutPoint(const OutPoint&) = delete;
	~OutPoint() { OutPoint_free(self); }
	OutPoint(OutPoint&& o) : self(o.self) { o.self.inner = NULL; }
	OutPoint(LDKOutPoint&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKOutPoint() { LDKOutPoint res = self; self.inner = NULL; return res; }
	LDKOutPoint* operator &() { return &self; }
};
class SpendableOutputDescriptor {
private:
	LDKSpendableOutputDescriptor self;
public:
	SpendableOutputDescriptor(const SpendableOutputDescriptor&) = delete;
	~SpendableOutputDescriptor() { SpendableOutputDescriptor_free(self); }
	SpendableOutputDescriptor(SpendableOutputDescriptor&& o) : self(o.self) { o.self.inner = NULL; }
	SpendableOutputDescriptor(LDKSpendableOutputDescriptor&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKSpendableOutputDescriptor() { LDKSpendableOutputDescriptor res = self; self.inner = NULL; return res; }
	LDKSpendableOutputDescriptor* operator &() { return &self; }
};
typedef LDKChannelKeys ChannelKeys;
typedef LDKKeysInterface KeysInterface;
class InMemoryChannelKeys {
private:
	LDKInMemoryChannelKeys self;
public:
	InMemoryChannelKeys(const InMemoryChannelKeys&) = delete;
	~InMemoryChannelKeys() { InMemoryChannelKeys_free(self); }
	InMemoryChannelKeys(InMemoryChannelKeys&& o) : self(o.self) { o.self.inner = NULL; }
	InMemoryChannelKeys(LDKInMemoryChannelKeys&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKInMemoryChannelKeys() { LDKInMemoryChannelKeys res = self; self.inner = NULL; return res; }
	LDKInMemoryChannelKeys* operator &() { return &self; }
};
class KeysManager {
private:
	LDKKeysManager self;
public:
	KeysManager(const KeysManager&) = delete;
	~KeysManager() { KeysManager_free(self); }
	KeysManager(KeysManager&& o) : self(o.self) { o.self.inner = NULL; }
	KeysManager(LDKKeysManager&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKKeysManager() { LDKKeysManager res = self; self.inner = NULL; return res; }
	LDKKeysManager* operator &() { return &self; }
};
class ChannelManager {
private:
	LDKChannelManager self;
public:
	ChannelManager(const ChannelManager&) = delete;
	~ChannelManager() { ChannelManager_free(self); }
	ChannelManager(ChannelManager&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelManager(LDKChannelManager&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelManager() { LDKChannelManager res = self; self.inner = NULL; return res; }
	LDKChannelManager* operator &() { return &self; }
};
class ChannelDetails {
private:
	LDKChannelDetails self;
public:
	ChannelDetails(const ChannelDetails&) = delete;
	~ChannelDetails() { ChannelDetails_free(self); }
	ChannelDetails(ChannelDetails&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelDetails(LDKChannelDetails&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelDetails() { LDKChannelDetails res = self; self.inner = NULL; return res; }
	LDKChannelDetails* operator &() { return &self; }
};
class PaymentSendFailure {
private:
	LDKPaymentSendFailure self;
public:
	PaymentSendFailure(const PaymentSendFailure&) = delete;
	~PaymentSendFailure() { PaymentSendFailure_free(self); }
	PaymentSendFailure(PaymentSendFailure&& o) : self(o.self) { o.self.inner = NULL; }
	PaymentSendFailure(LDKPaymentSendFailure&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKPaymentSendFailure() { LDKPaymentSendFailure res = self; self.inner = NULL; return res; }
	LDKPaymentSendFailure* operator &() { return &self; }
};
class ChannelMonitorUpdate {
private:
	LDKChannelMonitorUpdate self;
public:
	ChannelMonitorUpdate(const ChannelMonitorUpdate&) = delete;
	~ChannelMonitorUpdate() { ChannelMonitorUpdate_free(self); }
	ChannelMonitorUpdate(ChannelMonitorUpdate&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelMonitorUpdate(LDKChannelMonitorUpdate&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelMonitorUpdate() { LDKChannelMonitorUpdate res = self; self.inner = NULL; return res; }
	LDKChannelMonitorUpdate* operator &() { return &self; }
};
typedef LDKChannelMonitorUpdateErr ChannelMonitorUpdateErr;
class MonitorUpdateError {
private:
	LDKMonitorUpdateError self;
public:
	MonitorUpdateError(const MonitorUpdateError&) = delete;
	~MonitorUpdateError() { MonitorUpdateError_free(self); }
	MonitorUpdateError(MonitorUpdateError&& o) : self(o.self) { o.self.inner = NULL; }
	MonitorUpdateError(LDKMonitorUpdateError&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKMonitorUpdateError() { LDKMonitorUpdateError res = self; self.inner = NULL; return res; }
	LDKMonitorUpdateError* operator &() { return &self; }
};
class HTLCUpdate {
private:
	LDKHTLCUpdate self;
public:
	HTLCUpdate(const HTLCUpdate&) = delete;
	~HTLCUpdate() { HTLCUpdate_free(self); }
	HTLCUpdate(HTLCUpdate&& o) : self(o.self) { o.self.inner = NULL; }
	HTLCUpdate(LDKHTLCUpdate&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKHTLCUpdate() { LDKHTLCUpdate res = self; self.inner = NULL; return res; }
	LDKHTLCUpdate* operator &() { return &self; }
};
class ChannelMonitor {
private:
	LDKChannelMonitor self;
public:
	ChannelMonitor(const ChannelMonitor&) = delete;
	~ChannelMonitor() { ChannelMonitor_free(self); }
	ChannelMonitor(ChannelMonitor&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelMonitor(LDKChannelMonitor&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelMonitor() { LDKChannelMonitor res = self; self.inner = NULL; return res; }
	LDKChannelMonitor* operator &() { return &self; }
};
typedef LDKManyChannelMonitor ManyChannelMonitor;
class DecodeError {
private:
	LDKDecodeError self;
public:
	DecodeError(const DecodeError&) = delete;
	~DecodeError() { DecodeError_free(self); }
	DecodeError(DecodeError&& o) : self(o.self) { o.self.inner = NULL; }
	DecodeError(LDKDecodeError&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKDecodeError() { LDKDecodeError res = self; self.inner = NULL; return res; }
	LDKDecodeError* operator &() { return &self; }
};
class Init {
private:
	LDKInit self;
public:
	Init(const Init&) = delete;
	~Init() { Init_free(self); }
	Init(Init&& o) : self(o.self) { o.self.inner = NULL; }
	Init(LDKInit&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKInit() { LDKInit res = self; self.inner = NULL; return res; }
	LDKInit* operator &() { return &self; }
};
class ErrorMessage {
private:
	LDKErrorMessage self;
public:
	ErrorMessage(const ErrorMessage&) = delete;
	~ErrorMessage() { ErrorMessage_free(self); }
	ErrorMessage(ErrorMessage&& o) : self(o.self) { o.self.inner = NULL; }
	ErrorMessage(LDKErrorMessage&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKErrorMessage() { LDKErrorMessage res = self; self.inner = NULL; return res; }
	LDKErrorMessage* operator &() { return &self; }
};
class Ping {
private:
	LDKPing self;
public:
	Ping(const Ping&) = delete;
	~Ping() { Ping_free(self); }
	Ping(Ping&& o) : self(o.self) { o.self.inner = NULL; }
	Ping(LDKPing&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKPing() { LDKPing res = self; self.inner = NULL; return res; }
	LDKPing* operator &() { return &self; }
};
class Pong {
private:
	LDKPong self;
public:
	Pong(const Pong&) = delete;
	~Pong() { Pong_free(self); }
	Pong(Pong&& o) : self(o.self) { o.self.inner = NULL; }
	Pong(LDKPong&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKPong() { LDKPong res = self; self.inner = NULL; return res; }
	LDKPong* operator &() { return &self; }
};
class OpenChannel {
private:
	LDKOpenChannel self;
public:
	OpenChannel(const OpenChannel&) = delete;
	~OpenChannel() { OpenChannel_free(self); }
	OpenChannel(OpenChannel&& o) : self(o.self) { o.self.inner = NULL; }
	OpenChannel(LDKOpenChannel&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKOpenChannel() { LDKOpenChannel res = self; self.inner = NULL; return res; }
	LDKOpenChannel* operator &() { return &self; }
};
class AcceptChannel {
private:
	LDKAcceptChannel self;
public:
	AcceptChannel(const AcceptChannel&) = delete;
	~AcceptChannel() { AcceptChannel_free(self); }
	AcceptChannel(AcceptChannel&& o) : self(o.self) { o.self.inner = NULL; }
	AcceptChannel(LDKAcceptChannel&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKAcceptChannel() { LDKAcceptChannel res = self; self.inner = NULL; return res; }
	LDKAcceptChannel* operator &() { return &self; }
};
class FundingCreated {
private:
	LDKFundingCreated self;
public:
	FundingCreated(const FundingCreated&) = delete;
	~FundingCreated() { FundingCreated_free(self); }
	FundingCreated(FundingCreated&& o) : self(o.self) { o.self.inner = NULL; }
	FundingCreated(LDKFundingCreated&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKFundingCreated() { LDKFundingCreated res = self; self.inner = NULL; return res; }
	LDKFundingCreated* operator &() { return &self; }
};
class FundingSigned {
private:
	LDKFundingSigned self;
public:
	FundingSigned(const FundingSigned&) = delete;
	~FundingSigned() { FundingSigned_free(self); }
	FundingSigned(FundingSigned&& o) : self(o.self) { o.self.inner = NULL; }
	FundingSigned(LDKFundingSigned&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKFundingSigned() { LDKFundingSigned res = self; self.inner = NULL; return res; }
	LDKFundingSigned* operator &() { return &self; }
};
class FundingLocked {
private:
	LDKFundingLocked self;
public:
	FundingLocked(const FundingLocked&) = delete;
	~FundingLocked() { FundingLocked_free(self); }
	FundingLocked(FundingLocked&& o) : self(o.self) { o.self.inner = NULL; }
	FundingLocked(LDKFundingLocked&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKFundingLocked() { LDKFundingLocked res = self; self.inner = NULL; return res; }
	LDKFundingLocked* operator &() { return &self; }
};
class Shutdown {
private:
	LDKShutdown self;
public:
	Shutdown(const Shutdown&) = delete;
	~Shutdown() { Shutdown_free(self); }
	Shutdown(Shutdown&& o) : self(o.self) { o.self.inner = NULL; }
	Shutdown(LDKShutdown&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKShutdown() { LDKShutdown res = self; self.inner = NULL; return res; }
	LDKShutdown* operator &() { return &self; }
};
class ClosingSigned {
private:
	LDKClosingSigned self;
public:
	ClosingSigned(const ClosingSigned&) = delete;
	~ClosingSigned() { ClosingSigned_free(self); }
	ClosingSigned(ClosingSigned&& o) : self(o.self) { o.self.inner = NULL; }
	ClosingSigned(LDKClosingSigned&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKClosingSigned() { LDKClosingSigned res = self; self.inner = NULL; return res; }
	LDKClosingSigned* operator &() { return &self; }
};
class UpdateAddHTLC {
private:
	LDKUpdateAddHTLC self;
public:
	UpdateAddHTLC(const UpdateAddHTLC&) = delete;
	~UpdateAddHTLC() { UpdateAddHTLC_free(self); }
	UpdateAddHTLC(UpdateAddHTLC&& o) : self(o.self) { o.self.inner = NULL; }
	UpdateAddHTLC(LDKUpdateAddHTLC&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUpdateAddHTLC() { LDKUpdateAddHTLC res = self; self.inner = NULL; return res; }
	LDKUpdateAddHTLC* operator &() { return &self; }
};
class UpdateFulfillHTLC {
private:
	LDKUpdateFulfillHTLC self;
public:
	UpdateFulfillHTLC(const UpdateFulfillHTLC&) = delete;
	~UpdateFulfillHTLC() { UpdateFulfillHTLC_free(self); }
	UpdateFulfillHTLC(UpdateFulfillHTLC&& o) : self(o.self) { o.self.inner = NULL; }
	UpdateFulfillHTLC(LDKUpdateFulfillHTLC&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUpdateFulfillHTLC() { LDKUpdateFulfillHTLC res = self; self.inner = NULL; return res; }
	LDKUpdateFulfillHTLC* operator &() { return &self; }
};
class UpdateFailHTLC {
private:
	LDKUpdateFailHTLC self;
public:
	UpdateFailHTLC(const UpdateFailHTLC&) = delete;
	~UpdateFailHTLC() { UpdateFailHTLC_free(self); }
	UpdateFailHTLC(UpdateFailHTLC&& o) : self(o.self) { o.self.inner = NULL; }
	UpdateFailHTLC(LDKUpdateFailHTLC&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUpdateFailHTLC() { LDKUpdateFailHTLC res = self; self.inner = NULL; return res; }
	LDKUpdateFailHTLC* operator &() { return &self; }
};
class UpdateFailMalformedHTLC {
private:
	LDKUpdateFailMalformedHTLC self;
public:
	UpdateFailMalformedHTLC(const UpdateFailMalformedHTLC&) = delete;
	~UpdateFailMalformedHTLC() { UpdateFailMalformedHTLC_free(self); }
	UpdateFailMalformedHTLC(UpdateFailMalformedHTLC&& o) : self(o.self) { o.self.inner = NULL; }
	UpdateFailMalformedHTLC(LDKUpdateFailMalformedHTLC&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUpdateFailMalformedHTLC() { LDKUpdateFailMalformedHTLC res = self; self.inner = NULL; return res; }
	LDKUpdateFailMalformedHTLC* operator &() { return &self; }
};
class CommitmentSigned {
private:
	LDKCommitmentSigned self;
public:
	CommitmentSigned(const CommitmentSigned&) = delete;
	~CommitmentSigned() { CommitmentSigned_free(self); }
	CommitmentSigned(CommitmentSigned&& o) : self(o.self) { o.self.inner = NULL; }
	CommitmentSigned(LDKCommitmentSigned&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKCommitmentSigned() { LDKCommitmentSigned res = self; self.inner = NULL; return res; }
	LDKCommitmentSigned* operator &() { return &self; }
};
class RevokeAndACK {
private:
	LDKRevokeAndACK self;
public:
	RevokeAndACK(const RevokeAndACK&) = delete;
	~RevokeAndACK() { RevokeAndACK_free(self); }
	RevokeAndACK(RevokeAndACK&& o) : self(o.self) { o.self.inner = NULL; }
	RevokeAndACK(LDKRevokeAndACK&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKRevokeAndACK() { LDKRevokeAndACK res = self; self.inner = NULL; return res; }
	LDKRevokeAndACK* operator &() { return &self; }
};
class UpdateFee {
private:
	LDKUpdateFee self;
public:
	UpdateFee(const UpdateFee&) = delete;
	~UpdateFee() { UpdateFee_free(self); }
	UpdateFee(UpdateFee&& o) : self(o.self) { o.self.inner = NULL; }
	UpdateFee(LDKUpdateFee&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUpdateFee() { LDKUpdateFee res = self; self.inner = NULL; return res; }
	LDKUpdateFee* operator &() { return &self; }
};
class ChannelReestablish {
private:
	LDKChannelReestablish self;
public:
	ChannelReestablish(const ChannelReestablish&) = delete;
	~ChannelReestablish() { ChannelReestablish_free(self); }
	ChannelReestablish(ChannelReestablish&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelReestablish(LDKChannelReestablish&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelReestablish() { LDKChannelReestablish res = self; self.inner = NULL; return res; }
	LDKChannelReestablish* operator &() { return &self; }
};
class AnnouncementSignatures {
private:
	LDKAnnouncementSignatures self;
public:
	AnnouncementSignatures(const AnnouncementSignatures&) = delete;
	~AnnouncementSignatures() { AnnouncementSignatures_free(self); }
	AnnouncementSignatures(AnnouncementSignatures&& o) : self(o.self) { o.self.inner = NULL; }
	AnnouncementSignatures(LDKAnnouncementSignatures&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKAnnouncementSignatures() { LDKAnnouncementSignatures res = self; self.inner = NULL; return res; }
	LDKAnnouncementSignatures* operator &() { return &self; }
};
class NetAddress {
private:
	LDKNetAddress self;
public:
	NetAddress(const NetAddress&) = delete;
	~NetAddress() { NetAddress_free(self); }
	NetAddress(NetAddress&& o) : self(o.self) { o.self.inner = NULL; }
	NetAddress(LDKNetAddress&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNetAddress() { LDKNetAddress res = self; self.inner = NULL; return res; }
	LDKNetAddress* operator &() { return &self; }
};
class UnsignedNodeAnnouncement {
private:
	LDKUnsignedNodeAnnouncement self;
public:
	UnsignedNodeAnnouncement(const UnsignedNodeAnnouncement&) = delete;
	~UnsignedNodeAnnouncement() { UnsignedNodeAnnouncement_free(self); }
	UnsignedNodeAnnouncement(UnsignedNodeAnnouncement&& o) : self(o.self) { o.self.inner = NULL; }
	UnsignedNodeAnnouncement(LDKUnsignedNodeAnnouncement&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUnsignedNodeAnnouncement() { LDKUnsignedNodeAnnouncement res = self; self.inner = NULL; return res; }
	LDKUnsignedNodeAnnouncement* operator &() { return &self; }
};
class NodeAnnouncement {
private:
	LDKNodeAnnouncement self;
public:
	NodeAnnouncement(const NodeAnnouncement&) = delete;
	~NodeAnnouncement() { NodeAnnouncement_free(self); }
	NodeAnnouncement(NodeAnnouncement&& o) : self(o.self) { o.self.inner = NULL; }
	NodeAnnouncement(LDKNodeAnnouncement&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNodeAnnouncement() { LDKNodeAnnouncement res = self; self.inner = NULL; return res; }
	LDKNodeAnnouncement* operator &() { return &self; }
};
class UnsignedChannelAnnouncement {
private:
	LDKUnsignedChannelAnnouncement self;
public:
	UnsignedChannelAnnouncement(const UnsignedChannelAnnouncement&) = delete;
	~UnsignedChannelAnnouncement() { UnsignedChannelAnnouncement_free(self); }
	UnsignedChannelAnnouncement(UnsignedChannelAnnouncement&& o) : self(o.self) { o.self.inner = NULL; }
	UnsignedChannelAnnouncement(LDKUnsignedChannelAnnouncement&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKUnsignedChannelAnnouncement() { LDKUnsignedChannelAnnouncement res = self; self.inner = NULL; return res; }
	LDKUnsignedChannelAnnouncement* operator &() { return &self; }
};
class ChannelAnnouncement {
private:
	LDKChannelAnnouncement self;
public:
	ChannelAnnouncement(const ChannelAnnouncement&) = delete;
	~ChannelAnnouncement() { ChannelAnnouncement_free(self); }
	ChannelAnnouncement(ChannelAnnouncement&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelAnnouncement(LDKChannelAnnouncement&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelAnnouncement() { LDKChannelAnnouncement res = self; self.inner = NULL; return res; }
	LDKChannelAnnouncement* operator &() { return &self; }
};
class ChannelUpdate {
private:
	LDKChannelUpdate self;
public:
	ChannelUpdate(const ChannelUpdate&) = delete;
	~ChannelUpdate() { ChannelUpdate_free(self); }
	ChannelUpdate(ChannelUpdate&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelUpdate(LDKChannelUpdate&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelUpdate() { LDKChannelUpdate res = self; self.inner = NULL; return res; }
	LDKChannelUpdate* operator &() { return &self; }
};
class LightningError {
private:
	LDKLightningError self;
public:
	LightningError(const LightningError&) = delete;
	~LightningError() { LightningError_free(self); }
	LightningError(LightningError&& o) : self(o.self) { o.self.inner = NULL; }
	LightningError(LDKLightningError&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKLightningError() { LDKLightningError res = self; self.inner = NULL; return res; }
	LDKLightningError* operator &() { return &self; }
};
class CommitmentUpdate {
private:
	LDKCommitmentUpdate self;
public:
	CommitmentUpdate(const CommitmentUpdate&) = delete;
	~CommitmentUpdate() { CommitmentUpdate_free(self); }
	CommitmentUpdate(CommitmentUpdate&& o) : self(o.self) { o.self.inner = NULL; }
	CommitmentUpdate(LDKCommitmentUpdate&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKCommitmentUpdate() { LDKCommitmentUpdate res = self; self.inner = NULL; return res; }
	LDKCommitmentUpdate* operator &() { return &self; }
};
class HTLCFailChannelUpdate {
private:
	LDKHTLCFailChannelUpdate self;
public:
	HTLCFailChannelUpdate(const HTLCFailChannelUpdate&) = delete;
	~HTLCFailChannelUpdate() { HTLCFailChannelUpdate_free(self); }
	HTLCFailChannelUpdate(HTLCFailChannelUpdate&& o) : self(o.self) { o.self.inner = NULL; }
	HTLCFailChannelUpdate(LDKHTLCFailChannelUpdate&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKHTLCFailChannelUpdate() { LDKHTLCFailChannelUpdate res = self; self.inner = NULL; return res; }
	LDKHTLCFailChannelUpdate* operator &() { return &self; }
};
typedef LDKChannelMessageHandler ChannelMessageHandler;
typedef LDKRoutingMessageHandler RoutingMessageHandler;
class MessageHandler {
private:
	LDKMessageHandler self;
public:
	MessageHandler(const MessageHandler&) = delete;
	~MessageHandler() { MessageHandler_free(self); }
	MessageHandler(MessageHandler&& o) : self(o.self) { o.self.inner = NULL; }
	MessageHandler(LDKMessageHandler&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKMessageHandler() { LDKMessageHandler res = self; self.inner = NULL; return res; }
	LDKMessageHandler* operator &() { return &self; }
};
typedef LDKSocketDescriptor SocketDescriptor;
class PeerHandleError {
private:
	LDKPeerHandleError self;
public:
	PeerHandleError(const PeerHandleError&) = delete;
	~PeerHandleError() { PeerHandleError_free(self); }
	PeerHandleError(PeerHandleError&& o) : self(o.self) { o.self.inner = NULL; }
	PeerHandleError(LDKPeerHandleError&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKPeerHandleError() { LDKPeerHandleError res = self; self.inner = NULL; return res; }
	LDKPeerHandleError* operator &() { return &self; }
};
class PeerManager {
private:
	LDKPeerManager self;
public:
	PeerManager(const PeerManager&) = delete;
	~PeerManager() { PeerManager_free(self); }
	PeerManager(PeerManager&& o) : self(o.self) { o.self.inner = NULL; }
	PeerManager(LDKPeerManager&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKPeerManager() { LDKPeerManager res = self; self.inner = NULL; return res; }
	LDKPeerManager* operator &() { return &self; }
};
class TxCreationKeys {
private:
	LDKTxCreationKeys self;
public:
	TxCreationKeys(const TxCreationKeys&) = delete;
	~TxCreationKeys() { TxCreationKeys_free(self); }
	TxCreationKeys(TxCreationKeys&& o) : self(o.self) { o.self.inner = NULL; }
	TxCreationKeys(LDKTxCreationKeys&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKTxCreationKeys() { LDKTxCreationKeys res = self; self.inner = NULL; return res; }
	LDKTxCreationKeys* operator &() { return &self; }
};
class ChannelPublicKeys {
private:
	LDKChannelPublicKeys self;
public:
	ChannelPublicKeys(const ChannelPublicKeys&) = delete;
	~ChannelPublicKeys() { ChannelPublicKeys_free(self); }
	ChannelPublicKeys(ChannelPublicKeys&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelPublicKeys(LDKChannelPublicKeys&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelPublicKeys() { LDKChannelPublicKeys res = self; self.inner = NULL; return res; }
	LDKChannelPublicKeys* operator &() { return &self; }
};
class HTLCOutputInCommitment {
private:
	LDKHTLCOutputInCommitment self;
public:
	HTLCOutputInCommitment(const HTLCOutputInCommitment&) = delete;
	~HTLCOutputInCommitment() { HTLCOutputInCommitment_free(self); }
	HTLCOutputInCommitment(HTLCOutputInCommitment&& o) : self(o.self) { o.self.inner = NULL; }
	HTLCOutputInCommitment(LDKHTLCOutputInCommitment&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKHTLCOutputInCommitment() { LDKHTLCOutputInCommitment res = self; self.inner = NULL; return res; }
	LDKHTLCOutputInCommitment* operator &() { return &self; }
};
class LocalCommitmentTransaction {
private:
	LDKLocalCommitmentTransaction self;
public:
	LocalCommitmentTransaction(const LocalCommitmentTransaction&) = delete;
	~LocalCommitmentTransaction() { LocalCommitmentTransaction_free(self); }
	LocalCommitmentTransaction(LocalCommitmentTransaction&& o) : self(o.self) { o.self.inner = NULL; }
	LocalCommitmentTransaction(LDKLocalCommitmentTransaction&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKLocalCommitmentTransaction() { LDKLocalCommitmentTransaction res = self; self.inner = NULL; return res; }
	LDKLocalCommitmentTransaction* operator &() { return &self; }
};
class InitFeatures {
private:
	LDKInitFeatures self;
public:
	InitFeatures(const InitFeatures&) = delete;
	~InitFeatures() { InitFeatures_free(self); }
	InitFeatures(InitFeatures&& o) : self(o.self) { o.self.inner = NULL; }
	InitFeatures(LDKInitFeatures&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKInitFeatures() { LDKInitFeatures res = self; self.inner = NULL; return res; }
	LDKInitFeatures* operator &() { return &self; }
};
class NodeFeatures {
private:
	LDKNodeFeatures self;
public:
	NodeFeatures(const NodeFeatures&) = delete;
	~NodeFeatures() { NodeFeatures_free(self); }
	NodeFeatures(NodeFeatures&& o) : self(o.self) { o.self.inner = NULL; }
	NodeFeatures(LDKNodeFeatures&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNodeFeatures() { LDKNodeFeatures res = self; self.inner = NULL; return res; }
	LDKNodeFeatures* operator &() { return &self; }
};
class ChannelFeatures {
private:
	LDKChannelFeatures self;
public:
	ChannelFeatures(const ChannelFeatures&) = delete;
	~ChannelFeatures() { ChannelFeatures_free(self); }
	ChannelFeatures(ChannelFeatures&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelFeatures(LDKChannelFeatures&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelFeatures() { LDKChannelFeatures res = self; self.inner = NULL; return res; }
	LDKChannelFeatures* operator &() { return &self; }
};
class RouteHop {
private:
	LDKRouteHop self;
public:
	RouteHop(const RouteHop&) = delete;
	~RouteHop() { RouteHop_free(self); }
	RouteHop(RouteHop&& o) : self(o.self) { o.self.inner = NULL; }
	RouteHop(LDKRouteHop&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKRouteHop() { LDKRouteHop res = self; self.inner = NULL; return res; }
	LDKRouteHop* operator &() { return &self; }
};
class Route {
private:
	LDKRoute self;
public:
	Route(const Route&) = delete;
	~Route() { Route_free(self); }
	Route(Route&& o) : self(o.self) { o.self.inner = NULL; }
	Route(LDKRoute&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKRoute() { LDKRoute res = self; self.inner = NULL; return res; }
	LDKRoute* operator &() { return &self; }
};
class RouteHint {
private:
	LDKRouteHint self;
public:
	RouteHint(const RouteHint&) = delete;
	~RouteHint() { RouteHint_free(self); }
	RouteHint(RouteHint&& o) : self(o.self) { o.self.inner = NULL; }
	RouteHint(LDKRouteHint&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKRouteHint() { LDKRouteHint res = self; self.inner = NULL; return res; }
	LDKRouteHint* operator &() { return &self; }
};
class NetGraphMsgHandler {
private:
	LDKNetGraphMsgHandler self;
public:
	NetGraphMsgHandler(const NetGraphMsgHandler&) = delete;
	~NetGraphMsgHandler() { NetGraphMsgHandler_free(self); }
	NetGraphMsgHandler(NetGraphMsgHandler&& o) : self(o.self) { o.self.inner = NULL; }
	NetGraphMsgHandler(LDKNetGraphMsgHandler&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNetGraphMsgHandler() { LDKNetGraphMsgHandler res = self; self.inner = NULL; return res; }
	LDKNetGraphMsgHandler* operator &() { return &self; }
};
class DirectionalChannelInfo {
private:
	LDKDirectionalChannelInfo self;
public:
	DirectionalChannelInfo(const DirectionalChannelInfo&) = delete;
	~DirectionalChannelInfo() { DirectionalChannelInfo_free(self); }
	DirectionalChannelInfo(DirectionalChannelInfo&& o) : self(o.self) { o.self.inner = NULL; }
	DirectionalChannelInfo(LDKDirectionalChannelInfo&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKDirectionalChannelInfo() { LDKDirectionalChannelInfo res = self; self.inner = NULL; return res; }
	LDKDirectionalChannelInfo* operator &() { return &self; }
};
class ChannelInfo {
private:
	LDKChannelInfo self;
public:
	ChannelInfo(const ChannelInfo&) = delete;
	~ChannelInfo() { ChannelInfo_free(self); }
	ChannelInfo(ChannelInfo&& o) : self(o.self) { o.self.inner = NULL; }
	ChannelInfo(LDKChannelInfo&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKChannelInfo() { LDKChannelInfo res = self; self.inner = NULL; return res; }
	LDKChannelInfo* operator &() { return &self; }
};
class RoutingFees {
private:
	LDKRoutingFees self;
public:
	RoutingFees(const RoutingFees&) = delete;
	~RoutingFees() { RoutingFees_free(self); }
	RoutingFees(RoutingFees&& o) : self(o.self) { o.self.inner = NULL; }
	RoutingFees(LDKRoutingFees&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKRoutingFees() { LDKRoutingFees res = self; self.inner = NULL; return res; }
	LDKRoutingFees* operator &() { return &self; }
};
class NodeAnnouncementInfo {
private:
	LDKNodeAnnouncementInfo self;
public:
	NodeAnnouncementInfo(const NodeAnnouncementInfo&) = delete;
	~NodeAnnouncementInfo() { NodeAnnouncementInfo_free(self); }
	NodeAnnouncementInfo(NodeAnnouncementInfo&& o) : self(o.self) { o.self.inner = NULL; }
	NodeAnnouncementInfo(LDKNodeAnnouncementInfo&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNodeAnnouncementInfo() { LDKNodeAnnouncementInfo res = self; self.inner = NULL; return res; }
	LDKNodeAnnouncementInfo* operator &() { return &self; }
};
class NodeInfo {
private:
	LDKNodeInfo self;
public:
	NodeInfo(const NodeInfo&) = delete;
	~NodeInfo() { NodeInfo_free(self); }
	NodeInfo(NodeInfo&& o) : self(o.self) { o.self.inner = NULL; }
	NodeInfo(LDKNodeInfo&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNodeInfo() { LDKNodeInfo res = self; self.inner = NULL; return res; }
	LDKNodeInfo* operator &() { return &self; }
};
class NetworkGraph {
private:
	LDKNetworkGraph self;
public:
	NetworkGraph(const NetworkGraph&) = delete;
	~NetworkGraph() { NetworkGraph_free(self); }
	NetworkGraph(NetworkGraph&& o) : self(o.self) { o.self.inner = NULL; }
	NetworkGraph(LDKNetworkGraph&& m_self) : self(m_self) { m_self.inner = NULL; }
	operator LDKNetworkGraph() { LDKNetworkGraph res = self; self.inner = NULL; return res; }
	LDKNetworkGraph* operator &() { return &self; }
};
struct C2Tuple_SecretKey_u832Z {
	LDKC2Tuple_SecretKey_u832Z self;
	C2Tuple_SecretKey_u832Z(const C2Tuple_SecretKey_u832Z&) = delete;
	~C2Tuple_SecretKey_u832Z() { C2Tuple_SecretKey_u832Z_free(self); }
	C2Tuple_SecretKey_u832Z(C2Tuple_SecretKey_u832Z&& o) : self(o.self) { memset(&o, 0, sizeof(C2Tuple_SecretKey_u832Z)); }
	C2Tuple_SecretKey_u832Z(LDKC2Tuple_SecretKey_u832Z&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKC2Tuple_SecretKey_u832Z)); }
	operator LDKC2Tuple_SecretKey_u832Z() { LDKC2Tuple_SecretKey_u832Z res = self; memset(&self, 0, sizeof(LDKC2Tuple_SecretKey_u832Z)); return res; }
	LDKC2Tuple_SecretKey_u832Z* operator &() { return &self; }
};
struct CVec_u8Z {
	LDKCVec_u8Z self;
	CVec_u8Z(const CVec_u8Z&) = delete;
	~CVec_u8Z() { CVec_u8Z_free(self); }
	CVec_u8Z(CVec_u8Z&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_u8Z)); }
	CVec_u8Z(LDKCVec_u8Z&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_u8Z)); }
	operator LDKCVec_u8Z() { LDKCVec_u8Z res = self; memset(&self, 0, sizeof(LDKCVec_u8Z)); return res; }
	LDKCVec_u8Z* operator &() { return &self; }
};
struct CVec_u64Z {
	LDKCVec_u64Z self;
	CVec_u64Z(const CVec_u64Z&) = delete;
	~CVec_u64Z() { CVec_u64Z_free(self); }
	CVec_u64Z(CVec_u64Z&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_u64Z)); }
	CVec_u64Z(LDKCVec_u64Z&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_u64Z)); }
	operator LDKCVec_u64Z() { LDKCVec_u64Z res = self; memset(&self, 0, sizeof(LDKCVec_u64Z)); return res; }
	LDKCVec_u64Z* operator &() { return &self; }
};
struct CVec_UpdateFailMalformedHTLCZ {
	LDKCVec_UpdateFailMalformedHTLCZ self;
	CVec_UpdateFailMalformedHTLCZ(const CVec_UpdateFailMalformedHTLCZ&) = delete;
	~CVec_UpdateFailMalformedHTLCZ() { CVec_UpdateFailMalformedHTLCZ_free(self); }
	CVec_UpdateFailMalformedHTLCZ(CVec_UpdateFailMalformedHTLCZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_UpdateFailMalformedHTLCZ)); }
	CVec_UpdateFailMalformedHTLCZ(LDKCVec_UpdateFailMalformedHTLCZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_UpdateFailMalformedHTLCZ)); }
	operator LDKCVec_UpdateFailMalformedHTLCZ() { LDKCVec_UpdateFailMalformedHTLCZ res = self; memset(&self, 0, sizeof(LDKCVec_UpdateFailMalformedHTLCZ)); return res; }
	LDKCVec_UpdateFailMalformedHTLCZ* operator &() { return &self; }
};
struct CResult_boolLightningErrorZ {
	LDKCResult_boolLightningErrorZ self;
	CResult_boolLightningErrorZ(const CResult_boolLightningErrorZ&) = delete;
	~CResult_boolLightningErrorZ() { CResult_boolLightningErrorZ_free(self); }
	CResult_boolLightningErrorZ(CResult_boolLightningErrorZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_boolLightningErrorZ)); }
	CResult_boolLightningErrorZ(LDKCResult_boolLightningErrorZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_boolLightningErrorZ)); }
	operator LDKCResult_boolLightningErrorZ() { LDKCResult_boolLightningErrorZ res = self; memset(&self, 0, sizeof(LDKCResult_boolLightningErrorZ)); return res; }
	LDKCResult_boolLightningErrorZ* operator &() { return &self; }
};
struct CVec_MessageSendEventZ {
	LDKCVec_MessageSendEventZ self;
	CVec_MessageSendEventZ(const CVec_MessageSendEventZ&) = delete;
	~CVec_MessageSendEventZ() { CVec_MessageSendEventZ_free(self); }
	CVec_MessageSendEventZ(CVec_MessageSendEventZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_MessageSendEventZ)); }
	CVec_MessageSendEventZ(LDKCVec_MessageSendEventZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_MessageSendEventZ)); }
	operator LDKCVec_MessageSendEventZ() { LDKCVec_MessageSendEventZ res = self; memset(&self, 0, sizeof(LDKCVec_MessageSendEventZ)); return res; }
	LDKCVec_MessageSendEventZ* operator &() { return &self; }
};
struct CVec_EventZ {
	LDKCVec_EventZ self;
	CVec_EventZ(const CVec_EventZ&) = delete;
	~CVec_EventZ() { CVec_EventZ_free(self); }
	CVec_EventZ(CVec_EventZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_EventZ)); }
	CVec_EventZ(LDKCVec_EventZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_EventZ)); }
	operator LDKCVec_EventZ() { LDKCVec_EventZ res = self; memset(&self, 0, sizeof(LDKCVec_EventZ)); return res; }
	LDKCVec_EventZ* operator &() { return &self; }
};
struct CVec_HTLCUpdateZ {
	LDKCVec_HTLCUpdateZ self;
	CVec_HTLCUpdateZ(const CVec_HTLCUpdateZ&) = delete;
	~CVec_HTLCUpdateZ() { CVec_HTLCUpdateZ_free(self); }
	CVec_HTLCUpdateZ(CVec_HTLCUpdateZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_HTLCUpdateZ)); }
	CVec_HTLCUpdateZ(LDKCVec_HTLCUpdateZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_HTLCUpdateZ)); }
	operator LDKCVec_HTLCUpdateZ() { LDKCVec_HTLCUpdateZ res = self; memset(&self, 0, sizeof(LDKCVec_HTLCUpdateZ)); return res; }
	LDKCVec_HTLCUpdateZ* operator &() { return &self; }
};
struct CVec_PublicKeyZ {
	LDKCVec_PublicKeyZ self;
	CVec_PublicKeyZ(const CVec_PublicKeyZ&) = delete;
	~CVec_PublicKeyZ() { CVec_PublicKeyZ_free(self); }
	CVec_PublicKeyZ(CVec_PublicKeyZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_PublicKeyZ)); }
	CVec_PublicKeyZ(LDKCVec_PublicKeyZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_PublicKeyZ)); }
	operator LDKCVec_PublicKeyZ() { LDKCVec_PublicKeyZ res = self; memset(&self, 0, sizeof(LDKCVec_PublicKeyZ)); return res; }
	LDKCVec_PublicKeyZ* operator &() { return &self; }
};
struct CResult_SignatureNoneZ {
	LDKCResult_SignatureNoneZ self;
	CResult_SignatureNoneZ(const CResult_SignatureNoneZ&) = delete;
	~CResult_SignatureNoneZ() { CResult_SignatureNoneZ_free(self); }
	CResult_SignatureNoneZ(CResult_SignatureNoneZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_SignatureNoneZ)); }
	CResult_SignatureNoneZ(LDKCResult_SignatureNoneZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_SignatureNoneZ)); }
	operator LDKCResult_SignatureNoneZ() { LDKCResult_SignatureNoneZ res = self; memset(&self, 0, sizeof(LDKCResult_SignatureNoneZ)); return res; }
	LDKCResult_SignatureNoneZ* operator &() { return &self; }
};
struct CResult_NoneAPIErrorZ {
	LDKCResult_NoneAPIErrorZ self;
	CResult_NoneAPIErrorZ(const CResult_NoneAPIErrorZ&) = delete;
	~CResult_NoneAPIErrorZ() { CResult_NoneAPIErrorZ_free(self); }
	CResult_NoneAPIErrorZ(CResult_NoneAPIErrorZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_NoneAPIErrorZ)); }
	CResult_NoneAPIErrorZ(LDKCResult_NoneAPIErrorZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_NoneAPIErrorZ)); }
	operator LDKCResult_NoneAPIErrorZ() { LDKCResult_NoneAPIErrorZ res = self; memset(&self, 0, sizeof(LDKCResult_NoneAPIErrorZ)); return res; }
	LDKCResult_NoneAPIErrorZ* operator &() { return &self; }
};
struct CResult_NonePeerHandleErrorZ {
	LDKCResult_NonePeerHandleErrorZ self;
	CResult_NonePeerHandleErrorZ(const CResult_NonePeerHandleErrorZ&) = delete;
	~CResult_NonePeerHandleErrorZ() { CResult_NonePeerHandleErrorZ_free(self); }
	CResult_NonePeerHandleErrorZ(CResult_NonePeerHandleErrorZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_NonePeerHandleErrorZ)); }
	CResult_NonePeerHandleErrorZ(LDKCResult_NonePeerHandleErrorZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_NonePeerHandleErrorZ)); }
	operator LDKCResult_NonePeerHandleErrorZ() { LDKCResult_NonePeerHandleErrorZ res = self; memset(&self, 0, sizeof(LDKCResult_NonePeerHandleErrorZ)); return res; }
	LDKCResult_NonePeerHandleErrorZ* operator &() { return &self; }
};
struct CVec_CVec_RouteHopZZ {
	LDKCVec_CVec_RouteHopZZ self;
	CVec_CVec_RouteHopZZ(const CVec_CVec_RouteHopZZ&) = delete;
	~CVec_CVec_RouteHopZZ() { CVec_CVec_RouteHopZZ_free(self); }
	CVec_CVec_RouteHopZZ(CVec_CVec_RouteHopZZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_CVec_RouteHopZZ)); }
	CVec_CVec_RouteHopZZ(LDKCVec_CVec_RouteHopZZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_CVec_RouteHopZZ)); }
	operator LDKCVec_CVec_RouteHopZZ() { LDKCVec_CVec_RouteHopZZ res = self; memset(&self, 0, sizeof(LDKCVec_CVec_RouteHopZZ)); return res; }
	LDKCVec_CVec_RouteHopZZ* operator &() { return &self; }
};
struct CVec_NetAddressZ {
	LDKCVec_NetAddressZ self;
	CVec_NetAddressZ(const CVec_NetAddressZ&) = delete;
	~CVec_NetAddressZ() { CVec_NetAddressZ_free(self); }
	CVec_NetAddressZ(CVec_NetAddressZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_NetAddressZ)); }
	CVec_NetAddressZ(LDKCVec_NetAddressZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_NetAddressZ)); }
	operator LDKCVec_NetAddressZ() { LDKCVec_NetAddressZ res = self; memset(&self, 0, sizeof(LDKCVec_NetAddressZ)); return res; }
	LDKCVec_NetAddressZ* operator &() { return &self; }
};
struct CTuple2_Scriptu64Z {
	LDKCTuple2_Scriptu64Z self;
	CTuple2_Scriptu64Z(const CTuple2_Scriptu64Z&) = delete;
	~CTuple2_Scriptu64Z() { CTuple2_Scriptu64Z_free(self); }
	CTuple2_Scriptu64Z(CTuple2_Scriptu64Z&& o) : self(o.self) { memset(&o, 0, sizeof(CTuple2_Scriptu64Z)); }
	CTuple2_Scriptu64Z(LDKCTuple2_Scriptu64Z&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCTuple2_Scriptu64Z)); }
	operator LDKCTuple2_Scriptu64Z() { LDKCTuple2_Scriptu64Z res = self; memset(&self, 0, sizeof(LDKCTuple2_Scriptu64Z)); return res; }
	LDKCTuple2_Scriptu64Z* operator &() { return &self; }
};
struct CVec_UpdateFulfillHTLCZ {
	LDKCVec_UpdateFulfillHTLCZ self;
	CVec_UpdateFulfillHTLCZ(const CVec_UpdateFulfillHTLCZ&) = delete;
	~CVec_UpdateFulfillHTLCZ() { CVec_UpdateFulfillHTLCZ_free(self); }
	CVec_UpdateFulfillHTLCZ(CVec_UpdateFulfillHTLCZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_UpdateFulfillHTLCZ)); }
	CVec_UpdateFulfillHTLCZ(LDKCVec_UpdateFulfillHTLCZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_UpdateFulfillHTLCZ)); }
	operator LDKCVec_UpdateFulfillHTLCZ() { LDKCVec_UpdateFulfillHTLCZ res = self; memset(&self, 0, sizeof(LDKCVec_UpdateFulfillHTLCZ)); return res; }
	LDKCVec_UpdateFulfillHTLCZ* operator &() { return &self; }
};
struct CVec_UpdateFailHTLCZ {
	LDKCVec_UpdateFailHTLCZ self;
	CVec_UpdateFailHTLCZ(const CVec_UpdateFailHTLCZ&) = delete;
	~CVec_UpdateFailHTLCZ() { CVec_UpdateFailHTLCZ_free(self); }
	CVec_UpdateFailHTLCZ(CVec_UpdateFailHTLCZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_UpdateFailHTLCZ)); }
	CVec_UpdateFailHTLCZ(LDKCVec_UpdateFailHTLCZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_UpdateFailHTLCZ)); }
	operator LDKCVec_UpdateFailHTLCZ() { LDKCVec_UpdateFailHTLCZ res = self; memset(&self, 0, sizeof(LDKCVec_UpdateFailHTLCZ)); return res; }
	LDKCVec_UpdateFailHTLCZ* operator &() { return &self; }
};
struct CResult_CVec_u8ZPeerHandleErrorZ {
	LDKCResult_CVec_u8ZPeerHandleErrorZ self;
	CResult_CVec_u8ZPeerHandleErrorZ(const CResult_CVec_u8ZPeerHandleErrorZ&) = delete;
	~CResult_CVec_u8ZPeerHandleErrorZ() { CResult_CVec_u8ZPeerHandleErrorZ_free(self); }
	CResult_CVec_u8ZPeerHandleErrorZ(CResult_CVec_u8ZPeerHandleErrorZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_CVec_u8ZPeerHandleErrorZ)); }
	CResult_CVec_u8ZPeerHandleErrorZ(LDKCResult_CVec_u8ZPeerHandleErrorZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_CVec_u8ZPeerHandleErrorZ)); }
	operator LDKCResult_CVec_u8ZPeerHandleErrorZ() { LDKCResult_CVec_u8ZPeerHandleErrorZ res = self; memset(&self, 0, sizeof(LDKCResult_CVec_u8ZPeerHandleErrorZ)); return res; }
	LDKCResult_CVec_u8ZPeerHandleErrorZ* operator &() { return &self; }
};
struct CVec_ChannelDetailsZ {
	LDKCVec_ChannelDetailsZ self;
	CVec_ChannelDetailsZ(const CVec_ChannelDetailsZ&) = delete;
	~CVec_ChannelDetailsZ() { CVec_ChannelDetailsZ_free(self); }
	CVec_ChannelDetailsZ(CVec_ChannelDetailsZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_ChannelDetailsZ)); }
	CVec_ChannelDetailsZ(LDKCVec_ChannelDetailsZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_ChannelDetailsZ)); }
	operator LDKCVec_ChannelDetailsZ() { LDKCVec_ChannelDetailsZ res = self; memset(&self, 0, sizeof(LDKCVec_ChannelDetailsZ)); return res; }
	LDKCVec_ChannelDetailsZ* operator &() { return &self; }
};
struct CVec_UpdateAddHTLCZ {
	LDKCVec_UpdateAddHTLCZ self;
	CVec_UpdateAddHTLCZ(const CVec_UpdateAddHTLCZ&) = delete;
	~CVec_UpdateAddHTLCZ() { CVec_UpdateAddHTLCZ_free(self); }
	CVec_UpdateAddHTLCZ(CVec_UpdateAddHTLCZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_UpdateAddHTLCZ)); }
	CVec_UpdateAddHTLCZ(LDKCVec_UpdateAddHTLCZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_UpdateAddHTLCZ)); }
	operator LDKCVec_UpdateAddHTLCZ() { LDKCVec_UpdateAddHTLCZ res = self; memset(&self, 0, sizeof(LDKCVec_UpdateAddHTLCZ)); return res; }
	LDKCVec_UpdateAddHTLCZ* operator &() { return &self; }
};
struct CVec_RouteHopZ {
	LDKCVec_RouteHopZ self;
	CVec_RouteHopZ(const CVec_RouteHopZ&) = delete;
	~CVec_RouteHopZ() { CVec_RouteHopZ_free(self); }
	CVec_RouteHopZ(CVec_RouteHopZ&& o) : self(o.self) { memset(&o, 0, sizeof(CVec_RouteHopZ)); }
	CVec_RouteHopZ(LDKCVec_RouteHopZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCVec_RouteHopZ)); }
	operator LDKCVec_RouteHopZ() { LDKCVec_RouteHopZ res = self; memset(&self, 0, sizeof(LDKCVec_RouteHopZ)); return res; }
	LDKCVec_RouteHopZ* operator &() { return &self; }
};
struct CResult_NoneChannelMonitorUpdateErrZ {
	LDKCResult_NoneChannelMonitorUpdateErrZ self;
	CResult_NoneChannelMonitorUpdateErrZ(const CResult_NoneChannelMonitorUpdateErrZ&) = delete;
	~CResult_NoneChannelMonitorUpdateErrZ() { CResult_NoneChannelMonitorUpdateErrZ_free(self); }
	CResult_NoneChannelMonitorUpdateErrZ(CResult_NoneChannelMonitorUpdateErrZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_NoneChannelMonitorUpdateErrZ)); }
	CResult_NoneChannelMonitorUpdateErrZ(LDKCResult_NoneChannelMonitorUpdateErrZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_NoneChannelMonitorUpdateErrZ)); }
	operator LDKCResult_NoneChannelMonitorUpdateErrZ() { LDKCResult_NoneChannelMonitorUpdateErrZ res = self; memset(&self, 0, sizeof(LDKCResult_NoneChannelMonitorUpdateErrZ)); return res; }
	LDKCResult_NoneChannelMonitorUpdateErrZ* operator &() { return &self; }
};
struct CResult_NonePaymentSendFailureZ {
	LDKCResult_NonePaymentSendFailureZ self;
	CResult_NonePaymentSendFailureZ(const CResult_NonePaymentSendFailureZ&) = delete;
	~CResult_NonePaymentSendFailureZ() { CResult_NonePaymentSendFailureZ_free(self); }
	CResult_NonePaymentSendFailureZ(CResult_NonePaymentSendFailureZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_NonePaymentSendFailureZ)); }
	CResult_NonePaymentSendFailureZ(LDKCResult_NonePaymentSendFailureZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_NonePaymentSendFailureZ)); }
	operator LDKCResult_NonePaymentSendFailureZ() { LDKCResult_NonePaymentSendFailureZ res = self; memset(&self, 0, sizeof(LDKCResult_NonePaymentSendFailureZ)); return res; }
	LDKCResult_NonePaymentSendFailureZ* operator &() { return &self; }
};
struct CResult_boolPeerHandleErrorZ {
	LDKCResult_boolPeerHandleErrorZ self;
	CResult_boolPeerHandleErrorZ(const CResult_boolPeerHandleErrorZ&) = delete;
	~CResult_boolPeerHandleErrorZ() { CResult_boolPeerHandleErrorZ_free(self); }
	CResult_boolPeerHandleErrorZ(CResult_boolPeerHandleErrorZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_boolPeerHandleErrorZ)); }
	CResult_boolPeerHandleErrorZ(LDKCResult_boolPeerHandleErrorZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_boolPeerHandleErrorZ)); }
	operator LDKCResult_boolPeerHandleErrorZ() { LDKCResult_boolPeerHandleErrorZ res = self; memset(&self, 0, sizeof(LDKCResult_boolPeerHandleErrorZ)); return res; }
	LDKCResult_boolPeerHandleErrorZ* operator &() { return &self; }
};
struct CResult_CTuple2_Scriptu64ZChainErrorZ {
	LDKCResult_CTuple2_Scriptu64ZChainErrorZ self;
	CResult_CTuple2_Scriptu64ZChainErrorZ(const CResult_CTuple2_Scriptu64ZChainErrorZ&) = delete;
	~CResult_CTuple2_Scriptu64ZChainErrorZ() { CResult_CTuple2_Scriptu64ZChainErrorZ_free(self); }
	CResult_CTuple2_Scriptu64ZChainErrorZ(CResult_CTuple2_Scriptu64ZChainErrorZ&& o) : self(o.self) { memset(&o, 0, sizeof(CResult_CTuple2_Scriptu64ZChainErrorZ)); }
	CResult_CTuple2_Scriptu64ZChainErrorZ(LDKCResult_CTuple2_Scriptu64ZChainErrorZ&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCResult_CTuple2_Scriptu64ZChainErrorZ)); }
	operator LDKCResult_CTuple2_Scriptu64ZChainErrorZ() { LDKCResult_CTuple2_Scriptu64ZChainErrorZ res = self; memset(&self, 0, sizeof(LDKCResult_CTuple2_Scriptu64ZChainErrorZ)); return res; }
	LDKCResult_CTuple2_Scriptu64ZChainErrorZ* operator &() { return &self; }
};
}
