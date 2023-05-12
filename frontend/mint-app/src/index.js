import React from 'react';
import ReactDOM from 'react-dom';
import { UseInkathonProvider, shibuya } from '@scio-labs/use-inkathon';
import App from './App';

async function getDeployments() {
  const abi = await import('./nft.json'); // replace with the path to your ABI JSON file
  return [
    {
      contractId: 'nft_contract',
      networkId: shibuya.network,
      abi,
      address: 'Zcr6iqzzcwq8vHnFqwaGrKJ1PrqGDNUGGd5UQ5w1eimjT7S', // replace with your contract's address
    },
  ];
}

ReactDOM.render(
  <React.StrictMode>
    <UseInkathonProvider
      appName="mint-app"
      defaultChain={shibuya}
      deployments={getDeployments()}
    >
      <App />
    </UseInkathonProvider>
  </React.StrictMode>,
  document.getElementById('root')
);
