import React, { Component } from 'react';
import { Grid } from 'react-bootstrap';
import { Col } from 'react-bootstrap';
import { Row } from 'react-bootstrap';
import { PageHeader } from 'react-bootstrap';
import { ListGroup } from 'react-bootstrap';
import { ListGroupItem } from 'react-bootstrap';
import './Headlines.css';

function getCurrentEvents() {
  var events = [{
    "title": "Title",
    "summary": "Summary",
    "articleMoods": [{
      source: "Source name",
      mood: 5
    }],
    date: "9:57PM"
  }, {
    "title": "Title",
    "summary": "Summary",
    "articleMoods": [{
      source: "Source name",
      mood: 5 
    }],
    date: "9:57PM"
  }, {
    "title": "Title",
    "summary": "Summary",
    "articleMoods": [{
      source: "Source name",
      mood: 5
    }],
    date: "9:57PM"
  }];

  return events;
}

var events = getCurrentEvents();

const gridInstance = (
  <Grid>
    <Row className="show-grid">
      <Col xs={12} md={8}></Col>
      <Col xs={6} md={4}></Col>
    </Row>
  </Grid>
);

const pageHeaderInstance = (
  <PageHeader>Headlines <small>from various news sources</small></PageHeader>
);

const listgroupInstance = (
  <ListGroup>

    <ListGroupItem header="Heading 1">Lorem ipsum dolor sit amet, consectetur 
    adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna 
    aliqua. Ac odio tempor orci dapibus. Massa ultricies mi quis hendrerit dolor 
    magna eget. Pulvinar etiam non quam lacus suspendisse faucibus interdum. 
    Ac odio tempor orci dapibus ultrices in iaculis nunc.</ListGroupItem>

    <ListGroupItem header="Heading 2">Lorem ipsum dolor sit amet, 
    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et 
    dolore magna aliqua. Ac odio tempor orci dapibus. Massa ultricies mi quis 
    hendrerit dolor magna eget. Pulvinar etiam non quam lacus suspendisse 
    faucibus interdum. Ac odio tempor orci dapibus ultrices in iaculis nunc.</ListGroupItem>
    
    <ListGroupItem header="Heading 3">Lorem ipsum dolor sit 
    amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut 
    labore et dolore magna aliqua. Ac odio tempor orci dapibus. Massa ultricies 
    mi quis hendrerit dolor magna eget. Pulvinar etiam non quam lacus 
    suspendisse faucibus interdum. Ac odio tempor orci dapibus ultrices in 
    iaculis nunc.</ListGroupItem>

    <ListGroupItem header="Heading 4">Lorem ipsum dolor sit 
    amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut 
    labore et dolore magna aliqua. Ac odio tempor orci dapibus. Massa ultricies 
    mi quis hendrerit dolor magna eget. Pulvinar etiam non quam lacus 
    suspendisse faucibus interdum. Ac odio tempor orci dapibus ultrices in 
    iaculis nunc.</ListGroupItem>

    <ListGroupItem header="Heading 5">Lorem ipsum dolor sit 
    amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut 
    labore et dolore magna aliqua. Ac odio tempor orci dapibus. Massa ultricies 
    mi quis hendrerit dolor magna eget. Pulvinar etiam non quam lacus 
    suspendisse faucibus interdum. Ac odio tempor orci dapibus ultrices in 
    iaculis nunc.</ListGroupItem>

  </ListGroup>
);

class Headlines extends Component {
  render() {
    return ( 
      <div>
        {pageHeaderInstance}
        {listgroupInstance}
      </div>
    );
  }
}

export default Headlines;