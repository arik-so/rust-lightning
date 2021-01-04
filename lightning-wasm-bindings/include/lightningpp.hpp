#include <string.h>
namespace LDK {
class Event {
private:
	LDKEvent self;
public:
	Event(const Event&) = delete;
	~Event() { Event_free(self); }
	Event(Event&& o) : self(o.self) { memset(&o, 0, sizeof(Event)); }
	Event(LDKEvent&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKEvent)); }
	operator LDKEvent() { LDKEvent res = self; memset(&self, 0, sizeof(LDKEvent)); return res; }
	LDKEvent* operator &() { return &self; }
	LDKEvent* operator ->() { return &self; }
	const LDKEvent* operator &() const { return &self; }
	const LDKEvent* operator ->() const { return &self; }
};
class MessageSendEvent {
private:
	LDKMessageSendEvent self;
public:
	MessageSendEvent(const MessageSendEvent&) = delete;
	~MessageSendEvent() { MessageSendEvent_free(self); }
	MessageSendEvent(MessageSendEvent&& o) : self(o.self) { memset(&o, 0, sizeof(MessageSendEvent)); }
	MessageSendEvent(LDKMessageSendEvent&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKMessageSendEvent)); }
	operator LDKMessageSendEvent() { LDKMessageSendEvent res = self; memset(&self, 0, sizeof(LDKMessageSendEvent)); return res; }
	LDKMessageSendEvent* operator &() { return &self; }
	LDKMessageSendEvent* operator ->() { return &self; }
	const LDKMessageSendEvent* operator &() const { return &self; }
	const LDKMessageSendEvent* operator ->() const { return &self; }
};
class MessageSendEventsProvider {
private:
	LDKMessageSendEventsProvider self;
public:
	MessageSendEventsProvider(const MessageSendEventsProvider&) = delete;
	~MessageSendEventsProvider() { MessageSendEventsProvider_free(self); }
	MessageSendEventsProvider(MessageSendEventsProvider&& o) : self(o.self) { memset(&o, 0, sizeof(MessageSendEventsProvider)); }
	MessageSendEventsProvider(LDKMessageSendEventsProvider&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKMessageSendEventsProvider)); }
	operator LDKMessageSendEventsProvider() { LDKMessageSendEventsProvider res = self; memset(&self, 0, sizeof(LDKMessageSendEventsProvider)); return res; }
	LDKMessageSendEventsProvider* operator &() { return &self; }
	LDKMessageSendEventsProvider* operator ->() { return &self; }
	const LDKMessageSendEventsProvider* operator &() const { return &self; }
	const LDKMessageSendEventsProvider* operator ->() const { return &self; }
};
class EventsProvider {
private:
	LDKEventsProvider self;
public:
	EventsProvider(const EventsProvider&) = delete;
	~EventsProvider() { EventsProvider_free(self); }
	EventsProvider(EventsProvider&& o) : self(o.self) { memset(&o, 0, sizeof(EventsProvider)); }
	EventsProvider(LDKEventsProvider&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKEventsProvider)); }
	operator LDKEventsProvider() { LDKEventsProvider res = self; memset(&self, 0, sizeof(LDKEventsProvider)); return res; }
	LDKEventsProvider* operator &() { return &self; }
	LDKEventsProvider* operator ->() { return &self; }
	const LDKEventsProvider* operator &() const { return &self; }
	const LDKEventsProvider* operator ->() const { return &self; }
};
class APIError {
private:
	LDKAPIError self;
public:
	APIError(const APIError&) = delete;
	~APIError() { APIError_free(self); }
	APIError(APIError&& o) : self(o.self) { memset(&o, 0, sizeof(APIError)); }
	APIError(LDKAPIError&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKAPIError)); }
	operator LDKAPIError() { LDKAPIError res = self; memset(&self, 0, sizeof(LDKAPIError)); return res; }
	LDKAPIError* operator &() { return &self; }
	LDKAPIError* operator ->() { return &self; }
	const LDKAPIError* operator &() const { return &self; }
	const LDKAPIError* operator ->() const { return &self; }
};
class Level {
private:
	LDKLevel self;
public:
	Level(const Level&) = delete;
	Level(Level&& o) : self(o.self) { memset(&o, 0, sizeof(Level)); }
	Level(LDKLevel&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKLevel)); }
	operator LDKLevel() { LDKLevel res = self; memset(&self, 0, sizeof(LDKLevel)); return res; }
	LDKLevel* operator &() { return &self; }
	LDKLevel* operator ->() { return &self; }
	const LDKLevel* operator &() const { return &self; }
	const LDKLevel* operator ->() const { return &self; }
};
class Logger {
private:
	LDKLogger self;
public:
	Logger(const Logger&) = delete;
	~Logger() { Logger_free(self); }
	Logger(Logger&& o) : self(o.self) { memset(&o, 0, sizeof(Logger)); }
	Logger(LDKLogger&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKLogger)); }
	operator LDKLogger() { LDKLogger res = self; memset(&self, 0, sizeof(LDKLogger)); return res; }
	LDKLogger* operator &() { return &self; }
	LDKLogger* operator ->() { return &self; }
	const LDKLogger* operator &() const { return &self; }
	const LDKLogger* operator ->() const { return &self; }
};
class ChannelHandshakeConfig {
private:
	LDKChannelHandshakeConfig self;
public:
	ChannelHandshakeConfig(const ChannelHandshakeConfig&) = delete;
	~ChannelHandshakeConfig() { ChannelHandshakeConfig_free(self); }
	ChannelHandshakeConfig(ChannelHandshakeConfig&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelHandshakeConfig)); }
	ChannelHandshakeConfig(LDKChannelHandshakeConfig&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelHandshakeConfig)); }
	operator LDKChannelHandshakeConfig() { LDKChannelHandshakeConfig res = self; memset(&self, 0, sizeof(LDKChannelHandshakeConfig)); return res; }
	LDKChannelHandshakeConfig* operator &() { return &self; }
	LDKChannelHandshakeConfig* operator ->() { return &self; }
	const LDKChannelHandshakeConfig* operator &() const { return &self; }
	const LDKChannelHandshakeConfig* operator ->() const { return &self; }
};
class ChannelHandshakeLimits {
private:
	LDKChannelHandshakeLimits self;
public:
	ChannelHandshakeLimits(const ChannelHandshakeLimits&) = delete;
	~ChannelHandshakeLimits() { ChannelHandshakeLimits_free(self); }
	ChannelHandshakeLimits(ChannelHandshakeLimits&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelHandshakeLimits)); }
	ChannelHandshakeLimits(LDKChannelHandshakeLimits&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelHandshakeLimits)); }
	operator LDKChannelHandshakeLimits() { LDKChannelHandshakeLimits res = self; memset(&self, 0, sizeof(LDKChannelHandshakeLimits)); return res; }
	LDKChannelHandshakeLimits* operator &() { return &self; }
	LDKChannelHandshakeLimits* operator ->() { return &self; }
	const LDKChannelHandshakeLimits* operator &() const { return &self; }
	const LDKChannelHandshakeLimits* operator ->() const { return &self; }
};
class ChannelConfig {
private:
	LDKChannelConfig self;
public:
	ChannelConfig(const ChannelConfig&) = delete;
	~ChannelConfig() { ChannelConfig_free(self); }
	ChannelConfig(ChannelConfig&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelConfig)); }
	ChannelConfig(LDKChannelConfig&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelConfig)); }
	operator LDKChannelConfig() { LDKChannelConfig res = self; memset(&self, 0, sizeof(LDKChannelConfig)); return res; }
	LDKChannelConfig* operator &() { return &self; }
	LDKChannelConfig* operator ->() { return &self; }
	const LDKChannelConfig* operator &() const { return &self; }
	const LDKChannelConfig* operator ->() const { return &self; }
};
class UserConfig {
private:
	LDKUserConfig self;
public:
	UserConfig(const UserConfig&) = delete;
	~UserConfig() { UserConfig_free(self); }
	UserConfig(UserConfig&& o) : self(o.self) { memset(&o, 0, sizeof(UserConfig)); }
	UserConfig(LDKUserConfig&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUserConfig)); }
	operator LDKUserConfig() { LDKUserConfig res = self; memset(&self, 0, sizeof(LDKUserConfig)); return res; }
	LDKUserConfig* operator &() { return &self; }
	LDKUserConfig* operator ->() { return &self; }
	const LDKUserConfig* operator &() const { return &self; }
	const LDKUserConfig* operator ->() const { return &self; }
};
class BroadcasterInterface {
private:
	LDKBroadcasterInterface self;
public:
	BroadcasterInterface(const BroadcasterInterface&) = delete;
	~BroadcasterInterface() { BroadcasterInterface_free(self); }
	BroadcasterInterface(BroadcasterInterface&& o) : self(o.self) { memset(&o, 0, sizeof(BroadcasterInterface)); }
	BroadcasterInterface(LDKBroadcasterInterface&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKBroadcasterInterface)); }
	operator LDKBroadcasterInterface() { LDKBroadcasterInterface res = self; memset(&self, 0, sizeof(LDKBroadcasterInterface)); return res; }
	LDKBroadcasterInterface* operator &() { return &self; }
	LDKBroadcasterInterface* operator ->() { return &self; }
	const LDKBroadcasterInterface* operator &() const { return &self; }
	const LDKBroadcasterInterface* operator ->() const { return &self; }
};
class ConfirmationTarget {
private:
	LDKConfirmationTarget self;
public:
	ConfirmationTarget(const ConfirmationTarget&) = delete;
	ConfirmationTarget(ConfirmationTarget&& o) : self(o.self) { memset(&o, 0, sizeof(ConfirmationTarget)); }
	ConfirmationTarget(LDKConfirmationTarget&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKConfirmationTarget)); }
	operator LDKConfirmationTarget() { LDKConfirmationTarget res = self; memset(&self, 0, sizeof(LDKConfirmationTarget)); return res; }
	LDKConfirmationTarget* operator &() { return &self; }
	LDKConfirmationTarget* operator ->() { return &self; }
	const LDKConfirmationTarget* operator &() const { return &self; }
	const LDKConfirmationTarget* operator ->() const { return &self; }
};
class FeeEstimator {
private:
	LDKFeeEstimator self;
public:
	FeeEstimator(const FeeEstimator&) = delete;
	~FeeEstimator() { FeeEstimator_free(self); }
	FeeEstimator(FeeEstimator&& o) : self(o.self) { memset(&o, 0, sizeof(FeeEstimator)); }
	FeeEstimator(LDKFeeEstimator&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKFeeEstimator)); }
	operator LDKFeeEstimator() { LDKFeeEstimator res = self; memset(&self, 0, sizeof(LDKFeeEstimator)); return res; }
	LDKFeeEstimator* operator &() { return &self; }
	LDKFeeEstimator* operator ->() { return &self; }
	const LDKFeeEstimator* operator &() const { return &self; }
	const LDKFeeEstimator* operator ->() const { return &self; }
};
class ChainMonitor {
private:
	LDKChainMonitor self;
public:
	ChainMonitor(const ChainMonitor&) = delete;
	~ChainMonitor() { ChainMonitor_free(self); }
	ChainMonitor(ChainMonitor&& o) : self(o.self) { memset(&o, 0, sizeof(ChainMonitor)); }
	ChainMonitor(LDKChainMonitor&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChainMonitor)); }
	operator LDKChainMonitor() { LDKChainMonitor res = self; memset(&self, 0, sizeof(LDKChainMonitor)); return res; }
	LDKChainMonitor* operator &() { return &self; }
	LDKChainMonitor* operator ->() { return &self; }
	const LDKChainMonitor* operator &() const { return &self; }
	const LDKChainMonitor* operator ->() const { return &self; }
};
class ChannelMonitorUpdate {
private:
	LDKChannelMonitorUpdate self;
public:
	ChannelMonitorUpdate(const ChannelMonitorUpdate&) = delete;
	~ChannelMonitorUpdate() { ChannelMonitorUpdate_free(self); }
	ChannelMonitorUpdate(ChannelMonitorUpdate&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelMonitorUpdate)); }
	ChannelMonitorUpdate(LDKChannelMonitorUpdate&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelMonitorUpdate)); }
	operator LDKChannelMonitorUpdate() { LDKChannelMonitorUpdate res = self; memset(&self, 0, sizeof(LDKChannelMonitorUpdate)); return res; }
	LDKChannelMonitorUpdate* operator &() { return &self; }
	LDKChannelMonitorUpdate* operator ->() { return &self; }
	const LDKChannelMonitorUpdate* operator &() const { return &self; }
	const LDKChannelMonitorUpdate* operator ->() const { return &self; }
};
class ChannelMonitorUpdateErr {
private:
	LDKChannelMonitorUpdateErr self;
public:
	ChannelMonitorUpdateErr(const ChannelMonitorUpdateErr&) = delete;
	ChannelMonitorUpdateErr(ChannelMonitorUpdateErr&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelMonitorUpdateErr)); }
	ChannelMonitorUpdateErr(LDKChannelMonitorUpdateErr&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelMonitorUpdateErr)); }
	operator LDKChannelMonitorUpdateErr() { LDKChannelMonitorUpdateErr res = self; memset(&self, 0, sizeof(LDKChannelMonitorUpdateErr)); return res; }
	LDKChannelMonitorUpdateErr* operator &() { return &self; }
	LDKChannelMonitorUpdateErr* operator ->() { return &self; }
	const LDKChannelMonitorUpdateErr* operator &() const { return &self; }
	const LDKChannelMonitorUpdateErr* operator ->() const { return &self; }
};
class MonitorUpdateError {
private:
	LDKMonitorUpdateError self;
public:
	MonitorUpdateError(const MonitorUpdateError&) = delete;
	~MonitorUpdateError() { MonitorUpdateError_free(self); }
	MonitorUpdateError(MonitorUpdateError&& o) : self(o.self) { memset(&o, 0, sizeof(MonitorUpdateError)); }
	MonitorUpdateError(LDKMonitorUpdateError&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKMonitorUpdateError)); }
	operator LDKMonitorUpdateError() { LDKMonitorUpdateError res = self; memset(&self, 0, sizeof(LDKMonitorUpdateError)); return res; }
	LDKMonitorUpdateError* operator &() { return &self; }
	LDKMonitorUpdateError* operator ->() { return &self; }
	const LDKMonitorUpdateError* operator &() const { return &self; }
	const LDKMonitorUpdateError* operator ->() const { return &self; }
};
class MonitorEvent {
private:
	LDKMonitorEvent self;
public:
	MonitorEvent(const MonitorEvent&) = delete;
	~MonitorEvent() { MonitorEvent_free(self); }
	MonitorEvent(MonitorEvent&& o) : self(o.self) { memset(&o, 0, sizeof(MonitorEvent)); }
	MonitorEvent(LDKMonitorEvent&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKMonitorEvent)); }
	operator LDKMonitorEvent() { LDKMonitorEvent res = self; memset(&self, 0, sizeof(LDKMonitorEvent)); return res; }
	LDKMonitorEvent* operator &() { return &self; }
	LDKMonitorEvent* operator ->() { return &self; }
	const LDKMonitorEvent* operator &() const { return &self; }
	const LDKMonitorEvent* operator ->() const { return &self; }
};
class HTLCUpdate {
private:
	LDKHTLCUpdate self;
public:
	HTLCUpdate(const HTLCUpdate&) = delete;
	~HTLCUpdate() { HTLCUpdate_free(self); }
	HTLCUpdate(HTLCUpdate&& o) : self(o.self) { memset(&o, 0, sizeof(HTLCUpdate)); }
	HTLCUpdate(LDKHTLCUpdate&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKHTLCUpdate)); }
	operator LDKHTLCUpdate() { LDKHTLCUpdate res = self; memset(&self, 0, sizeof(LDKHTLCUpdate)); return res; }
	LDKHTLCUpdate* operator &() { return &self; }
	LDKHTLCUpdate* operator ->() { return &self; }
	const LDKHTLCUpdate* operator &() const { return &self; }
	const LDKHTLCUpdate* operator ->() const { return &self; }
};
class ChannelMonitor {
private:
	LDKChannelMonitor self;
public:
	ChannelMonitor(const ChannelMonitor&) = delete;
	~ChannelMonitor() { ChannelMonitor_free(self); }
	ChannelMonitor(ChannelMonitor&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelMonitor)); }
	ChannelMonitor(LDKChannelMonitor&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelMonitor)); }
	operator LDKChannelMonitor() { LDKChannelMonitor res = self; memset(&self, 0, sizeof(LDKChannelMonitor)); return res; }
	LDKChannelMonitor* operator &() { return &self; }
	LDKChannelMonitor* operator ->() { return &self; }
	const LDKChannelMonitor* operator &() const { return &self; }
	const LDKChannelMonitor* operator ->() const { return &self; }
};
class Persist {
private:
	LDKPersist self;
public:
	Persist(const Persist&) = delete;
	~Persist() { Persist_free(self); }
	Persist(Persist&& o) : self(o.self) { memset(&o, 0, sizeof(Persist)); }
	Persist(LDKPersist&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPersist)); }
	operator LDKPersist() { LDKPersist res = self; memset(&self, 0, sizeof(LDKPersist)); return res; }
	LDKPersist* operator &() { return &self; }
	LDKPersist* operator ->() { return &self; }
	const LDKPersist* operator &() const { return &self; }
	const LDKPersist* operator ->() const { return &self; }
};
class OutPoint {
private:
	LDKOutPoint self;
public:
	OutPoint(const OutPoint&) = delete;
	~OutPoint() { OutPoint_free(self); }
	OutPoint(OutPoint&& o) : self(o.self) { memset(&o, 0, sizeof(OutPoint)); }
	OutPoint(LDKOutPoint&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKOutPoint)); }
	operator LDKOutPoint() { LDKOutPoint res = self; memset(&self, 0, sizeof(LDKOutPoint)); return res; }
	LDKOutPoint* operator &() { return &self; }
	LDKOutPoint* operator ->() { return &self; }
	const LDKOutPoint* operator &() const { return &self; }
	const LDKOutPoint* operator ->() const { return &self; }
};
class SpendableOutputDescriptor {
private:
	LDKSpendableOutputDescriptor self;
public:
	SpendableOutputDescriptor(const SpendableOutputDescriptor&) = delete;
	~SpendableOutputDescriptor() { SpendableOutputDescriptor_free(self); }
	SpendableOutputDescriptor(SpendableOutputDescriptor&& o) : self(o.self) { memset(&o, 0, sizeof(SpendableOutputDescriptor)); }
	SpendableOutputDescriptor(LDKSpendableOutputDescriptor&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKSpendableOutputDescriptor)); }
	operator LDKSpendableOutputDescriptor() { LDKSpendableOutputDescriptor res = self; memset(&self, 0, sizeof(LDKSpendableOutputDescriptor)); return res; }
	LDKSpendableOutputDescriptor* operator &() { return &self; }
	LDKSpendableOutputDescriptor* operator ->() { return &self; }
	const LDKSpendableOutputDescriptor* operator &() const { return &self; }
	const LDKSpendableOutputDescriptor* operator ->() const { return &self; }
};
class ChannelKeys {
private:
	LDKChannelKeys self;
public:
	ChannelKeys(const ChannelKeys&) = delete;
	~ChannelKeys() { ChannelKeys_free(self); }
	ChannelKeys(ChannelKeys&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelKeys)); }
	ChannelKeys(LDKChannelKeys&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelKeys)); }
	operator LDKChannelKeys() { LDKChannelKeys res = self; memset(&self, 0, sizeof(LDKChannelKeys)); return res; }
	LDKChannelKeys* operator &() { return &self; }
	LDKChannelKeys* operator ->() { return &self; }
	const LDKChannelKeys* operator &() const { return &self; }
	const LDKChannelKeys* operator ->() const { return &self; }
};
class KeysInterface {
private:
	LDKKeysInterface self;
public:
	KeysInterface(const KeysInterface&) = delete;
	~KeysInterface() { KeysInterface_free(self); }
	KeysInterface(KeysInterface&& o) : self(o.self) { memset(&o, 0, sizeof(KeysInterface)); }
	KeysInterface(LDKKeysInterface&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKKeysInterface)); }
	operator LDKKeysInterface() { LDKKeysInterface res = self; memset(&self, 0, sizeof(LDKKeysInterface)); return res; }
	LDKKeysInterface* operator &() { return &self; }
	LDKKeysInterface* operator ->() { return &self; }
	const LDKKeysInterface* operator &() const { return &self; }
	const LDKKeysInterface* operator ->() const { return &self; }
};
class InMemoryChannelKeys {
private:
	LDKInMemoryChannelKeys self;
public:
	InMemoryChannelKeys(const InMemoryChannelKeys&) = delete;
	~InMemoryChannelKeys() { InMemoryChannelKeys_free(self); }
	InMemoryChannelKeys(InMemoryChannelKeys&& o) : self(o.self) { memset(&o, 0, sizeof(InMemoryChannelKeys)); }
	InMemoryChannelKeys(LDKInMemoryChannelKeys&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKInMemoryChannelKeys)); }
	operator LDKInMemoryChannelKeys() { LDKInMemoryChannelKeys res = self; memset(&self, 0, sizeof(LDKInMemoryChannelKeys)); return res; }
	LDKInMemoryChannelKeys* operator &() { return &self; }
	LDKInMemoryChannelKeys* operator ->() { return &self; }
	const LDKInMemoryChannelKeys* operator &() const { return &self; }
	const LDKInMemoryChannelKeys* operator ->() const { return &self; }
};
class KeysManager {
private:
	LDKKeysManager self;
public:
	KeysManager(const KeysManager&) = delete;
	~KeysManager() { KeysManager_free(self); }
	KeysManager(KeysManager&& o) : self(o.self) { memset(&o, 0, sizeof(KeysManager)); }
	KeysManager(LDKKeysManager&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKKeysManager)); }
	operator LDKKeysManager() { LDKKeysManager res = self; memset(&self, 0, sizeof(LDKKeysManager)); return res; }
	LDKKeysManager* operator &() { return &self; }
	LDKKeysManager* operator ->() { return &self; }
	const LDKKeysManager* operator &() const { return &self; }
	const LDKKeysManager* operator ->() const { return &self; }
};
class AccessError {
private:
	LDKAccessError self;
public:
	AccessError(const AccessError&) = delete;
	AccessError(AccessError&& o) : self(o.self) { memset(&o, 0, sizeof(AccessError)); }
	AccessError(LDKAccessError&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKAccessError)); }
	operator LDKAccessError() { LDKAccessError res = self; memset(&self, 0, sizeof(LDKAccessError)); return res; }
	LDKAccessError* operator &() { return &self; }
	LDKAccessError* operator ->() { return &self; }
	const LDKAccessError* operator &() const { return &self; }
	const LDKAccessError* operator ->() const { return &self; }
};
class Access {
private:
	LDKAccess self;
public:
	Access(const Access&) = delete;
	~Access() { Access_free(self); }
	Access(Access&& o) : self(o.self) { memset(&o, 0, sizeof(Access)); }
	Access(LDKAccess&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKAccess)); }
	operator LDKAccess() { LDKAccess res = self; memset(&self, 0, sizeof(LDKAccess)); return res; }
	LDKAccess* operator &() { return &self; }
	LDKAccess* operator ->() { return &self; }
	const LDKAccess* operator &() const { return &self; }
	const LDKAccess* operator ->() const { return &self; }
};
class Watch {
private:
	LDKWatch self;
public:
	Watch(const Watch&) = delete;
	~Watch() { Watch_free(self); }
	Watch(Watch&& o) : self(o.self) { memset(&o, 0, sizeof(Watch)); }
	Watch(LDKWatch&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKWatch)); }
	operator LDKWatch() { LDKWatch res = self; memset(&self, 0, sizeof(LDKWatch)); return res; }
	LDKWatch* operator &() { return &self; }
	LDKWatch* operator ->() { return &self; }
	const LDKWatch* operator &() const { return &self; }
	const LDKWatch* operator ->() const { return &self; }
};
class Filter {
private:
	LDKFilter self;
public:
	Filter(const Filter&) = delete;
	~Filter() { Filter_free(self); }
	Filter(Filter&& o) : self(o.self) { memset(&o, 0, sizeof(Filter)); }
	Filter(LDKFilter&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKFilter)); }
	operator LDKFilter() { LDKFilter res = self; memset(&self, 0, sizeof(LDKFilter)); return res; }
	LDKFilter* operator &() { return &self; }
	LDKFilter* operator ->() { return &self; }
	const LDKFilter* operator &() const { return &self; }
	const LDKFilter* operator ->() const { return &self; }
};
class ChannelManager {
private:
	LDKChannelManager self;
public:
	ChannelManager(const ChannelManager&) = delete;
	~ChannelManager() { ChannelManager_free(self); }
	ChannelManager(ChannelManager&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelManager)); }
	ChannelManager(LDKChannelManager&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelManager)); }
	operator LDKChannelManager() { LDKChannelManager res = self; memset(&self, 0, sizeof(LDKChannelManager)); return res; }
	LDKChannelManager* operator &() { return &self; }
	LDKChannelManager* operator ->() { return &self; }
	const LDKChannelManager* operator &() const { return &self; }
	const LDKChannelManager* operator ->() const { return &self; }
};
class ChannelDetails {
private:
	LDKChannelDetails self;
public:
	ChannelDetails(const ChannelDetails&) = delete;
	~ChannelDetails() { ChannelDetails_free(self); }
	ChannelDetails(ChannelDetails&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelDetails)); }
	ChannelDetails(LDKChannelDetails&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelDetails)); }
	operator LDKChannelDetails() { LDKChannelDetails res = self; memset(&self, 0, sizeof(LDKChannelDetails)); return res; }
	LDKChannelDetails* operator &() { return &self; }
	LDKChannelDetails* operator ->() { return &self; }
	const LDKChannelDetails* operator &() const { return &self; }
	const LDKChannelDetails* operator ->() const { return &self; }
};
class PaymentSendFailure {
private:
	LDKPaymentSendFailure self;
public:
	PaymentSendFailure(const PaymentSendFailure&) = delete;
	~PaymentSendFailure() { PaymentSendFailure_free(self); }
	PaymentSendFailure(PaymentSendFailure&& o) : self(o.self) { memset(&o, 0, sizeof(PaymentSendFailure)); }
	PaymentSendFailure(LDKPaymentSendFailure&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPaymentSendFailure)); }
	operator LDKPaymentSendFailure() { LDKPaymentSendFailure res = self; memset(&self, 0, sizeof(LDKPaymentSendFailure)); return res; }
	LDKPaymentSendFailure* operator &() { return &self; }
	LDKPaymentSendFailure* operator ->() { return &self; }
	const LDKPaymentSendFailure* operator &() const { return &self; }
	const LDKPaymentSendFailure* operator ->() const { return &self; }
};
class ChannelManagerReadArgs {
private:
	LDKChannelManagerReadArgs self;
public:
	ChannelManagerReadArgs(const ChannelManagerReadArgs&) = delete;
	~ChannelManagerReadArgs() { ChannelManagerReadArgs_free(self); }
	ChannelManagerReadArgs(ChannelManagerReadArgs&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelManagerReadArgs)); }
	ChannelManagerReadArgs(LDKChannelManagerReadArgs&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelManagerReadArgs)); }
	operator LDKChannelManagerReadArgs() { LDKChannelManagerReadArgs res = self; memset(&self, 0, sizeof(LDKChannelManagerReadArgs)); return res; }
	LDKChannelManagerReadArgs* operator &() { return &self; }
	LDKChannelManagerReadArgs* operator ->() { return &self; }
	const LDKChannelManagerReadArgs* operator &() const { return &self; }
	const LDKChannelManagerReadArgs* operator ->() const { return &self; }
};
class DecodeError {
private:
	LDKDecodeError self;
public:
	DecodeError(const DecodeError&) = delete;
	~DecodeError() { DecodeError_free(self); }
	DecodeError(DecodeError&& o) : self(o.self) { memset(&o, 0, sizeof(DecodeError)); }
	DecodeError(LDKDecodeError&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKDecodeError)); }
	operator LDKDecodeError() { LDKDecodeError res = self; memset(&self, 0, sizeof(LDKDecodeError)); return res; }
	LDKDecodeError* operator &() { return &self; }
	LDKDecodeError* operator ->() { return &self; }
	const LDKDecodeError* operator &() const { return &self; }
	const LDKDecodeError* operator ->() const { return &self; }
};
class Init {
private:
	LDKInit self;
public:
	Init(const Init&) = delete;
	~Init() { Init_free(self); }
	Init(Init&& o) : self(o.self) { memset(&o, 0, sizeof(Init)); }
	Init(LDKInit&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKInit)); }
	operator LDKInit() { LDKInit res = self; memset(&self, 0, sizeof(LDKInit)); return res; }
	LDKInit* operator &() { return &self; }
	LDKInit* operator ->() { return &self; }
	const LDKInit* operator &() const { return &self; }
	const LDKInit* operator ->() const { return &self; }
};
class ErrorMessage {
private:
	LDKErrorMessage self;
public:
	ErrorMessage(const ErrorMessage&) = delete;
	~ErrorMessage() { ErrorMessage_free(self); }
	ErrorMessage(ErrorMessage&& o) : self(o.self) { memset(&o, 0, sizeof(ErrorMessage)); }
	ErrorMessage(LDKErrorMessage&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKErrorMessage)); }
	operator LDKErrorMessage() { LDKErrorMessage res = self; memset(&self, 0, sizeof(LDKErrorMessage)); return res; }
	LDKErrorMessage* operator &() { return &self; }
	LDKErrorMessage* operator ->() { return &self; }
	const LDKErrorMessage* operator &() const { return &self; }
	const LDKErrorMessage* operator ->() const { return &self; }
};
class Ping {
private:
	LDKPing self;
public:
	Ping(const Ping&) = delete;
	~Ping() { Ping_free(self); }
	Ping(Ping&& o) : self(o.self) { memset(&o, 0, sizeof(Ping)); }
	Ping(LDKPing&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPing)); }
	operator LDKPing() { LDKPing res = self; memset(&self, 0, sizeof(LDKPing)); return res; }
	LDKPing* operator &() { return &self; }
	LDKPing* operator ->() { return &self; }
	const LDKPing* operator &() const { return &self; }
	const LDKPing* operator ->() const { return &self; }
};
class Pong {
private:
	LDKPong self;
public:
	Pong(const Pong&) = delete;
	~Pong() { Pong_free(self); }
	Pong(Pong&& o) : self(o.self) { memset(&o, 0, sizeof(Pong)); }
	Pong(LDKPong&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPong)); }
	operator LDKPong() { LDKPong res = self; memset(&self, 0, sizeof(LDKPong)); return res; }
	LDKPong* operator &() { return &self; }
	LDKPong* operator ->() { return &self; }
	const LDKPong* operator &() const { return &self; }
	const LDKPong* operator ->() const { return &self; }
};
class OpenChannel {
private:
	LDKOpenChannel self;
public:
	OpenChannel(const OpenChannel&) = delete;
	~OpenChannel() { OpenChannel_free(self); }
	OpenChannel(OpenChannel&& o) : self(o.self) { memset(&o, 0, sizeof(OpenChannel)); }
	OpenChannel(LDKOpenChannel&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKOpenChannel)); }
	operator LDKOpenChannel() { LDKOpenChannel res = self; memset(&self, 0, sizeof(LDKOpenChannel)); return res; }
	LDKOpenChannel* operator &() { return &self; }
	LDKOpenChannel* operator ->() { return &self; }
	const LDKOpenChannel* operator &() const { return &self; }
	const LDKOpenChannel* operator ->() const { return &self; }
};
class AcceptChannel {
private:
	LDKAcceptChannel self;
public:
	AcceptChannel(const AcceptChannel&) = delete;
	~AcceptChannel() { AcceptChannel_free(self); }
	AcceptChannel(AcceptChannel&& o) : self(o.self) { memset(&o, 0, sizeof(AcceptChannel)); }
	AcceptChannel(LDKAcceptChannel&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKAcceptChannel)); }
	operator LDKAcceptChannel() { LDKAcceptChannel res = self; memset(&self, 0, sizeof(LDKAcceptChannel)); return res; }
	LDKAcceptChannel* operator &() { return &self; }
	LDKAcceptChannel* operator ->() { return &self; }
	const LDKAcceptChannel* operator &() const { return &self; }
	const LDKAcceptChannel* operator ->() const { return &self; }
};
class FundingCreated {
private:
	LDKFundingCreated self;
public:
	FundingCreated(const FundingCreated&) = delete;
	~FundingCreated() { FundingCreated_free(self); }
	FundingCreated(FundingCreated&& o) : self(o.self) { memset(&o, 0, sizeof(FundingCreated)); }
	FundingCreated(LDKFundingCreated&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKFundingCreated)); }
	operator LDKFundingCreated() { LDKFundingCreated res = self; memset(&self, 0, sizeof(LDKFundingCreated)); return res; }
	LDKFundingCreated* operator &() { return &self; }
	LDKFundingCreated* operator ->() { return &self; }
	const LDKFundingCreated* operator &() const { return &self; }
	const LDKFundingCreated* operator ->() const { return &self; }
};
class FundingSigned {
private:
	LDKFundingSigned self;
public:
	FundingSigned(const FundingSigned&) = delete;
	~FundingSigned() { FundingSigned_free(self); }
	FundingSigned(FundingSigned&& o) : self(o.self) { memset(&o, 0, sizeof(FundingSigned)); }
	FundingSigned(LDKFundingSigned&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKFundingSigned)); }
	operator LDKFundingSigned() { LDKFundingSigned res = self; memset(&self, 0, sizeof(LDKFundingSigned)); return res; }
	LDKFundingSigned* operator &() { return &self; }
	LDKFundingSigned* operator ->() { return &self; }
	const LDKFundingSigned* operator &() const { return &self; }
	const LDKFundingSigned* operator ->() const { return &self; }
};
class FundingLocked {
private:
	LDKFundingLocked self;
public:
	FundingLocked(const FundingLocked&) = delete;
	~FundingLocked() { FundingLocked_free(self); }
	FundingLocked(FundingLocked&& o) : self(o.self) { memset(&o, 0, sizeof(FundingLocked)); }
	FundingLocked(LDKFundingLocked&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKFundingLocked)); }
	operator LDKFundingLocked() { LDKFundingLocked res = self; memset(&self, 0, sizeof(LDKFundingLocked)); return res; }
	LDKFundingLocked* operator &() { return &self; }
	LDKFundingLocked* operator ->() { return &self; }
	const LDKFundingLocked* operator &() const { return &self; }
	const LDKFundingLocked* operator ->() const { return &self; }
};
class Shutdown {
private:
	LDKShutdown self;
public:
	Shutdown(const Shutdown&) = delete;
	~Shutdown() { Shutdown_free(self); }
	Shutdown(Shutdown&& o) : self(o.self) { memset(&o, 0, sizeof(Shutdown)); }
	Shutdown(LDKShutdown&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKShutdown)); }
	operator LDKShutdown() { LDKShutdown res = self; memset(&self, 0, sizeof(LDKShutdown)); return res; }
	LDKShutdown* operator &() { return &self; }
	LDKShutdown* operator ->() { return &self; }
	const LDKShutdown* operator &() const { return &self; }
	const LDKShutdown* operator ->() const { return &self; }
};
class ClosingSigned {
private:
	LDKClosingSigned self;
public:
	ClosingSigned(const ClosingSigned&) = delete;
	~ClosingSigned() { ClosingSigned_free(self); }
	ClosingSigned(ClosingSigned&& o) : self(o.self) { memset(&o, 0, sizeof(ClosingSigned)); }
	ClosingSigned(LDKClosingSigned&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKClosingSigned)); }
	operator LDKClosingSigned() { LDKClosingSigned res = self; memset(&self, 0, sizeof(LDKClosingSigned)); return res; }
	LDKClosingSigned* operator &() { return &self; }
	LDKClosingSigned* operator ->() { return &self; }
	const LDKClosingSigned* operator &() const { return &self; }
	const LDKClosingSigned* operator ->() const { return &self; }
};
class UpdateAddHTLC {
private:
	LDKUpdateAddHTLC self;
public:
	UpdateAddHTLC(const UpdateAddHTLC&) = delete;
	~UpdateAddHTLC() { UpdateAddHTLC_free(self); }
	UpdateAddHTLC(UpdateAddHTLC&& o) : self(o.self) { memset(&o, 0, sizeof(UpdateAddHTLC)); }
	UpdateAddHTLC(LDKUpdateAddHTLC&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUpdateAddHTLC)); }
	operator LDKUpdateAddHTLC() { LDKUpdateAddHTLC res = self; memset(&self, 0, sizeof(LDKUpdateAddHTLC)); return res; }
	LDKUpdateAddHTLC* operator &() { return &self; }
	LDKUpdateAddHTLC* operator ->() { return &self; }
	const LDKUpdateAddHTLC* operator &() const { return &self; }
	const LDKUpdateAddHTLC* operator ->() const { return &self; }
};
class UpdateFulfillHTLC {
private:
	LDKUpdateFulfillHTLC self;
public:
	UpdateFulfillHTLC(const UpdateFulfillHTLC&) = delete;
	~UpdateFulfillHTLC() { UpdateFulfillHTLC_free(self); }
	UpdateFulfillHTLC(UpdateFulfillHTLC&& o) : self(o.self) { memset(&o, 0, sizeof(UpdateFulfillHTLC)); }
	UpdateFulfillHTLC(LDKUpdateFulfillHTLC&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUpdateFulfillHTLC)); }
	operator LDKUpdateFulfillHTLC() { LDKUpdateFulfillHTLC res = self; memset(&self, 0, sizeof(LDKUpdateFulfillHTLC)); return res; }
	LDKUpdateFulfillHTLC* operator &() { return &self; }
	LDKUpdateFulfillHTLC* operator ->() { return &self; }
	const LDKUpdateFulfillHTLC* operator &() const { return &self; }
	const LDKUpdateFulfillHTLC* operator ->() const { return &self; }
};
class UpdateFailHTLC {
private:
	LDKUpdateFailHTLC self;
public:
	UpdateFailHTLC(const UpdateFailHTLC&) = delete;
	~UpdateFailHTLC() { UpdateFailHTLC_free(self); }
	UpdateFailHTLC(UpdateFailHTLC&& o) : self(o.self) { memset(&o, 0, sizeof(UpdateFailHTLC)); }
	UpdateFailHTLC(LDKUpdateFailHTLC&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUpdateFailHTLC)); }
	operator LDKUpdateFailHTLC() { LDKUpdateFailHTLC res = self; memset(&self, 0, sizeof(LDKUpdateFailHTLC)); return res; }
	LDKUpdateFailHTLC* operator &() { return &self; }
	LDKUpdateFailHTLC* operator ->() { return &self; }
	const LDKUpdateFailHTLC* operator &() const { return &self; }
	const LDKUpdateFailHTLC* operator ->() const { return &self; }
};
class UpdateFailMalformedHTLC {
private:
	LDKUpdateFailMalformedHTLC self;
public:
	UpdateFailMalformedHTLC(const UpdateFailMalformedHTLC&) = delete;
	~UpdateFailMalformedHTLC() { UpdateFailMalformedHTLC_free(self); }
	UpdateFailMalformedHTLC(UpdateFailMalformedHTLC&& o) : self(o.self) { memset(&o, 0, sizeof(UpdateFailMalformedHTLC)); }
	UpdateFailMalformedHTLC(LDKUpdateFailMalformedHTLC&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUpdateFailMalformedHTLC)); }
	operator LDKUpdateFailMalformedHTLC() { LDKUpdateFailMalformedHTLC res = self; memset(&self, 0, sizeof(LDKUpdateFailMalformedHTLC)); return res; }
	LDKUpdateFailMalformedHTLC* operator &() { return &self; }
	LDKUpdateFailMalformedHTLC* operator ->() { return &self; }
	const LDKUpdateFailMalformedHTLC* operator &() const { return &self; }
	const LDKUpdateFailMalformedHTLC* operator ->() const { return &self; }
};
class CommitmentSigned {
private:
	LDKCommitmentSigned self;
public:
	CommitmentSigned(const CommitmentSigned&) = delete;
	~CommitmentSigned() { CommitmentSigned_free(self); }
	CommitmentSigned(CommitmentSigned&& o) : self(o.self) { memset(&o, 0, sizeof(CommitmentSigned)); }
	CommitmentSigned(LDKCommitmentSigned&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCommitmentSigned)); }
	operator LDKCommitmentSigned() { LDKCommitmentSigned res = self; memset(&self, 0, sizeof(LDKCommitmentSigned)); return res; }
	LDKCommitmentSigned* operator &() { return &self; }
	LDKCommitmentSigned* operator ->() { return &self; }
	const LDKCommitmentSigned* operator &() const { return &self; }
	const LDKCommitmentSigned* operator ->() const { return &self; }
};
class RevokeAndACK {
private:
	LDKRevokeAndACK self;
public:
	RevokeAndACK(const RevokeAndACK&) = delete;
	~RevokeAndACK() { RevokeAndACK_free(self); }
	RevokeAndACK(RevokeAndACK&& o) : self(o.self) { memset(&o, 0, sizeof(RevokeAndACK)); }
	RevokeAndACK(LDKRevokeAndACK&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKRevokeAndACK)); }
	operator LDKRevokeAndACK() { LDKRevokeAndACK res = self; memset(&self, 0, sizeof(LDKRevokeAndACK)); return res; }
	LDKRevokeAndACK* operator &() { return &self; }
	LDKRevokeAndACK* operator ->() { return &self; }
	const LDKRevokeAndACK* operator &() const { return &self; }
	const LDKRevokeAndACK* operator ->() const { return &self; }
};
class UpdateFee {
private:
	LDKUpdateFee self;
public:
	UpdateFee(const UpdateFee&) = delete;
	~UpdateFee() { UpdateFee_free(self); }
	UpdateFee(UpdateFee&& o) : self(o.self) { memset(&o, 0, sizeof(UpdateFee)); }
	UpdateFee(LDKUpdateFee&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUpdateFee)); }
	operator LDKUpdateFee() { LDKUpdateFee res = self; memset(&self, 0, sizeof(LDKUpdateFee)); return res; }
	LDKUpdateFee* operator &() { return &self; }
	LDKUpdateFee* operator ->() { return &self; }
	const LDKUpdateFee* operator &() const { return &self; }
	const LDKUpdateFee* operator ->() const { return &self; }
};
class DataLossProtect {
private:
	LDKDataLossProtect self;
public:
	DataLossProtect(const DataLossProtect&) = delete;
	~DataLossProtect() { DataLossProtect_free(self); }
	DataLossProtect(DataLossProtect&& o) : self(o.self) { memset(&o, 0, sizeof(DataLossProtect)); }
	DataLossProtect(LDKDataLossProtect&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKDataLossProtect)); }
	operator LDKDataLossProtect() { LDKDataLossProtect res = self; memset(&self, 0, sizeof(LDKDataLossProtect)); return res; }
	LDKDataLossProtect* operator &() { return &self; }
	LDKDataLossProtect* operator ->() { return &self; }
	const LDKDataLossProtect* operator &() const { return &self; }
	const LDKDataLossProtect* operator ->() const { return &self; }
};
class ChannelReestablish {
private:
	LDKChannelReestablish self;
public:
	ChannelReestablish(const ChannelReestablish&) = delete;
	~ChannelReestablish() { ChannelReestablish_free(self); }
	ChannelReestablish(ChannelReestablish&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelReestablish)); }
	ChannelReestablish(LDKChannelReestablish&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelReestablish)); }
	operator LDKChannelReestablish() { LDKChannelReestablish res = self; memset(&self, 0, sizeof(LDKChannelReestablish)); return res; }
	LDKChannelReestablish* operator &() { return &self; }
	LDKChannelReestablish* operator ->() { return &self; }
	const LDKChannelReestablish* operator &() const { return &self; }
	const LDKChannelReestablish* operator ->() const { return &self; }
};
class AnnouncementSignatures {
private:
	LDKAnnouncementSignatures self;
public:
	AnnouncementSignatures(const AnnouncementSignatures&) = delete;
	~AnnouncementSignatures() { AnnouncementSignatures_free(self); }
	AnnouncementSignatures(AnnouncementSignatures&& o) : self(o.self) { memset(&o, 0, sizeof(AnnouncementSignatures)); }
	AnnouncementSignatures(LDKAnnouncementSignatures&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKAnnouncementSignatures)); }
	operator LDKAnnouncementSignatures() { LDKAnnouncementSignatures res = self; memset(&self, 0, sizeof(LDKAnnouncementSignatures)); return res; }
	LDKAnnouncementSignatures* operator &() { return &self; }
	LDKAnnouncementSignatures* operator ->() { return &self; }
	const LDKAnnouncementSignatures* operator &() const { return &self; }
	const LDKAnnouncementSignatures* operator ->() const { return &self; }
};
class NetAddress {
private:
	LDKNetAddress self;
public:
	NetAddress(const NetAddress&) = delete;
	~NetAddress() { NetAddress_free(self); }
	NetAddress(NetAddress&& o) : self(o.self) { memset(&o, 0, sizeof(NetAddress)); }
	NetAddress(LDKNetAddress&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKNetAddress)); }
	operator LDKNetAddress() { LDKNetAddress res = self; memset(&self, 0, sizeof(LDKNetAddress)); return res; }
	LDKNetAddress* operator &() { return &self; }
	LDKNetAddress* operator ->() { return &self; }
	const LDKNetAddress* operator &() const { return &self; }
	const LDKNetAddress* operator ->() const { return &self; }
};
class UnsignedNodeAnnouncement {
private:
	LDKUnsignedNodeAnnouncement self;
public:
	UnsignedNodeAnnouncement(const UnsignedNodeAnnouncement&) = delete;
	~UnsignedNodeAnnouncement() { UnsignedNodeAnnouncement_free(self); }
	UnsignedNodeAnnouncement(UnsignedNodeAnnouncement&& o) : self(o.self) { memset(&o, 0, sizeof(UnsignedNodeAnnouncement)); }
	UnsignedNodeAnnouncement(LDKUnsignedNodeAnnouncement&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUnsignedNodeAnnouncement)); }
	operator LDKUnsignedNodeAnnouncement() { LDKUnsignedNodeAnnouncement res = self; memset(&self, 0, sizeof(LDKUnsignedNodeAnnouncement)); return res; }
	LDKUnsignedNodeAnnouncement* operator &() { return &self; }
	LDKUnsignedNodeAnnouncement* operator ->() { return &self; }
	const LDKUnsignedNodeAnnouncement* operator &() const { return &self; }
	const LDKUnsignedNodeAnnouncement* operator ->() const { return &self; }
};
class NodeAnnouncement {
private:
	LDKNodeAnnouncement self;
public:
	NodeAnnouncement(const NodeAnnouncement&) = delete;
	~NodeAnnouncement() { NodeAnnouncement_free(self); }
	NodeAnnouncement(NodeAnnouncement&& o) : self(o.self) { memset(&o, 0, sizeof(NodeAnnouncement)); }
	NodeAnnouncement(LDKNodeAnnouncement&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKNodeAnnouncement)); }
	operator LDKNodeAnnouncement() { LDKNodeAnnouncement res = self; memset(&self, 0, sizeof(LDKNodeAnnouncement)); return res; }
	LDKNodeAnnouncement* operator &() { return &self; }
	LDKNodeAnnouncement* operator ->() { return &self; }
	const LDKNodeAnnouncement* operator &() const { return &self; }
	const LDKNodeAnnouncement* operator ->() const { return &self; }
};
class UnsignedChannelAnnouncement {
private:
	LDKUnsignedChannelAnnouncement self;
public:
	UnsignedChannelAnnouncement(const UnsignedChannelAnnouncement&) = delete;
	~UnsignedChannelAnnouncement() { UnsignedChannelAnnouncement_free(self); }
	UnsignedChannelAnnouncement(UnsignedChannelAnnouncement&& o) : self(o.self) { memset(&o, 0, sizeof(UnsignedChannelAnnouncement)); }
	UnsignedChannelAnnouncement(LDKUnsignedChannelAnnouncement&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUnsignedChannelAnnouncement)); }
	operator LDKUnsignedChannelAnnouncement() { LDKUnsignedChannelAnnouncement res = self; memset(&self, 0, sizeof(LDKUnsignedChannelAnnouncement)); return res; }
	LDKUnsignedChannelAnnouncement* operator &() { return &self; }
	LDKUnsignedChannelAnnouncement* operator ->() { return &self; }
	const LDKUnsignedChannelAnnouncement* operator &() const { return &self; }
	const LDKUnsignedChannelAnnouncement* operator ->() const { return &self; }
};
class ChannelAnnouncement {
private:
	LDKChannelAnnouncement self;
public:
	ChannelAnnouncement(const ChannelAnnouncement&) = delete;
	~ChannelAnnouncement() { ChannelAnnouncement_free(self); }
	ChannelAnnouncement(ChannelAnnouncement&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelAnnouncement)); }
	ChannelAnnouncement(LDKChannelAnnouncement&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelAnnouncement)); }
	operator LDKChannelAnnouncement() { LDKChannelAnnouncement res = self; memset(&self, 0, sizeof(LDKChannelAnnouncement)); return res; }
	LDKChannelAnnouncement* operator &() { return &self; }
	LDKChannelAnnouncement* operator ->() { return &self; }
	const LDKChannelAnnouncement* operator &() const { return &self; }
	const LDKChannelAnnouncement* operator ->() const { return &self; }
};
class UnsignedChannelUpdate {
private:
	LDKUnsignedChannelUpdate self;
public:
	UnsignedChannelUpdate(const UnsignedChannelUpdate&) = delete;
	~UnsignedChannelUpdate() { UnsignedChannelUpdate_free(self); }
	UnsignedChannelUpdate(UnsignedChannelUpdate&& o) : self(o.self) { memset(&o, 0, sizeof(UnsignedChannelUpdate)); }
	UnsignedChannelUpdate(LDKUnsignedChannelUpdate&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKUnsignedChannelUpdate)); }
	operator LDKUnsignedChannelUpdate() { LDKUnsignedChannelUpdate res = self; memset(&self, 0, sizeof(LDKUnsignedChannelUpdate)); return res; }
	LDKUnsignedChannelUpdate* operator &() { return &self; }
	LDKUnsignedChannelUpdate* operator ->() { return &self; }
	const LDKUnsignedChannelUpdate* operator &() const { return &self; }
	const LDKUnsignedChannelUpdate* operator ->() const { return &self; }
};
class ChannelUpdate {
private:
	LDKChannelUpdate self;
public:
	ChannelUpdate(const ChannelUpdate&) = delete;
	~ChannelUpdate() { ChannelUpdate_free(self); }
	ChannelUpdate(ChannelUpdate&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelUpdate)); }
	ChannelUpdate(LDKChannelUpdate&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelUpdate)); }
	operator LDKChannelUpdate() { LDKChannelUpdate res = self; memset(&self, 0, sizeof(LDKChannelUpdate)); return res; }
	LDKChannelUpdate* operator &() { return &self; }
	LDKChannelUpdate* operator ->() { return &self; }
	const LDKChannelUpdate* operator &() const { return &self; }
	const LDKChannelUpdate* operator ->() const { return &self; }
};
class QueryChannelRange {
private:
	LDKQueryChannelRange self;
public:
	QueryChannelRange(const QueryChannelRange&) = delete;
	~QueryChannelRange() { QueryChannelRange_free(self); }
	QueryChannelRange(QueryChannelRange&& o) : self(o.self) { memset(&o, 0, sizeof(QueryChannelRange)); }
	QueryChannelRange(LDKQueryChannelRange&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKQueryChannelRange)); }
	operator LDKQueryChannelRange() { LDKQueryChannelRange res = self; memset(&self, 0, sizeof(LDKQueryChannelRange)); return res; }
	LDKQueryChannelRange* operator &() { return &self; }
	LDKQueryChannelRange* operator ->() { return &self; }
	const LDKQueryChannelRange* operator &() const { return &self; }
	const LDKQueryChannelRange* operator ->() const { return &self; }
};
class ReplyChannelRange {
private:
	LDKReplyChannelRange self;
public:
	ReplyChannelRange(const ReplyChannelRange&) = delete;
	~ReplyChannelRange() { ReplyChannelRange_free(self); }
	ReplyChannelRange(ReplyChannelRange&& o) : self(o.self) { memset(&o, 0, sizeof(ReplyChannelRange)); }
	ReplyChannelRange(LDKReplyChannelRange&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKReplyChannelRange)); }
	operator LDKReplyChannelRange() { LDKReplyChannelRange res = self; memset(&self, 0, sizeof(LDKReplyChannelRange)); return res; }
	LDKReplyChannelRange* operator &() { return &self; }
	LDKReplyChannelRange* operator ->() { return &self; }
	const LDKReplyChannelRange* operator &() const { return &self; }
	const LDKReplyChannelRange* operator ->() const { return &self; }
};
class QueryShortChannelIds {
private:
	LDKQueryShortChannelIds self;
public:
	QueryShortChannelIds(const QueryShortChannelIds&) = delete;
	~QueryShortChannelIds() { QueryShortChannelIds_free(self); }
	QueryShortChannelIds(QueryShortChannelIds&& o) : self(o.self) { memset(&o, 0, sizeof(QueryShortChannelIds)); }
	QueryShortChannelIds(LDKQueryShortChannelIds&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKQueryShortChannelIds)); }
	operator LDKQueryShortChannelIds() { LDKQueryShortChannelIds res = self; memset(&self, 0, sizeof(LDKQueryShortChannelIds)); return res; }
	LDKQueryShortChannelIds* operator &() { return &self; }
	LDKQueryShortChannelIds* operator ->() { return &self; }
	const LDKQueryShortChannelIds* operator &() const { return &self; }
	const LDKQueryShortChannelIds* operator ->() const { return &self; }
};
class ReplyShortChannelIdsEnd {
private:
	LDKReplyShortChannelIdsEnd self;
public:
	ReplyShortChannelIdsEnd(const ReplyShortChannelIdsEnd&) = delete;
	~ReplyShortChannelIdsEnd() { ReplyShortChannelIdsEnd_free(self); }
	ReplyShortChannelIdsEnd(ReplyShortChannelIdsEnd&& o) : self(o.self) { memset(&o, 0, sizeof(ReplyShortChannelIdsEnd)); }
	ReplyShortChannelIdsEnd(LDKReplyShortChannelIdsEnd&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKReplyShortChannelIdsEnd)); }
	operator LDKReplyShortChannelIdsEnd() { LDKReplyShortChannelIdsEnd res = self; memset(&self, 0, sizeof(LDKReplyShortChannelIdsEnd)); return res; }
	LDKReplyShortChannelIdsEnd* operator &() { return &self; }
	LDKReplyShortChannelIdsEnd* operator ->() { return &self; }
	const LDKReplyShortChannelIdsEnd* operator &() const { return &self; }
	const LDKReplyShortChannelIdsEnd* operator ->() const { return &self; }
};
class GossipTimestampFilter {
private:
	LDKGossipTimestampFilter self;
public:
	GossipTimestampFilter(const GossipTimestampFilter&) = delete;
	~GossipTimestampFilter() { GossipTimestampFilter_free(self); }
	GossipTimestampFilter(GossipTimestampFilter&& o) : self(o.self) { memset(&o, 0, sizeof(GossipTimestampFilter)); }
	GossipTimestampFilter(LDKGossipTimestampFilter&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKGossipTimestampFilter)); }
	operator LDKGossipTimestampFilter() { LDKGossipTimestampFilter res = self; memset(&self, 0, sizeof(LDKGossipTimestampFilter)); return res; }
	LDKGossipTimestampFilter* operator &() { return &self; }
	LDKGossipTimestampFilter* operator ->() { return &self; }
	const LDKGossipTimestampFilter* operator &() const { return &self; }
	const LDKGossipTimestampFilter* operator ->() const { return &self; }
};
class ErrorAction {
private:
	LDKErrorAction self;
public:
	ErrorAction(const ErrorAction&) = delete;
	~ErrorAction() { ErrorAction_free(self); }
	ErrorAction(ErrorAction&& o) : self(o.self) { memset(&o, 0, sizeof(ErrorAction)); }
	ErrorAction(LDKErrorAction&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKErrorAction)); }
	operator LDKErrorAction() { LDKErrorAction res = self; memset(&self, 0, sizeof(LDKErrorAction)); return res; }
	LDKErrorAction* operator &() { return &self; }
	LDKErrorAction* operator ->() { return &self; }
	const LDKErrorAction* operator &() const { return &self; }
	const LDKErrorAction* operator ->() const { return &self; }
};
class LightningError {
private:
	LDKLightningError self;
public:
	LightningError(const LightningError&) = delete;
	~LightningError() { LightningError_free(self); }
	LightningError(LightningError&& o) : self(o.self) { memset(&o, 0, sizeof(LightningError)); }
	LightningError(LDKLightningError&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKLightningError)); }
	operator LDKLightningError() { LDKLightningError res = self; memset(&self, 0, sizeof(LDKLightningError)); return res; }
	LDKLightningError* operator &() { return &self; }
	LDKLightningError* operator ->() { return &self; }
	const LDKLightningError* operator &() const { return &self; }
	const LDKLightningError* operator ->() const { return &self; }
};
class CommitmentUpdate {
private:
	LDKCommitmentUpdate self;
public:
	CommitmentUpdate(const CommitmentUpdate&) = delete;
	~CommitmentUpdate() { CommitmentUpdate_free(self); }
	CommitmentUpdate(CommitmentUpdate&& o) : self(o.self) { memset(&o, 0, sizeof(CommitmentUpdate)); }
	CommitmentUpdate(LDKCommitmentUpdate&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKCommitmentUpdate)); }
	operator LDKCommitmentUpdate() { LDKCommitmentUpdate res = self; memset(&self, 0, sizeof(LDKCommitmentUpdate)); return res; }
	LDKCommitmentUpdate* operator &() { return &self; }
	LDKCommitmentUpdate* operator ->() { return &self; }
	const LDKCommitmentUpdate* operator &() const { return &self; }
	const LDKCommitmentUpdate* operator ->() const { return &self; }
};
class HTLCFailChannelUpdate {
private:
	LDKHTLCFailChannelUpdate self;
public:
	HTLCFailChannelUpdate(const HTLCFailChannelUpdate&) = delete;
	~HTLCFailChannelUpdate() { HTLCFailChannelUpdate_free(self); }
	HTLCFailChannelUpdate(HTLCFailChannelUpdate&& o) : self(o.self) { memset(&o, 0, sizeof(HTLCFailChannelUpdate)); }
	HTLCFailChannelUpdate(LDKHTLCFailChannelUpdate&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKHTLCFailChannelUpdate)); }
	operator LDKHTLCFailChannelUpdate() { LDKHTLCFailChannelUpdate res = self; memset(&self, 0, sizeof(LDKHTLCFailChannelUpdate)); return res; }
	LDKHTLCFailChannelUpdate* operator &() { return &self; }
	LDKHTLCFailChannelUpdate* operator ->() { return &self; }
	const LDKHTLCFailChannelUpdate* operator &() const { return &self; }
	const LDKHTLCFailChannelUpdate* operator ->() const { return &self; }
};
class ChannelMessageHandler {
private:
	LDKChannelMessageHandler self;
public:
	ChannelMessageHandler(const ChannelMessageHandler&) = delete;
	~ChannelMessageHandler() { ChannelMessageHandler_free(self); }
	ChannelMessageHandler(ChannelMessageHandler&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelMessageHandler)); }
	ChannelMessageHandler(LDKChannelMessageHandler&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelMessageHandler)); }
	operator LDKChannelMessageHandler() { LDKChannelMessageHandler res = self; memset(&self, 0, sizeof(LDKChannelMessageHandler)); return res; }
	LDKChannelMessageHandler* operator &() { return &self; }
	LDKChannelMessageHandler* operator ->() { return &self; }
	const LDKChannelMessageHandler* operator &() const { return &self; }
	const LDKChannelMessageHandler* operator ->() const { return &self; }
};
class RoutingMessageHandler {
private:
	LDKRoutingMessageHandler self;
public:
	RoutingMessageHandler(const RoutingMessageHandler&) = delete;
	~RoutingMessageHandler() { RoutingMessageHandler_free(self); }
	RoutingMessageHandler(RoutingMessageHandler&& o) : self(o.self) { memset(&o, 0, sizeof(RoutingMessageHandler)); }
	RoutingMessageHandler(LDKRoutingMessageHandler&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKRoutingMessageHandler)); }
	operator LDKRoutingMessageHandler() { LDKRoutingMessageHandler res = self; memset(&self, 0, sizeof(LDKRoutingMessageHandler)); return res; }
	LDKRoutingMessageHandler* operator &() { return &self; }
	LDKRoutingMessageHandler* operator ->() { return &self; }
	const LDKRoutingMessageHandler* operator &() const { return &self; }
	const LDKRoutingMessageHandler* operator ->() const { return &self; }
};
class MessageHandler {
private:
	LDKMessageHandler self;
public:
	MessageHandler(const MessageHandler&) = delete;
	~MessageHandler() { MessageHandler_free(self); }
	MessageHandler(MessageHandler&& o) : self(o.self) { memset(&o, 0, sizeof(MessageHandler)); }
	MessageHandler(LDKMessageHandler&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKMessageHandler)); }
	operator LDKMessageHandler() { LDKMessageHandler res = self; memset(&self, 0, sizeof(LDKMessageHandler)); return res; }
	LDKMessageHandler* operator &() { return &self; }
	LDKMessageHandler* operator ->() { return &self; }
	const LDKMessageHandler* operator &() const { return &self; }
	const LDKMessageHandler* operator ->() const { return &self; }
};
class SocketDescriptor {
private:
	LDKSocketDescriptor self;
public:
	SocketDescriptor(const SocketDescriptor&) = delete;
	~SocketDescriptor() { SocketDescriptor_free(self); }
	SocketDescriptor(SocketDescriptor&& o) : self(o.self) { memset(&o, 0, sizeof(SocketDescriptor)); }
	SocketDescriptor(LDKSocketDescriptor&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKSocketDescriptor)); }
	operator LDKSocketDescriptor() { LDKSocketDescriptor res = self; memset(&self, 0, sizeof(LDKSocketDescriptor)); return res; }
	LDKSocketDescriptor* operator &() { return &self; }
	LDKSocketDescriptor* operator ->() { return &self; }
	const LDKSocketDescriptor* operator &() const { return &self; }
	const LDKSocketDescriptor* operator ->() const { return &self; }
};
class PeerHandleError {
private:
	LDKPeerHandleError self;
public:
	PeerHandleError(const PeerHandleError&) = delete;
	~PeerHandleError() { PeerHandleError_free(self); }
	PeerHandleError(PeerHandleError&& o) : self(o.self) { memset(&o, 0, sizeof(PeerHandleError)); }
	PeerHandleError(LDKPeerHandleError&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPeerHandleError)); }
	operator LDKPeerHandleError() { LDKPeerHandleError res = self; memset(&self, 0, sizeof(LDKPeerHandleError)); return res; }
	LDKPeerHandleError* operator &() { return &self; }
	LDKPeerHandleError* operator ->() { return &self; }
	const LDKPeerHandleError* operator &() const { return &self; }
	const LDKPeerHandleError* operator ->() const { return &self; }
};
class PeerManager {
private:
	LDKPeerManager self;
public:
	PeerManager(const PeerManager&) = delete;
	~PeerManager() { PeerManager_free(self); }
	PeerManager(PeerManager&& o) : self(o.self) { memset(&o, 0, sizeof(PeerManager)); }
	PeerManager(LDKPeerManager&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPeerManager)); }
	operator LDKPeerManager() { LDKPeerManager res = self; memset(&self, 0, sizeof(LDKPeerManager)); return res; }
	LDKPeerManager* operator &() { return &self; }
	LDKPeerManager* operator ->() { return &self; }
	const LDKPeerManager* operator &() const { return &self; }
	const LDKPeerManager* operator ->() const { return &self; }
};
class TxCreationKeys {
private:
	LDKTxCreationKeys self;
public:
	TxCreationKeys(const TxCreationKeys&) = delete;
	~TxCreationKeys() { TxCreationKeys_free(self); }
	TxCreationKeys(TxCreationKeys&& o) : self(o.self) { memset(&o, 0, sizeof(TxCreationKeys)); }
	TxCreationKeys(LDKTxCreationKeys&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKTxCreationKeys)); }
	operator LDKTxCreationKeys() { LDKTxCreationKeys res = self; memset(&self, 0, sizeof(LDKTxCreationKeys)); return res; }
	LDKTxCreationKeys* operator &() { return &self; }
	LDKTxCreationKeys* operator ->() { return &self; }
	const LDKTxCreationKeys* operator &() const { return &self; }
	const LDKTxCreationKeys* operator ->() const { return &self; }
};
class PreCalculatedTxCreationKeys {
private:
	LDKPreCalculatedTxCreationKeys self;
public:
	PreCalculatedTxCreationKeys(const PreCalculatedTxCreationKeys&) = delete;
	~PreCalculatedTxCreationKeys() { PreCalculatedTxCreationKeys_free(self); }
	PreCalculatedTxCreationKeys(PreCalculatedTxCreationKeys&& o) : self(o.self) { memset(&o, 0, sizeof(PreCalculatedTxCreationKeys)); }
	PreCalculatedTxCreationKeys(LDKPreCalculatedTxCreationKeys&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKPreCalculatedTxCreationKeys)); }
	operator LDKPreCalculatedTxCreationKeys() { LDKPreCalculatedTxCreationKeys res = self; memset(&self, 0, sizeof(LDKPreCalculatedTxCreationKeys)); return res; }
	LDKPreCalculatedTxCreationKeys* operator &() { return &self; }
	LDKPreCalculatedTxCreationKeys* operator ->() { return &self; }
	const LDKPreCalculatedTxCreationKeys* operator &() const { return &self; }
	const LDKPreCalculatedTxCreationKeys* operator ->() const { return &self; }
};
class ChannelPublicKeys {
private:
	LDKChannelPublicKeys self;
public:
	ChannelPublicKeys(const ChannelPublicKeys&) = delete;
	~ChannelPublicKeys() { ChannelPublicKeys_free(self); }
	ChannelPublicKeys(ChannelPublicKeys&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelPublicKeys)); }
	ChannelPublicKeys(LDKChannelPublicKeys&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelPublicKeys)); }
	operator LDKChannelPublicKeys() { LDKChannelPublicKeys res = self; memset(&self, 0, sizeof(LDKChannelPublicKeys)); return res; }
	LDKChannelPublicKeys* operator &() { return &self; }
	LDKChannelPublicKeys* operator ->() { return &self; }
	const LDKChannelPublicKeys* operator &() const { return &self; }
	const LDKChannelPublicKeys* operator ->() const { return &self; }
};
class HTLCOutputInCommitment {
private:
	LDKHTLCOutputInCommitment self;
public:
	HTLCOutputInCommitment(const HTLCOutputInCommitment&) = delete;
	~HTLCOutputInCommitment() { HTLCOutputInCommitment_free(self); }
	HTLCOutputInCommitment(HTLCOutputInCommitment&& o) : self(o.self) { memset(&o, 0, sizeof(HTLCOutputInCommitment)); }
	HTLCOutputInCommitment(LDKHTLCOutputInCommitment&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKHTLCOutputInCommitment)); }
	operator LDKHTLCOutputInCommitment() { LDKHTLCOutputInCommitment res = self; memset(&self, 0, sizeof(LDKHTLCOutputInCommitment)); return res; }
	LDKHTLCOutputInCommitment* operator &() { return &self; }
	LDKHTLCOutputInCommitment* operator ->() { return &self; }
	const LDKHTLCOutputInCommitment* operator &() const { return &self; }
	const LDKHTLCOutputInCommitment* operator ->() const { return &self; }
};
class HolderCommitmentTransaction {
private:
	LDKHolderCommitmentTransaction self;
public:
	HolderCommitmentTransaction(const HolderCommitmentTransaction&) = delete;
	~HolderCommitmentTransaction() { HolderCommitmentTransaction_free(self); }
	HolderCommitmentTransaction(HolderCommitmentTransaction&& o) : self(o.self) { memset(&o, 0, sizeof(HolderCommitmentTransaction)); }
	HolderCommitmentTransaction(LDKHolderCommitmentTransaction&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKHolderCommitmentTransaction)); }
	operator LDKHolderCommitmentTransaction() { LDKHolderCommitmentTransaction res = self; memset(&self, 0, sizeof(LDKHolderCommitmentTransaction)); return res; }
	LDKHolderCommitmentTransaction* operator &() { return &self; }
	LDKHolderCommitmentTransaction* operator ->() { return &self; }
	const LDKHolderCommitmentTransaction* operator &() const { return &self; }
	const LDKHolderCommitmentTransaction* operator ->() const { return &self; }
};
class InitFeatures {
private:
	LDKInitFeatures self;
public:
	InitFeatures(const InitFeatures&) = delete;
	~InitFeatures() { InitFeatures_free(self); }
	InitFeatures(InitFeatures&& o) : self(o.self) { memset(&o, 0, sizeof(InitFeatures)); }
	InitFeatures(LDKInitFeatures&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKInitFeatures)); }
	operator LDKInitFeatures() { LDKInitFeatures res = self; memset(&self, 0, sizeof(LDKInitFeatures)); return res; }
	LDKInitFeatures* operator &() { return &self; }
	LDKInitFeatures* operator ->() { return &self; }
	const LDKInitFeatures* operator &() const { return &self; }
	const LDKInitFeatures* operator ->() const { return &self; }
};
class NodeFeatures {
private:
	LDKNodeFeatures self;
public:
	NodeFeatures(const NodeFeatures&) = delete;
	~NodeFeatures() { NodeFeatures_free(self); }
	NodeFeatures(NodeFeatures&& o) : self(o.self) { memset(&o, 0, sizeof(NodeFeatures)); }
	NodeFeatures(LDKNodeFeatures&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKNodeFeatures)); }
	operator LDKNodeFeatures() { LDKNodeFeatures res = self; memset(&self, 0, sizeof(LDKNodeFeatures)); return res; }
	LDKNodeFeatures* operator &() { return &self; }
	LDKNodeFeatures* operator ->() { return &self; }
	const LDKNodeFeatures* operator &() const { return &self; }
	const LDKNodeFeatures* operator ->() const { return &self; }
};
class ChannelFeatures {
private:
	LDKChannelFeatures self;
public:
	ChannelFeatures(const ChannelFeatures&) = delete;
	~ChannelFeatures() { ChannelFeatures_free(self); }
	ChannelFeatures(ChannelFeatures&& o) : self(o.self) { memset(&o, 0, sizeof(ChannelFeatures)); }
	ChannelFeatures(LDKChannelFeatures&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKChannelFeatures)); }
	operator LDKChannelFeatures() { LDKChannelFeatures res = self; memset(&self, 0, sizeof(LDKChannelFeatures)); return res; }
	LDKChannelFeatures* operator &() { return &self; }
	LDKChannelFeatures* operator ->() { return &self; }
	const LDKChannelFeatures* operator &() const { return &self; }
	const LDKChannelFeatures* operator ->() const { return &self; }
};
class RouteHop {
private:
	LDKRouteHop self;
public:
	RouteHop(const RouteHop&) = delete;
	~RouteHop() { RouteHop_free(self); }
	RouteHop(RouteHop&& o) : self(o.self) { memset(&o, 0, sizeof(RouteHop)); }
	RouteHop(LDKRouteHop&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKRouteHop)); }
	operator LDKRouteHop() { LDKRouteHop res = self; memset(&self, 0, sizeof(LDKRouteHop)); return res; }
	LDKRouteHop* operator &() { return &self; }
	LDKRouteHop* operator ->() { return &self; }
	const LDKRouteHop* operator &() const { return &self; }
	const LDKRouteHop* operator ->() const { return &self; }
};
class Route {
private:
	LDKRoute self;
public:
	Route(const Route&) = delete;
	~Route() { Route_free(self); }
	Route(Route&& o) : self(o.self) { memset(&o, 0, sizeof(Route)); }
	Route(LDKRoute&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKRoute)); }
	operator LDKRoute() { LDKRoute res = self; memset(&self, 0, sizeof(LDKRoute)); return res; }
	LDKRoute* operator &() { return &self; }
	LDKRoute* operator ->() { return &self; }
	const LDKRoute* operator &() const { return &self; }
	const LDKRoute* operator ->() const { return &self; }
};
class RouteHint {
private:
	LDKRouteHint self;
public:
	RouteHint(const RouteHint&) = delete;
	~RouteHint() { RouteHint_free(self); }
	RouteHint(RouteHint&& o) : self(o.self) { memset(&o, 0, sizeof(RouteHint)); }
	RouteHint(LDKRouteHint&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKRouteHint)); }
	operator LDKRouteHint() { LDKRouteHint res = self; memset(&self, 0, sizeof(LDKRouteHint)); return res; }
	LDKRouteHint* operator &() { return &self; }
	LDKRouteHint* operator ->() { return &self; }
	const LDKRouteHint* operator &() const { return &self; }
	const LDKRouteHint* operator ->() const { return &self; }
};
class NetworkGraph {
private:
	LDKNetworkGraph self;
public:
	NetworkGraph(const NetworkGraph&) = delete;
	~NetworkGraph() { NetworkGraph_free(self); }
	NetworkGraph(NetworkGraph&& o) : self(o.self) { memset(&o, 0, sizeof(NetworkGraph)); }
	NetworkGraph(LDKNetworkGraph&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKNetworkGraph)); }
	operator LDKNetworkGraph() { LDKNetworkGraph res = self; memset(&self, 0, sizeof(LDKNetworkGraph)); return res; }
	LDKNetworkGraph* operator &() { return &self; }
	LDKNetworkGraph* operator ->() { return &self; }
	const LDKNetworkGraph* operator &() const { return &self; }
	const LDKNetworkGraph* operator ->() const { return &self; }
};
class LockedNetworkGraph {
private:
	LDKLockedNetworkGraph self;
public:
	LockedNetworkGraph(const LockedNetworkGraph&) = delete;
	~LockedNetworkGraph() { LockedNetworkGraph_free(self); }
	LockedNetworkGraph(LockedNetworkGraph&& o) : self(o.self) { memset(&o, 0, sizeof(LockedNetworkGraph)); }
	LockedNetworkGraph(LDKLockedNetworkGraph&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKLockedNetworkGraph)); }
	operator LDKLockedNetworkGraph() { LDKLockedNetworkGraph res = self; memset(&self, 0, sizeof(LDKLockedNetworkGraph)); return res; }
	LDKLockedNetworkGraph* operator &() { return &self; }
	LDKLockedNetworkGraph* operator ->() { return &self; }
	const LDKLockedNetworkGraph* operator &() const { return &self; }
	const LDKLockedNetworkGraph* operator ->() const { return &self; }
};
class NetGraphMsgHandler {
private:
	LDKNetGraphMsgHandler self;
public:
	NetGraphMsgHandler(const NetGraphMsgHandler&) = delete;
	~NetGraphMsgHandler() { NetGraphMsgHandler_free(self); }
	NetGraphMsgHandler(NetGraphMsgHandler&& o) : self(o.self) { memset(&o, 0, sizeof(NetGraphMsgHandler)); }
	NetGraphMsgHandler(LDKNetGraphMsgHandler&& m_self) : self(m_self) { memset(&m_self, 0, sizeof(LDKNetGraphMsgHandler)); }
	operator LDKNetGraphMsgHandler() { LDKNetGraphMsgHandler res = self; memset(&self, 0, sizeof(LDKNetGraphMsgHandler)); return res; }
	LDKNetGraphMsgHandler* operator &() { return &self; }
	LDKNetGraphMsgHandler* operator ->() { return &self; }
	const LDKNetGraphMsgHandler* operator &() const { return &self; }
	const LDKNetGraphMsgHandler* operator ->() const { return &self; }
};
