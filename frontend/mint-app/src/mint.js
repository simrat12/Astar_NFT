import { useState } from 'react';
import { useInkathon, useRegisteredContract } from '@scio-labs/use-inkathon';

function Mint() {
  const { connect, activeAccount } = useInkathon();
  const { contract } = useRegisteredContract('nft_contract');
  const [amount, setAmount] = useState(0);

  const mintTokens = async () => {
    if (!activeAccount) {
      await connect();
    }
    if (activeAccount) {
      const gasLimit = await contract.estimate('mint', activeAccount, amount);
      await contract.contractTx('mint', gasLimit, activeAccount, amount);
    }
  };

  return (
    <div>
      <input type="number" value={amount} onChange={e => setAmount(e.target.value)} />
      <button onClick={mintTokens}>Mint Tokens</button>
    </div>
  );
}

export default Mint;
