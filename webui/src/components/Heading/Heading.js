import React, { Component } from 'react';
import { Jumbotron } from 'react-bootstrap'
import logo from '../../assets/GELogo.png';

import './Heading.css';

const jumbotronInstance = (
  <Jumbotron className="Jumbotron">
    <img src={logo} className="App-logo" alt="logo" />
    <h2>GE Alexa Flash Briefing</h2>
  </Jumbotron>
);

class Heading extends Component {
  render() {
    return jumbotronInstance;
  }
}

export default Heading;