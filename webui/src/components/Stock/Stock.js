import React, { Component } from 'react';
import { Button } from 'react-bootstrap';
import { PageHeader } from 'react-bootstrap';
import { Image } from 'react-bootstrap';
import { ListGroup } from 'react-bootstrap';
import { ListGroupItem } from 'react-bootstrap';
import './Stock.css';
import axios from 'axios';
import {LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend} from 'recharts';

function getStockPrice() {
	return { "stock" : 50 };
}

var stock = getStockPrice();

const pageHeaderInstance = (
  <PageHeader>Stock <small>current stock information</small></PageHeader>
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
  constructor(props) {
    super(props);

    this.state = {
      y_domain: [15, 20],
      stock_data: [],
      current_price: [0]
    };
  }
  componentDidMount() {
    axios.get(`feed/finance.json`)
    .then(res => {
      const stock_data = res.data;
      this.setState({ stock_data });
			const current_price = [res.data[res.data.length-1]];
      this.setState({ current_price });
      var min_val = res.data[0].open > res.data[0].close ? res.data[0].close : res.data[0].open;
      var max_val = res.data[0].open > res.data[0].close ? res.data[0].open : res.data[0].close;
      for (var sd in stock_data) {
        var me = stock_data[sd];
        if (me.close > max_val) {
          max_val = me.close;
        }
        if (me.close < min_val) {
          min_val = me.close;
        }
        if (me.open > max_val) {
          max_val = me.open;
        }
        if (me.open < min_val) {
          min_val = me.open;
        }
      }
      const min_y = min_val - min_val*.0005;
      const max_y = max_val + max_val*.0005;
      const y_domain = [min_y, max_y]
      this.setState({ y_domain });
    });
  }
	render() {
		return (
			<div>
				{pageHeaderInstance}
				{this.state.current_price.map(cur =>
						<h2 className="stockPrice">Current stock: {cur.close} USD</h2>
				)}

			  <br />
			  <br />
        <LineChart width={600} height={300} data={this.state.stock_data}
            margin={{top: 5, right: 30, left: 20, bottom: 5}}>
       <XAxis dataKey="time"/>
       <YAxis domain={this.state.y_domain} />
       <CartesianGrid strokeDasharray="3 3"/>
       <Tooltip/>
       <Legend />
       <Line type="monotone" dataKey="open" stroke="#8884d8" activeDot={{r: 8}}/>
       <Line type="monotone" dataKey="close" stroke="#82ca9d" />
      </LineChart>   
			  <br />
			  <br />
			  <br />
			</div>
		);
	}
}

export default Stock;