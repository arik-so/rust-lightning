FILTER_MESSAGES = True
SUPPORTED_MESSAGES = ['Ping', 'Pong', 'OpenChannel', 'AcceptChannel', 'FundingCreated', 'FundingSigned',
					  'FundingLocked', 'Shutdown', 'ClosingSigned',
					  # 'UpdateAddHTLC' # OnionPacket is hard to convert
					  'UpdateFulfillHTLC',
					  # 'UpdateFailHTLC', # OnionErrorPacket is hard to convert
					  'UpdateFailMalformedHTLC',
					  'CommitmentSigned'
					  ]
