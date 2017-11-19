import React, { Component } from 'react';
import { Button } from 'react-bootstrap';
import { PageHeader } from 'react-bootstrap';
import { Image } from 'react-bootstrap';
import { ListGroup } from 'react-bootstrap';
import { ListGroupItem } from 'react-bootstrap';
import sampleStock from '../../assets/stock.png';
import './Stock.css';

function getStockPrice() {
	return { "stock" : 50 };
}

var stock = getStockPrice();

const pageHeaderInstance = (
  <PageHeader>Stock <small>current stock information</small></PageHeader>
);

const imageResponsiveInstance = (
  <Image src={sampleStock} responsive />
);

const listgroupInstance = (
  <ListGroup>
    <ListGroupItem header="Financials">
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed 
    do eiusmod tempor incididunt ut labore et dolore magna aliqua. Volutpat 
    consequat mauris nunc congue nisi vitae suscipit. Aliquam vestibulum morbi 
    blandit cursus risus. Odio ut enim blandit volutpat maecenas.</ListGroupItem>
  </ListGroup>
);

class Stock extends Component {
	render() {
		return (
			<div>
			  {pageHeaderInstance}
			  <h2 className="stockPrice">Current stock: 18.26 USD</h2>
			  <br />
			  <br />
			  {imageResponsiveInstance}
			  <br />
			  <br />
			  <br />
			  {listgroupInstance}
			</div>
		);
	}
}

export default Stock;