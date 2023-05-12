import React from 'react';
import { useInkathon } from '@scio-labs/use-inkathon';

function ConnectButton() {
  const { connect, isConnecting, activeAccount } = useInkathon();

  const handleConnect = async () => {
    if (!activeAccount) {
      await connect();
    }
  };

  return (
    <button onClick={handleConnect} disabled={isConnecting}>
      {isConnecting ? 'Connecting...' : activeAccount ? 'Connected' : 'Connect Wallet'}
    </button>
  );
}

export default ConnectButton;
