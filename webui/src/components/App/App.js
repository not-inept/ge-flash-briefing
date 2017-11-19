import React, { Component } from 'react';
import Heading from '../Heading/Heading';
import Headlines from '../Headlines/Headlines';
import Stock from '../Stock/Stock';
import logo from '../../assets/GELogo.png';
import { Grid } from 'react-bootstrap';
import { Col } from 'react-bootstrap';
import { Row } from 'react-bootstrap';
import './App.css';


const gridInstance = (
  <Grid fluid={false}>
    <Row className="show-grid">

      <Col xs={12} sm={10} md={8} lg={7}><Headlines /></Col>
      <Col xs={6} sm={6} md={4} lg={5}><Stock /></Col>
    </Row>
  </Grid>
);


class App extends Component {
  render() {
    return (
      <div className="App">
        <Heading />
        {gridInstance}
      </div>
    );
  }
}

export default App;
