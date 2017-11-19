import React, { Component } from 'react';
import './Stock.css';

function getStockPrice() {
	return { "stock" : 50 };
}

var stock = getStockPrice();

class Stock extends Component {
	render() {
		return (
			<div className="Stock">
				{stock.stock}
			</div>
		)
	}
}

export default Stock;