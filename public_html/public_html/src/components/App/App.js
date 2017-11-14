import React, { Component } from 'react';
import Headlines from '../Headlines/Headlines';
import Stock from '../Stock/Stock';
import logo from '../../assets/GELogo.png';
import './App.css';

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h1 className="App-title">GE Alexa Flash Briefing</h1>
        </header>
        <p className="App-intro">
          To get started, edit <code>src/App.js</code> and save to reload.
        </p>
        <Headlines />
        <Stock />
      </div>
    );
  }
}

export default App;
