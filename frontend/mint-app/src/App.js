import React from 'react';
import Mint from './mint';
import ConnectButton from './connectButton';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>Welcome to the Mint App</h1>
      </header>
      <main>
        <ConnectButton />
        <Mint />
      </main>
    </div>
  );
}

export default App;
